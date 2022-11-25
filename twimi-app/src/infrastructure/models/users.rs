use sqlx::FromRow;
use time::OffsetDateTime;

use twimi_core::domain::models::User;

#[allow(unused)]
#[derive(FromRow)]
pub struct UsersTable {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<UsersTable> for User {
    fn from(value: UsersTable) -> Self {
        if value.user_id == i64::default() {
            panic!("user_id is not set.");
        }
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
