use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATABASE_URL: String = set_database_url();
}

fn set_database_url() -> String {
    dotenv().ok();
    let database_url = std::env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if database_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    database_url
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}