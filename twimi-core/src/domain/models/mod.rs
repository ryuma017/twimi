mod user;

pub use user::{
    email::Email,
    password::{self, Password},
    username::Username,
    NewUser, UpdatedUser, User,
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

fn validate_length(s: &str, min: usize, max: usize) -> bool {
    use unicode_segmentation::UnicodeSegmentation;

    let trimmed = s.trim();
    (min..=max).contains(&trimmed.graphemes(true).count())
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
