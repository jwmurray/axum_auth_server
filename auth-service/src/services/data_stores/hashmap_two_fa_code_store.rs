use std::collections::HashMap;

use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domain::email::Email;

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

impl HashmapTwoFACodeStore {
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let code = self
            .codes
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        Ok((code.0.clone(), code.1.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;

        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), Some(&(login_attempt_id, code)));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.remove_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), None);
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.get_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (login_attempt_id, code));
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();

        let result = store.get_code(&email).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TwoFACodeStoreError::LoginAttemptIdNotFound
        );
    }
}
