use axum::{routing::{get, post}, Router};

use crate::controller::auth::{forgot_password, login, register, reset_password, verify_email};

pub fn auth_handler() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}