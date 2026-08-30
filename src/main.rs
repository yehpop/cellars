mod cli;
mod backend;
mod cellar;

use clap::Parser;

/// is this thing on?
fn main() {
    // Move this to a function in cli/mod.rs and call it from here. This is the main entry point for the CLI.?
    let args = cli::Cli::try_parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    match args.command {
        cli::Commands::Run(args) => {
            if let Err(e) = cli::handler::run(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Create(args) => {
            if let Err(e) = cli::handler::create(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Install(args) => {
            if let Err(e) = cli::handler::install(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Config(args) => {
            if let Err(e) = cli::handler::config(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Exit(args) => {
            if let Err(e) = cli::handler::exit(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Kill(args) => {
            if let Err(e) = cli::handler::kill(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::Discard(args) => {
            if let Err(e) = cli::handler::discard(&args) {
                eprintln!("Error: {}", e);
            }
        }
        cli::Commands::List(args) => {
            if let Err(e) = cli::handler::list(&args) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
