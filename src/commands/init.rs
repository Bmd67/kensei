use std::fs;
use std::path::Path;

pub fn run(name: &str, template: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let target = Path::new(name);
    if target.exists() {
        return Err(format!("Directory '{}' already exists", name).into());
    }
    fs::create_dir_all(target.join("src"))?;
    fs::write(
        target.join("Cargo.toml"),
        format!(
            "[package]
name = "{}"
version = "0.1.0"
edition = "2021"
",
            name
        ),
    )?;
    fs::write(target.join("src").join("main.rs"), "fn main() { println!("Hello, warrior!"); }
")?;
    println!("Created project '{}'", name);
    Ok(())
}
