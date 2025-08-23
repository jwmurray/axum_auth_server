#![allow(unused_variables)]
#![allow(unused_imports)]

#![allow(dead_code)]

use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::Application;
use reqwest::cookie::Jar;
use serde;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;

use auth_service::app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType};

use reqwest;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
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
        }
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
