use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_create_cellar() {
    let mut cmd = Command::cargo_bin("cellars").unwrap();
    let assert = cmd.arg("create").arg("test_cellar").assert();
    assert.success().stdout(predicate::str::contains("Created environment cellar: test_cellar")); 
}

#[test]
fn test_install_package() {
    let mut cmd = Command::cargo_bin("cellars").unwrap();
    let assert = cmd.arg("install").arg("hello").arg("--env").arg("test_cellar").assert();
    assert.success().stdout(predicate::str::contains("Installed package hello in environment cellar: test_cellar")); 
}

#[test]
fn test_kill_cellar() {
    let mut cmd = Command::cargo_bin("cellars").unwrap();
    let assert = cmd.arg("kill").arg("test_cellar").assert();
    assert.success().stdout(predicate::str::contains("cleaned up cellar: test_cellar")); 
}