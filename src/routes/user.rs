use axum::{routing::{get, put}, Router};
use axum::middleware;

use crate::controller::user::{get_me, get_users, update_user_name, update_user_role, update_user_password};
use crate::middleware::role_check;
use crate::models::UserRole;

pub fn users_handler() -> Router {
    Router::new()
        .route(
            "/me", 
            get(get_me)
            .layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin, UserRole::User])
            }))
        )
        .route(
            "/users", 
            get(get_users)
            .layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin])
            }))
        )
        .route("/name", put(update_user_name))
        .route("/role", put(update_user_role))
        .route("/password", put(update_user_password))
}