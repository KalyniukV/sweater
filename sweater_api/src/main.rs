use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use sweater_api::{get_postgres_pool, Application};
use sweater_api::app_state::AppState;
use sweater_api::data_store::{PostgresNotificationStore, PostgresUserStore, PostgresAuthenticationStore};
use sweater_api::utils::DATABASE_URL;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let notification_store = Arc::new(RwLock::new(PostgresNotificationStore::new(pg_pool.clone())));
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool.clone())));
    let authentication_store = Arc::new(RwLock::new(PostgresAuthenticationStore::new(pg_pool.clone())));

    let app_state = AppState::new(notification_store, user_store, authentication_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}