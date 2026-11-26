//! Minimal dependency-free Rust JNI probe; no device I/O.
use core::ffi::c_void;

fn classify(expected: i32, actual: i32, rgb: i32) -> i32 {
    if expected <= 0 || actual != expected { return 0; }
    if rgb != 0x7139a1 { return 1; }
    2
}

#[no_mangle]
pub extern "system" fn Java_live_vivim_surfaceprobe_MainActivity_nativeProbeVersion(
    _env: *mut c_void, _receiver: *mut c_void
) -> i32 { 2 }

#[no_mangle]
pub extern "system" fn Java_live_vivim_surfaceprobe_MainActivity_nativeClassify(
    _env: *mut c_void, _receiver: *mut c_void, expected: i32, actual: i32, rgb: i32
) -> i32 { classify(expected, actual, rgb) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn missing_display() { assert_eq!(classify(-1,-1,0x7139a1),0); }
    #[test] fn physical_display_not_private() { assert_eq!(classify(0,0,0x7139a1),0); }
    #[test] fn mismatched_display() { assert_eq!(classify(3,4,0x7139a1),0); }
    #[test] fn wrong_pixel() { assert_eq!(classify(3,3,0x125fab),1); }
    #[test] fn both_match() { assert_eq!(classify(3,3,0x7139a1),2); }
}
