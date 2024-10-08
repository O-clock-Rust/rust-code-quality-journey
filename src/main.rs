use clap::{Arg, Command};
use password_generator::PasswordGenerator;

fn main() {
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

    let length = *matches.get_one::<u8>("length").expect("Required");
    
    let password = PasswordGenerator::new(length as usize)
        .with_uppercase(!matches.get_flag("no-uppercase"))
        .with_lowercase(!matches.get_flag("no-lowercase"))
        .with_numbers(!matches.get_flag("no-numbers"))
        .with_symbols(!matches.get_flag("no-symbols"))
        .generate();

    println!("Generated password: {}", password);
}
