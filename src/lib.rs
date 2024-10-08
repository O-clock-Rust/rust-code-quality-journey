use rand::Rng;

pub struct PasswordGenerator {
    length: usize,
    use_uppercase: bool,
    use_lowercase: bool,
    use_numbers: bool,
    use_symbols: bool,
}

impl PasswordGenerator {
    pub fn new(length: usize) -> Self {
        PasswordGenerator {
            length,
            use_uppercase: true,
            use_lowercase: true,
            use_numbers: true,
            use_symbols: true,
        }
    }

    pub fn with_uppercase(mut self, use_uppercase: bool) -> Self {
        self.use_uppercase = use_uppercase;
        self
    }

    pub fn with_lowercase(mut self, use_lowercase: bool) -> Self {
        self.use_lowercase = use_lowercase;
        self
    }

    pub fn with_numbers(mut self, use_numbers: bool) -> Self {
        self.use_numbers = use_numbers;
        self
    }

    pub fn with_symbols(mut self, use_symbols: bool) -> Self {
        self.use_symbols = use_symbols;
        self
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut charset = String::new();

        if self.use_uppercase {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.use_lowercase {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.use_numbers {
            charset.push_str("0123456789");
        }
        if self.use_symbols {
            charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
        }

        if charset.is_empty() {
            return String::new();
        }

        (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect()
    }
}

pub fn generate_password(length: usize) -> String {
    PasswordGenerator::new(length).generate()
}
