use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

// TODO: Use Axum's state extractor to pass in AppState
pub async fn signup(
    State(app_state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    println!("Signup endpoint called!"); // Add this

    let user = User {
        email: request.email,
        password: request.password,
        requires_2fa: request.requires_2fa,
    };

    // Get the lock on the user store
    let mut user_store = app_state.user_store.write().await;

    // Add the user to the user store
    if let Err(_e) = user_store.add_user(user) {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    } else {
        let response = SignupResponse {
            message: "User created successfully".to_string(),
        };
        return (StatusCode::CREATED, Json(response)).into_response();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
