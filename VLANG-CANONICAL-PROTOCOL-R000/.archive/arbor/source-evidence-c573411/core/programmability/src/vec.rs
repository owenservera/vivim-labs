//! Pure VEC visual IR: explicit rendering semantics over the Host-admitted
//! TierFlow read model. No Host actions, grants, external data, UI mutation.
//! @hq:capability vivim.native-vec-visual-ir@1
//! @hq:consumes vivim.native-visual-validation@1
//! @hq:consumes vivim.ui-render-plan@1
//! @hq:consumes vivim.rust-ui-value@1
use crate::visual::{self, Visual, Node, VisualError};
use serde::Serialize;
use serde_json::{Value,json};
use std::collections::{BTreeSet,BTreeMap};
use sha2::{Digest,Sha256};
pub const SCHEMA:&str="vivim.vec-visual-ir/1";
#[derive(Clone,Debug,Serialize,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Style{
    pub background:Option<String>,pub foreground:Option<String>,
    pub border_color:Option<String>,pub border_width_px:f32,
    pub padding_px:f32,pub gap_px:f32,
    pub radius_px:Option<f32>,pub font_size_px:Option<f32>,
}
#[derive(Clone,Debug,Serialize,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Bar{
    pub label:String,pub value:f64,pub zero:f64,pub start:f64,pub extent:f64,
}
#[derive(Clone,Debug,Serialize,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Element{
    pub id:String,pub component_id:String,pub kind:String,
    pub role:&'static str,pub layout:&'static str,pub columns:u8,
    pub label:Option<String>,pub detail:Option<String>,
    pub style:Style,pub items:Vec<String>,pub bars:Vec<Bar>,
    pub children:Vec<Element>,
    /// A button is a display-only preview here, not an executable action.
    pub preview_only:bool,
}
#[derive(Clone,Debug,Serialize,PartialEq,Eq)]
#[serde(rename_all="camelCase")]
pub struct BindingRevision {pub id:String,pub revision:u64,pub digest:String}
#[derive(Clone,Debug,Serialize,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct View{
    pub schema:&'static str,pub state:&'static str,pub authority:&'static str,
    pub surface_id:String,pub revision:u64,
    /// SHA256 of the exact admitted Host design; not authorization.
    pub design_digest:String,
    /// Per-binding revision/digest metadata retained only in native memory.
    #[serde(skip_serializing)]
    pub binding_watermarks:BTreeMap<String,(u64,String)>,
    /// Source revision and digest only, never the bound content or an approval.
    pub binding_revisions:Vec<BindingRevision>,
    pub background:String,pub foreground:String,
    pub secondary:String,pub accent:String,
    pub root:Element,pub node_count:usize,pub chart_count:usize,
    pub preview_action_ids:Vec<String>,pub live_binding_ids:Vec<String>,
    pub execution_authorized:bool,pub effect_performed:bool,
}
fn hex(n:u32)->String{format!("#{n:06x}")}
// The same typed color grammar now backs VEC native style serialization.
fn typed_color(rgb:u32)->Result<String,VisualError>{
    let checked=crate::ui_value::validate(&json!({"type":"color","value":hex(rgb)}));
    if checked["ok"]!=true{return Err(VisualError::InvalidProjection)}
    checked["value"].as_str().map(str::to_owned).ok_or(VisualError::InvalidProjection)
}
fn scalar(v:f32)->bool{v.is_finite()&&v>=0.&&v<=128.}
fn element(node:&Node,view:&Visual,seen:&mut BTreeSet<String>,
           actions:&mut Vec<String>,charts:&mut usize,depth:usize)->Result<Element,VisualError>{
    if depth>7||seen.len()>=64||node.children.len()>24||node.columns==0||node.columns>4
       ||!seen.insert(node.id.clone()){return Err(VisualError::InvalidProjection)}
    let role=match node.kind.as_str(){
        "stack"|"grid"=>"layout","text"=>"text",
        "card"|"profile-card"|"project-card"=>"group",
        "button"=>"preview-action","collection"|"timeline"=>"list",
        "chart"=>"chart",_=>return Err(VisualError::InvalidProjection),
    };
    let card=matches!(node.kind.as_str(),"card"|"profile-card"|"project-card");
    let bg=node.background.or(if card{Some(view.secondary)}else{None});
    let padding=node.padding.unwrap_or(if card{12.}else{2.});
    let gap=node.gap.unwrap_or(8.);
    let border_width=node.border_width.unwrap_or(if node.border.is_some(){1.}else{0.});
    if !scalar(padding)||!scalar(gap)||!scalar(border_width)
        ||node.radius.is_some_and(|x|!scalar(x))
        ||node.font_size.is_some_and(|x|!scalar(x)){return Err(VisualError::InvalidProjection)}
    let style=Style{
        background:bg.map(typed_color).transpose()?,
        foreground:bg.map(|x|typed_color(visual::contrast_ink(x))).transpose()?,
        border_color:if border_width>0.{Some(typed_color(node.border.unwrap_or(view.accent))?)}else{None},
        border_width_px:border_width,padding_px:padding,gap_px:gap,
        radius_px:node.radius,font_size_px:node.font_size,
    };
    let layout=if node.kind=="grid"{"grid"}else if node.horizontal{"row"}
        else if node.children.is_empty(){"intrinsic"}else{"column"};
    let preview_only=node.kind=="button";
    if preview_only{
        if node.label.as_deref().unwrap_or("").is_empty(){
            return Err(VisualError::InvalidProjection)
        }
        actions.push(node.id.clone());
    }
    if node.kind=="chart"{*charts+=1;}
    let bars=node.bars.iter().map(|b|Bar{
        label:b.label.clone(),value:b.value,zero:b.zero,start:b.start,extent:b.extent
    }).collect();
    let mut children=Vec::with_capacity(node.children.len());
    for child in &node.children{
        children.push(element(child,view,seen,actions,charts,depth+1)?);
    }
    Ok(Element{id:node.id.clone(),component_id:node.component.clone(),
        kind:node.kind.clone(),role,layout,columns:node.columns as u8,
        label:node.label.clone(),detail:node.detail.clone(),
        style,items:node.items.clone(),bars,children,preview_only})
}
pub fn compile(view:&Visual)->Result<View,VisualError>{
    let mut seen=BTreeSet::new();let mut actions=Vec::new();let mut charts=0;
    let root=element(&view.root,view,&mut seen,&mut actions,&mut charts,0)?;
    let binding_revisions=view.binding_watermarks.iter().map(|(id,(revision,digest))|
        BindingRevision{id:id.clone(),revision:*revision,digest:digest.clone()}).collect();
    Ok(View{schema:SCHEMA,state:"compiled",authority:"none",
        surface_id:view.surface_id.clone(),revision:view.revision,
        design_digest:format!("{:x}",Sha256::digest(view.fingerprint.as_bytes())),
        binding_watermarks:view.binding_watermarks.clone(),binding_revisions,
        background:typed_color(view.background)?,foreground:typed_color(visual::contrast_ink(view.background))?,
        secondary:typed_color(view.secondary)?,accent:typed_color(view.accent)?,
        root,node_count:seen.len(),chart_count:charts,preview_action_ids:actions,
        live_binding_ids:view.binding_watermarks.keys().cloned().collect(),
        execution_authorized:false,effect_performed:false})
}
/// Strict JSON entry point. The original Host UI revision and TierFlow plan
/// must first pass the existing native visual admission logic.
pub fn compile_projection(projection:&Value)->Result<Value,VisualError>{
    match visual::parse(projection)?{
        Some(view)=>serde_json::to_value(compile(&view)?)
            .map_err(|_|VisualError::InvalidProjection),
        None=>Ok(json!({"schema":SCHEMA,"state":"absent","authority":"none",
            "executionAuthorized":false,"effectPerformed":false})),
    }
}

/// Source-derived change-set, not a Host UI mutation or GPU partial-draw claim.
pub const DELTA_SCHEMA:&str="vivim.vec-change-set/1";
/// Source-identity-only transition, no private bound value or Host effect.
#[derive(Clone,Debug,Serialize,PartialEq,Eq)]
#[serde(rename_all="camelCase")]
pub struct BindingTransition {
    pub id:String,pub from_revision:Option<u64>,pub to_revision:Option<u64>,
    pub from_digest:Option<String>,pub to_digest:Option<String>,
}
#[derive(Clone,Debug,Serialize,PartialEq,Eq)]
#[serde(rename_all="camelCase")]
pub struct Delta{
    pub schema:&'static str,pub surface_id:Option<String>,
    pub from_revision:Option<u64>,pub to_revision:Option<u64>,
    pub added:Vec<String>,pub removed:Vec<String>,pub updated:Vec<String>,
    pub moved:Vec<String>,pub reordered:Vec<String>,
    pub changed_bindings:Vec<String>,pub binding_transitions:Vec<BindingTransition>,
    pub theme_changed:bool,pub revision_changed:bool,pub repaint_required:bool,
}
fn flatten_ir<'a>(node:&'a Element,parent:Option<&'a str>,index:usize,
    out:&mut BTreeMap<&'a str,(&'a Element,Option<&'a str>,usize)>)->Result<(),VisualError>{
    if out.len()>=64||out.insert(node.id.as_str(),(node,parent,index)).is_some(){
        return Err(VisualError::InvalidProjection)
    }
    for(i,child)in node.children.iter().enumerate(){
        flatten_ir(child,Some(&node.id),i,out)?;
    }
    Ok(())
}
fn same_local(a:&Element,b:&Element)->bool{
    let mut a=a.clone();let mut b=b.clone();a.children.clear();b.children.clear();a==b
}
pub fn diff(previous:Option<&View>,next:Option<&View>)->Result<Delta,VisualError>{
    if let(Some(a),Some(b))=(previous,next){
        if a.surface_id!=b.surface_id||a.revision>b.revision||
            (a.revision==b.revision&&a.design_digest!=b.design_digest){
            return Err(VisualError::InvalidProjection)
        }
        // Live bindings advance independently of the Host design revision.
        // The session RevisionGuard retains high marks even through outages.
        for (id,(old_revision,old_digest)) in &a.binding_watermarks {
            if let Some((new_revision,new_digest))=b.binding_watermarks.get(id) {
                if new_revision<old_revision||
                    (new_revision==old_revision&&new_digest!=old_digest){
                    return Err(VisualError::InvalidProjection)
                }
            }
        }
    }
    let mut old=BTreeMap::new();let mut new=BTreeMap::new();
    if let Some(v)=previous{flatten_ir(&v.root,None,0,&mut old)?}
    if let Some(v)=next{flatten_ir(&v.root,None,0,&mut new)?}
    let mut out=Delta{
        schema:DELTA_SCHEMA,
        surface_id:next.or(previous).map(|v|v.surface_id.clone()),
        from_revision:previous.map(|v|v.revision),to_revision:next.map(|v|v.revision),
        added:Vec::new(),removed:Vec::new(),updated:Vec::new(),
        moved:Vec::new(),reordered:Vec::new(),changed_bindings:Vec::new(),
        binding_transitions:Vec::new(),
        theme_changed:previous.map(|v|(&v.background,&v.foreground,&v.secondary,&v.accent))
            !=next.map(|v|(&v.background,&v.foreground,&v.secondary,&v.accent)),
        revision_changed:previous.map(|v|v.revision)!=next.map(|v|v.revision),
        repaint_required:false
    };
    for(id,(n,parent,index))in &new{
        match old.get(id){
            None=>out.added.push((*id).to_owned()),
            Some((prior,old_parent,old_index))=>{
                if !same_local(prior,n){out.updated.push((*id).to_owned())}
                if old_parent!=parent||old_index!=index{out.moved.push((*id).to_owned())}
                let before=prior.children.iter().map(|n|n.id.as_str()).collect::<Vec<_>>();
                let after=n.children.iter().map(|n|n.id.as_str()).collect::<Vec<_>>();
                if before!=after&&before.iter().copied().collect::<BTreeSet<_>>()
                    ==after.iter().copied().collect::<BTreeSet<_>>(){
                    out.reordered.push((*id).to_owned());
                }
            }
        }
    }
    for id in old.keys(){if !new.contains_key(id){out.removed.push((*id).to_owned())}}
    let mut keys=BTreeSet::new();
    if let Some(v)=previous{keys.extend(v.binding_watermarks.keys().cloned())}
    if let Some(v)=next{keys.extend(v.binding_watermarks.keys().cloned())}
    for id in keys{
        let before=previous.and_then(|v|v.binding_watermarks.get(&id));
        let after=next.and_then(|v|v.binding_watermarks.get(&id));
        if before!=after{
            out.changed_bindings.push(id.clone());
            out.binding_transitions.push(BindingTransition{
                id,from_revision:before.map(|v|v.0),to_revision:after.map(|v|v.0),
                from_digest:before.map(|v|v.1.clone()),to_digest:after.map(|v|v.1.clone())
            });
        }
    }
    out.repaint_required=out.theme_changed||out.revision_changed||
        !out.added.is_empty()||!out.removed.is_empty()||!out.updated.is_empty()||
        !out.moved.is_empty()||!out.changed_bindings.is_empty();
    Ok(out)
}

#[cfg(test)]
mod vec_revision_tests{
    use super::*;
    fn observed()->Visual{
        Visual{surface_id:"my-space".into(),revision:7,background:0x111722,
            secondary:0x243146,accent:0x78b8f6,fingerprint:"host-design".into(),
            root:Node{id:"root".into(),component:"vivim.component.stack@1".into(),
                kind:"stack".into(),label:None,detail:None,horizontal:false,columns:1,
                background:None,padding:None,gap:None,radius:None,font_size:None,
                border:None,border_width:None,children:vec![],items:vec![],bars:vec![]},
            binding_watermarks:BTreeMap::from([
                ("personal.open-tasks".into(),(11,"a".repeat(64)))])}
    }
    #[test]fn ir_exports_revision_stamps_without_host_authority(){
        let ir=compile(&observed()).unwrap();
        let value=serde_json::to_value(&ir).unwrap();
        assert_eq!(value["bindingRevisions"][0]["id"],"personal.open-tasks");
        assert_eq!(value["bindingRevisions"][0]["revision"],11);
        assert_eq!(value["bindingRevisions"][0]["digest"],"a".repeat(64));
        assert_eq!(value["background"],"#111722");
        assert_eq!(value["executionAuthorized"],false);
        assert_eq!(value["effectPerformed"],false);
        assert!(value.get("bindingWatermarks").is_none());
    }
    #[test]fn live_revisions_invalidate_without_design_changes(){
        let old=compile(&observed()).unwrap();
        let mut next=observed();
        next.binding_watermarks.insert("personal.open-tasks".into(),(12,"b".repeat(64)));
        let updated=compile(&next).unwrap();
        let change=diff(Some(&old),Some(&updated)).unwrap();
        assert_eq!(change.changed_bindings,vec!["personal.open-tasks"]);
        assert_eq!(change.binding_transitions.len(),1);
        assert_eq!(change.binding_transitions[0].from_revision,Some(11));
        assert_eq!(change.binding_transitions[0].to_revision,Some(12));
        assert_eq!(change.binding_transitions[0].from_digest.as_deref(),Some("a".repeat(64).as_str()));
        assert_eq!(change.binding_transitions[0].to_digest.as_deref(),Some("b".repeat(64).as_str()));
        let wire=serde_json::to_value(&change).unwrap();
        assert_eq!(wire["bindingTransitions"][0]["toRevision"],12);
        assert_eq!(wire["bindingTransitions"][0]["fromDigest"],"a".repeat(64));
        assert_eq!(wire["bindingTransitions"][0]["toDigest"],"b".repeat(64));
        assert!(wire.get("boundContent").is_none());
        assert!(!change.revision_changed);
        assert!(change.repaint_required);
        let mut stale=updated.clone();stale.revision=6;
        assert!(diff(Some(&updated),Some(&stale)).is_err());
        let mut rollback=updated.clone();
        rollback.binding_watermarks.insert("personal.open-tasks".into(),(11,"a".repeat(64)));
        assert!(diff(Some(&updated),Some(&rollback)).is_err());
        let mut conflict=updated.clone();
        conflict.binding_watermarks.insert("personal.open-tasks".into(),(12,"c".repeat(64)));
        assert!(diff(Some(&updated),Some(&conflict)).is_err());
        assert!(diff(Some(&updated),Some(&updated)).unwrap().changed_bindings.is_empty());
    }
}
