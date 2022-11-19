use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::Component;

use twimi_core::{
    domain::models::user::{NewUser, User},
    domain::{
        models::user::Username,
        repositories::users::{InsertionError, UsersRepository},
    },
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
    async fn insert_user(&self, new_user: NewUser) -> Result<User, InsertionError> {
        let user: UserRecord = new_user.try_into()?;
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at
        )
        .execute(self.database.pool())
        .await
        .context("username or email is already taken.")?
        .last_insert_id();

        Ok(user
            .try_into()
            .map(|user: User| user.with_id(user_id as i64))?)
    }

    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error> {
        sqlx::query_as!(
            UserRecord,
            r#"
            SELECT * FROM users WHERE username = ?;
            "#,
            username.as_ref()
        )
        .fetch_optional(self.database.pool())
        .await?
        .map(|user| user.try_into())
        .transpose()
    }
}
