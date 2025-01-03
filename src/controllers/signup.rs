use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

use crate::model::{AuthRequest, AuthResponse, ErrorResponse, User, UserResponse, Users};
use crate::controllers::create_jwt;

pub async fn signup(
    State(users): State<Users>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut users = users.lock().unwrap();
    
    if users.values().any(|u| u.get_email() == req.email) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Email already registered".to_string(),
            }),
        ));
    }

    let hashed_password = hash(req.password.as_bytes(), DEFAULT_COST)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            }),
        ))?;

    let user = User {
        id: Uuid::new_v4().to_string(),
        email: req.email.clone(),
        password: hashed_password,
    };

    let token = create_jwt(&user.id, &user.email)
        .map_err(|_| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create token".to_string(),
            }),
        ))?;

    let user_response = UserResponse {
        id: user.id.clone(),
        email: user.email.clone(),
    };

    users.insert(user.id.clone(), user);

    Ok(Json(AuthResponse { token, user: user_response }))
}