// AJOUT DES TESTS UNITAIRES

//! A password generator library.
//!
//! This library provides functionality to generate secure passwords
//! with customizable options for length and character types.

use rand::Rng;
use log::{info, debug, warn};
use anyhow::{Result, anyhow};

/// Struct representing a password generator with customizable options.
#[derive(Debug)]
pub struct PasswordGenerator {
    /// The length of the password to generate.
    length: usize,
    /// Whether to include uppercase letters in the password.
    use_uppercase: bool,
    /// Whether to include lowercase letters in the password.
    use_lowercase: bool,
    /// Whether to include numbers in the password.
    use_numbers: bool,
    /// Whether to include symbols in the password.
    use_symbols: bool,
}

impl PasswordGenerator {
    /// Creates a new `PasswordGenerator` with the specified length.
    ///
    /// # Arguments
    ///
    /// * `length` - The desired length of the password.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `PasswordGenerator` if successful,
    /// or an error if the length is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use password_generator::PasswordGenerator;
    ///
    /// let generator = PasswordGenerator::new(12).unwrap();
    /// ```
    pub fn new(length: usize) -> Result<Self> {
        if length == 0 {
            warn!("Attempt to create PasswordGenerator with zero length");
            return Err(anyhow!("Password length must be greater than 0"));
        }
        info!("Creating new PasswordGenerator with length {}", length);
        Ok(PasswordGenerator {
            length,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
        })
    }

    /// Sets whether to use uppercase letters in the password.
    ///
    /// # Arguments
    ///
    /// * `use_uppercase` - Boolean indicating whether to use uppercase letters.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    pub fn with_uppercase(mut self, use_uppercase: bool) -> Self {
        debug!("Setting use_uppercase to {}", use_uppercase);
        self.use_uppercase = use_uppercase;
        self
    }

    /// Sets whether to use lowercase letters in the password.
    ///
    /// # Arguments
    ///
    /// * `use_lowercase` - Boolean indicating whether to use lowercase letters.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    pub fn with_lowercase(mut self, use_lowercase: bool) -> Self {
        debug!("Setting use_lowercase to {}", use_lowercase);
        self.use_lowercase = use_lowercase;
        self
    }

    /// Sets whether to use numbers in the password.
    ///
    /// # Arguments
    ///
    /// * `use_numbers` - Boolean indicating whether to use numbers.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    pub fn with_numbers(mut self, use_numbers: bool) -> Self {
        debug!("Setting use_numbers to {}", use_numbers);
        self.use_numbers = use_numbers;
        self
    }

    /// Sets whether to use symbols in the password.
    ///
    /// # Arguments
    ///
    /// * `use_symbols` - Boolean indicating whether to use symbols.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    pub fn with_symbols(mut self, use_symbols: bool) -> Self {
        debug!("Setting use_symbols to {}", use_symbols);
        self.use_symbols = use_symbols;
        self
    }

    /// Generates a password based on the current configuration.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated password as a `String` if successful,
    /// or an error if no character set is selected.
    ///
    /// # Examples
    ///
    /// ```
    /// use password_generator::PasswordGenerator;
    ///
    /// let generator = PasswordGenerator::new(12).unwrap();
    /// let password = generator.generate().unwrap();
    /// assert_eq!(password.len(), 12);
    /// ```
    pub fn generate(&self) -> Result<String> {
        info!("Generating password with length {}", self.length);
        let mut rng = rand::thread_rng();
        let mut charset = String::new();
        if self.use_uppercase {
            debug!("Adding uppercase letters to charset");
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.use_lowercase {
            debug!("Adding lowercase letters to charset");
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.use_numbers {
            debug!("Adding numbers to charset");
            charset.push_str("0123456789");
        }
        if self.use_symbols {
            debug!("Adding symbols to charset");
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }
        debug!("Charset length: {}", charset.len());
        if charset.is_empty() {
            warn!("Empty charset, cannot generate password");
            return Err(anyhow!("No character set selected for password generation"));
        }
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();
        debug!("Password generated successfully");
        Ok(password)
    }
}

/// Generates a password with default settings and the specified length.
///
/// This is a convenience function that creates a `PasswordGenerator` with default settings
/// and generates a password.
///
/// # Arguments
///
/// * `length` - The desired length of the password.
///
/// # Returns
///
/// Returns a `Result` containing the generated password as a `String` if successful,
/// or an error if the password generation fails.
///
/// # Examples
///
/// ```
/// use password_generator::generate_password;
///
/// let password = generate_password(12).unwrap();
/// assert_eq!(password.len(), 12);
/// ```
pub fn generate_password(length: usize) -> Result<String> {
    info!("Generating password with default settings, length {}", length);
    PasswordGenerator::new(length)?.generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_generator_new() {
        // Test successful creation with a valid length
        assert!(PasswordGenerator::new(10).is_ok());
        // Test creation failure with a length of 0
        assert!(PasswordGenerator::new(0).is_err());
    }

    #[test]
    fn test_password_generator_length() {
        // Verify that the generated password has the specified length
        let generator = PasswordGenerator::new(15).unwrap();
        let password = generator.generate().unwrap();
        assert_eq!(password.len(), 15);
    }

    #[test]
    fn test_password_generator_charset() {
        // Check that only characters from the specified set are used
        let generator = PasswordGenerator::new(100)
            .unwrap()
            .with_uppercase(true)
            .with_lowercase(false)
            .with_numbers(false)
            .with_symbols(false);
        let password = generator.generate().unwrap();
        assert!(password.chars().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_password_generator_all_charsets() {
        // Ensure all character types are present when all sets are enabled
        let generator = PasswordGenerator::new(1000).unwrap();
        let password = generator.generate().unwrap();
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
    }

    #[test]
    fn test_password_generator_no_charset() {
        // Verify that generation fails if no character set is selected
        let generator = PasswordGenerator::new(10)
            .unwrap()
            .with_uppercase(false)
            .with_lowercase(false)
            .with_numbers(false)
            .with_symbols(false);
        assert!(generator.generate().is_err());
    }

    #[test]
    fn test_generate_password_function() {
        // Test the convenience function generate_password
        let password = generate_password(20).unwrap();
        assert_eq!(password.len(), 20);
    }
}
