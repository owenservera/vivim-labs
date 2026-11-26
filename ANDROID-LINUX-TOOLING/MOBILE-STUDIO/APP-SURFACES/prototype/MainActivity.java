package live.vivim.surfaceprobe;

import android.app.Activity;
import android.app.ActivityManager;
import android.app.ActivityOptions;
import android.content.Intent;
import android.app.Presentation;
import android.content.Context;
import android.graphics.Color;
import android.graphics.PixelFormat;
import android.hardware.display.DisplayManager;
import android.hardware.display.VirtualDisplay;
import android.media.Image;
import android.media.ImageReader;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.TextView;
import java.security.MessageDigest;
import java.nio.ByteBuffer;

/** Visible, local, own-content-only feasibility test; no third-party capture. */
public final class MainActivity extends Activity {
    private ImageReader reader;
    private VirtualDisplay display;
    private Presentation presentation;
    private TextView status, fixture;
    private final Handler handler = new Handler(Looper.getMainLooper());
    private String lastDigest = "";
    private int distinct, frames, colorIndex;
    private boolean activityMode;
    private String launchStatus = "";
    private final int[] colors = {Color.rgb(18,95,171),Color.rgb(22,145,70),Color.rgb(178,65,30)};
    @Override public void onCreate(Bundle saved) {
        super.onCreate(saved);
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setPadding(24,48,24,24);
        TextView title = new TextView(this);
        title.setText("VIVIM Surface Probe — own content only");
        title.setTextSize(22);
        root.addView(title);
        status = new TextView(this);
        status.setText("Idle. Tap Start to create a private display.");
        root.addView(status);
        button(root,"Start private display",v -> startProbe(false));
        button(root,"Launch owned activity on private display",v -> startProbe(true));
        button(root,"Change fixture color",v -> changeFixture());
        button(root,"Stop and release",v -> stopProbe("Stopped"));
        setContentView(root);
    }
    private void button(LinearLayout root,String name,View.OnClickListener action) {
        Button b = new Button(this); b.setText(name); b.setOnClickListener(action); root.addView(b);
    }
    private void startProbe(boolean launchActivity) {
        stopProbe("Reset");
        try {
            distinct=frames=colorIndex=0; lastDigest="";
            activityMode=launchActivity; launchStatus="";
            reader = ImageReader.newInstance(320,480,PixelFormat.RGBA_8888,2);
            reader.setOnImageAvailableListener(this::receiveFrame,handler);
            DisplayManager dm = (DisplayManager)getSystemService(Context.DISPLAY_SERVICE);
            display = dm.createVirtualDisplay("VIVIM own-content probe",320,480,160,reader.getSurface(),
                DisplayManager.VIRTUAL_DISPLAY_FLAG_OWN_CONTENT_ONLY |
                DisplayManager.VIRTUAL_DISPLAY_FLAG_PRESENTATION);
            if(display==null) throw new IllegalStateException("No display returned");
            if (activityMode) {
                int id=display.getDisplay().getDisplayId();
                Intent intent=new Intent(this,FixtureActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                if(android.os.Build.VERSION.SDK_INT>=29 && !ActivityManager.isActivityStartAllowedOnDisplay(this,id,intent)) {
                    stopProbe("Android denied owned activity launch on display "+id); return;
                }
                FixtureActivity.expectedDisplay=id;
                FixtureActivity.actualDisplay=-1;
                ActivityOptions options=ActivityOptions.makeBasic().setLaunchDisplayId(id);
                startActivity(intent,options.toBundle());
                launchStatus="Owned activity launch dispatched; waiting for evidence";
                status.setText(launchStatus); return;
            }
            presentation = new Presentation(this,display.getDisplay());
            fixture = new TextView(presentation.getContext());
            fixture.setText("VIVIM owned fixture 0");
            fixture.setTextColor(Color.WHITE);
            fixture.setTextSize(24);
            fixture.setBackgroundColor(colors[0]);
            presentation.setContentView(fixture);
            presentation.show();
            status.setText("Display "+display.getDisplay().getDisplayId()+" created; awaiting frame.");
        } catch(Exception e) { stopProbe("Unavailable: "+e.getClass().getSimpleName()); }
    }
    private void changeFixture() {
        if(activityMode) { status.setText("Activity mode: use Stop and release."); return; }
        if(fixture==null) { status.setText("Start the display first."); return; }
        colorIndex=(colorIndex+1)%colors.length;
        fixture.setText("VIVIM owned fixture "+colorIndex);
        fixture.setBackgroundColor(colors[colorIndex]);
    }
    private void receiveFrame(ImageReader source) {
        if(source!=reader) return;
        try(Image image=source.acquireLatestImage()) {
            if(image==null) return;
            Image.Plane plane=image.getPlanes()[0];
            ByteBuffer b=plane.getBuffer().duplicate();
            int center=(image.getHeight()/2)*plane.getRowStride()+(image.getWidth()/2)*plane.getPixelStride();
            String sample=String.format("#%02x%02x%02x",b.get(center)&255,b.get(center+1)&255,b.get(center+2)&255);
            MessageDigest md=MessageDigest.getInstance("SHA-256"); md.update(b);
            byte[] bytes=md.digest(); StringBuilder hex=new StringBuilder();
            for(byte x:bytes) hex.append(String.format("%02x",x&255));
            String digest=hex.toString();
            frames++; if(!digest.equals(lastDigest)){ distinct++; lastDigest=digest; }
            status.setText("Private display: "+display.getDisplay().getDisplayId()
                +"\nFrames: "+frames+"\nFrame digest transitions: "+distinct+"\nCenter RGB: "+sample+"\nSHA-256: "+digest
                +(activityMode ? "\nOwned activity actual display: "+FixtureActivity.actualDisplay
                    +"\n"+(FixtureActivity.actualDisplay==display.getDisplay().getDisplayId() && sample.equalsIgnoreCase("#7139a1") ? "OWNED ACTIVITY RENDER CONFIRMED" : "Awaiting matching display and fixture color") : ""));
        } catch(Exception e) { stopProbe("Frame error: "+e.getClass().getSimpleName()); }
    }
    private void stopProbe(String message) {
        fixture=null;
        activityMode=false;
        FixtureActivity.expectedDisplay=-1;
        if(presentation!=null){ presentation.dismiss(); presentation=null; }
        if(display!=null){ display.release(); display=null; }
        if(reader!=null){ reader.setOnImageAvailableListener(null,null); reader.close(); reader=null; }
        if(status!=null) status.setText(message);
    }
    @Override protected void onStop(){ stopProbe("Stopped on leaving activity"); super.onStop(); }
}
