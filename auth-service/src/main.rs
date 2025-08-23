#![allow(unused_variables)]
#![allow(unused_imports)]

use auth_service::domain::data_stores::UserStore;
use auth_service::{get_postgres_pool, AppState, Application, HashmapUserStore};
use tokio::sync::RwLock;

use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::data_stores::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;
use auth_service::services::mock_email_client::MockEmailClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    let db_pool = get_postgres_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run database migrations");

    println!("Database migrations completed successfully!");

    // let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(db_pool.clone())));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient::default()));
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}
