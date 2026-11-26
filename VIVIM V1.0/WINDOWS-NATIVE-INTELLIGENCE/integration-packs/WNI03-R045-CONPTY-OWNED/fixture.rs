use std::io::{self,BufRead,Write};
use windows_sys::Win32::System::Console::{GetStdHandle,GetConsoleMode,GetConsoleScreenBufferInfo,CONSOLE_SCREEN_BUFFER_INFO,STD_INPUT_HANDLE,STD_OUTPUT_HANDLE,STD_ERROR_HANDLE};
fn main(){
 for(i,which)in [STD_INPUT_HANDLE,STD_OUTPUT_HANDLE,STD_ERROR_HANDLE].into_iter().enumerate(){let mut mode=0;assert_ne!(unsafe{GetConsoleMode(GetStdHandle(which),&mut mode)},0,"standard handle {i} not console");}
 println!("VIVIM_PTY_READY");io::stdout().flush().unwrap();
 let mut line=String::new();io::stdin().lock().read_line(&mut line).unwrap();assert_eq!(line.trim(),"native-roundtrip");
 let mut info:CONSOLE_SCREEN_BUFFER_INFO=unsafe{std::mem::zeroed()};assert_ne!(unsafe{GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE),&mut info)},0);
 println!("VIVIM_PTY_SIZE={}x{}",info.dwSize.X,info.dwSize.Y);eprintln!("VIVIM_PTY_ACK:{}",line.trim());io::stderr().flush().unwrap();
 line.clear();io::stdin().lock().read_line(&mut line).unwrap();assert_eq!(line.trim(),"exit");println!("VIVIM_PTY_BYE");
}
