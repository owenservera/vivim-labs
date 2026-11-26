use serde_json::{Value, json};
use universal_stream_parser::intelligence::{Learner, Mode, Observation, Step};
fn window<'a>(a: &'a Value, b: &'a Value, noise: &'a Value) -> Vec<Observation<'a>> {
    vec![
        Observation {
            source: "/reply",
            stream: "r",
            start: 0,
            end: 10,
            value: a,
        },
        Observation {
            source: "/telemetry",
            stream: "t",
            start: 0,
            end: 8,
            value: noise,
        },
        Observation {
            source: "/reply",
            stream: "r",
            start: 10,
            end: 20,
            value: b,
        },
    ]
}
fn train(snapshot: bool) -> Learner {
    let mut l = Learner::default();
    for (i, (a, b)) in [("hel", "lo"), ("wor", "ld"), ("goo", "d")]
        .into_iter()
        .enumerate()
    {
        let target = format!("{a}{b}");
        let x = json!({"x":[{"z":a}]});
        let y = json!({"x":[{"z":if snapshot{target.as_str()}else{b}}]});
        let n = json!({"content":"unrelated"});
        l.observe(&i.to_string(), Some(&target), &window(&x, &y, &n))
            .unwrap();
    }
    let n = json!({"content":"unrelated"});
    l.observe(
        "negative",
        None,
        &[Observation {
            source: "/telemetry",
            stream: "t",
            start: 0,
            end: 10,
            value: &n,
        }],
    )
    .unwrap();
    l
}
#[test]
fn learns_obfuscated_delta_path() {
    let l = train(false);
    let p = l.select().unwrap();
    assert_eq!(p.source, "/reply");
    assert_eq!(p.mode, Mode::Append);
    assert_eq!(
        p.path,
        vec![Step::Field("x".into()), Step::Each, Step::Field("z".into())]
    );
    let x = json!({"x":[{"z":"new "}]});
    let y = json!({"x":[{"z":"answer"}]});
    let n = json!({"content":"wrong"});
    let r = p.execute(&window(&x, &y, &n)).unwrap().unwrap();
    assert_eq!(r.text, "new answer");
    assert_eq!(r.spans.len(), 2);
    assert!(p.validate(Some("new answer"), &window(&x, &y, &n)).unwrap());
    assert!(!p.validate(Some("changed"), &window(&x, &y, &n)).unwrap());
}
#[test]
fn learns_cumulative_snapshots() {
    let p = train(true).select().unwrap();
    assert_eq!(p.mode, Mode::Latest);
}
#[test]
fn ambiguous_modes_abstain() {
    let mut l = Learner::default();
    for (i, s) in ["one", "two", "three"].into_iter().enumerate() {
        let v = json!({"a":s});
        l.observe(
            &i.to_string(),
            Some(s),
            &[Observation {
                source: "r",
                stream: "s",
                start: 0,
                end: 1,
                value: &v,
            }],
        )
        .unwrap();
    }
    l.observe("negative", None, &[]).unwrap();
    assert!(l.select().unwrap_err().contains("ambiguous"));
}
#[test]
fn rejects_repeated_trial() {
    let mut l = Learner::default();
    l.observe("a", None, &[]).unwrap();
    assert!(l.observe("a", None, &[]).is_err());
}
#[test]
fn detects_type_drift_and_parallel_requests() {
    let p = train(false).select().unwrap();
    let v = json!({"x":[{"z":42}]});
    assert!(
        p.execute(&[Observation {
            source: "/reply",
            stream: "r",
            start: 0,
            end: 1,
            value: &v
        }])
        .is_err()
    );
    let v = json!({"x":[{"z":"hi"}]});
    assert!(
        p.execute(&[
            Observation {
                source: "/reply",
                stream: "r",
                start: 0,
                end: 1,
                value: &v
            },
            Observation {
                source: "/reply",
                stream: "r2",
                start: 0,
                end: 1,
                value: &v
            }
        ])
        .is_err()
    );
}
#[test]
fn negative_control_rejects_candidate() {
    let mut l = train(false);
    let v = json!({"x":[{"z":"unexpected"}]});
    l.observe(
        "negative2",
        None,
        &[Observation {
            source: "/reply",
            stream: "r",
            start: 0,
            end: 1,
            value: &v,
        }],
    )
    .unwrap();
    assert!(l.select().is_err());
}

#[test]
fn active_learning_finds_discriminating_window() {
    let mut l = Learner::default();
    for (i, s) in ["one", "two", "three"].into_iter().enumerate() {
        let v = json!({"a":s});
        l.observe(
            &i.to_string(),
            Some(s),
            &[Observation {
                source: "r",
                stream: "s",
                start: 0,
                end: 1,
                value: &v,
            }],
        )
        .unwrap();
    }
    l.observe("negative", None, &[]).unwrap();
    let a = json!({"a":"hel"});
    let b = json!({"a":"lo"});
    let single = [Observation {
        source: "r",
        stream: "s",
        start: 0,
        end: 1,
        value: &a,
    }];
    assert_eq!(l.probe(&single).unwrap().separating_pairs, 0);
    let both = [
        Observation {
            source: "r",
            stream: "s",
            start: 0,
            end: 1,
            value: &a,
        },
        Observation {
            source: "r",
            stream: "s",
            start: 1,
            end: 2,
            value: &b,
        },
    ];
    let p = l.probe(&both).unwrap();
    assert_eq!(p.candidates, 2);
    assert_eq!(p.separating_pairs, 1);
    l.observe("discriminator", Some("hello"), &both).unwrap();
    assert_eq!(l.select().unwrap().mode, Mode::Append);
}
