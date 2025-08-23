#![allow(unused_imports)]
use std::error::Error;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    serve::Serve,
    Json, Router,
};

use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

pub mod app_state;
pub mod domain;
pub use domain::AuthAPIError;
pub mod routes;
pub use routes::login::TwoFactorAuthResponse;
pub use routes::signup::SignupResponse; // publicly expose the SignupResponse struct for testing // publicly expose the TwoFactorAuthResponse struct for testing
pub mod services;
pub use services::data_stores::hashmap_user_store::HashmapUserStore;
pub mod utils;
pub use app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType, UserStoreType};
pub use utils::constants::JWT_COOKIE_NAME;

// this struct encapsulates our application-related logic
pub struct Application {
    pub address: String, // public address to allow tests to access it
    router: Router,
    listener: tokio::net::TcpListener,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route("/", get(root))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify_2fa", post(verify_2fa)) // Keep both for compatibility
            .route("/verify_token", post(verify_token))
            .route("/hello", get(hello))
            .fallback_service(ServeDir::new("assets"))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        let address = listener.local_addr()?.to_string();

        // create a new Application instance and return it
        Ok(Application {
            router,
            address,
            listener,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on http://{}", self.address);
        axum::serve(self.listener, self.router).await
    }
}

// Root route handler that serves the main HTML page
async fn root() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"), // Conflict = 409
            AuthAPIError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}
