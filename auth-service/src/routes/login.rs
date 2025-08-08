use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{Email, Password},
    AuthAPIError,
};

pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    println!("Login endpoint called!");

    let email: Email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Perform login validation in a single transaction
    {
        let user_store = app_state.user_store.read().await;

        // First, check if user exists
        match user_store.get_user(&email).await {
            Ok(_user) => {
                // User exists, now validate password
                if let Err(_) = user_store.validate_user(&email, &password).await {
                    return Err(AuthAPIError::InvalidCredentials);
                }
                // Both user exists and password is valid - success!
            }
            Err(crate::domain::data_stores::UserStoreError::UserNotFound) => {
                return Err(AuthAPIError::InvalidCredentials);
            }
            Err(_) => {
                return Err(AuthAPIError::UnexpectedError);
            }
        }
    } // Lock is released here

    // Create successful login response
    let response = LoginResponse {
        message: "Login successful!".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
