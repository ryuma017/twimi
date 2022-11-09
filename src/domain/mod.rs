use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

pub trait Parse<T>: Sized {
    fn parse(self) -> Result<T, ParseError>;
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ParseError(String);

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

#[derive(Debug)]
pub struct NewUser {
    pub username: Username,
    pub email: Email,
    pub password: Password,
}
