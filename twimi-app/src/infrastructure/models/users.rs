use sqlx::FromRow;
use time::OffsetDateTime;

use twimi_core::domain::models::user::{Hashed, NewUser, User};

#[allow(unused)]
#[derive(FromRow)]
pub struct UserRecord {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserRecord {
    pub fn set_id(&mut self, id: i64) {
        self.user_id = id;
    }
}

impl From<UserRecord> for User {
    fn from(value: UserRecord) -> Self {
        User {
            id: value.user_id.into(),
            username: value.username.try_into().unwrap(),
            email: value.email.try_into().unwrap(),
            password_hash: value.password_hash.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<NewUser<Hashed>> for UserRecord {
    fn from(value: NewUser<Hashed>) -> Self {
        let timestamp = OffsetDateTime::now_utc();
        Self {
            user_id: Default::default(),
            username: value.username.into(),
            email: value.email.into(),
            password_hash: value.password.into(),
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}
