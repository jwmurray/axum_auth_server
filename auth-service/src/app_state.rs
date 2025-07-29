use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::hashmap_user_store::HashmapUserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

// App state holds all the data we want to share across route handlers.
#[derive(Clone)]
pub struct AppState {
    // user_store is Arc<RwLock<Hash>> -- RwLock allows thread safe changes to the hash.  Arc allows thread save reference counting sharing of the hashmap.
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(HashmapUserStore::default())),
        }
    }
}
