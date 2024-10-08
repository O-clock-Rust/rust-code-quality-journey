//! Command-line interface for the password generator.
//!
//! This module provides a CLI application that uses the password generator library
//! to create passwords based on user-specified criteria.

use clap::{Arg, Command};
use env_logger::Env;
use password_generator::PasswordGenerator;
use log::debug;
use anyhow::{Result, Context};

/// The main entry point for the password generator CLI application.
///
/// This function sets up the command-line interface, parses arguments,
/// and uses the `PasswordGenerator` to create a password based on the
/// specified criteria.
///
/// # Returns
///
/// Returns a `Result<()>`, which is `Ok(())` if the password was successfully
/// generated and printed, or an `Err` if an error occurred during the process.
///
/// # Errors
///
/// This function will return an error if:
/// - It fails to parse the command-line arguments
/// - It fails to create the `PasswordGenerator`
/// - It fails to generate the password
///
/// # Examples
///
/// Generate a password with default settings:
/// ```
/// $ password-generator-cli --length 12
/// ```
///
/// Generate a password without uppercase letters:
/// ```
/// $ password-generator-cli --length 16 --no-uppercase
/// ```
///
/// Generate a password with only lowercase letters:
/// ```
/// $ password-generator-cli --length 8 --no-uppercase --no-numbers --no-symbols
/// ```
///
/// Show help information:
/// ```
/// $ password-generator-cli --help
/// ```
fn main() -> Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    debug!("Starting password generator CLI");

    // Set up the command-line interface
    let matches = Command::new("Password Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates random passwords")
        .arg(Arg::new("length")
            .short('l')
            .long("length")
            .value_name("LENGTH")
            .help("Sets the length of the password")
            .required(true)
            .value_parser(clap::value_parser!(u8)))
        .arg(Arg::new("no-uppercase")
            .long("no-uppercase")
            .help("Excludes uppercase letters")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("no-lowercase")
            .long("no-lowercase")
            .help("Excludes lowercase letters")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("no-numbers")
            .long("no-numbers")
            .help("Excludes numbers")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("no-symbols")
            .long("no-symbols")
            .help("Excludes symbols")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    // Extract the password length from the command-line arguments
    let length = *matches.get_one::<u8>("length").context("Failed to get length")?;
    debug!("Creating PasswordGenerator with length {}", length);

    // Create and configure the PasswordGenerator based on command-line arguments
    let password = PasswordGenerator::new(length as usize)
        .context("Failed to create PasswordGenerator")?
        .with_uppercase(!matches.get_flag("no-uppercase"))
        .with_lowercase(!matches.get_flag("no-lowercase"))
        .with_numbers(!matches.get_flag("no-numbers"))
        .with_symbols(!matches.get_flag("no-symbols"))
        .generate()
        .context("Failed to generate password")?;

    // Print the generated password
    println!("Generated password: {}", password);
    debug!("Password generation complete");

    Ok(())
}
