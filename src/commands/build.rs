use std::process::Command;
use std::time::Instant;

pub fn run(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    println!("Building project...");
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if release { cmd.arg("--release"); }
    let status = cmd.status()?;
    if status.success() {
        println!("Build completed in {:.2}s", start.elapsed().as_secs_f64());
        Ok(())
    } else {
        Err("Build failed".into())
    }
}
