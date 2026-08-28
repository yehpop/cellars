pub mod args;
pub mod handler;

use clap::{Parser, Subcommand};
use args::{RunArgs, CreateArgs, ConfigArgs, ExitArgs, KillArgs, DiscardArgs, ListArgs};


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
    Config(ConfigArgs),
    Exit(ExitArgs),
    Kill(KillArgs),
    Discard(DiscardArgs),
    #[command(name = "list", about = "List environments", aliases = ["ls"])]
    List(ListArgs)
}