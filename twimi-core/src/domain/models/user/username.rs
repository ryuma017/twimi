use super::{ensure_validated, validate_length, ValidationError};

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

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

impl ToString for Username {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
