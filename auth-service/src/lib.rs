use axum::{
    routing::{get, post},
    Router,
};

use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use std::error::Error;
use tower_http::services::ServeDir;

mod app_state;
pub use app_state::{AppState, UserStoreType};
mod domain;
mod routes;
pub use routes::signup::SignupResponse; // publicly expose the SignupResponse struct for testing
mod services;
pub use services::hashmap_user_store::HashmapUserStore;

// this struct encapsulates our application-related logic
pub struct Application {
    pub address: String, // public address to allow tests to access it
    router: Router,
    listener: tokio::net::TcpListener,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify_2fa", post(verify_2fa))
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
