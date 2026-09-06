use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_create_cellar() {
    let cmd = Command::cargo_bin("cellars").unwrap();
    let assert = cmd.arg("create").arg("test_cellar").assert();
    assert.success().stdout(predicate::str::contains("Cellar 'test_cellar' created successfully")); 
}