use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod routes;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = Router::new().merge(routes::router());

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}
