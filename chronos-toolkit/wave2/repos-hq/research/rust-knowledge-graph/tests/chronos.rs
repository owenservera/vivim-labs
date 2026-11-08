use std::sync::Arc;
use hqkg::{ComputeEngine, EngineRegistry, EngineRequest};
use hqkg::chronos::{
    ChronosCompute, ClockReading, DeadlineState, EventKind, FixedClock, LifecycleState,
    Misfire, Rule, Schedule, SubjectKind, TemporalEvent, TimeWindow, INTERFACE, SLOT,
    deadline, duration_ms, elapsed_ms, project_timeline,
};

fn reading(boot: &str, wall: i64, mono: u64) -> ClockReading {
    ClockReading { boot_id: boot.into(), utc_ms: wall, monotonic_ms: mono,
        source: "test".into(), uncertainty_ms: None }
}
fn schedule(rule: Rule, misfire: Misfire) -> Schedule {
    Schedule { id: "daily-check".into(), revision: 7, owner_scope: "owner:1".into(),
        target_ref: "task:42".into(), rule, end_ms: None,
        max_runs: None, misfire }
}
fn event(id: &str, seq: u64, kind: EventKind, occurred: i64, recorded: i64) -> TemporalEvent {
    TemporalEvent { event_id: id.into(), subject: SubjectKind::Conversation,
        subject_id: "thread:a".into(), kind, seq, occurred_at_ms: occurred,
        recorded_at_ms: recorded, source_ref: "host:local".into(), policy_revision: 4 }
}

#[test]
fn clock_is_injected_and_elapsed_is_boot_fenced() {
    let a = reading("boot-one", 10_000, 100);
    let b = reading("boot-one", 9_000, 220);
    assert_eq!(elapsed_ms(&a, &b).unwrap(), 120); // Wall time moved backwards.
    assert!(elapsed_ms(&a, &reading("boot-two", 20_000, 400)).is_err());
    assert!(elapsed_ms(&b, &a).is_err());
    assert!(duration_ms(100, 99).is_err());
    assert_eq!(duration_ms(-100, 150).unwrap(), 250);
}

#[test]
fn windows_are_half_open_and_deadlines_use_grace() {
    let x = TimeWindow::new(0, 10).unwrap();
    let y = TimeWindow::new(10, 20).unwrap();
    assert!(x.contains(9));
    assert!(!x.contains(10));
    assert!(!x.overlaps(&y));
    assert!(TimeWindow::new(10, 10).is_err());
    assert_eq!(deadline(8, 10, 2).state, DeadlineState::Upcoming);
    assert_eq!(deadline(11, 10, 2).state, DeadlineState::Due);
    assert_eq!(deadline(13, 10, 2).state, DeadlineState::Expired);
}

#[test]
fn recurrence_is_deterministic_and_bounded() {
    let s = schedule(Rule::Every { anchor_ms: 10_000, period_ms: 5_000 }, Misfire::CoalesceOne);
    assert_eq!(s.next_at_or_after(1).unwrap().unwrap().at_ms, 10_000);
    assert_eq!(s.next_at_or_after(10_001).unwrap().unwrap().at_ms, 15_000);
    assert_eq!(s.latest_at_or_before(19_000).unwrap().unwrap().at_ms, 15_000);
    let due = s.due_candidates(1, 1_000_000).unwrap();
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].at_ms, 1_000_000);
    assert!(due[0].claim_key.contains("owner:1:11:daily-check:7"));
    assert_eq!(s.between(0, 20_000, 10).unwrap().len(), 3);
    assert!(s.between(0, 20_000, 257).is_err());
    let capped = Schedule { max_runs: Some(2), ..s.clone() };
    assert!(capped.next_at_or_after(20_000).unwrap().is_none());
    assert_eq!(capped.latest_at_or_before(1_000_000).unwrap().unwrap().at_ms, 15_000);
    let once = schedule(Rule::Once { at_ms: 17 }, Misfire::CoalesceOne);
    assert!(once.next_at_or_after(18).unwrap().is_none());
}

#[test]
fn catchup_and_skip_are_explicit() {
    let rule = Rule::Every { anchor_ms: 0, period_ms: 1_000 };
    let limited = schedule(rule, Misfire::CatchUpBounded { max: 2 });
    let due = limited.due_candidates(0, 10_000).unwrap();
    assert_eq!(due.iter().map(|x| x.at_ms).collect::<Vec<_>>(), vec![1000, 2000]);
    assert!(schedule(rule, Misfire::Skip).due_candidates(0, 10_500).unwrap().len() == 1);
    assert!(schedule(rule, Misfire::Skip).due_candidates(0, 10_999).unwrap().len() == 1);
    assert!(schedule(rule, Misfire::Skip).due_candidates(0, 11_999).unwrap().len() == 1);
    assert!(schedule(Rule::Every { anchor_ms: 0, period_ms: 500 }, Misfire::Skip).validate().is_err());
}

#[test]
fn timeline_tracks_active_time_without_counting_pauses() {
    let events = vec![
        event("s", 1, EventKind::Started, 100, 100),
        event("p", 2, EventKind::Paused, 200, 210),
        event("r", 3, EventKind::Resumed, 300, 305),
        event("e", 4, EventKind::Stopped, 500, 550),
    ];
    let projected = project_timeline(&events, 600).unwrap();
    assert_eq!(projected.state, LifecycleState::Stopped);
    assert_eq!(projected.active_ms, 300);
    assert_eq!(projected.included_events, 4);
    let before_recording = project_timeline(&events, 250).unwrap();
    assert_eq!(before_recording.state, LifecycleState::Paused);
    assert_eq!(before_recording.active_ms, 100);
    assert!(project_timeline(&[events[0].clone(), events[0].clone()], 600).is_err());
    assert!(project_timeline(&[events[0].clone(), event("bad", 2, EventKind::Resumed, 150, 150)], 600).is_err());
}

#[test]
fn compute_adapter_reuses_registry_and_refuses_effects() {
    let registry = EngineRegistry::new();
    let clock = Arc::new(FixedClock { reading: reading("boot-one", 10_000, 100) });
    let engine = ChronosCompute::new(clock);
    assert_eq!(engine.descriptor().interface, INTERFACE);
    registry.install(Arc::new(engine), None).unwrap();
    let lease = registry.pin(SLOT).unwrap();
    let output = lease.execute(&EngineRequest { operation: "deadline".into(), data: "9000,500".into() }).unwrap();
    assert_eq!(output.data, "Expired|1000");
    assert!(lease.execute(&EngineRequest { operation: "register".into(), data: "task42".into() }).is_err());
    assert!(lease.execute(&EngineRequest { operation: "elapsed".into(), data: "40|wrong-boot".into() }).is_err());
    assert_eq!(lease.execute(&EngineRequest { operation: "elapsed".into(), data: "40|boot-one".into() }).unwrap().data, "60");
}

#[test]
fn bounded_catch_up_preserves_cursor_and_is_never_an_execution_grant() {
    let s = schedule(Rule::Every { anchor_ms: 0, period_ms: 1_000 }, Misfire::CatchUpBounded { max: 2 });
    let first = s.due_preview(0, 10_000).unwrap();
    assert_eq!(first.occurrences.iter().map(|o| o.at_ms).collect::<Vec<_>>(), vec![1000,2000]);
    assert_eq!(first.next_cursor_exclusive_ms, 2_000);
    assert!(first.has_more);
    assert!(!first.effect_allowed);
    let next = s.due_preview(first.next_cursor_exclusive_ms, 10_000).unwrap();
    assert_eq!(next.occurrences[0].at_ms, 3000);
    let skipped = schedule(Rule::Every { anchor_ms: 0, period_ms: 1_000 }, Misfire::CoalesceOne)
        .due_preview(0, 10_000).unwrap();
    assert_eq!(skipped.next_cursor_exclusive_ms, 10_000);
    assert!(!skipped.has_more);
}

#[test]
fn temporal_safety_primitives_fail_closed() {
    use hqkg::chronos::{freshness, FreshnessState, heartbeat, HeartbeatState, retry_delay_ms};
    assert_eq!(freshness(2_000, Some(2_001), 5_000, 100).state, FreshnessState::Future);
    assert_eq!(freshness(2_000, Some(2_001), 5_000, 0).state, FreshnessState::SkewedFuture);
    assert_eq!(freshness(2_000, None, 5_000, 0).state, FreshnessState::Unknown);
    assert_eq!(freshness(2_000, Some(1_000), 1_000, 0).state, FreshnessState::Fresh);
    assert_eq!(freshness(2_001, Some(1_000), 1_000, 0).state, FreshnessState::Stale);
    assert_eq!(heartbeat(1_000, Some(1_001), 10), HeartbeatState::FutureClock);
    assert_eq!(heartbeat(1_000, None, 10), HeartbeatState::Unknown);
    assert_eq!(retry_delay_ms(500, 5000, 1).unwrap(), 500);
    assert_eq!(retry_delay_ms(500, 5000, 5).unwrap(), 5000);
    assert!(retry_delay_ms(0, 10, 2).is_err());
}

#[test]
fn chronos_exposes_safe_read_only_compute_extensions() {
    let clock = Arc::new(FixedClock { reading: reading("boot-one", 10_000, 10) });
    let engine = ChronosCompute::new(clock);
    for (op, data, expected) in [
        ("heartbeat", "8000,3000", "Healthy"),
        ("freshness", "5000,5000", "Fresh|5000"),
        ("retry", "100,1000,5", "1000"),
        ("due", "0,1000,0,10000,2", "2000|true|1000,2000")
    ] {
        let output = engine.execute(&EngineRequest { operation: op.into(), data: data.into() }).unwrap();
        assert_eq!(output.data, expected);
    }
    assert!(engine.execute(&EngineRequest { operation: "schedule.create".into(), data: "".into() }).is_err());
}
