pub mod nix;

use std::{cell, ffi::OsStr, io, os::unix::process::CommandExt, process::Command};

use crate::cellar::Cellar;

/// Maybe split package-manager backends from docker-image type backends??
pub trait Backend {
    fn create(&self, cellar: &Cellar);
    fn exists(&self, cellar: &Cellar) -> bool;
    fn run_cellar(&self, );
    fn kill_cellar(&self, );
    fn install_package(&self, );
}

//pub enum Use {
//    //Nix
//    Nix(nix::NixBackend), //??
//    //Homebrew,
//}

/// Not what i wanted.
//pub fn use_backend(backend: Use) -> dyn Backend {
//    match backend {
//        Use::Nix => nix::NixBackend{} //nix::NixBackend::create(&Cellar::new("default", false)), // default name for now
//    }
//}

pub fn run() {}

pub fn run_in_cur_terminal(shell_cmd: &str) -> Result<(), std::io::Error> {
    Command::new("bash")
        .arg("-lc")
        .arg(shell_cmd)
        .status()?;
    Ok(())
}

/// What's even going on here??
pub fn run_in_set_terminal(shell_cmd: &str, env_var: String) -> Result<(), std::io::Error> {
    let e: io::Error = Command::new("bash")
        .arg("-lc")
        .arg(OsStr::new(&format!("exec {} {}", env_var, shell_cmd)))
        .exec();
    Ok(())
}

pub fn run_in_new_terminal() {}

// didnt even check and didnt even write this
pub fn run_in_new_terminal_with_cmd(shell_cmd: &str) -> Result<(), std::io::Error> {
    Command::new("gnome-terminal")
        .arg("--")
        .arg("bash")
        .arg("-c")
        .arg(shell_cmd)
        .status()?;
    Ok(())
}  


mod tests {
    use super::*;
}