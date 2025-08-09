#![allow(unused_imports, unused_variables, dead_code)]

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{Email, Password},
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
    {
        let user_store = app_state.user_store.read().await;

        // First, check if user exists
        match user_store.get_user(&email).await {
            Ok(_user) => {
                // User exists, now validate password
                if let Err(_) = user_store.validate_user(&email, &password).await {
                    return (jar, Err(AuthAPIError::IncorrectCredentials));
                }
                // Both user exists and password is valid - success!
            }
            Err(crate::domain::data_stores::UserStoreError::UserNotFound) => {
                return (jar, Err(AuthAPIError::IncorrectCredentials));
            }
            Err(_) => {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }
        }
    } // Lock is released here

    // Create successful login response
    let response = LoginResponse {
        message: "Login successful!".to_string(),
    };

    // Ok((StatusCode::OK, Json(response)))

    // Call the generate_auth_cookie function defined in the auth module.
    // If the function call fails return AuthAPIError::UnexpectedError.
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
