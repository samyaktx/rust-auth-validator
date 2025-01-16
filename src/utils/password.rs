use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2
};

use crate::error::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;

/// Hashes a plaintext password.
pub fn hash_password(password: impl Into<String>) -> Result<String, ErrorMessage> {
    let password = password.into();

    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HashingError)
        .and_then(|hash| Ok(hash.to_string()));

    hashed_password
}

/// Compares a plaintext password with a hashed password.
pub fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {
    if password.is_empty() || hashed_password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }

    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|_| ErrorMessage::InvalidHashFormat)?;

    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(result)
}