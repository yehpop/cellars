use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_create_cellar() {
    let mut cmd = Command::cargo_bin("cellars").unwrap();
    let assert = cmd.arg("create").arg("test_cellar").assert();
    assert.success().stdout(predicate::str::contains("Created environment cellar: test_cellar")); 
}