use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_password(&s) {
            Ok(Password(s))
        } else {
            Err("Failed to parse string to a Password type".to_owned())
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let empty_password = "".to_owned();

        let password = Password::parse(empty_password);

        assert_eq!(
            password,
            Err("Failed to parse string to a Password type".to_owned())
        );
    }

    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        assert!(Password::parse(password).is_err());
    }
}
