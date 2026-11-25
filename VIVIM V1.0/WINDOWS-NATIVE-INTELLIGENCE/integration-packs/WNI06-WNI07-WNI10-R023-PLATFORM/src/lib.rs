//! Optional read-only WinRT support discovery. No capture, model load,
//! activation of other apps, package install, or user permission changes.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Support{Present,Absent,Unknown}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct Report{
 pub graphics_capture:Support,pub action_catalog_type:Support,
 pub legacy_ml_session_type:Support,pub new_ml_catalog_type:Support
}
#[cfg(windows)]
mod win{
 use super::*;
 use windows::{core::HSTRING,Foundation::Metadata::ApiInformation,Graphics::Capture::GraphicsCaptureSession,Win32::System::WinRT::{RoInitialize,RoUninitialize,RO_INIT_MULTITHREADED}};
 struct Apartment;
 impl Drop for Apartment{fn drop(&mut self){unsafe{RoUninitialize();}}}
 fn state(v:windows::core::Result<bool>)->Support{match v{Ok(true)=>Support::Present,Ok(false)=>Support::Absent,Err(_)=>Support::Unknown}}
 fn ty(s:&str)->Support{state(ApiInformation::IsTypePresent(&HSTRING::from(s)))}
 pub fn probe()->Report{
  if unsafe{RoInitialize(RO_INIT_MULTITHREADED)}.is_err(){return super::unknown();}
  let _apartment=Apartment;
  Report{
   graphics_capture:state(GraphicsCaptureSession::IsSupported()),
   action_catalog_type:ty("Windows.AI.Actions.Hosting.ActionCatalog"),
   legacy_ml_session_type:ty("Windows.AI.MachineLearning.LearningModelSession"),
   new_ml_catalog_type:ty("Microsoft.Windows.AI.MachineLearning.ExecutionProviderCatalog")
  }
 }
}
fn unknown()->Report{Report{graphics_capture:Support::Unknown,action_catalog_type:Support::Unknown,legacy_ml_session_type:Support::Unknown,new_ml_catalog_type:Support::Unknown}}
#[cfg(windows)]pub use win::probe;
#[cfg(not(windows))]pub fn probe()->Report{unknown()}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn support_probe_only(){let r=probe();println!("PLATFORM_SUPPORT={r:?}");}
}
