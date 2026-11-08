//! Chronos: canonical, std-only temporal primitives for VIVIM's existing Rust core.
//!
//! Time computation and evidence projection only: no dispatcher, executor, network,
//! filesystem, permissions store, background thread, or second scheduler.
//! The Host/supervisor remain responsible for durable claims and effect authority.
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::engine::{ComputeEngine, EngineDescriptor, EngineOutput, EngineRequest};
use crate::{Error, Result};

pub const INTERFACE: &str = "vivim.chronos/1";
pub const SLOT: &str = "utility:chronos";
pub const MAX_EVENTS: usize = 4096;
pub const MAX_OCCURRENCES: usize = 256;
const MAX_ID_BYTES: usize = 160;

fn ident(value: &str, field: &str) -> Result<()> {
    if value.is_empty() || value.len() > MAX_ID_BYTES ||
        value.chars().any(|c| c.is_control()) || value.trim() != value {
        return Err(Error::Invalid(format!("invalid {field}")));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClockReading {
    /// UTC milliseconds since 1970-01-01 (may change if OS wall clock adjusts).
    pub utc_ms: i64,
    /// Elapsed time since this source instance was created. Comparable only in one boot.
    pub monotonic_ms: u64,
    pub boot_id: String,
    pub source: String,
    pub uncertainty_ms: Option<u64>,
}

/// Always inject for deterministic tests. A clock observation is not authorization.
pub trait ClockSource: Send + Sync {
    fn read(&self) -> Result<ClockReading>;
}

pub struct SystemClock { boot_id: String, started: Instant }
impl SystemClock {
    pub fn new(boot_id: &str) -> Result<Self> {
        ident(boot_id, "boot id")?;
        Ok(Self { boot_id: boot_id.into(), started: Instant::now() })
    }
}
impl ClockSource for SystemClock {
    fn read(&self) -> Result<ClockReading> {
        let now = SystemTime::now();
        let millis: i128 = match now.duration_since(UNIX_EPOCH) {
            Ok(d) => i128::try_from(d.as_millis()).map_err(|_| Error::Invalid("wall clock overflow".into()))?,
            Err(e) => -i128::try_from(e.duration().as_millis()).map_err(|_| Error::Invalid("wall clock overflow".into()))?,
        };
        Ok(ClockReading {
            utc_ms: i64::try_from(millis).map_err(|_| Error::Invalid("wall clock overflow".into()))?,
            monotonic_ms: u64::try_from(self.started.elapsed().as_millis())
                .map_err(|_| Error::Invalid("monotonic overflow".into()))?,
            boot_id: self.boot_id.clone(), source: "os".into(), uncertainty_ms: None,
        })
    }
}

#[derive(Clone)]
pub struct FixedClock { pub reading: ClockReading }
impl ClockSource for FixedClock {
    fn read(&self) -> Result<ClockReading> { Ok(self.reading.clone()) }
}

/// A monotonic span must carry its source boot identity; cross-boot durations fail closed.
pub fn elapsed_ms(start: &ClockReading, end: &ClockReading) -> Result<u64> {
    if start.boot_id != end.boot_id || start.source != end.source {
        return Err(Error::Conflict("clock source/boot mismatch".into()));
    }
    end.monotonic_ms.checked_sub(start.monotonic_ms)
        .ok_or_else(|| Error::Conflict("monotonic regression".into()))
}

pub fn duration_ms(start_utc_ms: i64, end_utc_ms: i64) -> Result<u64> {
    if end_utc_ms < start_utc_ms { return Err(Error::Invalid("negative duration".into())); }
    u64::try_from(i128::from(end_utc_ms) - i128::from(start_utc_ms))
        .map_err(|_| Error::Invalid("duration overflow".into()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeWindow { pub start_ms: i64, pub end_ms: i64 }
impl TimeWindow {
    pub fn new(start_ms: i64, end_ms: i64) -> Result<Self> {
        if start_ms >= end_ms { return Err(Error::Invalid("invalid half-open time window".into())); }
        Ok(Self { start_ms, end_ms })
    }
    pub fn contains(&self, ms: i64) -> bool { ms >= self.start_ms && ms < self.end_ms }
    pub fn overlaps(&self, other: &Self) -> bool { self.start_ms < other.end_ms && other.start_ms < self.end_ms }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadlineState { Upcoming, Due, Expired }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeadlineEvaluation { pub state: DeadlineState, pub delta_ms: i128 }
/// `Due` within grace (including exact deadline); `Expired` strictly after grace.
pub fn deadline(now_ms: i64, due_ms: i64, grace_ms: u64) -> DeadlineEvaluation {
    let delta = i128::from(now_ms) - i128::from(due_ms);
    let state = if delta < 0 { DeadlineState::Upcoming }
        else if delta <= i128::from(grace_ms) { DeadlineState::Due }
        else { DeadlineState::Expired };
    DeadlineEvaluation { state, delta_ms: delta }
}


/// Finite, bounded freshness semantics. A source in the future is NOT fresh.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FreshnessState { Fresh, Stale, Future, SkewedFuture, Unknown }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreshnessEvaluation { pub state: FreshnessState, pub age_ms: Option<i128> }
pub fn freshness(now_ms: i64, observed_at_ms: Option<i64>, ttl_ms: u64, allowed_future_skew_ms: u64) -> FreshnessEvaluation {
    let Some(observed) = observed_at_ms else { return FreshnessEvaluation { state: FreshnessState::Unknown, age_ms: None }; };
    let age = i128::from(now_ms) - i128::from(observed);
    let state = if age < 0 {
        // Do not call an observation from the future fresh, even within skew tolerance.
        if -age <= i128::from(allowed_future_skew_ms) { FreshnessState::Future }
        else { FreshnessState::SkewedFuture }
    } else if age <= i128::from(ttl_ms) { FreshnessState::Fresh }
    else { FreshnessState::Stale };
    FreshnessEvaluation { state, age_ms: Some(age) }
}

/// A heartbeat older than the allowed window is stale; a future heartbeat is suspect.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeartbeatState { Healthy, Stale, FutureClock, Unknown }
pub fn heartbeat(now_ms: i64, last_seen_ms: Option<i64>, timeout_ms: u64) -> HeartbeatState {
    let Some(last) = last_seen_ms else { return HeartbeatState::Unknown };
    let age = i128::from(now_ms) - i128::from(last);
    if age < 0 { HeartbeatState::FutureClock }
    else if age <= i128::from(timeout_ms) { HeartbeatState::Healthy }
    else { HeartbeatState::Stale }
}

/// Bounded deterministic retry delay calculation; does not grant permission to retry.
/// The caller must distinguish effects requiring owner reconciliation from idempotent work.
pub fn retry_delay_ms(base_ms: u64, cap_ms: u64, attempt: u32) -> Result<u64> {
    if base_ms == 0 || cap_ms < base_ms || attempt == 0 || attempt > 64 {
        return Err(Error::Invalid("invalid retry policy".into()));
    }
    let multiplier = 1_u128.checked_shl((attempt - 1).min(63)).unwrap_or(u128::MAX);
    let delay = u128::from(base_ms).saturating_mul(multiplier).min(u128::from(cap_ms));
    u64::try_from(delay).map_err(|_| Error::Invalid("retry delay overflow".into()))
}

/// Returned cursor is a proposal. A durable Host claim transaction must commit the
/// selected occurrence identities before the caller saves or advances any cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuePreview {
    pub occurrences: Vec<Occurrence>,
    pub next_cursor_exclusive_ms: i64,
    pub has_more: bool,
    pub effect_allowed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Misfire { Skip, CoalesceOne, CatchUpBounded { max: u16 } }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule { Once { at_ms: i64 }, Every { anchor_ms: i64, period_ms: u64 } }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schedule {
    pub id: String,
    pub revision: u64,
    pub owner_scope: String,
    pub target_ref: String,
    pub rule: Rule,
    pub end_ms: Option<i64>,
    pub max_runs: Option<u64>,
    pub misfire: Misfire,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurrence { pub at_ms: i64, pub ordinal: u64, pub claim_key: String }
impl Schedule {
    pub fn validate(&self) -> Result<()> {
        ident(&self.id, "schedule id")?;
        ident(&self.owner_scope, "owner scope")?;
        ident(&self.target_ref, "target reference")?;
        if self.revision == 0 { return Err(Error::Invalid("zero schedule revision".into())); }
        if let Rule::Every { period_ms, .. } = self.rule {
            if period_ms < 1000 { return Err(Error::Invalid("interval below 1 second".into())); }
        }
        if self.max_runs == Some(0) { return Err(Error::Invalid("zero max runs".into())); }
        if let Misfire::CatchUpBounded { max } = self.misfire {
            if max == 0 || usize::from(max) > MAX_OCCURRENCES {
                return Err(Error::Invalid("invalid catch-up cap".into()));
            }
        }
        Ok(())
    }
    /// Pure, UTC-based occurrence planning. This is not an execution claim.
    /// Calendar recurrence and IANA timezone resolution need a separate tested adapter.
    pub fn next_at_or_after(&self, from_ms: i64) -> Result<Option<Occurrence>> {
        self.validate()?;
        let (ordinal, candidate) = match self.rule {
            Rule::Once { at_ms } => {
                if at_ms < from_ms { return Ok(None); }
                (0_u64, i128::from(at_ms))
            }
            Rule::Every { anchor_ms, period_ms } => {
                let ordinal = if from_ms <= anchor_ms { 0 } else {
                    let delta = i128::from(from_ms) - i128::from(anchor_ms);
                    let p = i128::from(period_ms);
                    u64::try_from((delta + p - 1) / p)
                        .map_err(|_| Error::Invalid("occurrence ordinal overflow".into()))?
                };
                (ordinal, i128::from(anchor_ms) + i128::from(ordinal) * i128::from(period_ms))
            }
        };
        if self.max_runs.is_some_and(|limit| ordinal >= limit) { return Ok(None); }
        // Never silently wrap a schedule past representable wall time.
        let at_ms = i64::try_from(candidate).map_err(|_| Error::Invalid("occurrence time overflow".into()))?;
        if self.end_ms.is_some_and(|until| at_ms > until) { return Ok(None); }
        Ok(Some(Occurrence {
            at_ms, ordinal, claim_key: format!("{}:{}:{}:{}:{}:{}", self.owner_scope.len(), self.owner_scope, self.id.len(), self.id, self.revision, at_ms),
        }))
    }
    pub fn between(&self, from_ms: i64, through_ms: i64, cap: usize) -> Result<Vec<Occurrence>> {
        if cap == 0 || cap > MAX_OCCURRENCES { return Err(Error::Invalid("invalid occurrence cap".into())); }
        if from_ms > through_ms { return Err(Error::Invalid("invalid occurrence window".into())); }
        let mut out = Vec::new();
        let mut cursor = from_ms;
        loop {
            let Some(next) = self.next_at_or_after(cursor)? else { break };
            if next.at_ms > through_ms || out.len() == cap { break; }
            cursor = match next.at_ms.checked_add(1) { Some(v) => v, None => { out.push(next); break; } };
            out.push(next);
        }
        Ok(out)
    }
    /// Latest scheduled occurrence not later than a time, using closed-form arithmetic.
    pub fn latest_at_or_before(&self, through_ms: i64) -> Result<Option<Occurrence>> {
        self.validate()?;
        let effective = self.end_ms.map_or(through_ms, |end| through_ms.min(end));
        let (ordinal, candidate) = match self.rule {
            Rule::Once { at_ms } if at_ms <= effective => (0u64, i128::from(at_ms)),
            Rule::Once { .. } => return Ok(None),
            Rule::Every { anchor_ms, .. } if anchor_ms > effective => return Ok(None),
            Rule::Every { anchor_ms, period_ms } => {
                let ordinal = u64::try_from((i128::from(effective) - i128::from(anchor_ms)) / i128::from(period_ms))
                    .map_err(|_| Error::Invalid("ordinal overflow".into()))?;
                let ordinal = self.max_runs.map_or(ordinal, |max| ordinal.min(max - 1));
                (ordinal, i128::from(anchor_ms) + i128::from(ordinal) * i128::from(period_ms))
            }
        };
        if self.max_runs.is_some_and(|max| ordinal >= max) { return Ok(None); }
        let at_ms = i64::try_from(candidate).map_err(|_| Error::Invalid("occurrence time overflow".into()))?;
        Ok(Some(Occurrence { at_ms, ordinal,
            claim_key: format!("{}:{}:{}:{}:{}:{}", self.owner_scope.len(), self.owner_scope, self.id.len(), self.id, self.revision, at_ms) }))
    }
    /// Candidate signals only; permissions/leases/replay belong to the Host and supervisor.
    /// Skip permits a 1-second due detection window, not arbitrary late catch-up.
    pub fn due_candidates(&self, last_checked_exclusive_ms: i64, now_ms: i64) -> Result<Vec<Occurrence>> {
        self.validate()?;
        if now_ms <= last_checked_exclusive_ms { return Ok(Vec::new()); }
        match self.misfire {
            Misfire::Skip | Misfire::CoalesceOne => {
                let Some(o) = self.latest_at_or_before(now_ms)? else { return Ok(Vec::new()); };
                if o.at_ms <= last_checked_exclusive_ms { return Ok(Vec::new()); }
                if self.misfire == Misfire::Skip && i128::from(now_ms) - i128::from(o.at_ms) > 1000 {
                    return Ok(Vec::new());
                }
                Ok(vec![o])
            }
            Misfire::CatchUpBounded { max } => {
                let from = match last_checked_exclusive_ms.checked_add(1) {
                    Some(t) => t, None => return Ok(Vec::new()),
                };
                self.between(from, now_ms, usize::from(max))
            }
        }
    }
    /// A bounded, resumable read-only plan. A capped catch-up batch never
    /// advances past the last emitted occurrence; skipping/coalescing does.
    pub fn due_preview(&self, last_checked_exclusive_ms: i64, now_ms: i64) -> Result<DuePreview> {
        let occurrences = self.due_candidates(last_checked_exclusive_ms, now_ms)?;
        let cursor = match self.misfire {
            Misfire::CatchUpBounded { .. } if !occurrences.is_empty() =>
                occurrences.last().expect("non-empty batch").at_ms,
            _ => now_ms.max(last_checked_exclusive_ms),
        };
        let has_more = if matches!(self.misfire, Misfire::CatchUpBounded { .. }) && cursor < now_ms {
            self.next_at_or_after(cursor.saturating_add(1))?
                .is_some_and(|next| next.at_ms <= now_ms)
        } else { false };
        Ok(DuePreview { occurrences, next_cursor_exclusive_ms: cursor,
            has_more, effect_allowed: false })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubjectKind { Task, Conversation, Session, Automation, Agent, Workflow, Device, Data, Other(String) }
impl SubjectKind {
    pub fn from_name(name: &str) -> Result<Self> {
        ident(name, "subject kind")?;
        Ok(match name {
            "task" => Self::Task, "conversation" => Self::Conversation, "session" => Self::Session,
            "automation" => Self::Automation, "agent" => Self::Agent, "workflow" => Self::Workflow,
            "device" => Self::Device, "data" => Self::Data, other => Self::Other(other.into()),
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind { Started, Paused, Resumed, Stopped, Completed, Failed, Cancelled, Due, Observed }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemporalEvent {
    pub event_id: String,
    pub subject: SubjectKind,
    pub subject_id: String,
    pub kind: EventKind,
    pub seq: u64,
    /// When the event was valid in the source domain.
    pub occurred_at_ms: i64,
    /// When the platform recorded it (bi-temporal audit evidence).
    pub recorded_at_ms: i64,
    pub source_ref: String,
    pub policy_revision: u64,
}
impl TemporalEvent {
    pub fn validate(&self) -> Result<()> {
        ident(&self.event_id, "event id")?;
        ident(&self.subject_id, "subject id")?;
        ident(&self.source_ref, "source reference")?;
        if let SubjectKind::Other(ref kind) = self.subject { ident(kind, "subject kind")?; }
        if self.seq == 0 { return Err(Error::Invalid("zero event sequence".into())); }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState { NotStarted, Running, Paused, Stopped, Completed, Failed, Cancelled }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineProjection {
    pub state: LifecycleState,
    pub active_ms: u64,
    pub running_since_ms: Option<i64>,
    pub last_occurred_ms: Option<i64>,
    pub included_events: usize,
    pub last_seq: u64,
}
/// Projects start/stop/pause/resume for conversations, sessions, tasks and agents.
/// Inputs must be source-sequenced, for one subject, without duplicate event ids.
/// A past `as_of_recorded_ms` excludes facts unknown at that moment.
pub fn project_timeline(events: &[TemporalEvent], as_of_recorded_ms: i64) -> Result<TimelineProjection> {
    if events.len() > MAX_EVENTS { return Err(Error::Invalid("event limit exceeded".into())); }
    let mut ids = BTreeSet::new();
    let mut previous_seq = 0u64;
    let mut last_included_seq = 0u64;
    let mut previous_time = None;
    let mut subject: Option<(&SubjectKind, &str)> = None;
    let mut state = LifecycleState::NotStarted;
    let mut active_ms = 0u64;
    let mut running_since = None;
    let mut count = 0;
    for event in events {
        event.validate()?;
        if let Some((kind, id)) = subject {
            if kind != &event.subject || id != event.subject_id.as_str() {
                return Err(Error::Conflict("mixed timeline subjects".into()));
            }
        } else { subject = Some((&event.subject, event.subject_id.as_str())); }
        if !ids.insert(event.event_id.as_str()) { return Err(Error::Conflict("duplicate event id".into())); }
        if event.seq <= previous_seq { return Err(Error::Conflict("non-increasing sequence".into())); }
        previous_seq = event.seq;
        if event.recorded_at_ms > as_of_recorded_ms || event.occurred_at_ms > as_of_recorded_ms { continue; }
        if previous_time.is_some_and(|t| event.occurred_at_ms < t) { return Err(Error::Conflict("timeline occurrence regression".into())); }
        previous_time = Some(event.occurred_at_ms);
        last_included_seq = event.seq;
        let next = match event.kind {
            EventKind::Started if state == LifecycleState::NotStarted => {
                running_since = Some(event.occurred_at_ms);
                LifecycleState::Running
            }
            EventKind::Paused if state == LifecycleState::Running => {
                let since = running_since.take().ok_or_else(|| Error::Corrupt("missing active start".into()))?;
                active_ms = active_ms.checked_add(duration_ms(since, event.occurred_at_ms)?)
                    .ok_or_else(|| Error::Invalid("active duration overflow".into()))?;
                LifecycleState::Paused
            }
            EventKind::Resumed if state == LifecycleState::Paused => {
                running_since = Some(event.occurred_at_ms);
                LifecycleState::Running
            }
            EventKind::Stopped | EventKind::Completed | EventKind::Failed | EventKind::Cancelled
                if state == LifecycleState::Running || state == LifecycleState::Paused => {
                if let Some(since) = running_since.take() {
                    active_ms = active_ms.checked_add(duration_ms(since, event.occurred_at_ms)?)
                        .ok_or_else(|| Error::Invalid("active duration overflow".into()))?;
                }
                match event.kind {
                    EventKind::Stopped => LifecycleState::Stopped,
                    EventKind::Completed => LifecycleState::Completed,
                    EventKind::Failed => LifecycleState::Failed,
                    _ => LifecycleState::Cancelled,
                }
            }
            EventKind::Observed | EventKind::Due => state,
            _ => return Err(Error::Conflict("invalid lifecycle transition".into())),
        };
        state = next;
        count += 1;
    }
    if let Some(since) = running_since {
        active_ms = active_ms.checked_add(duration_ms(since, as_of_recorded_ms)?)
            .ok_or_else(|| Error::Invalid("active duration overflow".into()))?;
    }
    Ok(TimelineProjection {
        state, active_ms, running_since_ms: running_since,
        last_occurred_ms: previous_time, included_events: count, last_seq: last_included_seq,
    })
}

/// Bounded read-only compute adapter. Install in the existing EngineRegistry;
/// never route schedule mutating or provider-effect commands through this slot.
pub struct ChronosCompute { clock: Arc<dyn ClockSource> }
impl ChronosCompute { pub fn new(clock: Arc<dyn ClockSource>) -> Self { Self { clock } } }
fn numbers(data: &str, min: usize, max: usize) -> Result<Vec<i64>> {
    if data.len() > 512 { return Err(Error::Invalid("chronos input too large".into())); }
    let parts: Vec<_> = data.split(',').map(str::trim).collect();
    if parts.len() < min || parts.len() > max { return Err(Error::Invalid("chronos argument count".into())); }
    parts.into_iter().map(|s| s.parse::<i64>()
        .map_err(|_| Error::Invalid("invalid chronos integer".into()))).collect()
}
impl ComputeEngine for ChronosCompute {
    fn descriptor(&self) -> EngineDescriptor {
        EngineDescriptor::new(SLOT, "chronos-std-v1", INTERFACE,
            &["now", "duration", "deadline", "next", "window", "elapsed", "freshness", "heartbeat", "retry", "due"])
    }
    fn execute(&self, req: &EngineRequest) -> Result<EngineOutput> {
        let data = match req.operation.as_str() {
            "now" => {
                if !req.data.is_empty() { return Err(Error::Invalid("now takes no arguments".into())); }
                let sample = self.clock.read()?;
                format!("{}|{}|{}|{}|{}", sample.utc_ms, sample.monotonic_ms,
                    sample.boot_id, sample.source, sample.uncertainty_ms.map_or_else(|| "unknown".into(), |n| n.to_string()))
            }
            "duration" => {
                let n = numbers(&req.data, 2, 2)?;
                duration_ms(n[0], n[1])?.to_string()
            }
            "deadline" => {
                let n = numbers(&req.data, 1, 2)?;
                let grace = if n.len() > 1 { u64::try_from(n[1]).map_err(|_| Error::Invalid("negative grace".into()))? } else { 0 };
                let d = deadline(self.clock.read()?.utc_ms, n[0], grace);
                format!("{:?}|{}", d.state, d.delta_ms)
            }
            "next" => {
                let n = numbers(&req.data, 3, 3)?;
                let period_ms = u64::try_from(n[1]).map_err(|_| Error::Invalid("negative interval".into()))?;
                let schedule = Schedule { id: "query".into(), revision: 1,
                    owner_scope: "read".into(), target_ref: "read".into(),
                    rule: Rule::Every { anchor_ms: n[0], period_ms }, end_ms: None,
                    max_runs: None, misfire: Misfire::Skip };
                schedule.next_at_or_after(n[2])?.map_or_else(|| "none".into(), |o| format!("{}|{}", o.at_ms, o.ordinal))
            }
            "window" => {
                let n = numbers(&req.data, 3, 3)?;
                TimeWindow::new(n[0], n[1])?.contains(n[2]).to_string()
            }
            "freshness" => {
                let n = numbers(&req.data, 2, 3)?;
                let ttl = u64::try_from(n[1]).map_err(|_| Error::Invalid("negative ttl".into()))?;
                let skew = u64::try_from(*n.get(2).unwrap_or(&0)).map_err(|_| Error::Invalid("negative skew".into()))?;
                let f = freshness(self.clock.read()?.utc_ms, if n[0] == i64::MIN { None } else { Some(n[0]) }, ttl, skew);
                format!("{:?}|{}", f.state, f.age_ms.map_or_else(|| "unknown".into(), |x| x.to_string()))
            }
            "heartbeat" => {
                let n = numbers(&req.data, 2, 2)?;
                let timeout = u64::try_from(n[1]).map_err(|_| Error::Invalid("negative timeout".into()))?;
                format!("{:?}", heartbeat(self.clock.read()?.utc_ms,
                    if n[0] == i64::MIN { None } else { Some(n[0]) }, timeout))
            }
            "retry" => {
                let n = numbers(&req.data, 3, 3)?;
                let base = u64::try_from(n[0]).map_err(|_| Error::Invalid("negative base".into()))?;
                let cap = u64::try_from(n[1]).map_err(|_| Error::Invalid("negative cap".into()))?;
                let attempt = u32::try_from(n[2]).map_err(|_| Error::Invalid("invalid attempt".into()))?;
                retry_delay_ms(base, cap, attempt)?.to_string()
            }
            "due" => {
                let n = numbers(&req.data, 4, 5)?;
                let period = u64::try_from(n[1]).map_err(|_| Error::Invalid("negative interval".into()))?;
                let max = u16::try_from(*n.get(4).unwrap_or(&4)).map_err(|_| Error::Invalid("invalid cap".into()))?;
                let schedule = Schedule { id: "preview".into(), revision: 1,
                    owner_scope: "read".into(), target_ref: "read".into(),
                    rule: Rule::Every { anchor_ms: n[0], period_ms: period }, end_ms: None,
                    max_runs: None, misfire: Misfire::CatchUpBounded { max } };
                let view = schedule.due_preview(n[2], n[3])?;
                format!("{}|{}|{}", view.next_cursor_exclusive_ms, view.has_more,
                    view.occurrences.iter().map(|o| o.at_ms.to_string()).collect::<Vec<_>>().join(","))
            }
            "elapsed" => {
                if req.data.len() > 512 { return Err(Error::Invalid("chronos input too large".into())); }
                let (start, boot_id) = req.data.split_once('|')
                    .ok_or_else(|| Error::Invalid("elapsed requires monotonic_ms|boot_id".into()))?;
                let start = start.parse::<u64>()
                    .map_err(|_| Error::Invalid("invalid monotonic start".into()))?;
                let sample = self.clock.read()?;
                if sample.boot_id != boot_id { return Err(Error::Conflict("clock boot mismatch".into())); }
                sample.monotonic_ms.checked_sub(start)
                    .ok_or_else(|| Error::Conflict("monotonic regression".into()))?.to_string()
            }
            other => return Err(Error::Unsupported(format!("unsupported chronos operation: {other}"))),
        };
        Ok(EngineOutput { data })
    }
}
