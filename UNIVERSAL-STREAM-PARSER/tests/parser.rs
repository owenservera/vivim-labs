use universal_stream_parser::{Format, Goal, Locator, Parser, browser};
#[test]
fn fragmented_json() {
    let b = br#"{"result":{"answer":"hello"}}{"result":{"answer":"world"}}"#;
    for split in 0..=b.len() {
        let mut p = Parser::new(Format::Auto, 1024);
        let mut r = p.feed(&b[..split]).unwrap();
        r.extend(p.feed(&b[split..]).unwrap());
        r.extend(p.finish().unwrap());
        assert_eq!(r.len(), 2);
        assert_eq!(r[1].end, b.len() as u64);
    }
}
#[test]
fn sse_every_byte() {
    let mut p = Parser::new(Format::Auto, 1024);
    let mut r = vec![];
    for b in b"data: {\"delta\":\"hi\"}\r\n\r\ndata: bye\n\n" {
        r.extend(p.feed(&[*b]).unwrap())
    }
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].value["data"]["delta"], "hi");
}
#[test]
fn binary_frames() {
    let mut p = Parser::new(Format::Auto, 1024);
    let r = p.feed(b"\0\0\0\x03one\0\0\0\x03two").unwrap();
    assert_eq!(r.len(), 2);
    assert_eq!(r[1].value, "two");
}
#[test]
fn locate_path() {
    let mut p = Parser::new(Format::Auto, 1024);
    let r = p
        .feed(br#"{"choices":[{"delta":{"content":"hi"}}],"usage":42}"#)
        .unwrap();
    let mut l = Locator::new(Goal::new("find answer text"));
    let h = l.locate("stream-1", &r[0]);
    assert_eq!(h[0].schema_path, "/choices/*/delta/content");
    assert_eq!(h[0].value, "hi");
}
#[test]
fn browser_envelopes() {
    let c=browser::decode_event(&serde_json::json!({"method":"Network.dataReceived","params":{"requestId":"a","data":"aGk="}})).unwrap().unwrap();
    assert_eq!(c.bytes, b"hi");
    assert!(!c.message_boundary);
}
#[test]
fn bounds_and_incomplete() {
    let mut p = Parser::new(Format::Json, 8);
    assert!(p.feed(b"123456789").is_err());
    assert!(p.feed(b"{").unwrap().is_empty());
    assert!(p.finish().is_err());
}
#[test]
fn real_loopback_tcp() {
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
    };
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = l.local_addr().unwrap();
    let t = std::thread::spawn(move || {
        let (mut s, _) = l.accept().unwrap();
        s.write_all(b"data: {\"content\":").unwrap();
        s.write_all(b"\"hello\"}\n\n").unwrap();
    });
    let mut s = TcpStream::connect(addr).unwrap();
    let mut p = Parser::new(Format::Auto, 1024);
    let mut r = vec![];
    let mut b = [0; 3];
    loop {
        let n = s.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        r.extend(p.feed(&b[..n]).unwrap());
    }
    r.extend(p.finish().unwrap());
    t.join().unwrap();
    assert_eq!(r[0].value["data"]["content"], "hello");
}
