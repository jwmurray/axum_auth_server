use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.

#[derive(Debug, Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>, // key: email as String, value: User object, email is unique
}

impl HashmapUserStore {
    pub fn new_arc_rwlock() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            users: HashMap::new(),
        }))
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user); // user is consumed by this function
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.get_user(email) {
            Ok(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            Err(e) => Err(UserStoreError::UserNotFound),
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    fn test_create_hashmap_user_store() -> HashmapUserStore {
        let user_store = HashmapUserStore::default();
        user_store
    }

    #[test]
    fn test_add_user() {
        let mut user_store = test_create_hashmap_user_store();
        let user = User {
            id: "user123".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };

        // Test successful user addition
        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());
        assert_eq!(user_store.users.len(), 1);

        // Test adding duplicate user
        let duplicate_result = user_store.add_user(user);
        assert_eq!(
            duplicate_result.unwrap_err(),
            UserStoreError::UserAlreadyExists
        );
    }

    #[test]
    fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            id: "user123".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };

        // Test getting non-existent user
        let result = user_store.get_user("nonexistent@example.com");
        assert_eq!(result.unwrap_err(), UserStoreError::UserNotFound);

        // Add user and test getting existing user
        user_store.add_user(user.clone()).unwrap();
        let retrieved_user = user_store.get_user("test@example.com").unwrap();
        assert_eq!(retrieved_user, user);
    }

    #[test]
    fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            id: "user123".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };

        // Test validating non-existent user
        let result = user_store.validate_user("nonexistent@example.com", "password123");
        assert_eq!(result.unwrap_err(), UserStoreError::UserNotFound);
        let result = user_store.validate_user("test@example.com", "password123");
        assert_eq!(result.unwrap_err(), UserStoreError::UserNotFound);

        // Add user and test validation
        user_store.add_user(user).unwrap();

        // Test correct credentials
        let valid_result = user_store.validate_user("test@example.com", "password123");
        assert!(valid_result.is_ok());

        // Test incorrect password
        let invalid_result = user_store.validate_user("test@example.com", "wrongpassword");
        assert_eq!(
            invalid_result.unwrap_err(),
            UserStoreError::InvalidCredentials
        );
    }
}
