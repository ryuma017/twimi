use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

#[derive(Debug)]
pub struct Username(String); // adana

impl FromStr for Username {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if valid_length(s, 1, 255) {
            Ok(Self(s.into()))
        } else {
            Err("Username must be between 1 and 255 characters long.".into())
        }
    }
}

#[derive(Debug)]
pub struct Password(String);

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if valid_length(s, 8, 50) {
            Ok(Self(s.into()))
        } else {
            Err("Username must be between 1 and 255 characters long.".into())
        }
    }
}

#[derive(Debug)]
pub struct Email(String);

impl FromStr for Email {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_email(s) {
            Ok(Self(s.into()))
        } else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

fn valid_length(s: &str, min: usize, max: usize) -> bool {
    let trimmed = s.trim();
    (min..=max).contains(&trimmed.graphemes(true).count())
}
