#![allow(unused_variables)]
#![allow(unused_imports)]

use auth_service::domain::data_stores::UserStore;
use auth_service::{AppState, Application, HashmapUserStore};
use tokio::sync::RwLock;

use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}
