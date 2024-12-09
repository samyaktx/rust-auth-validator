use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new().route("/", get( || async { "Hello, World!" }));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}
