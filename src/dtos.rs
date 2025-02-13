use core::str;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use validator::Validate;

use crate::models::{ User, UserRole };

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Role is required"),
        must_match(other = "password", message = "New passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            verified: user.verified,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: &Vec<User>) -> Vec<Self> {
        users
            .iter()
            .map(|user| Self::filter_user(user))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub result: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RoleUpdateDto {
    #[validate(custom(function = "validate_user_role"))]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        // _ => Err(validator::ValidationError::new("Invalid role")),
    }
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        must_match(other = "new_password", message = "New passwords do not match")
    )]
    pub new_password_confirm: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub old_password: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordRequestDto {
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct ResetPasswordDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        must_match(other = "new_password", message = "New passwords do not match")
    )]
    pub new_password_confirm: String,
}

