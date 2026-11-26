package live.vivim.surfaceprobe;
import android.app.*;
import android.content.*;
import android.os.*;
import android.widget.*;
import android.view.View;
import org.json.*;
import java.nio.charset.StandardCharsets;

/** Explicitly reviewed, bounded handoff to a user-selected local document. */
public final class ReviewedActivity extends Activity {
    private final EditText[] names=new EditText[3];
    private final EditText[][] messages=new EditText[3][3];
    private TextView status;
    private byte[] pending;
    @Override public void onCreate(Bundle saved){
        super.onCreate(saved);
        if(Build.VERSION.SDK_INT>=33)setRecentsScreenshotEnabled(false);
        ScrollView scroll=new ScrollView(this);
        LinearLayout root=new LinearLayout(this);root.setOrientation(1);root.setPadding(24,48,24,24);scroll.addView(root);
        TextView title=new TextView(this);title.setText("Reviewed WhatsApp conversation handoff");title.setTextSize(22);root.addView(title);
        TextView explanation=new TextView(this);explanation.setText("Enter or paste only the selected conversation names and recent messages. Up to 3 chats × 3 messages. Nothing is fetched automatically or saved until you export.");root.addView(explanation);
        status=new TextView(this);root.addView(status);
        for(int i=0;i<3;i++){
            names[i]=field(root,"Conversation "+(i+1)+" name",120);
            for(int j=0;j<3;j++)messages[i][j]=field(root,"Message "+(j+1)+" — sender / time / excerpt",500);
        }
        button(root,"Review and export JSON",v->review());
        button(root,"Clear all",v->{for(int i=0;i<3;i++){names[i].setText("");for(EditText e:messages[i])e.setText("");}status.setText("Cleared");});
        setContentView(scroll);
    }
    private EditText field(LinearLayout root,String hint,int max){
        EditText e=new EditText(this);e.setHint(hint);e.setSaveEnabled(false);
        e.setFilters(new android.text.InputFilter[]{new android.text.InputFilter.LengthFilter(max)});
        root.addView(e);return e;
    }
    private void button(LinearLayout r,String s,View.OnClickListener a){Button b=new Button(this);b.setText(s);b.setOnClickListener(a);r.addView(b);}
    private void review(){
        try{
            JSONObject o=new JSONObject();o.put("schema","vivim.whatsapp.reviewed-excerpts/1");o.put("source","owner-selected-manual-excerpts");o.put("complete_history",false);o.put("created_at_ms",System.currentTimeMillis());
            JSONArray chats=new JSONArray();
            for(int i=0;i<3;i++){
                String n=names[i].getText().toString().trim();JSONArray msgs=new JSONArray();
                for(EditText e:messages[i]){String t=e.getText().toString().trim();if(!t.isEmpty())msgs.put(t);}
                if(n.isEmpty() && msgs.length()==0)continue;
                if(n.isEmpty() || msgs.length()==0){status.setText("Each included chat needs a name and at least one message.");return;}
                chats.put(new JSONObject().put("name",n).put("excerpts",msgs));
            }
            if(chats.length()==0){status.setText("Nothing to export");return;}
            o.put("chats",chats);
            String json=o.toString(2);
            new AlertDialog.Builder(this).setTitle("Review exact JSON before export").setMessage(json)
                .setNegativeButton("Cancel",null).setPositiveButton("Save selected file",(d,w)->{
                    pending=json.getBytes(StandardCharsets.UTF_8);
                    Intent i=new Intent(Intent.ACTION_CREATE_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("application/json").putExtra(Intent.EXTRA_TITLE,"vivim-reviewed-whatsapp.json");
                    startActivityForResult(i,81);
                }).show();
        }catch(JSONException e){status.setText("Unable to create export");}
    }
    @Override protected void onActivityResult(int req,int result,Intent data){
        super.onActivityResult(req,result,data);
        if(req!=81)return;
        try{
            if(result==RESULT_OK && data!=null && data.getData()!=null && pending!=null){
                try(java.io.OutputStream out=getContentResolver().openOutputStream(data.getData())){if(out==null)throw new java.io.IOException();out.write(pending);}
                status.setText("Reviewed JSON saved. Only share it with VIVIM if you want it read.");
            }
        }catch(Exception e){status.setText("Export unavailable");}
        finally{pending=null;}
    }
    @Override protected void onDestroy(){pending=null;super.onDestroy();}
}
