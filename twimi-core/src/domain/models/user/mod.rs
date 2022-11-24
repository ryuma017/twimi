pub mod email;
pub mod password;
pub mod username;

use std::marker::PhantomData;

use time::OffsetDateTime;

use email::Email;
use password::{Hashed, Password, Plain, PwKind};
use username::Username;

use super::Id;

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

pub struct UpdatedUser {
    pub id: Id<User>,
    pub username: Option<Username>,
    pub email: Option<Email>,
}

impl From<String> for Id<User> {
    fn from(value: String) -> Self {
        Self {
            value: value.parse().unwrap(),
            _marker: PhantomData,
        }
    }
}
