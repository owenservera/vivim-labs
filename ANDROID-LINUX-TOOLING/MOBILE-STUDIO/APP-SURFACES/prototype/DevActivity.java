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
        save=button(root,"Export diagnostic receipt",v->exportReceipt());save.setEnabled(false);
        output=new TextView(this);output.setTextIsSelectable(true);root.addView(output);setContentView(scroll);
        diagnose();
    }
    private Button button(LinearLayout root,String label,View.OnClickListener action){Button b=new Button(this);b.setText(label);b.setOnClickListener(action);root.addView(b);return b;}
    private interface Probe { Object run() throws Exception; }
    private void probe(JSONObject root,String name,Probe action)throws JSONException{
        JSONObject item=new JSONObject();
        long start=SystemClock.elapsedRealtime();
        try{item.put("status","observed");item.put("value",action.run());}
        catch(Exception|LinkageError e){item.put("status","unavailable");item.put("error_type",e.getClass().getSimpleName());}
        item.put("elapsed_ms",SystemClock.elapsedRealtime()-start);root.put(name,item);
    }
    private void diagnose(){
        run.setEnabled(false);save.setEnabled(false);output.setText("Running actual diagnostics…");
        final boolean[] jni=getIntent().getBooleanArrayExtra("jni");
        final boolean notice=WhatsAppNoticeService.available(),capture=SnapshotService.active;
        new Thread(()->{
            String result;
            try{
                JSONObject r=new JSONObject();
                r.put("schema","vivim.android.developer-receipt/1");r.put("source","apk-process");r.put("created_at_ms",System.currentTimeMillis());
                r.put("platform_api",Build.VERSION.SDK_INT);r.put("supported_abis",new JSONArray(Arrays.asList(Build.SUPPORTED_ABIS)));
                r.put("contains_private_chat_content",false);
                probe(r,"installed_identity",()->{
                    PackageManager pm=getPackageManager();
                    PackageInfo info=pm.getPackageInfo(getPackageName(),Build.VERSION.SDK_INT>=28?PackageManager.GET_SIGNING_CERTIFICATES:PackageManager.GET_SIGNATURES);
                    android.content.pm.Signature[] sig=Build.VERSION.SDK_INT>=28?info.signingInfo.getApkContentsSigners():info.signatures;
                    JSONArray signers=new JSONArray();if(sig!=null)for(android.content.pm.Signature s:sig)signers.put(hex(MessageDigest.getInstance("SHA-256").digest(s.toByteArray())));
                    return new JSONObject().put("package",getPackageName()).put("version",info.versionName)
                        .put("version_code",Build.VERSION.SDK_INT>=28?info.getLongVersionCode():info.versionCode)
                        .put("base_apk_sha256",fileDigest(new File(getApplicationInfo().sourceDir))).put("signers_sha256",signers);
                });
                probe(r,"native_jni",()->{
                    JSONArray checks=new JSONArray();boolean passed=jni!=null&&jni.length==5;
                    if(jni!=null)for(boolean v:jni){checks.put(v);passed&=v;}
                    return new JSONObject().put("source","MainActivity actual JNI call immediately before opening console").put("checks",checks).put("all_passed",passed);
                });
                probe(r,"own_cache_roundtrip",()->{
                    File f=File.createTempFile("vivim-probe-",".tmp",getCacheDir());
                    byte[] expected="VIVIM_OWN_CACHE_TEST".getBytes(StandardCharsets.UTF_8);
                    try{
                        try(FileOutputStream out=new FileOutputStream(f)){out.write(expected);out.getFD().sync();}
                        byte[] got=new byte[expected.length];int n;
                        try(DataInputStream in=new DataInputStream(new FileInputStream(f))){in.readFully(got);n=in.read();}
                        return new JSONObject().put("bytes",got.length).put("exact_match",Arrays.equals(expected,got)&&n==-1);
                    }finally{f.delete();}
                });
                probe(r,"app_resources",()->{
                    Runtime rt=Runtime.getRuntime();StatFs fs=new StatFs(getFilesDir().getAbsolutePath());
                    JSONObject v=new JSONObject().put("java_heap_used_bytes",rt.totalMemory()-rt.freeMemory()).put("java_heap_limit_bytes",rt.maxMemory()).put("native_heap_bytes",Debug.getNativeHeapAllocatedSize()).put("available_storage_bytes",fs.getAvailableBytes());
                    BatteryManager bm=getSystemService(BatteryManager.class);
                    if(bm!=null)v.put("battery_percent",bm.getIntProperty(BatteryManager.BATTERY_PROPERTY_CAPACITY));
                    if(Build.VERSION.SDK_INT>=29)v.put("thermal_status",getSystemService(PowerManager.class).getCurrentThermalStatus());
                    return v;
                });
                probe(r,"whatsapp_contracts",()->{
                    PackageManager pm=getPackageManager();
                    Intent share=new Intent(Intent.ACTION_SEND).setType("text/plain").setPackage("com.whatsapp");
                    return new JSONObject().put("launcher_resolved",pm.getLaunchIntentForPackage("com.whatsapp")!=null)
                        .put("text_share_resolved",!pm.queryIntentActivities(share,0).isEmpty()).put("effect_attempted",false);
                });
                probe(r,"permission_state",()->new JSONObject().put("notification_listener_connected_at_start",notice)
                    .put("snapshot_active_at_start",capture).put("internet_declared",checkSelfPermission("android.permission.INTERNET")==PackageManager.PERMISSION_GRANTED)
                    .put("capture_permission_persistent",false));
                r.put("limitations",new JSONArray(Arrays.asList("No third-party content read","No foreground launch effect tested","No capture initiated","JNI evidence belongs to console-entry invocation")));
                result=r.toString(2);
            }catch(Exception e){result="{\"schema\":\"vivim.android.developer-receipt/1\",\"status\":\"diagnostic_failed\",\"error_type\":\""+e.getClass().getSimpleName()+"\"}";}
            final String done=result;
            runOnUiThread(()->{if(closed)return;report=done;output.setText(done);run.setEnabled(true);save.setEnabled(true);});
        },"vivim-diagnostics").start();
    }
    private static String hex(byte[] bytes){StringBuilder b=new StringBuilder();for(byte x:bytes)b.append(String.format("%02x",x&255));return b.toString();}
    private static String fileDigest(File f)throws Exception{
        MessageDigest d=MessageDigest.getInstance("SHA-256");byte[] b=new byte[8192];try(InputStream in=new FileInputStream(f)){int n;while((n=in.read(b))!=-1)d.update(b,0,n);}return hex(d.digest());
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
