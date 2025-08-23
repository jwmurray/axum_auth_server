#![allow(unused_imports, unused_variables, dead_code)]

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use rand::{rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    domain::{Email, LoginAttemptId, Password, TwoFACode},
    utils::auth::generate_auth_cookie,
    AuthAPIError,
};

pub async fn login(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    println!("Login endpoint called!");

    let password = match Password::parse(request.password.clone()) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let email: Email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    // Perform login validation in a single transaction
    let user = {
        let user_store = app_state.user_store.read().await;

        // First, check if user exists
        let user = match user_store.get_user(&email).await {
            Ok(user) => {
                // User exists, now validate password
                if let Err(_) = user_store.validate_user(&email, &password).await {
                    return (jar, Err(AuthAPIError::IncorrectCredentials));
                }
                // Both user exists and password is valid - success!
                user
            }
            Err(crate::domain::data_stores::UserStoreError::UserNotFound) => {
                return (jar, Err(AuthAPIError::IncorrectCredentials));
            }
            Err(_) => {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }
        };
        user
    }; // Lock is released here

    return match user.requires_2fa {
        true => handle_2fa(&email, &app_state, jar).await,
        // If the user does not require 2FA, add the auth cookie to the cookie jar
        false => handle_no_2fa(&user.email, add_auth_cookie(jar, &email).await).await,
    };
}

/// Add the auth cookie to the cookie jar
/// If the function call fails return the original cookie jar
async fn add_auth_cookie(jar: CookieJar, email: &Email) -> CookieJar {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return jar,
    };
    jar.add(auth_cookie)
}

async fn handle_2fa(
    email: &Email,
    app_state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // Generate login attempt ID and 2FA code
    let login_attempt_id = match LoginAttemptId::parse(Uuid::new_v4().to_string()) {
        Ok(id) => id,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let generated_two_fa_code = format!("{:06}", rand::rng().random_range(0..=999_999u32));
    let two_fa_code = match TwoFACode::parse(generated_two_fa_code.clone()) {
        Ok(code) => code,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    // Store the 2FA code
    let mut two_fa_code_store = app_state.two_fa_code_store.write().await;
    if let Err(_) = two_fa_code_store
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    // Send 2FA code via the email client
    let email_client = app_state.email_client.read().await;
    if let Err(_) = email_client
        .send_email(&email, "2FA code", &generated_two_fa_code)
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_string(),
                login_attempt_id: login_attempt_id.as_ref().to_string(),
            })),
        )),
    )
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    (jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
