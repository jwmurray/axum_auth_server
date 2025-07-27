use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(id: String, email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            id,
            email,
            password,
            requires_2fa,
        }
    }
}
