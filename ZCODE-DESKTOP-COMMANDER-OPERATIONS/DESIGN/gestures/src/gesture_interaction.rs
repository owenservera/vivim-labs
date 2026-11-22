//! Shared, pure gesture negotiation reducer for GPUI and phone PWA.
//! Staged pre-integration code. Neither a gesture nor this reducer authorizes
//! Host effects. The caller must obtain current targets and reviewed choices
//! from an existing trusted source, and revalidate them at the Host boundary.
use std::collections::BTreeSet;

pub const SCHEMA: &str = "vivim.gesture-interaction/1";
pub const MAX_CHOICES: usize = 8;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase { Armed, Revealing, Exploring, Released, Cancelled, Stale, Proposed }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Target {
    pub surface_id: String,
    pub target_id: String,
    pub source_id: String,
    pub source_revision: u64,
    pub source_digest: String,
    pub view_revision: u64,
    pub scope_fingerprint: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Preview {
    pub schema: &'static str,
    pub phase: Phase,
    pub target: Target,
    pub selected: Option<String>,
    pub candidates: Vec<String>,
    pub authorized: bool,
    pub effect_invoked: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InertProposal {
    pub target: Target,
    pub candidate_id: String,
    pub sequence: u64,
    pub host_authorization_required: bool,
    pub authorized: bool,
    pub effect_invoked: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Reject {
    InvalidTarget, InvalidChoices, InvalidHold, InvalidSequence,
    WrongActor, WrongPointer, StaleTarget, NotHeld, NotPreviewing,
    NotReleased, UnknownChoice, Closed,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GestureEvent { Dwell, Move(Option<String>), Release, Cancel }
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input {
    pub actor_id: String,
    pub pointer_id: u64,
    pub sequence: u64,
    pub elapsed_ms: u64,
    pub current_target: Target,
    pub event: GestureEvent,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExplicitApply {
    pub actor_id: String,
    pub sequence: u64,
    pub current_target: Target,
    pub candidate_id: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    target: Target,
    actor_id: String,
    pointer_id: u64,
    sequence: u64,
    elapsed_ms: u64,
    hold_ms: u64,
    choices: Vec<String>,
    selected: Option<String>,
    phase: Phase,
}
fn id_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 &&
        s.bytes().all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
}
fn digest_ok(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
impl Target {
    pub fn valid(&self) -> bool {
        id_ok(&self.surface_id) && id_ok(&self.target_id) &&
        id_ok(&self.source_id) && id_ok(&self.scope_fingerprint) &&
        self.source_revision > 0 && self.view_revision > 0 &&
        digest_ok(&self.source_digest)
    }
}
impl Session {
    /// Choices are supplied by the Host-reviewed semantic catalog, not by the pointer.
    pub fn begin(
        target: Target, actor_id: String, pointer_id: u64, sequence: u64,
        choices: Vec<String>, hold_ms: u64,
    ) -> Result<Self, Reject> {
        if !target.valid() || !id_ok(&actor_id) { return Err(Reject::InvalidTarget); }
        if pointer_id == 0 || sequence == 0 { return Err(Reject::InvalidSequence); }
        if !(250..=900).contains(&hold_ms) { return Err(Reject::InvalidHold); }
        if choices.is_empty() || choices.len() > MAX_CHOICES ||
           choices.iter().any(|s| !id_ok(s)) ||
           choices.iter().collect::<BTreeSet<_>>().len() != choices.len() {
            return Err(Reject::InvalidChoices);
        }
        Ok(Self { target, actor_id, pointer_id, sequence, elapsed_ms: 0,
            hold_ms, choices, selected: None, phase: Phase::Armed })
    }
    pub fn phase(&self) -> Phase { self.phase }
    pub fn preview(&self) -> Preview {
        Preview { schema: SCHEMA, phase: self.phase, target: self.target.clone(),
            selected: self.selected.clone(), candidates: self.choices.clone(),
            authorized: false, effect_invoked: false }
    }
    /// An input transition changes only inert preview state.
    pub fn advance(&mut self, input: &Input) -> Result<Preview, Reject> {
        if matches!(self.phase, Phase::Cancelled | Phase::Stale | Phase::Proposed) {
            return Err(Reject::Closed);
        }
        if input.actor_id != self.actor_id { return Err(Reject::WrongActor); }
        if input.current_target != self.target {
            self.phase = Phase::Stale;
            self.selected = None;
            return Err(Reject::StaleTarget);
        }
        if input.pointer_id != self.pointer_id { return Err(Reject::WrongPointer); }
        if input.sequence <= self.sequence || input.elapsed_ms < self.elapsed_ms {
            return Err(Reject::InvalidSequence);
        }
        match &input.event {
            GestureEvent::Cancel => { self.phase = Phase::Cancelled; self.selected = None; }
            GestureEvent::Dwell => {
                if self.phase != Phase::Armed || input.elapsed_ms < self.hold_ms {
                    return Err(Reject::NotHeld);
                }
                self.phase = Phase::Revealing;
            }
            GestureEvent::Move(choice) => {
                if !matches!(self.phase, Phase::Revealing | Phase::Exploring) {
                    return Err(Reject::NotPreviewing);
                }
                if choice.as_ref().is_some_and(|id| !self.choices.contains(id)) {
                    return Err(Reject::UnknownChoice);
                }
                self.selected = choice.clone();
                self.phase = Phase::Exploring;
            }
            GestureEvent::Release => {
                if self.phase == Phase::Armed {
                    // Ordinary tap remains owned by the platform's existing handler.
                    self.phase = Phase::Cancelled;
                    self.selected = None;
                } else if matches!(self.phase, Phase::Revealing | Phase::Exploring) {
                    self.phase = Phase::Released;
                } else { return Err(Reject::Closed); }
            }
        }
        self.sequence = input.sequence;
        self.elapsed_ms = input.elapsed_ms;
        Ok(self.preview())
    }
    /// Called only after a separate explicit Apply control. Returns no effect.
    pub fn propose(&mut self, input: &ExplicitApply) -> Result<InertProposal, Reject> {
        if self.phase != Phase::Released { return Err(Reject::NotReleased); }
        if input.actor_id != self.actor_id { return Err(Reject::WrongActor); }
        if input.current_target != self.target {
            self.phase = Phase::Stale; self.selected = None;
            return Err(Reject::StaleTarget);
        }
        if input.sequence <= self.sequence { return Err(Reject::InvalidSequence); }
        if !self.choices.contains(&input.candidate_id) ||
           self.selected.as_deref() != Some(input.candidate_id.as_str()) {
            return Err(Reject::UnknownChoice);
        }
        self.phase = Phase::Proposed;
        self.sequence = input.sequence;
        Ok(InertProposal { target: self.target.clone(),
            candidate_id: input.candidate_id.clone(), sequence: input.sequence,
            host_authorization_required: true, authorized: false, effect_invoked: false })
    }
    pub fn invalidate(&mut self) -> Preview {
        self.phase = Phase::Stale; self.selected = None; self.preview()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn target() -> Target { Target {
        surface_id: "vivim".into(), target_id: "wiki.notes".into(),
        source_id: "wiki.journal".into(), source_revision: 7,
        source_digest: "a".repeat(64), view_revision: 4,
        scope_fingerprint: "owner.scope.1".into(),
    }}
    fn begin() -> Session { Session::begin(target(), "owner".into(), 1, 1,
        vec!["inspect".into(), "pin".into()], 350).unwrap() }
    fn event(sequence: u64, elapsed_ms: u64, event: GestureEvent) -> Input {
        Input { actor_id:"owner".into(), pointer_id:1, sequence,
            elapsed_ms, current_target:target(), event }
    }
    fn released() -> Session {
        let mut s=begin();
        s.advance(&event(2,350,GestureEvent::Dwell)).unwrap();
        s.advance(&event(3,360,GestureEvent::Move(Some("inspect".into())))).unwrap();
        s.advance(&event(4,370,GestureEvent::Release)).unwrap();
        s
    }
    #[test] fn starts_inert() {
        let p=begin().preview(); assert_eq!(p.phase,Phase::Armed);
        assert!(!p.authorized && !p.effect_invoked);
    }
    #[test] fn hold_only_after_dwell() {
        let mut s=begin();
        assert_eq!(s.advance(&event(2,349,GestureEvent::Dwell)),Err(Reject::NotHeld));
        assert_eq!(s.phase(),Phase::Armed);
        assert_eq!(s.advance(&event(2,350,GestureEvent::Dwell)).unwrap().phase,Phase::Revealing);
    }
    #[test] fn movement_changes_inert_selection() {
        let mut s=begin();s.advance(&event(2,350,GestureEvent::Dwell)).unwrap();
        let p=s.advance(&event(3,360,GestureEvent::Move(Some("pin".into())))).unwrap();
        assert_eq!(p.selected.as_deref(),Some("pin"));assert!(!p.effect_invoked);
    }
    #[test] fn release_does_not_execute() {
        let s=released();assert_eq!(s.phase(),Phase::Released);
        assert!(!s.preview().authorized && !s.preview().effect_invoked);
    }
    #[test] fn explicit_apply_is_only_inert_proposal() {
        let mut s=released();
        let a=ExplicitApply{actor_id:"owner".into(),sequence:5,current_target:target(),candidate_id:"inspect".into()};
        let p=s.propose(&a).unwrap();assert!(p.host_authorization_required);
        assert!(!p.authorized && !p.effect_invoked);
        assert_eq!(s.propose(&a),Err(Reject::NotReleased));
    }
    #[test] fn early_apply_refused() {
        let mut s=begin();
        let a=ExplicitApply{actor_id:"owner".into(),sequence:2,current_target:target(),candidate_id:"pin".into()};
        assert_eq!(s.propose(&a),Err(Reject::NotReleased));
    }
    #[test] fn cancel_drops_selection() {
        let mut s=released();let p=s.advance(&event(5,380,GestureEvent::Cancel)).unwrap();
        assert_eq!(p.phase,Phase::Cancelled);assert_eq!(p.selected,None);
    }
    #[test] fn short_tap_is_platform_owned() {
        let mut s=begin();
        assert_eq!(s.advance(&event(2,120,GestureEvent::Release)).unwrap().phase,Phase::Cancelled);
    }
    #[test] fn changed_revision_fails_closed() {
        let mut s=begin();let mut e=event(2,350,GestureEvent::Dwell);
        e.current_target.source_revision=8;
        assert_eq!(s.advance(&e),Err(Reject::StaleTarget));assert_eq!(s.phase(),Phase::Stale);
    }
    #[test] fn wrong_pointer_and_actor_refuse() {
        let mut s=begin();let mut e=event(2,350,GestureEvent::Dwell);
        e.pointer_id=2;assert_eq!(s.advance(&e),Err(Reject::WrongPointer));
        e.pointer_id=1;e.actor_id="agent".into();
        assert_eq!(s.advance(&e),Err(Reject::WrongActor));
        assert_eq!(s.phase(),Phase::Armed);
    }
    #[test] fn duplicate_sample_refused() {
        let mut s=begin();assert_eq!(s.advance(&event(1,350,GestureEvent::Dwell)),Err(Reject::InvalidSequence));
    }
    #[test] fn unsupported_choice_refused() {
        let mut s=begin();s.advance(&event(2,350,GestureEvent::Dwell)).unwrap();
        assert_eq!(s.advance(&event(3,360,GestureEvent::Move(Some("delete".into())))),Err(Reject::UnknownChoice));
    }
    #[test] fn invalidated_session_is_terminal() {
        let mut s=released();assert_eq!(s.invalidate().phase,Phase::Stale);
        assert_eq!(s.advance(&event(5,380,GestureEvent::Cancel)),Err(Reject::Closed));
    }
    #[test] fn duplicate_candidates_rejected() {
        assert_eq!(Session::begin(target(),"owner".into(),1,1,vec!["pin".into(),"pin".into()],350),Err(Reject::InvalidChoices));
    }
    #[test] fn invalid_hold_and_digest_rejected() {
        assert_eq!(Session::begin(target(),"owner".into(),1,1,vec!["pin".into()],100),Err(Reject::InvalidHold));
        let mut t=target();t.source_digest="not-a-digest".into();
        assert_eq!(Session::begin(t,"owner".into(),1,1,vec!["pin".into()],350),Err(Reject::InvalidTarget));
    }
    #[test] fn decreasing_time_refused() {
        let mut s=begin();s.advance(&event(2,350,GestureEvent::Dwell)).unwrap();
        assert_eq!(s.advance(&event(3,100,GestureEvent::Release)),Err(Reject::InvalidSequence));
    }
    #[test] fn blank_selection_clears_previous_choice() {
        let mut s=begin();s.advance(&event(2,350,GestureEvent::Dwell)).unwrap();
        s.advance(&event(3,360,GestureEvent::Move(Some("pin".into())))).unwrap();
        let p=s.advance(&event(4,370,GestureEvent::Move(None))).unwrap();
        assert_eq!(p.selected,None);
    }
}