use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
struct Rect { left: i32, top: i32, right: i32, bottom: i32 }
impl Rect {
    fn valid(self) -> bool { self.left < self.right && self.top < self.bottom }
    fn contains(self, x: i32, y: i32) -> bool {
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }
}
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Window { hwnd: String, pid: u32, bounds: Rect, geometry_source: &'static str, minimized: bool }
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Element {
    index: usize, parent: Option<usize>, pid: Option<i32>, role: Option<i32>,
    bounds: Option<Rect>, enabled: Option<bool>, offscreen: Option<bool>,
    password: Option<bool>, label: Option<String>,
}
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Surface {
    desktop: Rect, foreground: String, cursor: Option<[i32; 2]>,
    windows: Vec<Window>, elements: Vec<Element>, cursor_element: Option<Element>,
    semantic_status: &'static str, tree_partial: bool,
}

#[cfg(windows)]
mod native {
    use super::*;
    use std::{io::{self, Write}, thread, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
    use windows::{core::Result, Win32::{
        Foundation::{BOOL, HWND, LPARAM, POINT, RECT},
        Graphics::Dwm::{DwmGetWindowAttribute, DWMWA_CLOAKED, DWMWA_EXTENDED_FRAME_BOUNDS},
        System::Com::{CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED},
        UI::{Accessibility::{CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationTreeWalker},
             HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
             WindowsAndMessaging::*}
    }};
    fn rect(r: RECT) -> Rect { Rect { left:r.left, top:r.top, right:r.right, bottom:r.bottom } }
    fn id(h: HWND) -> String { format!("0x{:x}", h.0 as usize) }
    unsafe extern "system" fn collect(h: HWND, arg: LPARAM) -> BOOL {
        let out = &mut *(arg.0 as *mut Vec<Window>);
        if !IsWindowVisible(h).as_bool() { return BOOL(1); }
        let mut cloaked = 0u32;
        if DwmGetWindowAttribute(h,DWMWA_CLOAKED,(&mut cloaked as *mut u32).cast(),4).is_ok() && cloaked != 0 { return BOOL(1); }
        let mut r = RECT::default();
        let source = if DwmGetWindowAttribute(h,DWMWA_EXTENDED_FRAME_BOUNDS,(&mut r as *mut RECT).cast(),std::mem::size_of::<RECT>() as u32).is_ok() { "dwm" } else if GetWindowRect(h,&mut r).is_ok() { "win32" } else { return BOOL(1); };
        let mut pid = 0;
        GetWindowThreadProcessId(h,Some(&mut pid));
        if rect(r).valid() { out.push(Window { hwnd:id(h),pid,bounds:rect(r),geometry_source:source,minimized:IsIconic(h).as_bool() }); }
        BOOL(1)
    }
    unsafe fn element(e: &IUIAutomationElement, index: usize, parent: Option<usize>, labels: bool) -> Element {
        let password = e.CurrentIsPassword().ok().map(|v|v.as_bool());
        Element { index,parent,pid:e.CurrentProcessId().ok(),role:e.CurrentControlType().ok().map(|v|v.0),
            bounds:e.CurrentBoundingRectangle().ok().map(rect).filter(|r|r.valid()),
            enabled:e.CurrentIsEnabled().ok().map(|v|v.as_bool()),
            offscreen:e.CurrentIsOffscreen().ok().map(|v|v.as_bool()),password,
            // No values, text patterns, typed characters, or password labels are read.
            label:if labels && password == Some(false) { e.CurrentName().ok().map(|s|s.to_string()) } else { None } }
    }
    unsafe fn walk(w: &IUIAutomationTreeWalker, e: &IUIAutomationElement, parent: Option<usize>, depth: usize, out: &mut Vec<Element>, labels: bool, partial: &mut bool) {
        if depth >= 24 || out.len() >= 1500 { *partial = true; return; }
        let index = out.len(); out.push(element(e,index,parent,labels));
        let mut child = w.GetFirstChildElement(e);
        while let Ok(c) = child {
            if c.as_raw().is_null() { break; }
            if out.len() >= 1500 { *partial = true; break; }
            walk(w,&c,Some(index),depth+1,out,labels,partial);
            child = w.GetNextSiblingElement(&c);
        }
    }
    use windows::core::Interface;
    struct Com;
    impl Drop for Com { fn drop(&mut self) { unsafe { CoUninitialize(); } } }
    pub fn run() -> Result<()> {
        let args:Vec<String> = std::env::args().skip(1).collect();
        if args.iter().any(|a|a=="--help") {
            eprintln!("virtual-semantic-surface [--once] [--labels] [--interval-ms N]\nLocal read-only NDJSON. Default: geometry and roles, no names. Ctrl+C stops."); return Ok(());
        }
        let labels=args.iter().any(|a|a=="--labels");
        let once=args.iter().any(|a|a=="--once");
        let interval=args.windows(2).find(|p|p[0]=="--interval-ms").and_then(|p|p[1].parse::<u64>().ok()).unwrap_or(500).clamp(100,10000);
        unsafe {
            // Fail closed rather than mix virtualized and physical coordinates.
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)?;
            CoInitializeEx(None,COINIT_MULTITHREADED).ok()?;
            let _com = Com;
            let automation: IUIAutomation = CoCreateInstance(&CUIAutomation,None,CLSCTX_INPROC_SERVER)?;
            let walker=automation.ControlViewWalker()?;
            eprintln!("Semantic surface active locally; no pixel capture or input injection. Ctrl+C stops. Labels: {labels}");
            let mut seq=0u64;
            let mut previous:Option<Surface>=None;
            let mut last_emit=Instant::now();
            let mut out=io::stdout().lock();
            loop {
                let x=GetSystemMetrics(SM_XVIRTUALSCREEN); let y=GetSystemMetrics(SM_YVIRTUALSCREEN);
                let desktop=Rect {left:x,top:y,right:x+GetSystemMetrics(SM_CXVIRTUALSCREEN),bottom:y+GetSystemMetrics(SM_CYVIRTUALSCREEN)};
                let foreground=GetForegroundWindow();
                let mut windows=Vec::<Window>::new();
                EnumWindows(Some(collect),LPARAM((&mut windows as *mut Vec<Window>) as isize))?;
                let mut p=POINT::default();
                let cursor=GetCursorPos(&mut p).ok().map(|_|[p.x,p.y]);
                let mut elements=Vec::new(); let mut tree_partial=false;
                let semantic_status=match automation.ElementFromHandle(foreground) {
                    Ok(e) if !e.as_raw().is_null()=>{walk(&walker,&e,None,0,&mut elements,labels,&mut tree_partial);"foreground_observed"},
                    _=>"foreground_unavailable",
                };
                let cursor_element=cursor.and_then(|[x,y]|automation.ElementFromPoint(POINT{x,y}).ok()).filter(|e|!e.as_raw().is_null()).map(|e|element(&e,0,None,labels));
                let s=Surface { desktop,foreground:id(foreground),cursor,windows,elements,cursor_element,semantic_status,tree_partial };
                if once || previous.as_ref()!=Some(&s) || last_emit.elapsed()>=Duration::from_secs(5) {
                    seq+=1;
                    let ts=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
                    let msg=serde_json::json!({"schema":"vivim.surface/1","revision":seq,"unix_ms":ts,"coordinate_space":"physical_desktop_pixels","surface":&s});
                    if serde_json::to_writer(&mut out,&msg).is_err() || writeln!(out).is_err() || out.flush().is_err() { break; }
                    previous=Some(s);last_emit=Instant::now();
                }
                if once { break; }
                thread::sleep(Duration::from_millis(interval));
            }
        }
        Ok(())
    }
}
#[cfg(windows)]
fn main() -> windows::core::Result<()> { native::run() }
#[cfg(not(windows))]
fn main() { eprintln!("Requires an interactive Windows desktop."); std::process::exit(1); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn negative_monitor_coordinates() { let r=Rect{left:-1920,top:0,right:0,bottom:1080}; assert!(r.valid());assert!(r.contains(-1,1079));assert!(!r.contains(0,1079)); }
    #[test] fn empty_rect_rejected() { assert!(!Rect{left:1,top:2,right:1,bottom:4}.valid()); }
}
