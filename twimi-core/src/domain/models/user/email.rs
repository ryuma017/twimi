use validator::validate_email;

use super::{ensure_validated, ValidationError};

#[derive(Debug)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_validated(&value, validate_email, "Invalid email format.")?;
        Ok(Email(value))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}
