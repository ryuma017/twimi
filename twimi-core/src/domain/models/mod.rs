pub mod user;

use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ValidationError(String);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ComputeHashError(#[from] argon2::password_hash::Error); // requires "std" feature

#[derive(Debug, thiserror::Error)]
#[error("User not found: {0}")]
pub struct InvalidCredentials(String);

pub struct Id<T> {
    pub value: u64,
    _marker: PhantomData<T>,
}

impl<T> From<u64> for Id<T> {
    fn from(value: u64) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            _marker: PhantomData,
        }
    }
}
