//! Minimal native JNI ABI probe. No device or app access; no runtime side effects.
use core::ffi::c_void;
#[no_mangle]
pub extern "system" fn Java_live_vivim_surfaceprobe_MainActivity_nativeProbeVersion(
    _env: *mut c_void, _receiver: *mut c_void
) -> i32 { 1 }
