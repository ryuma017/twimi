use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::Component;

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
        let mut record: UsersTable = new_user.into();
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            record.username,
            record.email,
            record.password_hash,
            record.created_at,
            record.updated_at,
        )
        .execute(self.database.pool())
        .await
        .context("username or email is already taken.")?
        .last_insert_id() as i64;

        record.set_id(user_id);

        Ok(record.into())
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
        todo!()
        // let record: UsersTable = updated_user.into();
        // sqlx::query!(
        //     r#"
        //     UPDATE users
        //     SET username = ?, email = ?, updated_at = ?
        //     WHERE user_id = ?;
        //     "#,
        //     record.username,
        //     record.email,
        //     record.password_hash,
        //     record.updated_at,
        //     record.user_id,
        // )
        // .execute(self.database.pool())
        // .await?;

        // Ok(record.into())
    }
}
