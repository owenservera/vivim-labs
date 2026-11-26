use std::{
    collections::BTreeMap,
    io::{self, BufRead, Read},
    net::TcpStream,
};
use universal_stream_parser::{Format, Goal, Locator, Parser, browser};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a = std::env::args().skip(1);
    let mode = a
        .next()
        .ok_or("usage: universal-stream-parser stdin|tcp|cdp <goal> [host:port]")?;
    let mut loc = Locator::new(Goal::new(&a.next().ok_or("missing goal")?));
    let mut emit = |source: &str,
                    records: Vec<universal_stream_parser::Record>|
     -> Result<(), Box<dyn std::error::Error>> {
        for r in records {
            for h in loc.locate(source, &r) {
                println!("{}", serde_json::to_string(&h)?);
            }
        }
        Ok(())
    };
    let limit = 8 * 1024 * 1024;
    if mode == "cdp" {
        let mut streams = BTreeMap::new();
        for line in io::stdin().lock().lines() {
            let v: serde_json::Value = serde_json::from_str(&line?)?;
            if matches!(
                v["method"].as_str(),
                Some(
                    "Network.loadingFinished" | "Network.loadingFailed" | "Network.webSocketClosed"
                )
            ) {
                if let Some(id) = v["params"]["requestId"].as_str() {
                    if let Some(mut p) = streams.remove(id) {
                        let p: &mut Parser = &mut p;
                        emit(id, p.finish()?)?;
                    }
                }
                continue;
            }
            if let Some(c) = browser::decode_event(&v)? {
                if c.message_boundary {
                    let mut p = Parser::new(Format::Opaque, limit);
                    p.feed(&c.bytes)?;
                    emit(&c.source, p.finish()?)?
                } else {
                    if !streams.contains_key(&c.source) && streams.len() >= 1024 {
                        return Err("stream limit".into());
                    }
                    let p = streams
                        .entry(c.source.clone())
                        .or_insert_with(|| Parser::new(Format::Auto, limit));
                    emit(&c.source, p.feed(&c.bytes)?)?;
                }
            }
        }
        for (id, mut p) in streams {
            emit(&id, p.finish()?)?
        }
    } else {
        let mut input: Box<dyn Read> = match mode.as_str() {
            "stdin" => Box::new(io::stdin()),
            "tcp" => Box::new(TcpStream::connect(a.next().ok_or("missing host:port")?)?),
            _ => return Err("unknown mode".into()),
        };
        let mut p = Parser::new(Format::Auto, limit);
        let mut b = [0; 16384];
        loop {
            let n = input.read(&mut b)?;
            if n == 0 {
                break;
            }
            emit(&mode, p.feed(&b[..n])?)?;
        }
        emit(&mode, p.finish()?)?;
    }
    eprintln!("learned source and schema paths: {:?}", loc.learned());
    Ok(())
}
