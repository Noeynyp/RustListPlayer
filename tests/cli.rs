
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_non() {
    let mut cmd = Command::cargo_bin("player").unwrap();

    cmd.assert()
        .stdout(predicate::str::contains("Player 1: N/A"))
        .stdout(predicate::str::contains("Player 2: N/A"))
        .success();
}

#[test]
fn test_list_one() {
    let mut cmd = Command::cargo_bin("player").unwrap();

    cmd.arg("Mike")
        .assert()
        .stdout(predicate::str::contains("Player 1: Mike"))
        .stdout(predicate::str::contains("Player 2: N/A"))
        .success();
}

#[test]
fn test_list_two() {
    let mut cmd = Command::cargo_bin("player").unwrap();

    cmd.args(&["Mike", "Leo"])
        .assert()
        .stdout(predicate::str::contains("Player 1: Mike"))
        .stdout(predicate::str::contains("Player 2: Leo"))
        .success();
}

#[test]
fn test_list_extra() {
    let mut cmd = Command::cargo_bin("player").unwrap();

    cmd.args(&["Mike", "Leo", "Ralph"])
        .assert()
        .stdout(predicate::str::contains("Player 1: Mike"))
        .stdout(predicate::str::contains("Player 2: Leo"))
        .success();
}
