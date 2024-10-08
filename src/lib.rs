use rand::Rng;
use log::{info, debug, warn};
use anyhow::{Result, anyhow};

pub struct PasswordGenerator {
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_numbers: bool,
    use_symbols: bool,
}

impl PasswordGenerator {
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

    pub fn with_uppercase(mut self, use_uppercase: bool) -> Self {
        debug!("Setting use_uppercase to {}", use_uppercase);
        self.use_uppercase = use_uppercase;
        self
    }

    pub fn with_lowercase(mut self, use_lowercase: bool) -> Self {
        debug!("Setting use_lowercase to {}", use_lowercase);
        self.use_lowercase = use_lowercase;
        self
    }

    pub fn with_numbers(mut self, use_numbers: bool) -> Self {
        debug!("Setting use_numbers to {}", use_numbers);
        self.use_numbers = use_numbers;
        self
    }

    pub fn with_symbols(mut self, use_symbols: bool) -> Self {
        debug!("Setting use_symbols to {}", use_symbols);
        self.use_symbols = use_symbols;
        self
    }

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

pub fn generate_password(length: usize) -> Result<String> {
    info!("Generating password with default settings, length {}", length);
    PasswordGenerator::new(length)?.generate()
}
