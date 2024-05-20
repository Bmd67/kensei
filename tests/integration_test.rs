use std::process::Command;

#[test]
fn test_help() {
    let o = Command::new("cargo").arg("run").arg("--").arg("--help").output().unwrap();
    let s = String::from_utf8_lossy(&o.stdout);
    assert!(s.contains("Build") && s.contains("Test"));
}

#[test]
fn test_init() {
    let o = Command::new("cargo").arg("run").arg("--").arg("init").arg("tp").output().unwrap();
    assert!(o.status.success());
    let _ = std::fs::remove_dir_all("tp");
}
