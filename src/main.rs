use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "kensei", about = "Kensei - The sword of the code warrior")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { #[arg(short, long)] release: bool, #[arg(short, long)] watch: bool },
    Test { #[arg(short, long)] name: Option<String> },
    Init { name: String, #[arg(short, long)] template: Option<String> },
    Clean { #[arg(short, long)] all: bool },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { release, watch } => {
            if watch { commands::watch::start(release).await?; }
            else { commands::build::run(release)?; }
        }
        Commands::Test { name } => commands::test::run(name)?,
        Commands::Init { name, template } => commands::init::run(&name, template)?,
        Commands::Clean { all } => commands::clean::run(all)?,
    }
    Ok(())
}

mod commands;
mod watcher;
mod utils;
