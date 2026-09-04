/// This module contains functions for interacting with the Nix package manager and shell environments.
/// I should create a struct in here with the backend trait implemented for it, and then have the backend 
/// trait be used in the cli handler functions instead of directly calling the nix functions. 
/// And I can later add others, like a docker backend or a conda backend?
/// 
/// So this file is probably going to change a lot.

use crate::cellar::Cellar;
use std::{fmt::format, fs::exists, os::unix::process::CommandExt, process::Command};

pub fn write() {}

/// Does this make sense?
fn directory(cellar: &Cellar) -> std::path::PathBuf {
    let path = cellar.cellar_dir().join("nix");
    if exists(&path).expect("can't tell if the cellar's config path for nix exists: ") {
        path
    } else {
        std::fs::create_dir_all(&path).expect("failed to create nix directory");
        path
    }
}

pub fn gen_shell(cellar: &Cellar) -> String {
    let packages = cellar
        .packages
        .iter()
        .map(|p| format!("  pkgs.{} ", p))
        .collect::<Vec<_>>()
        .join(" \n");

    format!(
    "{{ pkgs ? import <nixpkgs> {{ }} }}:\npkgs.mkShell {{\n  buildInputs = [ {} ];\n}}",
        packages
    )
}

pub fn write_shell(cellar: &Cellar) -> Result<(), String> {
    let shell_content = gen_shell(cellar);
    let shell_path = directory(&cellar).join("shell.nix");
    std::fs::write(&shell_path, shell_content)
        .map_err(|e| format!("Failed to write shell.nix: {}", e))?;
    Ok(())
}

pub fn add_package(cellar: &Cellar, package: &str) -> Result<(), String> {
    let profile = directory(&cellar).join("profiles").join("nix-profile");
    let status = Command::new("nix-env")
        .args(["--profile"])
        .arg(profile)
        .args(["--file", "<nixpkgs>", "--install", "--attr", package])
        .status()
        .map_err(|error| format!("Failed to run nix-env: {error}"))?;
    if !status.success() {
        return Err(format!("nix-env exited with status: {}", status));
    }
    Ok(())
}

pub fn run_shell(cellar: &Cellar, terminal: &str) -> Result<(), String> {
    let shell_path = directory(&cellar).join("shell.nix");
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
    let shell_path = directory(&cellar).join("shell.nix");
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
    

fn remove_packages(cellar: &Cellar) -> Result<(), String> {
    todo!("implement remove_packages function to remove all packages installed for a specific cellar");
    let profile_dir = directory(&cellar).join("profiles");
    let status = Command::new("nix-env")
        .args(["--profile"])
        .arg(profile_dir.join("nix-profile")) // i think these names should be variables?
        .args(["--uninstall", "--all"])
        .status()
        .map_err(|error| format!("Failed to run nix-env: {error}"))?;
    if !status.success() {
        return Err(format!("nix-env exited with status: {}", status));
    }
    Ok(())
}
/// Needs to be edited.
/// RN removes all packages installed. So wont be cellar specific.
/// I need to add profile keeping logic for nix aswell.
/// So each cellar install or cellar add or whatever call will, update the toml, update the shell.nix AND nix profile install the package.
/// Then when kill is called either there should be additional logic here that;
/// first deletes packages from the nix profile, nix profile remove --all --profile $PROFILE
/// deletes older generations of the nix profile nix profile wipe-history --profile $PROFILE
/// or go nix-env profile remove --profile $PROFILE (i'll look into this too)
/// and then deletes the profile rm $PROFILE
/// and then runs garbage collection. nix-collect-garbage i'll look into that.
/// 
/// or each part of this logic will be separated into different functions and called from the kill handler in cli/handler.rs
/// which seems the better option.
/// 
/// Tho I am unsure because I want to group backends to implement a common trait,
/// Other package managers will have a different logic.
/// Keep thinking be yusa
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
    directory(cellar).join("shell.nix")
}


mod tests {
use super::*;

    /// Seems like a useless test
    #[test]
    fn gen_shell_content_correct() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.add_package("jq");

        let shell_content = gen_shell(&cellar);
        assert!(shell_content.contains("pkgs.hello"));
        assert!(shell_content.contains("pkgs.jq"));
    }
    

    /// Test that adding a package twice doesn't create duplicates
    #[test]
    fn test_add_package_prevents_duplicates() {
        let mut cellar = Cellar::new("test");
        cellar.add_package("nodejs");
        cellar.add_package("nodejs"); // duplicate
        assert_eq!(cellar.packages.len(), 1);
    }

    /// Test that the shell is written correctly
    #[test]
    fn written_shell_contains_packages() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.add_package("jq");

        write_shell(&cellar).expect("Failed to write shell.nix");

        let shell_path = directory(&cellar).join("shell.nix");
        assert!(shell_path.exists());

        let content = std::fs::read_to_string(shell_path).expect("Failed to read shell.nix");
        assert!(content.contains("pkgs.hello"));
        assert!(content.contains("pkgs.jq"));
    }


    /// Doesn't use any of the backend functions, just tests that the cellar can be run and the packages can be used within the environment
    /// What kind of test is this? This is a functional test, not a unit test. 
    /// Just installed nix so this is kind of me trying to understand what does what.
    /// God I'm a pain in the ass.
    #[test]
    fn cellar_can_run_and_use_packages() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.add_package("jq");
        Cellar::save(&cellar).expect("Failed to save cellar");

        write_shell(&cellar).expect("Failed to write shell.nix");

        let shell_path = directory(&cellar).join("shell.nix");
    
        // Run jq --version inside nix-shell and exit
        let status = Command::new("nix-shell")
            .arg(&shell_path)
            .arg("--run")
            .arg("jq --version")
            .status()
            .expect("Failed to run jq in nix-shell");

        assert!(status.success(), "jq should work inside nix-shell");
    }

    /// Test that the cellar can be run and the packages can be used within the environment
    /// This test is interrupted because the nix-shell will take over the terminal and wait for user input.
    fn cellar_can_run() {
        let mut cellar = Cellar::new("test_env");
        cellar.add_package("hello");
        cellar.add_package("jq");
        Cellar::save(&cellar).expect("Failed to save cellar");

        write_shell(&cellar).expect("Failed to write shell.nix");

        run_cellar(&cellar).expect("Failed to run shell"); // test is interrupted here because the nix-shell will take over the terminal and wait for user input. 

        Command::new("jq")
            .arg("--version")
            .status()
            .expect("Failed to run jq");
    }

    //#[test]
    /// no. i would first need to remove the shell.nix file from the cellar before running garbage collection, otherwise it will fail because the shell.nix file is still in use.
    fn test_garbage_collect() {
        garbage_collect().expect("Failed to run garbage collection");
    }
}