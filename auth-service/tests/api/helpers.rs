use auth_service::Application;
use reqwest::Client;

pub struct TestApp {
    pub address: String,
    pub http_client: Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // create a reqwest http client instance
        let http_client = Client::builder()
            .build()
            .expect("Failed to create http client");

        // Create new TestApp instance with the address and http_client
        TestApp {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", self.address))
            .send()
            .await
            .expect("Failed to get root")
    }


    // TODO: Implement helper functions for all other routes 
    // (signup, login, logout, verify-2fa, and verify-token)
}
