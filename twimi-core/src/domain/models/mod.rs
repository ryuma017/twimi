mod user;

pub use user::{
    email::Email,
    password::{self, Password},
    username::Username,
    NewUser, User,
};

use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
#[error("Failed to validate: {0}")]
pub struct ValidationError(String);

pub struct Id<T> {
    value: i64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn value(&self) -> i64 {
        self.value
    }
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
