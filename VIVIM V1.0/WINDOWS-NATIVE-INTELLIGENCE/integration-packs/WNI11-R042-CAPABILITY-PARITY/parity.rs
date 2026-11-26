//! @hq:context Exercise the incumbent Rust planner and GPUI presentation against actual harvested candidates.
//! @hq:invariant A proposed Windows capability cannot become executable by projection.
use serde_json::{json,Value};
use crate::{control_planning,control_plan_presentation};
#[test]fn all_native_projections_are_inert_and_cross_surface_consistent(){
 let doc:Value=serde_json::from_str(include_str!("native-cases.json")).unwrap();
 assert_eq!(doc["candidateRows"],33);
 let rows=doc["cases"].as_array().unwrap();
 assert_eq!(rows.len(),33);
 let mut ids=std::collections::BTreeSet::new();
 for row in rows{
  let action=&row["action"];let id=action["id"].as_str().unwrap();assert!(ids.insert(id));
  assert_eq!(row["mcp"]["registered"],false);assert_eq!(row["app"]["enabled"],false);
  let args=&row["args"];
  for source in ["text","desktop","mobile","visual"]{
   let input=json!({"schema":"vivim.control-typed-plan-input/1","source":source,"origin":"typed","actionId":id,"action":action,"args":args});
   let result=control_planning::typed(&input);
   assert_eq!(result["outcome"],"planned","{id} {source}: {result}");
   assert_eq!(result["executionAuthorized"],false);assert_eq!(result["effectPerformed"],false);
   let plan=&result["plan"];assert_eq!(plan["outcome"],"unavailable");assert_eq!(plan["args"],*args);
   assert_eq!(plan["requiresApproval"],action["risk"]!="R0");
   if source=="desktop"{
    let detail=control_plan_presentation::project(&json!({"plan":plan}));
    assert_eq!(detail["outcome"],"computed");
    assert_eq!(detail["executionAuthorized"],false);
    assert!(detail["text"].as_str().unwrap().contains("no native approval token"));
   }
   let candidate=control_planning::candidate(&json!({"schema":"vivim.control-candidate-plan-input/1","source":source,"action":action,"candidate":{"actionId":id,"args":args}}));
   assert_eq!(candidate["code"],"CONTROL_CANDIDATE_REFUSED");
  }
  let mut bad=args.clone();bad.as_object_mut().unwrap().insert("request_id".into(),json!("a".repeat(32)));
  let p=control_planning::typed(&json!({"schema":"vivim.control-typed-plan-input/1","source":"desktop","origin":"typed","actionId":id,"action":action,"args":bad}));
  assert_eq!(p["code"],"CONTROL_ARGS_INVALID");
 }
 println!("NATIVE_CANDIDATE_PARITY_OK candidates=33 typed_projections=132 gpui_presentations=33 candidate_refusals=132");
}
