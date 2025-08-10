#![allow(unused_variables)]

use crate::utils::auth::validate_token;
use crate::{app_state::AppState, domain::AuthAPIError};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    match validate_token(&request.token, state.banned_token_store.clone()).await {
        Ok(user) => Ok(StatusCode::OK),
        Err(e) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
