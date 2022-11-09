use argon2::{
    password_hash::{self, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ValidationError(String);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ComputeHashError(#[from] password_hash::Error); // requires "std" feature

#[derive(Debug)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !valid_length(&value, 1, 255) {
            return Err(ValidationError(
                "Username must be between 1 and 255 characters long.".into(),
            ));
        }
        Ok(Username(value))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !valid_length(&value, 8, 50) {
            return Err(ValidationError(
                "Username must be between 1 and 255 characters long.".into(),
            ));
        }
        Ok(Password(value))
    }
}

impl Password {
    pub fn compute_hash(&self) -> Result<String, ComputeHashError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hashed = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(), // params が正しければ panic しない
        )
        .hash_password(self.0.as_bytes(), &salt)?
        .to_string();
        Ok(hashed)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !validate_email(value.as_str()) {
            return Err(ValidationError(format!(
                "{} is invalid email format.",
                value
            )));
        }
        Ok(Email(value))
    }
}

fn valid_length(s: &str, min: usize, max: usize) -> bool {
    let trimmed = s.trim();
    (min..=max).contains(&trimmed.graphemes(true).count())
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
