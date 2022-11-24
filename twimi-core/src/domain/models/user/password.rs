use std::marker::PhantomData;

use super::super::{ensure_validated, validate_length, ValidationError};

#[derive(Debug)]
pub struct Password<K: PwKind> {
    value: String,
    _marker: PhantomData<K>,
}

impl Password<Plain> {
    fn new(s: String) -> Self {
        Password {
            value: s,
            _marker: PhantomData,
        }
    }

    pub fn hash(self, hashed: String) -> Password<Hashed> {
        Password {
            value: hashed,
            _marker: PhantomData,
        }
    }
}

impl TryFrom<String> for Password<Plain> {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ensure_validated(
            &value,
            |v| validate_length(v, 8, 50),
            "Password must be between 8 and 50 characters long.",
        )?;
        Ok(Password::<Plain>::new(value))
    }
}

impl From<String> for Password<Hashed> {
    fn from(value: String) -> Self {
        Password {
            value,
            _marker: PhantomData,
        }
    }
}

impl From<Password<Hashed>> for String {
    fn from(value: Password<Hashed>) -> Self {
        value.value
    }
}

impl AsRef<str> for Password<Hashed> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

pub trait PwKind {}

pub struct Plain;

impl PwKind for Plain {}

pub struct Hashed;

impl PwKind for Hashed {}
