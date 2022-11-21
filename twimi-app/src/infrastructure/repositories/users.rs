use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::Component;

use twimi_core::domain::{
    models::user::{Hashed, NewUser, User, Username},
    repositories::users::{InsertionError, UsersRepository},
};

use crate::{infrastructure::models::users::UserRecord, infrastructure::services::Database};

#[derive(Component)]
#[shaku(interface = UsersRepository)]
pub struct UsersRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn insert_user(&self, new_user: NewUser<Hashed>) -> Result<User, InsertionError> {
        let mut user_record: UserRecord = new_user.into();
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            user_record.username,
            user_record.email,
            user_record.password_hash,
            user_record.created_at,
            user_record.updated_at,
        )
        .execute(self.database.pool())
        .await
        .context("username or email is already taken.")?
        .last_insert_id();

        user_record.set_id(user_id as i64);

        Ok(user_record.into())
    }

    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error> {
        Ok(sqlx::query_as!(
            UserRecord,
            r#"
            SELECT * FROM users WHERE username = ?;
            "#,
            username.as_ref()
        )
        .fetch_optional(self.database.pool())
        .await? // Option<UserRecord, None>
        .map(|user_record: UserRecord| user_record.into()))
    }
}
