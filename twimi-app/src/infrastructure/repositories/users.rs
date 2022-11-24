use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::Component;

use time::OffsetDateTime;
use twimi_core::domain::{
    models::{password::Hashed, Id, NewUser, UpdatedUser, User, Username},
    repositories::users::UsersRepository,
};

use crate::infrastructure::{models::UsersTable, Database};

#[derive(Component)]
#[shaku(interface = UsersRepository)]
pub struct UsersRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn insert_user(&self, new_user: NewUser<Hashed>) -> Result<User, anyhow::Error> {
        // let mut record: UsersTable = new_user.into();
        let timestamp = OffsetDateTime::now_utc();
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            new_user.username.as_ref(),
            new_user.email.as_ref(),
            new_user.password.as_ref(),
            timestamp,
            timestamp,
        )
        .execute(self.database.pool())
        .await
        .context("username or email is already taken.")?
        .last_insert_id() as i64;

        Ok(User {
            id: user_id.into(),
            username: new_user.username,
            email: new_user.email,
            password_hash: new_user.password,
            created_at: timestamp,
            updated_at: timestamp,
        })
    }

    async fn find_user_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error> {
        Ok(sqlx::query_as!(
            UsersTable,
            r#"
            SELECT * FROM users WHERE user_id = ?;
            "#,
            user_id.value(),
        )
        .fetch_optional(self.database.pool())
        .await?
        .map(Into::into))
    }

    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error> {
        Ok(sqlx::query_as!(
            UsersTable,
            r#"
            SELECT * FROM users WHERE username = ?;
            "#,
            username.as_ref()
        )
        .fetch_optional(self.database.pool())
        .await? // Option<UsersTable, None>
        .map(Into::into))
    }

    async fn update_user(&self, updated_user: UpdatedUser) -> Result<User, anyhow::Error> {
        let timestamp = OffsetDateTime::now_utc();
        let user_id = updated_user.id;
        let username = updated_user.username;
        let email = updated_user.email;

        match (username, email) {
            (Some(username), Some(email)) => {
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET username = ?, email = ?, updated_at = ?
                    WHERE user_id = ?;
                    "#,
                    username.as_ref(),
                    email.as_ref(),
                    timestamp,
                    user_id.value(),
                )
                .execute(self.database.pool())
                .await?;
            }
            (Some(username), None) => {
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET username = ?, updated_at = ?
                    WHERE user_id = ?;
                    "#,
                    username.as_ref(),
                    timestamp,
                    user_id.value(),
                )
                .execute(self.database.pool())
                .await?;
            }
            (None, Some(email)) => {
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET email = ?, updated_at = ?
                    WHERE user_id = ?;
                    "#,
                    email.as_ref(),
                    timestamp,
                    user_id.value(),
                )
                .execute(self.database.pool())
                .await?;
            }
            (None, None) => {}
        };

        Ok(Self::find_user_by_id(self, user_id).await?.unwrap())
    }
}
