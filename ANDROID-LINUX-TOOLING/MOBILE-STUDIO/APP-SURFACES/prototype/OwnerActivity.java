package live.vivim.surfaceprobe;
import android.app.Activity;
import android.content.*;
import android.content.pm.*;
import android.os.*;
import android.widget.*;
import android.view.View;
import java.util.*;

/** Visible owner-selected app browser. No background launches or privileged package inventory. */
public final class OwnerActivity extends Activity {
    private TextView status;
    private LinearLayout entries;
    private EditText search;
    private final ArrayList<ResolveInfo> apps=new ArrayList<>();
    @Override public void onCreate(Bundle b){
        super.onCreate(b);
        LinearLayout root=new LinearLayout(this);root.setOrientation(LinearLayout.VERTICAL);root.setPadding(20,48,20,12);
        TextView heading=new TextView(this);heading.setText("VIVIM Android Owner Console");heading.setTextSize(23);root.addView(heading);
        status=new TextView(this);root.addView(status);
        Button wa=new Button(this);wa.setText("WhatsApp Studio");wa.setOnClickListener(v->startActivity(new Intent(this,WhatsAppActivity.class)));root.addView(wa);
        Button review=new Button(this);review.setText("Reviewed conversation handoff");review.setOnClickListener(v->startActivity(new Intent(this,ReviewedActivity.class)));root.addView(review);
        Button capabilities=new Button(this);capabilities.setText("Capability status");capabilities.setOnClickListener(v->new android.app.AlertDialog.Builder(this).setTitle("Owner-device capabilities").setMessage("Native app discovery and tap-to-launch; owned virtual displays; local snapshot with system consent; optional active WhatsApp previews ("+(WhatsAppNoticeService.available()?"connected":"not connected")+"); draft and recipient handoff; reviewed local exports. No automatic send, remote app control, or unrestricted chat history.").setPositiveButton("OK",null).show());root.addView(capabilities);
        Button refresh=new Button(this);refresh.setText("Discover launchable apps");refresh.setOnClickListener(v->discover());root.addView(refresh);
        search=new EditText(this);search.setHint("Filter by name or package");search.setSingleLine(true);root.addView(search);
        search.addTextChangedListener(new android.text.TextWatcher(){
            public void beforeTextChanged(CharSequence s,int start,int count,int after){}
            public void onTextChanged(CharSequence s,int start,int before,int count){render();}
            public void afterTextChanged(android.text.Editable e){}
        });
        ScrollView scroll=new ScrollView(this);entries=new LinearLayout(this);entries.setOrientation(LinearLayout.VERTICAL);scroll.addView(entries);
        root.addView(scroll,new LinearLayout.LayoutParams(-1,0,1));setContentView(root);discover();
    }
    private void discover(){
        apps.clear();PackageManager pm=getPackageManager();
        Intent query=new Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER);
        apps.addAll(pm.queryIntentActivities(query,0));
        apps.sort(Comparator.comparing(a->a.loadLabel(pm).toString(),String.CASE_INSENSITIVE_ORDER));
        render();
    }
    private void render(){
        if(entries==null)return;entries.removeAllViews();
        PackageManager pm=getPackageManager();String filter=search.getText().toString().toLowerCase(Locale.ROOT);
        int shown=0;
        for(ResolveInfo a:apps){
            String label=a.loadLabel(pm).toString(),pkg=a.activityInfo.packageName;
            if(!label.toLowerCase(Locale.ROOT).contains(filter)&&!pkg.toLowerCase(Locale.ROOT).contains(filter))continue;
            shown++;Button b=new Button(this);b.setAllCaps(false);b.setText(label+"\n"+pkg);b.setOnClickListener(v->{
                try{
                    Intent i=new Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_LAUNCHER)
                        .setComponent(new ComponentName(pkg,a.activityInfo.name));
                    startActivity(i);status.setText("Launch requested: "+label+"; verify visible result.");
                }catch(ActivityNotFoundException|SecurityException e){status.setText("Launch denied or unavailable");}
            });entries.addView(b);
        }
        status.setText("Android API "+Build.VERSION.SDK_INT+" · "+apps.size()+" visible launcher activities · "+shown+" shown\nLaunch only on your tap. No private app data access.");
    }
}
