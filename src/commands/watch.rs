use crate::watcher::FileWatcher;

pub async fn start(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting watcher (Ctrl+C to stop)");
    let paths = vec!["src".to_string()];
    let rm = release;
    let w = FileWatcher::new(paths, move |files| {
        let start = std::time::Instant::now();
        println!("Changed: {:?}", files);
        let mut cmd = std::process::Command::new("cargo");
        cmd.arg("build");
        if rm { cmd.arg("--release"); }
        match cmd.status() {
            Ok(s) if s.success() => println!("Rebuild OK ({:.2}s)", start.elapsed().as_secs_f64()),
            _ => println!("Rebuild failed"),
        }
    });
    w.start()?;
    Ok(())
}
