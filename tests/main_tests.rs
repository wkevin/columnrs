use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_with_stdin_single_column() {
    let mut cmd = Command::cargo_bin("columnrs").unwrap();
    cmd.write_stdin("Hello\nWorld\nRust\nis\nawesome")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello\nWorld\nRust\nis\nawesome\n"));
}

#[test]
fn test_cli_with_stdin_multiple_columns() {
    let mut cmd = Command::cargo_bin("columnrs").unwrap();
    cmd.arg("--columns")
        .arg("3")
        .write_stdin("1 2 3\n4 5 6\n7 8 9\n10 11 12")
        .assert()
        .success()
        .stdout(predicate::str::contains("1   2   3\n4   5   6\n7   8   9\n10  11  12\n"));
}

#[test]
fn test_cli_with_file_input() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "Hello\nWorld\nRust\nis\nawesome").unwrap();

    let mut cmd = Command::cargo_bin("columnrs").unwrap();
    cmd.arg(temp_file.path())
        .arg("--columns")
        .arg("2")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello  Rust \nWorld  is   \n      awesome\n"));
}

#[test]
fn test_cli_with_empty_lines() {
    let mut cmd = Command::cargo_bin("columnrs").unwrap();
    cmd.arg("--empty")
        .write_stdin("Hello\n\nWorld\n\nRust")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello\n\nWorld\n\nRust\n"));
}

#[test]
fn test_cli_without_empty_lines() {
    let mut cmd = Command::cargo_bin("columnrs").unwrap();
    cmd.write_stdin("Hello\n\nWorld\n\nRust")
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello\nWorld\nRust\n"));
}