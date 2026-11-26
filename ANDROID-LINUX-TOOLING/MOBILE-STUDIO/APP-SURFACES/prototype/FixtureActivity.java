package live.vivim.surfaceprobe;
import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.view.WindowInsets;
import android.widget.TextView;

/** Non-exported same-package fixture. Never launches or observes third-party apps. */
public final class FixtureActivity extends Activity {
    static int expectedDisplay=-1, actualDisplay=-1;
    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        actualDisplay=getWindowManager().getDefaultDisplay().getDisplayId();
        if(expectedDisplay<0 || actualDisplay!=expectedDisplay) { finish(); return; }
        TextView view=new TextView(this);
        view.setBackgroundColor(Color.rgb(113,57,161));
        view.setTextColor(Color.WHITE);
        view.setText("VIVIM owned Activity — display "+actualDisplay);
        view.setTextSize(20);
        setContentView(view);
    }
}
