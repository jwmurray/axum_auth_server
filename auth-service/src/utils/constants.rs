use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
}

fn set_database_url() -> String {
    dotenv().ok(); // Load environment variables
    let url = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    url
}

fn set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
