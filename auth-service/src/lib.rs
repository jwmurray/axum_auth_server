use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};

use routes::{hello, login, logout, signup, verify_2fa, verify_token};
use std::error::Error;
use tower_http::services::ServeDir;

mod routes;
// this struct encapsulates our applicaiton-related logic
pub struct Application {
    server: Serve<Router, Router>, // TODO: what is this?

    pub address: String, // public address to allow tests to access it
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify_2fa", post(verify_2fa))
            .route("/verify_token", post(verify_token))
            .route("/hello", get(hello));

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on http://{}", self.address);
        self.server.await
    }
}
