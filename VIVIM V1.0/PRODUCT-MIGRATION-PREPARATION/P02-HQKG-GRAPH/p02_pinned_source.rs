//! P02 Labs-only source-bound import regression. Overlay into exact donor tests/.
//! Uses actual pinned source text; changes only an in-memory candidate, not ARBOR.
#![cfg(feature = "importer")]
use hqkg::{Graph, Journal, Node, NodeKind};
use hqkg::import::parse;
use hqkg::semantic_source::{sources, impact};
use serde_json::{json, Value};
use sha2::{Digest,Sha256};
const SRC: &str = include_str!("../src/semantic_source.rs");
const LIB: &str = include_str!("../src/lib.rs");
const SEM: &str = "hqkg.semantic-source-query@1";
const SOURCE_PATH: &str = "arbor/native/hqkg/src/semantic_source.rs";
const LIB_PATH: &str = "arbor/native/hqkg/src/lib.rs";
fn digest(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn id(key: &str) -> String { format!("hqg1:{}", digest(key.as_bytes())) }
fn file_id(path: &str) -> String { id(&format!("source-file\0arbor\0{}", path.strip_prefix("arbor/").unwrap())) }
fn node(id: &str, kind: &str, metadata: Value) -> Value {
    json!({"type":"node","id":id,"kind":kind,"title":kind,"metadata":metadata})
}
fn edge(key: &str, from: &str, relation: &str, to: &str) -> Value {
    json!({"type":"edge","id":id(key),"from":from,"to":to,"relation":relation,"weight":1,"metadata":{}})
}
fn packet(rows: Vec<Value>, reconcile: bool) -> Vec<u8> {
    let body: String = rows.iter().map(|r| format!("{r}\n")).collect();
    let count = |k: &str| rows.iter().filter(|r| r["type"].as_str()==Some(k)).count();
    let manifest = json!({"type":"manifest","schema":"repos-hq.hqkg-exchange/1","source":"repos-hq.index",
      "authority":"data-only; no executable code or provider actions","graph_sha256":digest(body.as_bytes()),
      "node_count":count("node"),"edge_count":count("edge"),"tombstone_count":count("tombstone"),"reconcile_edges":reconcile});
    format!("{manifest}\n{body}").into_bytes()
}
fn rows(src: &str, anchor_src: &str, include_import: bool) -> Vec<Value> {
    let f=file_id(SOURCE_PATH); let l=file_id(LIB_PATH); let a=id("p02-anchor"); let s=id("p02-semantic");
    let line=anchor_src.lines().position(|l| l.contains("@hq:capability hqkg.semantic-source-query@1")).unwrap()+1;
    assert!(LIB.contains("pub mod semantic_source;"));
    let mut rows=vec![
      node(&f,"source-file",json!({"source_path":SOURCE_PATH,"content_sha256":digest(src.as_bytes())})),
      node(&l,"source-file",json!({"source_path":LIB_PATH,"content_sha256":digest(LIB.as_bytes())})),
      node(&a,"semantic-file-anchor",json!({"source_path":SOURCE_PATH,"content_sha256":digest(anchor_src.as_bytes()),"semantic_id":SEM,"annotation_kind":"capability","status":"indexed-not-live-checked","first_line":line,"occurrence_count":1})),
      node(&s,"semantic-id",json!({"semantic_id":SEM})),
      edge("p02-anchor-edge",&f,"has-semantic-anchor",&a),
      edge("p02-semantic-edge",&a,"references-semantic-id",&s)];
    if include_import { rows.push(edge("p02-import",&l,"imports",&f)); }
    rows
}
#[test]
fn pinned_bytes_import_query_reconcile_delete() {
    let mut graph=Graph::new();
    graph.put_node(Node::new("manual",NodeKind::Fact,"unrelated owner")).unwrap();
    let initial=packet(rows(SRC,SRC,true),true);
    let e=parse(&initial).unwrap();
    assert!(e.apply(&mut graph).unwrap().changed());
    let epoch=graph.epoch();
    assert!(!e.apply(&mut graph).unwrap().changed());
    assert_eq!(graph.epoch(),epoch);
    assert_eq!(sources(&graph,SEM).unwrap().anchors[0].content_digest,digest(SRC.as_bytes()));
    assert_eq!(impact(&graph,SEM,1).unwrap().files.len(),2);
    let changed=format!("{SRC}\n// P02 isolated candidate revision\n");
    parse(&packet(rows(&changed,SRC,true),true)).unwrap().apply(&mut graph).unwrap();
    assert!(sources(&graph,SEM).is_err());
    parse(&packet(rows(&changed,&changed,false),true)).unwrap().apply(&mut graph).unwrap();
    assert_eq!(sources(&graph,SEM).unwrap().anchors[0].content_digest,digest(changed.as_bytes()));
    assert_eq!(impact(&graph,SEM,1).unwrap().files.len(),1);
    let tombstones=vec![json!({"type":"tombstone","id":file_id(SOURCE_PATH),"reason":"deleted"}),json!({"type":"tombstone","id":id("p02-anchor"),"reason":"deleted"}),json!({"type":"tombstone","id":id("p02-semantic"),"reason":"deleted"})];
    let deletion=parse(&packet(tombstones,false)).unwrap();
    assert!(deletion.apply(&mut graph).unwrap().changed());
    assert!(!deletion.apply(&mut graph).unwrap().changed());
    assert!(sources(&graph,SEM).is_err());
    assert!(graph.node("manual").is_some());
    graph.check_indexes().unwrap();
    let path=std::env::temp_dir().join(format!("p02-pinned-source-{}.hqj",std::process::id()));
    assert!(!path.exists());
    let journal=Journal::new(&path);
    journal.save(&graph).unwrap();
    let loaded=journal.load().unwrap();
    loaded.check_indexes().unwrap();
    assert!(loaded.node("manual").is_some());
    assert!(loaded.node(&file_id(SOURCE_PATH)).is_none());
    std::fs::remove_file(path).unwrap();
}
