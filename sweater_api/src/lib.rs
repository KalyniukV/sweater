use std::error::Error;
use axum::http::{header, Method};
use axum::Router;
use axum::routing::{get, post};
use axum::serve::Serve;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use crate::app_state::AppState;

pub mod routes;
pub mod domain;
pub mod data_store;
pub mod utils;
pub mod app_state;

pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://127.0.0.1:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_headers(vec![header::CONTENT_TYPE])
            .allow_origin(allowed_origins);

        let router = Router::new()
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/create_notification", post(routes::create_notification))
            .route("/notifications", get(routes::get_notifications))
            .with_state(app_state)
            .layer(cors);

        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}