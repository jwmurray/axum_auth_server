use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref REDIS_HOSTNAME: String = set_redis_hostname();
    pub static ref REDIS_PORT: String = set_redis_port();
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

// New!
fn set_redis_hostname() -> String {
    dotenv().ok();
    let hostname =
        std_env::var(env::REDIS_HOSTNAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_string());
    hostname
}

fn set_redis_port() -> String {
    dotenv().ok();
    let port = std_env::var(env::REDIS_PORT_ENV_VAR).unwrap_or(DEFAULT_REDIS_PORT.to_string());
    port
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOSTNAME_ENV_VAR: &str = "REDIS_HOSTNAME";
    pub const REDIS_PORT_ENV_VAR: &str = "REDIS_PORT";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "localhost";
pub const DEFAULT_REDIS_PORT: &str = "6379";

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub const REDIS_HOSTNAME: &str = "redis";
    pub const REDIS_PORT: &str = "6379";
}

pub mod dev {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub const REDIS_HOSTNAME: &str = "localhost";
    pub const REDIS_PORT: &str = "6379";
}
