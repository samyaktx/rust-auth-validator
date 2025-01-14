use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExists,
    UserNoLongerExists,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(length) => format!("Password cannot be longer than {} characters", length),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::HashingError => "An error occurred while hashing the password".to_string(),
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            ErrorMessage::ServerError => "An error occurred on the server".to_string(),
            ErrorMessage::WrongCredentials => "Wrong credentials".to_string(),
            ErrorMessage::EmailExists => "Email already exists".to_string(),
            ErrorMessage::UserNoLongerExists => "User no longer exists".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::PermissionDenied => "Permission denied".to_string(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unquie_constraint_violation(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }   

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: self.status.to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "HttpError: message: {}, status: {}", 
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}
