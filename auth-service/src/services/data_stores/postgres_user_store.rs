#![allow(unused_variables)]

use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Version,
};
use rand::rng;

use sqlx::PgPool;

use crate::domain::{data_stores::UserStore, Email, Password, User, UserStoreError};

// use async_trait::async_trait;
// use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::RwLock;

// use crate::domain::{Email, Password, User, UserStore, UserStoreError};

// #[derive(Debug, Default)]
// pub struct HashmapUserStore {
//     users: HashMap<Email, User>, // key: Email tuple as key, value: User object, email is unique
// }

#[derive(Debug)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

use argon2::password_hash::rand_core::OsRng;

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(user.password.as_ref().as_bytes(), &salt)
            .map_err(|_| UserStoreError::UnexpectedError)?;

        let result = sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3) RETURNING email",
            user.email.as_ref(),
            password_hash.to_string(),
            user.requires_2fa,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError)?;

        let user_id = result.email;

        Ok(())
    }

    // return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let result = sqlx::query!(
            "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1",
            email.as_ref(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                UserStoreError::UserNotFound
            } else {
                UserStoreError::UnexpectedError
            }
        })?;

        let user = User {
            email: Email::parse(result.email).unwrap(),
            password: Password::parse(result.password_hash).unwrap(),
            requires_2fa: result.requires_2fa,
        };

        Ok(user)
    }

    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let result = sqlx::query!(
            "SELECT email, password_hash, requires_2fa FROM users WHERE email = $1",
            email.as_ref(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserStoreError::UserNotFound,
            _ => UserStoreError::UnexpectedError,
        })?;

        // Verify the password using argon2
        use argon2::{PasswordHash, PasswordVerifier};

        let parsed_hash = PasswordHash::new(&result.password_hash)
            .map_err(|_| UserStoreError::UnexpectedError)?;

        Argon2::default()
            .verify_password(password.as_ref().as_bytes(), &parsed_hash)
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}
