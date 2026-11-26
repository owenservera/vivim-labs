//! P01 isolated contract tests; fixture inputs, never evidence of live Host actions.
use serde_json::{json, Value};
use vivim_programmability_core::{control_planning, control_plan_presentation};
fn inert(result: &Value) {
    assert_eq!(result["authority"], "none");
    assert_eq!(result["executionAuthorized"], false);
    assert_eq!(result["effectPerformed"], false);
}
fn action() -> Value {
    json!({"id":"fixture.inspect","title":"Inspect","target":"fixture","risk":"R0",
        "readiness":"source_wired","reversible":false,"aiCandidate":true,
        "args":{"name":{"type":"string","min":1,"max":2}}})
}
fn typed() -> Value {
    json!({"schema":"vivim.control-typed-plan-input/1","source":"desktop","origin":"fixture",
        "actionId":"fixture.inspect","action":action(),"args":{"name":"😀"}})
}
fn candidate() -> Value {
    json!({"schema":"vivim.control-candidate-plan-input/1","source":"desktop",
        "action":action(),"candidate":{"actionId":"fixture.inspect","args":{"name":"a"},"uncertainty":0.5}})
}
#[test]
fn typed_utf16_and_inert_plan() {
    let mut input=typed();
    let result=control_planning::typed(&input);
    inert(&result);
    assert_eq!(result["outcome"],"planned");
    assert_eq!(result["plan"]["outcome"],"ready");
    assert_eq!(result["plan"]["executionPerformed"],false);
    input["args"]["name"]=json!("😀a");
    let refused=control_planning::typed(&input);
    inert(&refused);
    assert_eq!(refused["code"],"CONTROL_ARGS_INVALID");
}
#[test]
fn typed_unknown_and_unavailable_fail_closed() {
    let mut input=typed();
    input["actionId"]=json!("fixture.other");
    let r=control_planning::typed(&input);
    inert(&r);
    assert_eq!(r["code"],"CONTROL_ACTION_UNKNOWN");
    input=typed();
    input["action"]["readiness"]=json!("adapter_pending");
    let r=control_planning::typed(&input);
    inert(&r);
    assert_eq!(r["plan"]["outcome"],"unavailable");
}
#[test]
fn candidate_needs_approval_and_cannot_inject_authority() {
    let mut input=candidate();
    let r=control_planning::candidate(&input);
    inert(&r);
    assert_eq!(r["plan"]["outcome"],"approval_required");
    assert_eq!(r["plan"]["requiresApproval"],true);
    input["candidate"]["approved"]=json!(true);
    let r=control_planning::candidate(&input);
    inert(&r);
    assert_eq!(r["code"],"CONTROL_CANDIDATE_INVALID");
    input=candidate();
    input["action"]["aiCandidate"]=json!(false);
    let r=control_planning::candidate(&input);
    inert(&r);
    assert_eq!(r["code"],"CONTROL_CANDIDATE_REFUSED");
}
#[test]
fn pure_projection_is_not_owner_review() {
    let planned=control_planning::candidate(&candidate());
    let r=control_plan_presentation::project(&json!({"plan":planned["plan"]}));
    inert(&r);
    assert_eq!(r["code"],"INVALID_CONTROL_PLAN");
}
fn bound_plan() -> Value {
    json!({"schema":"vivim.control-plan/1","source":"desktop","outcome":"approval_required",
        "actionId":"fixture.inspect","risk":"R1","planId":"ctl-0123456789abcdef",
        "planDigest":"a".repeat(64),"expiresAt":"fixture-expiry","args":{},
        "approval":{"required":true,"channels":["desktop-owner-token"]},"extra":{"value":7}})
}
#[test]
fn owner_review_preserves_payload_but_is_not_approval() {
    let mut plan=bound_plan();
    let r=control_plan_presentation::project(&json!({"plan":plan}));
    inert(&r);
    assert_eq!(r["outcome"],"computed");
    let text=r["text"].as_str().unwrap();
    let payload=text.split_once("COMPLETE HOST RESPONSE (UNMODIFIED VALUES)\n").unwrap().1;
    assert_eq!(serde_json::from_str::<Value>(payload).unwrap(),plan);
    plan["planDigest"]=json!("A".repeat(64));
    let r=control_plan_presentation::project(&json!({"plan":plan}));
    inert(&r);
    assert_eq!(r["code"],"INVALID_CONTROL_PLAN");
}
#[test]
fn receipt_must_match_binding_and_execution_state() {
    let binding=bound_plan();
    let mut receipt=json!({"schema":"vivim.control-receipt/1","planId":binding["planId"],
        "planDigest":binding["planDigest"],"actionId":binding["actionId"],"source":"desktop",
        "status":"applied","executionPerformed":true});
    let r=control_plan_presentation::receipt_project(&json!({"receipt":receipt,"binding":binding}));
    inert(&r);
    assert_eq!(r["outcome"],"computed");
    receipt["executionPerformed"]=json!(false);
    let r=control_plan_presentation::receipt_project(&json!({"receipt":receipt,"binding":binding}));
    inert(&r);
    assert_eq!(r["code"],"INVALID_CONTROL_RECEIPT");
    receipt["executionPerformed"]=json!(true);
    receipt["planDigest"]=json!("b".repeat(64));
    let r=control_plan_presentation::receipt_project(&json!({"receipt":receipt,"binding":binding}));
    inert(&r);
    assert_eq!(r["code"],"INVALID_CONTROL_RECEIPT");
}
#[test]
fn inspection_is_read_only_and_exact_id_bound() {
    let id="ctl-0123456789abcdef";
    let r=control_plan_presentation::inspection_project(&json!({"planId":id,"record":{"planId":id,"state":"approved"}}));
    inert(&r);
    assert_eq!(r["outcome"],"computed");
    let r=control_plan_presentation::inspection_project(&json!({"planId":id,"record":{"planId":"ctl-ffffffffffffffff","state":"approved"}}));
    inert(&r);
    assert_eq!(r["code"],"INVALID_CONTROL_INSPECTION");
}
