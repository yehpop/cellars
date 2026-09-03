pub mod args;
pub mod handler;

use clap::{Parser, Subcommand};
use args::{RunArgs, CreateArgs, ConfigArgs, ExitArgs, KillArgs, DiscardArgs, ListArgs, InstallArgs};


#[derive(Parser, Debug)]
#[command(
    name = "cellars", 
    version,
    about = "env manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(RunArgs),
    Create(CreateArgs),
    Install(InstallArgs),
    Config(ConfigArgs),
    Exit(ExitArgs),
    Kill(KillArgs),
    Discard(DiscardArgs),
    #[command(name = "list", about = "List environments", aliases = ["ls"])]
    List(ListArgs)
}


impl Cli {
    pub fn main() {
        let args = Cli::try_parse().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });
        match args.command {
            Commands::Run(args) => {
                if let Err(e) = handler::run(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Create(args) => {
                if let Err(e) = handler::create(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Install(args) => {
                if let Err(e) = handler::install(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Config(args) => {
                if let Err(e) = handler::config(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Exit(args) => {
                if let Err(e) = handler::exit(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Kill(args) => {
                if let Err(e) = handler::kill(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::Discard(args) => {
                if let Err(e) = handler::discard(&args) {
                    eprintln!("Error: {}", e);
                }
            }
            Commands::List(args) => {
                if let Err(e) = handler::list(&args) {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}