use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::Value;
#[derive(Debug)]
pub struct Chunk {
    pub source: String,
    pub bytes: Vec<u8>,
    pub message_boundary: bool,
}
pub fn decode_event(v: &Value) -> Result<Option<Chunk>, String> {
    let p = &v["params"];
    let id = match p["requestId"].as_str() {
        Some(s) => s,
        None => return Ok(None),
    };
    let method = v["method"].as_str().unwrap_or("");
    let (bytes, boundary) = match method {
        "Network.dataReceived" => {
            let Some(s) = p["data"].as_str() else {
                return Ok(None);
            };
            (STANDARD.decode(s).map_err(|e| e.to_string())?, false)
        }
        "Network.webSocketFrameReceived" => {
            let r = &p["response"];
            let s = r["payloadData"].as_str().ok_or("missing payload")?;
            let bytes = if r["opcode"].as_u64() == Some(1) {
                s.as_bytes().to_vec()
            } else {
                STANDARD.decode(s).map_err(|e| e.to_string())?
            };
            (bytes, true)
        }
        "Network.eventSourceMessageReceived" => {
            let data = p["data"].as_str().unwrap_or("");
            let val = serde_json::from_str::<Value>(data).unwrap_or(Value::String(data.into()));
            (
                serde_json::to_vec(
                    &serde_json::json!({"event":p["eventName"],"id":p["eventId"],"data":val}),
                )
                .map_err(|e| e.to_string())?,
                true,
            )
        }
        _ => return Ok(None),
    };
    Ok(Some(Chunk {
        source: id.into(),
        bytes,
        message_boundary: boundary,
    }))
}
