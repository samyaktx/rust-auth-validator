use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/v1/api/signup", get(signup))
        .route("/v1/api/signin", get(signin))
}

async fn signup() -> &'static str {
    "Hello, World!"
}

async fn signin() -> &'static str {
    "Hello, World!"
}
