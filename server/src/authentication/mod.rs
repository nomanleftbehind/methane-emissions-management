pub mod cookie;
mod manager;
mod passwords;
pub mod utils;

pub use manager::*;
pub use passwords::{register, validate_credentials, Credentials};

pub const USER_ID_SESSION_KEY: &str = "user_id";
