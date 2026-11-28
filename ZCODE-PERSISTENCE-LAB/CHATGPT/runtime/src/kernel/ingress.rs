//! Fixed-authority local broker. A trusted launcher owns the store, authority
//! file, process and OS permissions; clients receive only the framed transport.
use super::{authority::{Authority,SignedRequest},model::{ensure,MAX_VALUE_BYTES},persistence::KernelStore};
use crate::Result;
use serde_json::{Value,json};
use std::{io::{BufRead,Write},time::{SystemTime,UNIX_EPOCH}};
fn err(e:impl std::fmt::Display)->String{e.to_string()}
pub fn serve<R:BufRead,W:Write>(store:&KernelStore,authority:&Authority,mut input:R,mut output:W,limit:u64)->Result<()>{
    ensure((1..=1_000_000).contains(&limit),"invalid ingress request limit")?;
    for _ in 0..limit{
        let mut frame=Vec::new();
        loop{
            let buf=input.fill_buf().map_err(err)?;
            if buf.is_empty(){if frame.is_empty(){return Ok(());}return Err("truncated request frame".into());}
            let end=buf.iter().position(|b|*b==b'\n');
            let n=end.map_or(buf.len(),|i|i+1);
            ensure(frame.len().saturating_add(n)<=MAX_VALUE_BYTES*2,"ingress frame exceeds budget")?;
            frame.extend_from_slice(&buf[..n]);input.consume(n);
            if end.is_some(){break;}
        }
        let result=(||->Result<Value>{
            let signed:SignedRequest=serde_json::from_slice(&frame).map_err(|_|"invalid request frame")?;
            let now=u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_millis()).map_err(err)?;
            store.apply_signed(authority,signed,now)
        })();
        let response=match result{Ok(value)=>json!({"ok":true,"result":value}),Err(_)=>json!({"ok":false,"error":"request rejected"})};
        serde_json::to_writer(&mut output,&response).map_err(err)?;
        output.write_all(b"\n").map_err(err)?;output.flush().map_err(err)?;
    }
    Ok(())
}
