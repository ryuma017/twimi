pub mod email;
pub mod password;
pub mod username;

use time::OffsetDateTime;
use unicode_segmentation::UnicodeSegmentation;

use email::Email;
use password::{Hashed, Password, Plain, PwKind};
use username::Username;

use super::{Id, ValidationError};

pub struct User {
    pub id: Id<Self>,
    pub username: Username,
    pub email: Email,
    pub password_hash: Password<Hashed>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub struct NewUser<K: PwKind> {
    pub username: Username,
    pub email: Email,
    pub password: Password<K>,
}

impl NewUser<Plain> {
    pub fn with_password_hash(self, password_hash: String) -> NewUser<Hashed> {
        NewUser {
            username: self.username,
            email: self.email,
            password: self.password.hash(password_hash),
        }
    }
}

fn validate_length(s: &str, min: usize, max: usize) -> bool {
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
