use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};
use time::OffsetDateTime;

use super::Database;
use crate::domain::NewUser;

#[async_trait]
pub trait UsersRepository: Interface {
    async fn insert_user(&self, user: NewUser) -> Result<UserRecord, anyhow::Error>;
}

#[derive(Component)]
#[shaku(interface = UsersRepository)]
pub struct UsersRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn insert_user(
        &self,
        NewUser {
            username,
            email,
            password,
        }: NewUser,
    ) -> Result<UserRecord, anyhow::Error> {
        let timestamp = OffsetDateTime::now_utc();
        let result = sqlx::query!(
            r#"
            INSERT INTO kaiin (adana, mail_address, password, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            username.as_ref(),
            email.as_ref(),
            password.compute_hash()?,
            timestamp,
            timestamp,
        )
        .execute(self.database.pool())
        .await?;
        Ok(UserRecord {
            id: result.last_insert_id(),
            username: username.as_ref().to_owned(),
            email: email.as_ref().to_owned(),
            created_at: timestamp,
            updated_at: timestamp,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UserRecord {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
