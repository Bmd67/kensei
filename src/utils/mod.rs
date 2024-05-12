pub fn check_cargo_installed() -> bool {
    std::process::Command::new("cargo").arg("--version").output().is_ok()
}
pub fn print_banner() { println!("Kensei - The Sword of the Code Warrior"); }
