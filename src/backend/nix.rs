/// This module contains functions for interacting with the Nix package manager and shell environments.
/// I should create a struct in here with the backend trait implemented for it, and then have the backend 
/// trait be used in the cli handler functions instead of directly calling the nix functions. 
/// And I can later add others, like a docker backend or a conda backend?
/// 
/// So this file is probably going to change a lot.

use crate::cellar::Cellar;
use std::{os::unix::process::CommandExt ,process::Command};

pub fn write() {}

pub fn gen_shell(cellar: &Cellar) -> String {
    let packages = cellar
        .packages
        .iter()
        .map(|p| format!("  pkgs.{} ", p))
        .collect::<Vec<_>>()
        .join(" \n");

    format!(
    r#"{{ pkgs ? import <nixpkgs> {{ }} }}

    pkgs.mkShell {{
    buildInputs = [
    {}
    ];
    }}
    "#,
        packages
    )
}

pub fn write_shell(cellar: &Cellar) -> Result<(), String> {
    let shell_content = gen_shell(cellar);
    let shell_path = cellar.cellar_dir().join("shell.nix");
    std::fs::write(&shell_path, shell_content)
        .map_err(|e| format!("Failed to write shell.nix: {}", e))?;
    Ok(())
}

pub fn run_shell(cellar: &Cellar, terminal: &str) -> Result<(), String> {
    let shell_path = cellar.cellar_dir().join("shell.nix");
    if !shell_path.exists() {
        return Err(format!("shell.nix does not exist at {:?}", shell_path));
    }

    let status = Command::new(terminal)
        .arg("-c")
        .arg(format!("nix-shell {}", shell_path.display()))
        .env("CELLAR_ENV", &cellar.name)    
        .status()
        .map_err(|e| format!("Failed to launch terminal: {}", e))?;

    if !status.success() {
        return Err(format!("Terminal exited with status: {}", status));
    }

    Ok(())
}

pub fn run_cellar(cellar: &Cellar) -> Result<(), String> {
    let shell_path = cellar.cellar_dir().join("shell.nix");
    if !shell_path.exists() {
        return Err(format!("shell.nix does not exist at {:?}", shell_path));
    }

    println!("running cellar: {}", cellar.name);

    // I should probably use status instead. I
    Err(Command::new("nix-shell")
    .arg(shell_path)
    .env("CELLAR_ENV", &cellar.name)
    .exec().to_string())
}
    
pub fn garbage_collect() -> Result<(), String> {
    let status = Command::new("nix-collect-garbage")
        .arg("-d")
        .status()
        .map_err(|e| format!("Failed to run nix-collect-garbage: {}", e))?;

    if !status.success() {
        Err(format!("nix-collect-garbage exited with status: {}", status))
    } else {
        Ok(())
    }
}

pub fn shell_path(cellar: &Cellar) -> std::path::PathBuf {
    cellar.cellar_dir().join("shell.nix")
}


mod tests {
    use super::*;

    #[test]
    fn test_gen_shell() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.add_package("jq");

        let shell_content = gen_shell(&cellar);
        assert!(shell_content.contains("pkgs.hello"));
        assert!(shell_content.contains("pkgs.jq"));
    }
}