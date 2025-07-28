// use axum::{response::Html, routing::get, Router};
// use tower_http::services::ServeDir;

use auth_service::{AppState, Application, HashmapUserStore};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::new_arc_rwlock();
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}
