//! Bounded Windows Search SQL builder. No COM connection or filesystem access.
//! Host supplies canonical approved scope, independently validates each result,
//! and does not publish raw result paths until authorization is rechecked.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Error{Scope,Name,Limit}
fn literal(s:&str)->String{format!("'{}'",s.replace('\'',"''"))}
fn segments(s:&str)->bool{s.split('\\').all(|v|!v.is_empty()&&v!="."&&v!=".."&&!v.ends_with(['.',' '])&&!v.chars().any(|c|c.is_control()||":/\"*?<>|%#".contains(c)))}
fn scope(s:&str)->Result<String,Error>{
 let b=s.as_bytes();
 if b.len()<4||b.len()>1024||!b[0].is_ascii_alphabetic()||b[1]!=b':'||b[2]!=b'\\'||!segments(&s[3..]){return Err(Error::Scope);}
 Ok(format!("file:{}",s.replace('\\',"/")))
}
/// Exact filename equality intentionally avoids caller-supplied wildcard or
/// full-text syntax. Subsequent verified adapters can add explicit query modes.
pub fn exact_name(approved_root:&str,name:&str,limit:u16)->Result<String,Error>{
 let scope=scope(approved_root)?;
 if name.is_empty()||name.len()>255||!segments(name)||name.contains('\\'){return Err(Error::Name);}
 if !(1..=200).contains(&limit){return Err(Error::Limit);}
 Ok(format!("SELECT TOP {limit} System.ItemPathDisplay, System.Size, System.DateModified FROM SYSTEMINDEX WHERE SCOPE={} AND System.FileName={} ORDER BY System.DateModified DESC",literal(&scope),literal(name)))
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn escape_and_limit(){let q=exact_name("C:\\Owner's files","O'Brien.txt",12).unwrap();assert!(q.contains("SCOPE='file:C:/Owner''s files'"));assert!(q.contains("System.FileName='O''Brien.txt'"));assert!(q.starts_with("SELECT TOP 12 "));}
 #[test]fn reject_scope(){for s in ["C:\\","C:\\..\\x","\\\\server\\share","C:\\x#y","C:\\x%20y"]{assert_eq!(exact_name(s,"a.txt",1),Err(Error::Scope));}assert_eq!(exact_name("C:\\owned","a.txt",201),Err(Error::Limit));assert!(exact_name("C:\\owned","..\\a.txt",1).is_err());}
}
