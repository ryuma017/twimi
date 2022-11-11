pub mod user;

pub use user::User;

use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ValidationError(String);

pub struct Id<T> {
    pub value: i64,
    _marker: PhantomData<T>,
}

impl<T> From<i64> for Id<T> {
    fn from(value: i64) -> Self {
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
