use time::OffsetDateTime;

use super::{Id, ValidationError};
use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

pub struct NewUser {
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub password_hash: String,
}

impl NewUser {
    pub fn with_password_hash(mut self, password_hash: String) -> Self {
        self.password_hash = password_hash;
        self
    }
}

pub struct User {
    pub id: Id<Self>,
    pub username: Username,
    pub email: Email,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn id(&self) -> i64 {
        self.id.value
    }
    pub fn set_id(mut self, id: i64) -> Self {
        self.id = id.into();
        self
    }
}

pub struct UserCredentials {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_validated(
            &value,
            |v| validate_length(v, 1, 255),
            "Username must be between 1 and 255 characters long.",
        )?;
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
        ensure_validated(
            &value,
            |v| validate_length(v, 8, 50),
            "Password must be between 8 and 50 characters long.",
        )?;
        Ok(Password(value))
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
        ensure_validated(&value, validate_email, "Invalid email format.")?;
        Ok(Email(value))
    }
}

fn validate_length(s: &str, min: usize, max: usize) -> bool {
    let trimmed = s.trim();
    (min..=max).contains(&trimmed.graphemes(true).count())
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn ensure_validated<V, F>(value: V, f: F, msg: &str) -> Result<(), ValidationError>
where
    F: FnOnce(V) -> bool,
{
    match f(value) {
        true => Ok(()),
        false => Err(ValidationError(msg.into())),
    }
}
