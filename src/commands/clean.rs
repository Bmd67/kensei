use std::fs;
use std::path::Path;

pub fn run(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let dirs = if all { vec!["target", "dist"] } else { vec!["target"] };
    for d in &dirs {
        let p = Path::new(d);
        if p.exists() { fs::remove_dir_all(p)?; }
    }
    println!("Clean complete!");
    Ok(())
}
