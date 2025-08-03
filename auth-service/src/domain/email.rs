use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        if email.validate_email() {
            Ok(Email(email))
        } else {
            Err(format!("Invalid email: {}", email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_eq!(
            Email::parse(email.clone()),
            Err("Invalid email: ".to_string() + &email)
        );
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "test.com".to_string();
        assert_eq!(
            Email::parse(email.clone()),
            Err("Invalid email: ".to_string() + &email)
        );
    }
}
