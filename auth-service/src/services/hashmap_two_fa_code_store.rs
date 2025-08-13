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

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let code = self
            .codes
            .get(email)
            .ok_or(TwoFACodeStoreError::LoginAttemptNotFound)?;
        Ok((code.0.clone(), code.1.clone()))
    }
}
