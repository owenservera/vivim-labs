#[test]
fn native_arm64_android() {
    assert_eq!(std::env::consts::ARCH, "aarch64");
    assert_eq!(std::env::consts::OS, "android");
}
#[test]
fn durable_file_roundtrip() {
    let p = "native-receipt.tmp";
    std::fs::write(p, b"vivim-android").unwrap();
    assert_eq!(std::fs::read(p).unwrap(), b"vivim-android");
    std::fs::remove_file(p).unwrap();
}
#[test]
fn child_process_roundtrip() {
    let output = std::process::Command::new("uname").arg("-m").output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "aarch64");
}
