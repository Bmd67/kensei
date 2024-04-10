use std::process::Command;

pub fn run(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running tests...");
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    if let Some(n) = name { cmd.arg("--").arg(&n); }
    let status = cmd.status()?;
    if status.success() { Ok(()) }
    else { Err("Tests failed".into()) }
}
