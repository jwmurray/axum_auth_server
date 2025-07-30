use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }

    // Check if User is valid.
    // - Email must be non-empty
    // - Password must be non-empty
    // - email must contain an @ symbol
    // - Password must be at least 8 characters long
    pub fn is_valid(&self) -> bool {
        // Check if email and password are not empty
        if self.email.is_empty() {
            return false;
        }

        if self.password.is_empty() {
            return false;
        }

        if !self.email.contains('@') {
            return false;
        }

        if self.password.len() < 8 {
            return false;
        }
        true
    }
}
