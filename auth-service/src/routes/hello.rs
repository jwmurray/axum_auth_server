use axum::{http::StatusCode, response::IntoResponse};

pub async fn hello() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
