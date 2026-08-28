use std::path::PathBuf;

use clap::{Args, value_parser};


#[derive(Args, Debug)]
pub struct RunArgs {
    pub name: String,

    #[arg(long_help="Path to the environment, can be used if the environment is to be opened from, for example a foo.nix file, and does not have a name or the name is forgotten.")]
    #[arg(long="env-path", help = "Path to the environment", value_parser = value_parser!(PathBuf))]
    path: PathBuf,

    #[arg(short='t', long="terminal", default_value_t = String::from("bash"), help = "Terminal to use for the environment")]
    terminal: String
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    pub name: String,

    #[arg(long, help = "Run the environment after creating it")]
    pub(super) run: bool,

    #[arg(long="overwrite-existing", default_value_t = false, help = "Overwrite existing environment if it exists")]
    overwrite_existing: bool,

    #[arg(long="os-image-path", help = "Use OS image for the environment", value_parser = value_parser!(PathBuf))]
    os_image: PathBuf,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(long, help = "Set the default editor to open in new environment")]
    default_editor: bool,

    #[arg(long, default_value_t = String::from("bash"), help = "default terminal to use for new environments")]
    default_terminal: String,
}

#[derive(Args, Debug)]
pub struct ExitArgs {
    #[arg(short='A', long="all", help = "exit all environments")]
    exit_all: bool,
}

#[derive(Args, Debug)]
pub struct KillArgs {
    name: String,

    // make note
    clean:bool,
}

#[derive(Args, Debug)]
pub struct DiscardArgs {
    name: String,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(short, long, help = "Show discarded environments on list")]
    discarded: bool,

    #[arg(short='D', long, help = "Show only discarded environments on list")]
    only_discarded: bool,
    
    #[arg(short, long, help = "Show environments that aren't on disk on the list")]
    killed: bool,
    
    #[arg(short, long, help = "List only actively running environments")]
    running: bool,
    
    #[arg(short, long, help = "List all environments, includeding killed and discarded")]
    all: bool,

    #[arg(short, long, help = "Verbose output")]
    verbose: bool
}