use axum::{response::Html, routing::get, serve::Serve, Router};
use std::error::Error;
use tower_http::services::ServeDir;

// this struct encapsulates our applicaiton-related logic
pub struct Application {
    server: Serve<Router, Router>, // TODO: what is this?

    pub address: String, // public address to allow tests to access it
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello_handler));

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // create a new Application instance and return it
        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", self.address);
        self.server.await
    }
}

async fn hello_handler() -> Html<&'static str> {
    // TODO: Update this to a custom message!
    Html("<h1>Hello, World!</h1>")
}
