use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Format {
    Auto,
    Json,
    Sse,
    Lines,
    Length32Be,
    Opaque,
}
#[derive(Debug, Serialize)]
pub struct Record {
    pub start: u64,
    pub end: u64,
    pub format: Format,
    pub value: Value,
}
pub struct Parser {
    buf: Vec<u8>,
    offset: u64,
    format: Format,
    limit: usize,
}
impl Parser {
    pub fn new(format: Format, limit: usize) -> Self {
        Self {
            buf: vec![],
            offset: 0,
            format,
            limit: limit.max(8),
        }
    }
    pub fn pending(&self) -> usize {
        self.buf.len()
    }
    pub fn format(&self) -> Format {
        self.format
    }
    pub fn feed(&mut self, bytes: &[u8]) -> Result<Vec<Record>, String> {
        if bytes.len() > self.limit - self.buf.len() {
            return Err("buffer limit".into());
        }
        self.buf.extend_from_slice(bytes);
        self.drain(false)
    }
    pub fn finish(&mut self) -> Result<Vec<Record>, String> {
        self.drain(true)
    }
    fn detect(&self, eof: bool) -> Format {
        let b = &self.buf[..];
        let x = b
            .iter()
            .position(|v| !v.is_ascii_whitespace())
            .map(|i| &b[i..])
            .unwrap_or(&[]);
        if x.is_empty() {
            return if eof { Format::Lines } else { Format::Auto };
        }
        if matches!(x[0], b'{' | b'[') {
            return Format::Json;
        }
        if [b"data:".as_slice(), b"event:", b"id:", b"retry:", b":"]
            .iter()
            .any(|p| x.starts_with(p))
        {
            return Format::Sse;
        }
        if b.len() >= 4 && b[0] == 0 {
            let n = u32::from_be_bytes(b[..4].try_into().unwrap()) as usize;
            if n > 0 && n <= self.limit - 4 && b.len() >= n + 8 {
                let m = u32::from_be_bytes(b[n + 4..n + 8].try_into().unwrap()) as usize;
                if m > 0 && m <= self.limit - 4 && b.len() >= n + 8 + m {
                    return Format::Length32Be;
                }
            }
            return if eof { Format::Opaque } else { Format::Auto };
        }
        if b.contains(&b'\n') || eof {
            if std::str::from_utf8(b).is_ok() {
                Format::Lines
            } else {
                Format::Opaque
            }
        } else {
            Format::Auto
        }
    }
    fn drain(&mut self, eof: bool) -> Result<Vec<Record>, String> {
        let mut out = vec![];
        if self.format == Format::Auto {
            self.format = self.detect(eof)
        }
        let mut consumed = 0;
        loop {
            let b = &self.buf[consumed..];
            if b.is_empty() {
                break;
            }
            let next = next_record(self.format, b, eof, self.limit)?;
            match next {
                Some((n, value, emit)) => {
                    let start = self.offset + consumed as u64;
                    consumed += n;
                    if emit {
                        out.push(Record {
                            start,
                            end: self.offset + consumed as u64,
                            format: self.format,
                            value,
                        })
                    }
                }
                None => break,
            }
        }
        if eof && consumed != self.buf.len() {
            return Err(format!(
                "incomplete {:?} at {}",
                self.format,
                self.offset + consumed as u64
            ));
        }
        self.buf.drain(..consumed);
        self.offset += consumed as u64;
        Ok(out)
    }
}
fn decode(b: &[u8]) -> Value {
    use base64::Engine;
    serde_json::from_slice(b).unwrap_or_else(|_|match std::str::from_utf8(b){Ok(s)=>Value::String(s.into()),Err(_)=>serde_json::json!({"encoding":"base64","bytes":base64::engine::general_purpose::STANDARD.encode(b)})})
}

fn next_record(
    f: Format,
    b: &[u8],
    eof: bool,
    limit: usize,
) -> Result<Option<(usize, Value, bool)>, String> {
    Ok(match f {
        Format::Auto => None,
        Format::Json => {
            if b.iter().all(|v| v.is_ascii_whitespace()) {
                if eof {
                    Some((b.len(), Value::Null, false))
                } else {
                    None
                }
            } else {
                let mut it = serde_json::Deserializer::from_slice(b).into_iter::<Value>();
                match it.next() {
                    Some(Ok(v)) => {
                        let n = it.byte_offset();
                        if !eof && n == b.len() && (v.is_number() || v.is_boolean() || v.is_null())
                        {
                            None
                        } else {
                            Some((n, v, true))
                        }
                    }
                    Some(Err(e)) if e.is_eof() && !eof => None,
                    Some(Err(e)) => return Err(e.to_string()),
                    None => None,
                }
            }
        }
        Format::Sse => {
            let mut pos = 0;
            let mut end = None;
            while let Some(i) = b[pos..].iter().position(|v| *v == b'\n') {
                let line = &b[pos..pos + i];
                pos += i + 1;
                if line.is_empty() || line == b"\r" {
                    end = Some(pos);
                    break;
                }
            }
            if let Some(n) = end {
                let s = std::str::from_utf8(&b[..n]).map_err(|e| e.to_string())?;
                let mut data = vec![];
                let mut event = "message";
                let mut id = None;
                for line in s.lines() {
                    let (k, v) = line.split_once(':').unwrap_or((line, ""));
                    let v = v.strip_prefix(' ').unwrap_or(v);
                    match k {
                        "data" => data.push(v),
                        "event" => event = v,
                        "id" => id = Some(v),
                        _ => {}
                    }
                }
                let payload = data.join("\n");
                let val = serde_json::from_str(&payload).unwrap_or(Value::String(payload));
                Some((
                    n,
                    serde_json::json!({"event":event,"id":id,"data":val}),
                    !data.is_empty(),
                ))
            } else {
                None
            }
        }
        Format::Lines => {
            let n = b
                .iter()
                .position(|v| *v == b'\n')
                .map(|i| i + 1)
                .or(if eof { Some(b.len()) } else { None });
            match n {
                Some(n) => {
                    let s = std::str::from_utf8(&b[..n])
                        .map_err(|e| e.to_string())?
                        .trim_end_matches(['\r', '\n']);
                    Some((
                        n,
                        serde_json::from_str(s).unwrap_or_else(|_| Value::String(s.into())),
                        !s.is_empty(),
                    ))
                }
                None => None,
            }
        }
        Format::Length32Be => {
            if b.len() < 4 {
                None
            } else {
                let n = u32::from_be_bytes(b[..4].try_into().unwrap()) as usize;
                if n > limit - 4 {
                    return Err("length exceeds limit".into());
                }
                if b.len() < 4 + n {
                    None
                } else {
                    Some((4 + n, decode(&b[4..4 + n]), true))
                }
            }
        }
        Format::Opaque => {
            if eof {
                Some((b.len(), decode(b), true))
            } else {
                None
            }
        }
    })
}
