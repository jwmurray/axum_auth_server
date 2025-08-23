use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(&email);
        
        // 2. Create a TwoFATuple instance.
        let two_fa_tuple = TwoFATuple(login_attempt_id.as_ref().to_string(), code.as_ref().to_string());
        
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        let serialized_tuple = serde_json::to_string(&two_fa_tuple)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        let _result: () = self.conn.write().await
            .set_ex(key, serialized_tuple, TEN_MINUTES_IN_SECONDS)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
            
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(email);
        
        // 2. Call the del command on the Redis connection to delete the 2FA code entry.
        let _result: () = self.conn.write().await
            .del(&key)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
            
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(email);
        
        // 2. Call the get command on the Redis connection to get the value stored for the key.
        let serialized_tuple: String = self.conn.write().await
            .get(&key)
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        
        // 3. Parse the JSON string into a TwoFATuple
        let two_fa_tuple: TwoFATuple = serde_json::from_str(&serialized_tuple)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        
        // 4. Parse the login attempt ID string and 2FA code string into proper types
        let login_attempt_id = LoginAttemptId::parse(two_fa_tuple.0)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
        let code = TwoFACode::parse(two_fa_tuple.1)
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
            
        Ok((login_attempt_id, code))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}