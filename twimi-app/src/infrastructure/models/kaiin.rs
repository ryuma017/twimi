use sqlx::FromRow;
use time::OffsetDateTime;

use twimi_core::domain::models::user::{NewUser, User};

#[allow(unused)]
#[derive(FromRow)]
pub struct KaiinTable {
    pub kaiin_id: i64,
    pub adana: String,
    pub mail_address: String,
    pub password: String, // hashed
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TryFrom<KaiinTable> for User {
    type Error = anyhow::Error;

    fn try_from(value: KaiinTable) -> Result<Self, Self::Error> {
        Ok(User {
            id: Default::default(),
            username: value.adana.try_into()?,
            email: value.mail_address.try_into()?,
            password_hash: value.password,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

impl TryFrom<NewUser> for KaiinTable {
    type Error = anyhow::Error;

    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let timestamp = OffsetDateTime::now_utc();
        Ok(KaiinTable {
            kaiin_id: Default::default(),
            adana: value.username.as_ref().to_owned(),
            mail_address: value.email.as_ref().to_owned(),
            password: value.password_hash,
            created_at: timestamp,
            updated_at: timestamp,
        })
    }
}
