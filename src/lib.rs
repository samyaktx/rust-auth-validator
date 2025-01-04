use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod auth;
mod routes;
mod services;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = routes::app().await;

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
