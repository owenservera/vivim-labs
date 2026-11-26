use std::io::{self,BufRead,Write};
use serde_json::{Value,json};
fn emit(v:Value){let mut o=io::stdout().lock();writeln!(o,"{}",v).unwrap();o.flush().unwrap();}
fn model(current:&str)->Value{json!([{"id":"model","category":"model","type":"select","currentValue":current,"options":[{"value":"fixture/a","name":"A"},{"value":"fixture/b","name":"B"}]}])}
fn main(){
 let mut prompt_id=None;
 for l in io::stdin().lock().lines(){
  let l=l.unwrap();let q:Value=serde_json::from_str(&l).unwrap();
  match q["method"].as_str(){
   Some("initialize")=>emit(json!({"jsonrpc":"2.0","id":q["id"],"result":{"protocolVersion":1,"agentCapabilities":{"loadSession":true}}})),
   Some("session/new")=>emit(json!({"jsonrpc":"2.0","id":q["id"],"result":{"sessionId":"s1","configOptions":model("fixture/a")}})),
   Some("session/load")=>{emit(json!({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"s1","update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"historical fixture"}}}}));emit(json!({"jsonrpc":"2.0","id":q["id"],"result":{"configOptions":model("fixture/a")}}));},
   Some("session/set_config_option")=>emit(json!({"jsonrpc":"2.0","id":q["id"],"result":{"configOptions":model(q["params"]["value"].as_str().unwrap())}})),
   Some("session/cancel")=>emit(json!({"jsonrpc":"2.0","id":prompt_id.take().unwrap(),"result":{"stopReason":"cancelled"}})),
   Some("session/prompt")=>{prompt_id=Some(q["id"].clone());if q["params"]["prompt"][0]["text"]=="cancel fixture"{emit(json!({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"s1","update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"pending"}}}}));continue}emit(json!({"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"s1","options":[{"optionId":"allow","kind":"allow_once"}]}}));},
   None if q["id"]==99=>{assert_eq!(q["result"]["outcome"]["outcome"],"cancelled");emit(json!({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"s1","update":{"sessionUpdate":"agent_message_chunk","content":{"type":"text","text":"fixture"}}}}));emit(json!({"jsonrpc":"2.0","id":prompt_id.take().unwrap(),"result":{"stopReason":"end_turn"}}));},
   _=>panic!("unexpected request"),
  }
 }
}
