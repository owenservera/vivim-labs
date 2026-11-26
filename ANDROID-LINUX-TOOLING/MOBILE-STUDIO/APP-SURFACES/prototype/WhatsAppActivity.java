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
    private TextView notices;
    private String noticeSample="";
    private String pendingText;
    private android.graphics.Bitmap exportCopy;
    @Override public void onCreate(Bundle b){
        super.onCreate(b);
        if(Build.VERSION.SDK_INT>=33)setRecentsScreenshotEnabled(false);
        ScrollView scroll=new ScrollView(this);
        LinearLayout root=new LinearLayout(this); root.setOrientation(LinearLayout.VERTICAL);root.setPadding(24,48,24,24);scroll.addView(root);
        TextView heading=new TextView(this);heading.setText("VIVIM WhatsApp Studio — local prototype");heading.setTextSize(23);root.addView(heading);
        status=new TextView(this);root.addView(status);
        button(root,"Open WhatsApp",v->open());
        button(root,"Notification access settings",v->noticeSettings());
        button(root,"Read up to 3 active WhatsApp chat previews",v->{noticeSample=WhatsAppNoticeService.sample();notices.setText(noticeSample);});
        notices=new TextView(this);notices.setText("No notification sample requested.");notices.setTextIsSelectable(true);root.addView(notices);
        button(root,"Share reviewed preview text",v->shareNotices());
        button(root,"Export reviewed previews as JSON",v->exportNotices());
        button(root,"Open selected active notification chat",v->chooseNotice());
        button(root,"Review up to 3 chats × 3 excerpts",v->startActivity(new Intent(this,ReviewedActivity.class)));
        button(root,"Clear preview text",v->{noticeSample="";notices.setText("Cleared");});
        button(root,"Authorize one app snapshot",v->capture());
        button(root,"Show latest local snapshot",v->refresh());
        preview=new ImageView(this);preview.setAdjustViewBounds(true);root.addView(preview,new LinearLayout.LayoutParams(-1,-2));
        button(root,"Export reviewed snapshot",v->exportSnapshot());
        button(root,"Clear snapshot / stop capture",v->{preview.setImageDrawable(null);stopService(new Intent(this,SnapshotService.class));SnapshotService.clear();refresh();});
        phone=new EditText(this);phone.setHint("International number (optional, digits only)");phone.setInputType(3);root.addView(phone);
        draft=new EditText(this);draft.setHint("Message draft — not sent automatically");draft.setMinLines(3);root.addView(draft);
        button(root,"Choose existing chat in WhatsApp",v->compose(false));
        button(root,"Open specific chat with draft",v->compose(true));
        button(root,"Clear draft",v->{draft.setText("");phone.setText("");});
        setContentView(scroll);refresh();
    }
    private void button(LinearLayout root,String text,View.OnClickListener action){Button b=new Button(this);b.setText(text);b.setOnClickListener(action);root.addView(b);}
    private void noticeSettings(){
        new AlertDialog.Builder(this).setTitle("Optional notification access").setMessage("Android notification access is broad. VIVIM only samples active WhatsApp message previews when you press Read. It does not collect notification events, store history, or upload data. Android may redact content. Revoke access here whenever you wish.").setNegativeButton("Cancel",null).setPositiveButton("Open Android settings",(d,w)->{try{startActivity(new Intent(android.provider.Settings.ACTION_NOTIFICATION_LISTENER_SETTINGS));}catch(Exception e){status.setText("Settings unavailable");}}).show();
    }
    private void chooseNotice(){
        java.util.List<WhatsAppNoticeService.Target> targets=WhatsAppNoticeService.targets();
        if(targets.isEmpty()){status.setText("No eligible active chat notification");return;}
        CharSequence[] labels=new CharSequence[targets.size()];for(int j=0;j<labels.length;j++)labels[j]=targets.get(j).label;
        new AlertDialog.Builder(this).setTitle("Open chat — may mark messages read").setItems(labels,(d,j)->{
            status.setText(WhatsAppNoticeService.open(targets.get(j)));
        }).setNegativeButton("Cancel",null).show();
    }
    private void exportNotices(){
        if(noticeSample.isEmpty()){status.setText("Read and review previews first");return;}
        try{
            String reviewed=new org.json.JSONObject().put("schema","vivim.whatsapp.active-preview/1").put("source","active-notifications-only").put("complete_history",false).put("reviewed_text",noticeSample).put("created_at_ms",System.currentTimeMillis()).toString(2);
            new AlertDialog.Builder(this).setTitle("Export reviewed notification excerpts?").setMessage(reviewed).setNegativeButton("Cancel",null).setPositiveButton("Choose file",(d,w)->{
                pendingText=reviewed;Intent i=new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("application/json").putExtra(Intent.EXTRA_TITLE,"vivim-reviewed-previews.json");startActivityForResult(i,73);
            }).show();
        }catch(org.json.JSONException e){status.setText("Export preparation failed");}
    }
    private void shareNotices(){
        if(noticeSample.isEmpty()){status.setText("Read and review previews first");return;}
        String reviewed=noticeSample;
        new AlertDialog.Builder(this).setTitle("Share reviewed excerpts?").setMessage("Choose the recipient app yourself. This may send private chat content to that app.").setNegativeButton("Cancel",null).setPositiveButton("Choose app",(d,w)->startActivity(Intent.createChooser(new Intent(Intent.ACTION_SEND).setType("text/plain").putExtra(Intent.EXTRA_TEXT,reviewed),"Share reviewed excerpts"))).show();
    }
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
    private void exportSnapshot(){
        android.graphics.Bitmap shown=SnapshotService.snapshot;
        if(shown==null || SnapshotService.active || shown.isRecycled()){status.setText("No completed snapshot");return;}
        new AlertDialog.Builder(this).setTitle("Export reviewed image?").setMessage("Save exactly one reviewed image to a location you choose. No automatic upload.").setNegativeButton("Cancel",null).setPositiveButton("Choose file",(d,w)->{
            if(SnapshotService.snapshot!=shown || shown.isRecycled())return;
            if(exportCopy!=null)exportCopy.recycle();exportCopy=shown.copy(android.graphics.Bitmap.Config.ARGB_8888,false);
            Intent i=new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("image/png").putExtra(Intent.EXTRA_TITLE,"vivim-observation.png");startActivityForResult(i,72);
        }).show();
    }
    private void capture(){
        if(SnapshotService.active){status.setText("Capture already active");return;}
        new AlertDialog.Builder(this).setTitle("One local snapshot").setMessage("Choose a single app, WhatsApp, in Android's next dialog. Do not choose entire screen. A visible session captures one frame and stops automatically. No upload or history extraction. Select only content you want to inspect.")
            .setNegativeButton("Cancel",null).setPositiveButton("Continue",(d,w)->startActivityForResult(getSystemService(MediaProjectionManager.class).createScreenCaptureIntent(),71)).show();
    }
    @Override protected void onActivityResult(int request,int code,Intent data){
        super.onActivityResult(request,code,data);
        if(request==73){
            try{if(code==RESULT_OK&&data!=null&&data.getData()!=null&&pendingText!=null){try(java.io.OutputStream out=getContentResolver().openOutputStream(data.getData())){if(out==null)throw new java.io.IOException();out.write(pendingText.getBytes(java.nio.charset.StandardCharsets.UTF_8));}status.setText("Reviewed preview JSON saved");}}
            catch(Exception e){status.setText("Export failed");}finally{pendingText=null;}return;
        }
        if(request==72){
            try{if(code==RESULT_OK && data!=null && data.getData()!=null && exportCopy!=null){try(java.io.OutputStream out=getContentResolver().openOutputStream(data.getData())){if(out==null||!exportCopy.compress(android.graphics.Bitmap.CompressFormat.PNG,100,out))throw new java.io.IOException();}status.setText("Reviewed observation exported");}}
            catch(Exception e){status.setText("Export failed: "+e.getClass().getSimpleName());}
            finally{if(exportCopy!=null){exportCopy.recycle();exportCopy=null;}}return;
        }
        if(request!=71)return;
        if(code!=RESULT_OK||data==null){status.setText("Capture consent declined");return;}
        try{preview.setImageDrawable(null);startForegroundService(new Intent(this,SnapshotService.class).putExtra("consent",data));status.setText("Snapshot requested. Return here to view it.");}
        catch(RuntimeException e){status.setText("Capture start unavailable: "+e.getClass().getSimpleName());}
    }
    @Override protected void onResume(){super.onResume();if(status!=null)refresh();}
    @Override protected void onDestroy(){if(exportCopy!=null){exportCopy.recycle();exportCopy=null;}pendingText=null;noticeSample="";super.onDestroy();}
    private void refresh(){preview.setImageDrawable(null);if(SnapshotService.snapshot!=null)preview.setImageBitmap(SnapshotService.snapshot);status.setText(SnapshotService.active?"Capture active — return after it stops":SnapshotService.result);}
}
