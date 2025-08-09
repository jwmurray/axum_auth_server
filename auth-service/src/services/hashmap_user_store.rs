#![allow(unused_variables)]

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Debug, Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>, // key: Email tuple as key, value: User object, email is unique
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user); // user is consumed by this function
        Ok(())
    }

    // return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    fn test_create_hashmap_user_store() -> HashmapUserStore {
        let user_store = HashmapUserStore::default();
        user_store
    }

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = test_create_hashmap_user_store();
        let user = User {
            email: Email::parse("test@example.com".to_string()).unwrap(),
            password: Password::parse("password123".to_string()).unwrap(),
            requires_2fa: false,
        };

        // Test successful user addition
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());
        assert_eq!(user_store.users.len(), 1);

        // Test adding existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();

        let requires_2fa = true;
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let password = Password::parse("12345678".to_string()).unwrap();
        let user = User::new(email.clone(), password.clone(), requires_2fa);

        // Add user and test getting existing user
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.get_user(&email.clone()).await;
        assert_eq!(result, Ok(user));

        // Test getting non-existent user
        let non_existent_user = User::new(
            Email::parse("nonexistent@example.com".to_string()).unwrap(),
            password.clone(),
            requires_2fa,
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let password = Password::parse("password".to_owned()).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        // Test validating a user that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
