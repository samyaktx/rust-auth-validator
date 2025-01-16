mod models;
mod config;
mod dtos;
mod error;
mod database;
mod utils;

use axum::http::header::{
    ACCEPT, 
    AUTHORIZATION, 
    CONTENT_TYPE
};
use axum::http::HeaderValue;
use axum::http::Method;
use axum::Extension;
use axum::Router;
use config::Config;
use database::DBClient;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use dotenvy::dotenv;

#[derive(Debug,Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init()?;

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        },
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(e.into());
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let db_client = DBClient::new(pool);

    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    let app = Router::new()
        .layer(Extension(app_state))
        .layer(cors.clone());

    println!(
        "{}",
        format!("Server is running on port {}", config.port)
    );

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", config.port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}