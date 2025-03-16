use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref DATA_STORAGE_PATH: String = set_data_storage_path();
}

fn set_database_url() -> String {
    dotenv().ok();
    let database_url = std::env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if database_url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    database_url
}

fn set_data_storage_path() -> String {
    dotenv().ok();
    let data_storage_path = std::env::var(env::DATA_STORAGE_PATH_ENV_VAR).expect("DATA_STORAGE_PATH must be set.");
    if data_storage_path.is_empty() {
        panic!("DATA_STORAGE_PATH must not be empty.");
    }
    data_storage_path
}

pub mod env {
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const DATA_STORAGE_PATH_ENV_VAR: &str = "DATA_STORAGE_PATH";
}