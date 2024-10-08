use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generates random passwords"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("Password Generator 1.0"));
}

#[test]
fn test_cli_generate_password() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--length")
        .arg("12")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated password:"))
        // Check if the password matches the expected pattern and length
        .stdout(predicate::str::is_match(r"Generated password: [A-Za-z0-9!@#$%^&*()_+\-=\[\]{}|;:,.<>?]{12}").unwrap());
}

#[test]
fn test_cli_no_uppercase() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--length")
        .arg("20")
        .arg("--no-uppercase")
        .assert()
        .success()
        // Ensure the generated password contains no uppercase letters
        .stdout(predicate::str::is_match(r"Generated password: [a-z0-9!@#$%^&*()_+\-=\[\]{}|;:,.<>?]{20}").unwrap());
}

#[test]
fn test_cli_only_lowercase() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--length")
        .arg("15")
        .arg("--no-uppercase")
        .arg("--no-numbers")
        .arg("--no-symbols")
        .assert()
        .success()
        // Verify that the password only contains lowercase letters
        .stdout(predicate::str::is_match(r"Generated password: [a-z]{15}").unwrap());
}

#[test]
fn test_cli_invalid_length() {
    let mut cmd = Command::cargo_bin("password-generator-cli").unwrap();
    cmd.arg("--length")
        .arg("0")
        .assert()
        .failure()
        // Check for the specific error message when an invalid length is provided
        .stderr(predicate::str::contains("Password length must be greater than 0"));
}
