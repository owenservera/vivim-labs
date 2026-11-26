package live.vivim.surfaceprobe;

import android.app.*;
import android.content.*;
import android.content.pm.ServiceInfo;
import android.graphics.*;
import android.hardware.display.*;
import android.media.*;
import android.media.projection.*;
import android.os.*;
import java.nio.ByteBuffer;

/** One explicitly authorized snapshot, memory only. No network, automatic upload or persistent capture. */
public final class SnapshotService extends Service {
    static Bitmap snapshot;
    static String result="No snapshot yet";
    static boolean active;
    private MediaProjection projection;
    private VirtualDisplay display;
    private ImageReader reader;
    private final Handler main=new Handler(Looper.getMainLooper());
    private boolean visible, finished;
    private long readyAt;
    private int width,height,density;
    private final MediaProjection.Callback callback=new MediaProjection.Callback(){
        @Override public void onStop(){ finishCapture("Projection stopped by Android"); }
        @Override public void onCapturedContentVisibilityChanged(boolean v){ visible=v; }
        @Override public void onCapturedContentResize(int w,int h){
            if(display==null || finished || w<=0 || h<=0) return;
            try { configureReader(w,h); } catch(RuntimeException e){ finishCapture("Resize unavailable"); }
        }
    };
    static void clear(){ if(snapshot!=null){ snapshot.recycle(); snapshot=null; } result="No snapshot yet"; }
    @Override public IBinder onBind(Intent i){return null;}
    @Override public int onStartCommand(Intent i,int flags,int id){
        if(i!=null && "stop".equals(i.getAction())){finishCapture("Stopped by user");return START_NOT_STICKY;}
        if(i==null || active){return START_NOT_STICKY;}
        clear(); active=true; finished=false; visible=Build.VERSION.SDK_INT<34;
        try{
            NotificationManager nm=getSystemService(NotificationManager.class);
            nm.createNotificationChannel(new NotificationChannel("snapshot","One-shot screen observation",NotificationManager.IMPORTANCE_LOW));
            PendingIntent stop=PendingIntent.getService(this,1,new Intent(this,SnapshotService.class).setAction("stop"),PendingIntent.FLAG_IMMUTABLE);
            Notification note=new Notification.Builder(this,"snapshot").setSmallIcon(android.R.drawable.ic_menu_camera)
                .setContentTitle("VIVIM: one authorized snapshot").setContentText("Capturing selected app once; stop anytime")
                .setOngoing(true).addAction(android.R.drawable.ic_menu_close_clear_cancel,"Stop",stop).build();
            if(Build.VERSION.SDK_INT>=29) startForeground(41,note,ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION);
            else startForeground(41,note);
            Intent data=i.getParcelableExtra("consent");
            if(data==null){ finishCapture("No consent token"); return START_NOT_STICKY; }
            projection=getSystemService(MediaProjectionManager.class).getMediaProjection(Activity.RESULT_OK,data);
            if(projection==null){ finishCapture("Projection unavailable"); return START_NOT_STICKY; }
            projection.registerCallback(callback,main);
            density=getResources().getDisplayMetrics().densityDpi;
            Point p=new Point(); getSystemService(android.view.WindowManager.class).getDefaultDisplay().getRealSize(p);
            readyAt=SystemClock.elapsedRealtime()+2500;
            configureReader(p.x,p.y);
            display=projection.createVirtualDisplay("VIVIM one-shot",width,height,density,DisplayManager.VIRTUAL_DISPLAY_FLAG_AUTO_MIRROR,reader.getSurface(),null,main);
            if(display==null){finishCapture("Display unavailable"); return START_NOT_STICKY;}
            main.postDelayed(()->{if(reader!=null)frame(reader);},2600);
            main.postDelayed(()->finishCapture("No visible frame before timeout"),12000);
        }catch(Exception e){finishCapture("Capture unavailable: "+e.getClass().getSimpleName());}
        return START_NOT_STICKY;
    }
    private void configureReader(int w,int h){
        double scale=Math.min(1.0,1600.0/Math.max(w,h));
        int nw=Math.max(1,(int)(w*scale)),nh=Math.max(1,(int)(h*scale));
        if(reader!=null && nw==width && nh==height) return;
        ImageReader old=reader;
        reader=ImageReader.newInstance(nw,nh,PixelFormat.RGBA_8888,2); width=nw;height=nh;
        reader.setOnImageAvailableListener(this::frame,main);
        if(display!=null){display.resize(width,height,density);display.setSurface(reader.getSurface());}
        if(old!=null)old.close();
    }
    private void frame(ImageReader source){
        if(finished || source!=reader) return;
        try(Image image=source.acquireLatestImage()){
            if(image==null || snapshot!=null || !visible || SystemClock.elapsedRealtime()<readyAt) return;
            Image.Plane p=image.getPlanes()[0]; ByteBuffer b=p.getBuffer();
            int w=image.getWidth(),h=image.getHeight(),stride=p.getRowStride(),pixel=p.getPixelStride();
            int[] argb=new int[w*h]; boolean any=false;
            for(int y=0;y<h;y++)for(int x=0;x<w;x++){
                int n=y*stride+x*pixel; int r=b.get(n)&255,g=b.get(n+1)&255,bl=b.get(n+2)&255;
                argb[y*w+x]=Color.rgb(r,g,bl); any|=(r|g|bl)!=0;
            }
            if(!any)return;
            snapshot=Bitmap.createBitmap(argb,w,h,Bitmap.Config.ARGB_8888);
            main.post(()->finishCapture("One frame captured locally: "+w+" x "+h+"; review selected content"));
        }catch(RuntimeException e){finishCapture("Frame unavailable: "+e.getClass().getSimpleName());}
    }
    private void finishCapture(String message){
        if(finished)return;finished=true;active=false;result=message;main.removeCallbacksAndMessages(null);
        if(display!=null){display.release();display=null;}
        if(reader!=null){reader.close();reader=null;}
        if(projection!=null){projection.unregisterCallback(callback);projection.stop();projection=null;}
        stopForeground(true);stopSelf();
    }
    @Override public void onDestroy(){finishCapture("Snapshot session stopped");super.onDestroy();}
}
