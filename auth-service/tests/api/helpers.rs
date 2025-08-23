#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use auth_service::Application;
use reqwest::cookie::Jar;
use serde;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use sqlx::Connection;
use std::str::FromStr;

// use auth_service::services::data_stores::hashmap_user_store::HashmapUserStore;
use auth_service::services::data_stores::postgres_user_store::PostgresUserStore;

use auth_service::get_postgres_pool;
use auth_service::get_redis_client;
use auth_service::utils::constants::{DATABASE_URL, REDIS_HOSTNAME};
use sqlx::{postgres::PgConnectOptions, postgres::PgPoolOptions, Executor, PgConnection, PgPool};

use auth_service::services::data_stores::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
// use auth_service::services::data_stores::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::services::data_stores::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;

use auth_service::app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType};

use reqwest;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
    pub db_name: String,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new(db_name: String) -> Self {
        let db_pool = Self::configure_postgresql(&db_name).await;
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(db_pool)));
        let redis_conn = get_redis_client(REDIS_HOSTNAME.to_owned())
            .expect("Failed to get Redis client")
            .get_connection()
            .expect("Failed to get Redis connection");
        let redis_conn = Arc::new(RwLock::new(redis_conn));
        let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_conn)));
        let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let email_client = Arc::new(RwLock::new(MockEmailClient::default()));
        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client.clone(),
        );
        let app = Application::build(app_state, "0.0.0.0:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", &app.address);

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // create a reqwest http client instance
        let http_client = reqwest::Client::new();
        // Create new TestApp instance with the address and http_client
        TestApp {
            address,
            cookie_jar: Arc::new(Jar::default()),
            banned_token_store: banned_token_store.clone(),
            two_fa_code_store: two_fa_code_store.clone(),
            http_client,
            db_name,
            clean_up_called: false,
        }
    }

    pub async fn configure_postgresql(db_name: &str) -> PgPool {
        // Remove any existing database name from the URL to get the base connection string

        let postgresql_conn_url = DATABASE_URL
            .rsplit_once('/')
            .map(|(base, _)| base.to_string())
            .unwrap_or_else(|| DATABASE_URL.to_owned());

        // We are creating a new database for each test case, and we need to ensure each database has a unique name!
        let db_name = Uuid::new_v4().to_string();

        Self::configure_database(&postgresql_conn_url, &db_name).await;

        let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

        // Create a new connection pool and return it
        get_postgres_pool(&postgresql_conn_url_with_db)
            .await
            .expect("Failed to create Postgres connection pool!")
    }

    pub async fn configure_database(db_conn_string: &str, db_name: &str) {
        // Create database connection
        let connection = PgPoolOptions::new()
            .connect(db_conn_string)
            .await
            .expect("Failed to create Postgres connection pool.");

        // Create a new database
        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
            .await
            .expect("Failed to create database.");

        // Connect to new database
        let db_conn_string = format!("{}/{}", db_conn_string, db_name);

        let connection = PgPoolOptions::new()
            .connect(&db_conn_string)
            .await
            .expect("Failed to create Postgres connection pool.");

        // Run migrations against new database
        sqlx::migrate!()
            .run(&connection)
            .await
            .expect("Failed to migrate the database");
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to get root")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to get signup logout")
    }

    // pub async fn post_verify_2fa(&self) -> reqwest::Response {
    //     self.http_client
    //         .post(&format!("{}/verify_2fa", &self.address))
    //         .send()
    //         .await
    //         .expect("Failed to get signup logout")
    // }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify_token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to get signup logout")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clean_up(&mut self) {
        delete_database(&self.db_name).await;
        self.clean_up_called = true;
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("TestApp::clean_up() was not called! Test databases must be cleaned up.");
        }
    }
}

pub async fn setup_user_for_login_with_password_no_2fa(app: &TestApp) -> (String, String) {
    let random_email = get_random_email();
    let good_password = "password123".to_string();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": good_password,
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    (random_email, good_password)
}

pub async fn setup_user_for_login_with_password_and_2fa(app: &TestApp) -> (String, String) {
    let random_email = get_random_email();
    let good_password = "password123".to_string();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": good_password,
        "requires2FA": true,
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    (random_email, good_password)
}

pub fn get_random_email() -> String {
    format!("{}@example.com", &Uuid::new_v4())
}

pub async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}
