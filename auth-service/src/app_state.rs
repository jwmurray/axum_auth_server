#![allow(unused_variables)]
#![allow(unused_imports)]

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::UserStore;

// Using a type alias to improve readability!
// Now works with ANY implementation of UserStore trait
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

// App state holds all the data we want to share across route handlers.
#[derive(Clone)]
pub struct AppState {
    // user_store is now a trait object that can hold ANY UserStore implementation
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}
