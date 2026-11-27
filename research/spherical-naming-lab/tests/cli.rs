use std::process::Command;
use std::path::PathBuf;

fn run(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_spherical-naming-lab"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(args).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}
#[test]
fn validate_and_materialized_catalogue() {
    let text = run(&["validate"]);
    assert!(text.contains("ok\t10000\t"));
    let rendered = run(&["catalogue"]);
    assert_eq!(rendered.lines().count(), 10001);
    assert_eq!(rendered, include_str!("../data/catalogue.tsv").replace("\r\n", "\n"));
}
#[test]
fn match_and_pair_are_reproducible_and_machine_readable() {
    let m = run(&["match", "examples/list-a.tsv", "42", "20"]);
    assert_eq!(m.lines().count(), 8);
    assert!(m.lines().all(|l| l.split('\t').count() == 13));
    let args = ["pair", "examples/list-a.tsv", "examples/list-b.tsv", "42", "7", "20"];
    let a = run(&args);
    assert_eq!(a, run(&args));
    assert_eq!(a.lines().count(), 8);
    assert!(a.lines().all(|l| l.split('\t').count() == 28));
    for line in a.lines().skip(1) {
        let fields: Vec<_> = line.split('\t').collect();
        let angle: f64 = fields[7].parse().unwrap();
        assert!((0.0..=180.0).contains(&angle));
        for n in [13,17,21,25,26,27] {
            assert!((0.0..=180.0).contains(&fields[n].parse::<f64>().unwrap()));
        }
    }
}
#[test]
fn random_pairs_obey_band_and_feedback_is_append_only() {
    let r = run(&["random","27","10","20","100"]);
    assert_eq!(r.lines().count(), 11);
    for line in r.lines().skip(1) {
        let f: Vec<_> = line.split('\t').collect();
        assert_ne!(f[1], f[4]);
        assert!((20.0..=100.0).contains(&f[7].parse::<f64>().unwrap()));
    }
    let path: PathBuf = std::env::temp_dir().join(format!("spherical-naming-feedback-{}.tsv", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let p = path.to_str().unwrap();
    run(&["feedback",p,"P-test","user","keep","Anastomosis","functional fit"]);
    run(&["feedback",p,"P-test","llm","refine","Ostium","shorter"]);
    let contents = std::fs::read_to_string(&path).unwrap();
    assert_eq!(contents.lines().count(), 3);
    assert!(contents.contains("P-test\tuser\tkeep\tAnastomosis"));
    assert!(contents.contains("P-test\tllm\trefine\tOstium"));
    std::fs::remove_file(path).unwrap();
}
