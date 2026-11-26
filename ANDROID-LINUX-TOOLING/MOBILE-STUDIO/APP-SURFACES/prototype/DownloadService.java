package live.vivim.surfaceprobe;
import android.app.*;
import android.content.*;
import android.content.pm.ServiceInfo;
import android.net.Uri;
import android.os.*;
import org.json.*;
import java.io.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicBoolean;

/** App-private owner-controlled foreground engine. No exported service or network control API. */
public final class DownloadService extends Service {
    static final String OP="operation", REPLY="reply";
    private static final String CHANNEL="vivim-native-downloads";
    private static final int NOTIFICATION=42101, MAX_METAINFO=1024*1024;
    private static final AtomicBoolean CLOSING=new AtomicBoolean();
    static volatile boolean active=false;
    static final boolean available;
    static { boolean ready; try{System.loadLibrary("vivim_download_core");ready=true;}catch(LinkageError e){ready=false;} available=ready; }
    private static native String nativeOpen(String root);
    private static native String nativeDispatch(String command);
    private static native void nativeInterrupt();
    private static native void nativeClose();
    private final ExecutorService executor=Executors.newSingleThreadExecutor();
    private volatile boolean stopped;
    private boolean ownsEngine;
    @Override public IBinder onBind(Intent intent){return null;}
    @Override public int onStartCommand(Intent intent,int flags,int startId){
        if(intent==null){stopSelf();return START_NOT_STICKY;}
        final String op=intent.getStringExtra(OP);
        final ResultReceiver reply=intent.getParcelableExtra(REPLY);
        if("stop".equals(op)){
            stopped=true;active=false;
            if(ownsEngine&&available)nativeInterrupt();
            send(reply,"Download engine stopping");stopSelf();return START_NOT_STICKY;
        }
        if(!available||CLOSING.get()){
            send(reply,"Native engine unavailable or previous session closing");stopSelf();return START_NOT_STICKY;
        }
        if(!ownsEngine&&!"start".equals(op)){
            send(reply,"Start the engine explicitly first");stopSelf();return START_NOT_STICKY;
        }
        if(!ownsEngine&&"start".equals(op)){
            try{promote();ownsEngine=true;}
            catch(RuntimeException e){send(reply,"Foreground service denied: "+e.getClass().getSimpleName());stopSelf();return START_NOT_STICKY;}
        }
        executor.execute(()->{
            if(stopped)return;
            try{
                String result;
                if("start".equals(op)){
                    result=nativeOpen(new File(getFilesDir(),"native-downloads").getAbsolutePath());
                    active=!stopped&&result!=null&&new JSONObject(result).optBoolean("ok");
                    if(!active){send(reply,"Native initialization failed or stopped");stopSelf();return;}
                }else{
                    if(!active){send(reply,"Engine not active");return;}
                    JSONObject command=new JSONObject().put("op",op);
                    if("add_magnet".equals(op))command.put("magnet",intent.getStringExtra("magnet"));
                    else if("add_file".equals(op))command.put("metainfo",readMetainfo(intent.getData()));
                    else if("pause".equals(op)||"resume".equals(op)||"remove".equals(op))command.put("hash",intent.getStringExtra("hash"));
                    else if(!"list".equals(op)){send(reply,"Unknown download operation");return;}
                    result=nativeDispatch(command.toString());
                }
                send(reply,result==null?"Native operation returned no receipt":result);
            }catch(Exception e){send(reply,"Download operation failed: "+e.getClass().getSimpleName());}
        });
        return START_NOT_STICKY;
    }
    private JSONArray readMetainfo(Uri uri)throws Exception{
        if(uri==null||!"content".equals(uri.getScheme()))throw new IOException("SAF content URI required");
        JSONArray bytes=new JSONArray();
        try(InputStream stream=getContentResolver().openInputStream(uri)){
            if(stream==null)throw new IOException("No stream");
            byte[] block=new byte[8192];int n,total=0;
            while((n=stream.read(block))!=-1){
                total+=n;if(total>MAX_METAINFO)throw new IOException("Torrent file too large");
                for(int i=0;i<n;i++)bytes.put(block[i]&255);
            }
        }
        return bytes;
    }
    private void promote(){
        NotificationManager nm=getSystemService(NotificationManager.class);
        nm.createNotificationChannel(new NotificationChannel(CHANNEL,"VIVIM Downloads",NotificationManager.IMPORTANCE_LOW));
        PendingIntent action=PendingIntent.getService(this,1,new Intent(this,DownloadService.class).putExtra(OP,"stop"),PendingIntent.FLAG_IMMUTABLE|PendingIntent.FLAG_UPDATE_CURRENT);
        PendingIntent content=PendingIntent.getActivity(this,2,new Intent(this,DownloadActivity.class),PendingIntent.FLAG_IMMUTABLE|PendingIntent.FLAG_UPDATE_CURRENT);
        Notification n=new Notification.Builder(this,CHANNEL).setSmallIcon(android.R.drawable.stat_sys_download)
          .setContentTitle("VIVIM native downloads").setContentText("Local BitTorrent engine active; peers may upload and download")
          .setOngoing(true).setContentIntent(content).addAction(android.R.drawable.ic_menu_close_clear_cancel,"Stop",action).build();
        if(Build.VERSION.SDK_INT>=29)startForeground(NOTIFICATION,n,ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC);
        else startForeground(NOTIFICATION,n);
    }
    private void send(ResultReceiver receiver,String result){
        if(receiver==null)return;Bundle b=new Bundle();b.putString("result",result);receiver.send(0,b);
    }
    @Override public void onTimeout(int startId,int fgsType){
        stopped=true;active=false;if(ownsEngine&&available)nativeInterrupt();stopSelf();
    }
    @Override public void onDestroy(){
        stopped=true;active=false;
        if(ownsEngine&&available){
            CLOSING.set(true);nativeInterrupt();
            executor.execute(()->{try{nativeClose();}finally{CLOSING.set(false);}});
        }
        executor.shutdown();stopForeground(STOP_FOREGROUND_REMOVE);super.onDestroy();
    }
}
