package live.vivim.surfaceprobe;
import android.app.Notification;
import android.app.Person;
import android.content.ComponentName;
import android.os.Bundle;
import android.service.notification.NotificationListenerService;
import android.service.notification.StatusBarNotification;
import java.util.*;

/** Reads only on a visible user request. No event collection, persistence, or network. */
public final class WhatsAppNoticeService extends NotificationListenerService {
    private static WhatsAppNoticeService connected;
    @Override public void onListenerConnected(){connected=this;}
    @Override public void onListenerDisconnected(){if(connected==this)connected=null;}
    @Override public void onDestroy(){if(connected==this)connected=null;super.onDestroy();}
    static boolean available(){return connected!=null;}
    static String sample(){
        WhatsAppNoticeService svc=connected;
        if(svc==null)return "Notification access is not connected. Grant it in Android Settings, then return.";
        StatusBarNotification[] all;
        try{all=svc.getActiveNotifications();}catch(SecurityException e){return "Notification access revoked.";}
        if(all==null)return "No accessible active notifications.";
        ArrayList<StatusBarNotification> selected=new ArrayList<>();
        for(StatusBarNotification s:all){
            if(!"com.whatsapp".equals(s.getPackageName()))continue;
            Notification n=s.getNotification();
            if((n.flags&Notification.FLAG_GROUP_SUMMARY)!=0)continue;
            if(!Notification.CATEGORY_MESSAGE.equals(n.category))continue;
            if(n.visibility==Notification.VISIBILITY_SECRET)continue;
            selected.add(s);
        }
        selected.sort((a,b)->Long.compare(b.getPostTime(),a.getPostTime()));
        StringBuilder out=new StringBuilder("Active WhatsApp notification excerpts only — NOT complete chat history.\n");
        Set<String> seen=new HashSet<>();int count=0;
        for(StatusBarNotification s:selected){
            Notification n=s.getNotification();
            Bundle b=n.extras;if(b==null)continue;
            String name=bounded(b.getCharSequence(Notification.EXTRA_CONVERSATION_TITLE),120);
            if(name.isEmpty())name=bounded(b.getCharSequence(Notification.EXTRA_TITLE),120);
            if(name.isEmpty())continue;
            String identity=n.getShortcutId()!=null?n.getShortcutId():name;
            if(!seen.add(identity))continue;
            ArrayList<String> lines=new ArrayList<>();
            try{
                if(android.os.Build.VERSION.SDK_INT>=28){
                    android.os.Parcelable[] items=b.getParcelableArray(Notification.EXTRA_MESSAGES);
                    if(items!=null)for(int j=Math.max(0,items.length-3);j<items.length;j++){
                        if(!(items[j] instanceof Bundle))continue;
                        Bundle m=(Bundle)items[j];
                        CharSequence text=m.getCharSequence("text");
                        if(text==null)continue;
                        Person p=m.getParcelable("sender_person");
                        String sender=p==null?bounded(m.getCharSequence("sender"),100):bounded(p.getName(),100);
                        lines.add((sender.isEmpty()?"Sender not exposed":sender)+": "+bounded(text,500));
                    }
                }
            }catch(RuntimeException ignored){lines.clear();}
            if(lines.isEmpty()){
                String preview=bounded(b.getCharSequence(Notification.EXTRA_TEXT),500);
                if(!preview.isEmpty())lines.add("Preview: "+preview);
            }
            if(lines.isEmpty())continue;
            out.append("\n").append(++count).append(". ").append(name).append("\n");
            for(String l:lines)out.append(l).append("\n");
            if(count==3)break;
        }
        if(count==0)out.append("\nNo eligible conversation previews exposed by Android.");
        return out.toString();
    }
    static String bounded(CharSequence s,int max){
        if(s==null)return "";
        String v=s.toString().replace('\n',' ').replace('\r',' ').trim();
        return v.length()>max?v.substring(0,max)+"…":v;
    }
}
