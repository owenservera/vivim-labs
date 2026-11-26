package live.vivim.surfaceprobe;

import android.app.*;
import android.content.*;
import android.net.Uri;
import android.os.*;
import android.media.projection.MediaProjectionManager;
import android.view.View;
import android.widget.*;

/** User-operated WhatsApp adapter, no autonomous send or background monitoring. */
public final class WhatsAppActivity extends Activity {
    private EditText phone,draft;
    private TextView status;
    private ImageView preview;
    @Override public void onCreate(Bundle b){
        super.onCreate(b);
        ScrollView scroll=new ScrollView(this);
        LinearLayout root=new LinearLayout(this); root.setOrientation(LinearLayout.VERTICAL);root.setPadding(24,48,24,24);scroll.addView(root);
        TextView heading=new TextView(this);heading.setText("VIVIM WhatsApp Studio — local prototype");heading.setTextSize(23);root.addView(heading);
        status=new TextView(this);root.addView(status);
        button(root,"Open WhatsApp",v->open());
        button(root,"Authorize one app snapshot",v->capture());
        button(root,"Show latest local snapshot",v->refresh());
        preview=new ImageView(this);preview.setAdjustViewBounds(true);root.addView(preview,new LinearLayout.LayoutParams(-1,-2));
        button(root,"Clear snapshot / stop capture",v->{preview.setImageDrawable(null);stopService(new Intent(this,SnapshotService.class));SnapshotService.clear();refresh();});
        phone=new EditText(this);phone.setHint("International number (optional, digits only)");phone.setInputType(3);root.addView(phone);
        draft=new EditText(this);draft.setHint("Message draft — not sent automatically");draft.setMinLines(3);root.addView(draft);
        button(root,"Choose existing chat in WhatsApp",v->compose(false));
        button(root,"Open specific chat with draft",v->compose(true));
        button(root,"Clear draft",v->{draft.setText("");phone.setText("");});
        setContentView(scroll);refresh();
    }
    private void button(LinearLayout root,String text,View.OnClickListener action){Button b=new Button(this);b.setText(text);b.setOnClickListener(action);root.addView(b);}
    private void open(){
        Intent i=getPackageManager().getLaunchIntentForPackage("com.whatsapp");
        if(i==null){status.setText("WhatsApp unavailable");return;}dispatch(i);
    }
    private void compose(boolean direct){
        String text=draft.getText().toString();
        if(text.trim().isEmpty() || text.length()>4096){status.setText("Enter a draft of 1–4096 characters.");return;}
        Intent i;
        if(direct){
            String n=phone.getText().toString().trim();
            if(!n.matches("[1-9][0-9]{6,14}")){status.setText("Enter country code and number, digits only.");return;}
            i=new Intent(Intent.ACTION_VIEW,Uri.parse("https://wa.me/"+n+"?text="+Uri.encode(text)));
        }else{i=new Intent(Intent.ACTION_SEND).setType("text/plain").putExtra(Intent.EXTRA_TEXT,text);}
        i.setPackage("com.whatsapp");dispatch(i);
    }
    private void dispatch(Intent i){
        try{startActivity(i);status.setText("WhatsApp requested; review target and text in WhatsApp. Not sent.");}
        catch(ActivityNotFoundException|SecurityException e){status.setText("WhatsApp action unavailable");}
    }
    private void capture(){
        if(SnapshotService.active){status.setText("Capture already active");return;}
        new AlertDialog.Builder(this).setTitle("One local snapshot").setMessage("Choose a single app, WhatsApp, in Android's next dialog. Do not choose entire screen. A visible session captures one frame and stops automatically. No upload or history extraction. Select only content you want to inspect.")
            .setNegativeButton("Cancel",null).setPositiveButton("Continue",(d,w)->startActivityForResult(getSystemService(MediaProjectionManager.class).createScreenCaptureIntent(),71)).show();
    }
    @Override protected void onActivityResult(int request,int code,Intent data){
        super.onActivityResult(request,code,data);
        if(request!=71)return;
        if(code!=RESULT_OK||data==null){status.setText("Capture consent declined");return;}
        try{preview.setImageDrawable(null);startForegroundService(new Intent(this,SnapshotService.class).putExtra("consent",data));status.setText("Snapshot requested. Return here to view it.");}
        catch(RuntimeException e){status.setText("Capture start unavailable: "+e.getClass().getSimpleName());}
    }
    @Override protected void onResume(){super.onResume();if(status!=null)refresh();}
    private void refresh(){preview.setImageDrawable(null);if(SnapshotService.snapshot!=null)preview.setImageBitmap(SnapshotService.snapshot);status.setText(SnapshotService.active?"Capture active — return after it stops":SnapshotService.result);}
}
