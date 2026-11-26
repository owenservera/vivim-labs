package live.vivim.surfaceprobe;
import android.app.*;
import android.content.*;
import android.os.*;
import android.text.InputType;
import android.widget.*;
import org.json.JSONObject;

/** Native tool within the existing owner console, not a new app. */
public final class DownloadActivity extends Activity {
    private static final int PICK=4201;
    private EditText magnet,hash;
    private TextView status;
    @Override public void onCreate(Bundle saved){
        super.onCreate(saved);
        ScrollView scroll=new ScrollView(this);
        LinearLayout root=new LinearLayout(this);root.setOrientation(LinearLayout.VERTICAL);root.setPadding(24,32,24,24);scroll.addView(root);
        TextView heading=new TextView(this);heading.setText("Native downloads");heading.setTextSize(23);root.addView(heading);
        TextView note=new TextView(this);note.setText("App-private storage. BitTorrent peers can see your IP address and may receive uploaded pieces. Download only authorized content.");root.addView(note);
        magnet=new EditText(this);magnet.setHint("magnet:?xt=urn:btih:…");magnet.setInputType(InputType.TYPE_CLASS_TEXT|InputType.TYPE_TEXT_VARIATION_URI);root.addView(magnet);
        hash=new EditText(this);hash.setHint("40-character torrent info hash");hash.setSingleLine();root.addView(hash);
        status=new TextView(this);status.setText(DownloadService.available?"Start engine to restore local jobs.":"Native library not included or not loadable.");root.addView(status);
        button(root,"Start / restore engine",()->{
            if(Build.VERSION.SDK_INT>=33&&checkSelfPermission(android.Manifest.permission.POST_NOTIFICATIONS)!=android.content.pm.PackageManager.PERMISSION_GRANTED)
                requestPermissions(new String[]{android.Manifest.permission.POST_NOTIFICATIONS},44);
            new AlertDialog.Builder(this).setMessage("Start the local peer-to-peer engine and restore saved downloads?").setNegativeButton("Cancel",null).setPositiveButton("Start",(d,w)->send("start",null)).show();
        });
        button(root,"Add magnet",()->new AlertDialog.Builder(this).setMessage("Download and potentially upload pieces for this magnet?").setNegativeButton("Cancel",null).setPositiveButton("Add",(d,w)->send("add_magnet",null)).show());
        button(root,"Import .torrent",()->{
            if(!DownloadService.active){status.setText("Start the engine first.");return;}
            startActivityForResult(new Intent(Intent.ACTION_OPEN_DOCUMENT).setType("*/*").addCategory(Intent.CATEGORY_OPENABLE),PICK);
        });
        button(root,"Refresh list",()->send("list",null));
        button(root,"Pause by hash",()->send("pause",null));
        button(root,"Resume by hash",()->send("resume",null));
        button(root,"Remove job (keep files)",()->new AlertDialog.Builder(this).setMessage("Remove this torrent job, retaining the downloaded files?").setNegativeButton("Cancel",null).setPositiveButton("Remove",(d,w)->send("remove",null)).show());
        button(root,"Stop engine",()->send("stop",null));
        setContentView(scroll);
    }
    private void button(LinearLayout root,String text,Runnable action){Button b=new Button(this);b.setText(text);b.setOnClickListener(v->action.run());root.addView(b);}
    private void send(String op,android.net.Uri uri){
        if(!DownloadService.available){status.setText("Rust download library unavailable");return;}
        if(!"start".equals(op)&&!DownloadService.active){status.setText("Start engine first");return;}
        Intent intent=new Intent(this,DownloadService.class).putExtra(DownloadService.OP,op);
        if("add_magnet".equals(op)){String m=magnet.getText().toString().trim();if(!m.startsWith("magnet:?")||m.length()>8192){status.setText("Invalid magnet");return;}intent.putExtra("magnet",m);}
        if("pause".equals(op)||"resume".equals(op)||"remove".equals(op)){String h=hash.getText().toString().trim();if(!h.matches("[0-9a-fA-F]{40}")){status.setText("Invalid info hash");return;}intent.putExtra("hash",h);}
        if(uri!=null)intent.setData(uri).addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION);
        intent.putExtra(DownloadService.REPLY,new ResultReceiver(new Handler(Looper.getMainLooper())){
            @Override protected void onReceiveResult(int code,Bundle b){
                String result=b.getString("result","No receipt");
                try{status.setText(new JSONObject(result).toString(2));}catch(Exception e){status.setText(result);}
            }
        });
        status.setText("Awaiting native receipt…");
        try{if("start".equals(op))startForegroundService(intent);else startService(intent);}
        catch(RuntimeException e){status.setText("Android denied operation: "+e.getClass().getSimpleName());}
    }
    @Override protected void onActivityResult(int req,int result,Intent data){
        super.onActivityResult(req,result,data);
        if(req==PICK&&result==RESULT_OK&&data!=null&&data.getData()!=null)
            new AlertDialog.Builder(this).setMessage("Import and download the selected torrent?").setNegativeButton("Cancel",null).setPositiveButton("Import",(d,w)->send("add_file",data.getData())).show();
    }
}
