use sqlx::FromRow;
use time::OffsetDateTime;

use twimi_core::domain::models::user::{NewUser, User};

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

impl TryFrom<UserRecord> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserRecord) -> Result<Self, Self::Error> {
        Ok(User {
            id: Default::default(),
            username: value.username.try_into()?,
            email: value.email.try_into()?,
            password_hash: value.password_hash,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

impl TryFrom<NewUser> for UserRecord {
    type Error = anyhow::Error;

    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let timestamp = OffsetDateTime::now_utc();
        Ok(UserRecord {
            user_id: Default::default(),
            username: value.username.as_ref().to_owned(),
            email: value.email.as_ref().to_owned(),
            password_hash: value.password_hash,
            created_at: timestamp,
            updated_at: timestamp,
        })
    }
}
