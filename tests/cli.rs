use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Commang::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() {
    let cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() {
    run(&["Hello there"], "tests/expected/hello1.txt");
}

#[test]
fn hello2() {
    run(&["Hello", "there"], "tests/expected/hello2.txt");
}

#[test]
fn hello1_no_newline() {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt");
}

#[test]
fn hello2_no_newline() {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt");
}
