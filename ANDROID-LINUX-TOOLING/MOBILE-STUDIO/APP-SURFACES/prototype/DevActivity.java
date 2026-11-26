package live.vivim.surfaceprobe;

import android.app.*;
import android.content.*;
import android.content.pm.*;
import android.os.*;
import android.view.View;
import android.widget.*;
import org.json.*;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.Arrays;

/** App-owned diagnostic instrument. No chat text, tokens, or external file inspection. */
public final class DevActivity extends Activity {
    private TextView output;
    private Button run,save;
    private String report="";
    private byte[] pending;
    private boolean closed;
    @Override public void onCreate(Bundle state){
        super.onCreate(state);
        ScrollView scroll=new ScrollView(this);
        LinearLayout root=new LinearLayout(this);root.setOrientation(1);root.setPadding(24,48,24,24);scroll.addView(root);
        TextView title=new TextView(this);title.setText("VIVIM Developer Instrument");title.setTextSize(24);root.addView(title);
        TextView intro=new TextView(this);intro.setText("Real APK-context diagnostics. No conversation content, credentials or screen pixels included.");root.addView(intro);
        run=button(root,"Run native and Android diagnostics",v->diagnose());
        button(root,"Load automatic startup receipt",v->loadStartupReceipt());
        save=button(root,"Export diagnostic receipt",v->exportReceipt());save.setEnabled(false);
        output=new TextView(this);output.setTextIsSelectable(true);root.addView(output);setContentView(scroll);
        diagnose();
    }
    private Button button(LinearLayout root,String label,View.OnClickListener action){Button b=new Button(this);b.setText(label);b.setOnClickListener(action);root.addView(b);return b;}
    private void diagnose(){
        run.setEnabled(false);save.setEnabled(false);output.setText("Running actual diagnostics…");
        final boolean[] jni=getIntent().getBooleanArrayExtra("jni");
        final boolean notice=WhatsAppNoticeService.available(),capture=SnapshotService.active;
        final Context ctx=getApplicationContext();
        new Thread(()->{
            String result=StartupSelfTest.run(ctx,jni,notice,capture,"developer-console");
            final String done=result;
            runOnUiThread(()->{if(closed)return;report=done;output.setText(done);run.setEnabled(true);save.setEnabled(true);});
        },"vivim-diagnostics").start();
    }
    private void loadStartupReceipt(){
        run.setEnabled(false);save.setEnabled(false);
        final Context ctx=getApplicationContext();
        new Thread(()->{
            String result;
            try{
                android.util.AtomicFile f=new android.util.AtomicFile(new File(ctx.getFilesDir(),"latest-developer-receipt.json"));
                byte[] bytes=f.readFully();
                if(bytes.length>65536)throw new IOException("Receipt too large");
                result=new String(bytes,StandardCharsets.UTF_8);
            }catch(Exception e){result="No completed startup receipt: "+e.getClass().getSimpleName();}
            final String done=result;runOnUiThread(()->{if(closed)return;report=done.startsWith("{")?done:"";output.setText(done);run.setEnabled(true);save.setEnabled(!report.isEmpty());});
        },"vivim-read-startup-receipt").start();
    }
    private void exportReceipt(){
        if(report.isEmpty())return;pending=report.getBytes(StandardCharsets.UTF_8);
        Intent i=new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("application/json").putExtra(Intent.EXTRA_TITLE,"vivim-developer-receipt.json");
        startActivityForResult(i,91);
    }
    @Override protected void onActivityResult(int req,int result,Intent data){
        super.onActivityResult(req,result,data);if(req!=91)return;
        try{if(result==RESULT_OK&&data!=null&&data.getData()!=null&&pending!=null){try(OutputStream out=getContentResolver().openOutputStream(data.getData())){if(out==null)throw new IOException();out.write(pending);}Toast.makeText(this,"Diagnostic receipt saved",Toast.LENGTH_SHORT).show();}}
        catch(Exception e){Toast.makeText(this,"Export failed",Toast.LENGTH_SHORT).show();}finally{pending=null;}
    }
    @Override protected void onDestroy(){closed=true;pending=null;super.onDestroy();}
}
