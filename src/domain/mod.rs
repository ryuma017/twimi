use argon2::{
    password_hash::{self, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

pub trait Parse<T>: Sized {
    fn parse(self) -> Result<T, ParseError>;
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ParseError(String);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ComputeHashError(#[from] password_hash::Error); // requires "std" feature

#[derive(Debug)]
pub struct Username(String);

impl Parse<Username> for String {
    fn parse(self) -> Result<Username, ParseError> {
        if valid_length(&self, 1, 255) {
            Ok(Username(self))
        } else {
            Err(ParseError(
                "Username must be between 1 and 255 characters long.".into(),
            ))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Password(String);

impl Parse<Password> for String {
    fn parse(self) -> Result<Password, ParseError> {
        if valid_length(self.as_str(), 8, 50) {
            Ok(Password(self))
        } else {
            Err(ParseError(
                "Username must be between 1 and 255 characters long.".into(),
            ))
        }
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

impl Parse<Email> for String {
    fn parse(self) -> Result<Email, ParseError> {
        if validate_email(&self) {
            Ok(Email(self))
        } else {
            Err(ParseError(format!("{} is invalid email format.", self)))
        }
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
