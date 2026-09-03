use std::path::PathBuf;

use clap::{Args, value_parser};


#[derive(Args, Debug)]
pub struct RunArgs {
    pub(super) name: String,

    //#[arg(long_help="Path to the environment, can be used if the environment is to be opened from, for example a foo.nix file, and does not have a name or the name is forgotten.")]
    //#[arg(long="env-path", help = "Path to the environment", value_parser = value_parser!(PathBuf))]
   // path: PathBuf,

    #[arg(short='t', long="terminal", default_value_t = String::from("bash"), help = "Terminal to use for the environment")]
    pub(super) terminal: String
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    pub(super) name: String,

    #[arg(long, help = "Run the environment after creating it")]
    pub(super) run: bool,

    #[arg(long="overwrite-existing", default_value_t = false, help = "Overwrite existing environment if it exists")]
    pub(super) overwrite_existing: bool,

    //#[arg(long="os-image-path", help = "Use OS image for the environment", value_parser = value_parser!(PathBuf), required = false)]
    //os_image: PathBuf,
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    #[arg(long, help = "Install a package in the environment")]
    pub(super) package: String,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[arg(long, help = "Set the default editor to open in new environment")]
    pub(super) default_editor: bool,

    #[arg(long, default_value_t = String::from("bash"), help = "default terminal to use for new environments")]
    pub(super) default_terminal: String,

    #[arg(long, help = "If all created cellars are to be logged")]
    pub(super) keep_cellar_log: bool,

    //#[arg(long, help = "How long to keep log files")]
    //pub(super) log_life: bool,
}

#[derive(Args, Debug)]
pub struct ExitArgs {
    #[arg(short='A', long="all", help = "exit all environments")]
    pub(super) exit_all: bool,
}

#[derive(Args, Debug)]
pub struct KillArgs {
    pub(super) name: String,

    #[arg(long, help = "Erase dependencies and packages installed for all cellars on the device.")]
    pub(super) remove_all: bool,

    #[arg(long, help = "Discard the environment after killing. Will remove the configuration files for the environment after erasing all packages and dependencies.")]
    pub(super) discard: bool,

    // make note
    //clean:bool,
}

#[derive(Args, Debug)]
pub struct DiscardArgs {
    pub(super) name: String,

    #[arg(long, default_value_t = false, help = "Keep the folder for the environment, but remove the configuration file.")]
    pub(super) keep_folder: bool,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    // how would this even work tho. should i keep logs of all environments kept? then i'd have an erase history command?
    #[arg(short, long, help = "Show discarded environments on list")]
    pub(super) discarded: bool,

    #[arg(short='D', long, help = "Show only discarded environments on list")]
    pub(super) only_discarded: bool,
    
    #[arg(short, long, help = "Show environments that aren't on disk on the list")]
    pub(super) killed: bool,
    
    #[arg(short, long, help = "List only actively running environments")]
    pub(super) running: bool,
    
    #[arg(short, long, help = "List all environments, includeding killed and discarded")]
    pub(super) all: bool,

    #[arg(short, long, help = "Verbose output")]
    pub(super) verbose: bool
}