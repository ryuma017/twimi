use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use twimi_core::{
    domain::models::user::{NewUser, User},
    domain::{models::user::Username, repositories::users::UsersRepository},
};

use crate::{infrastructure::models::kaiin::KaiinTable, infrastructure::Database};

#[derive(Component)]
#[shaku(interface = UsersRepository)]
pub struct UsersRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn insert_user(&self, new_user: NewUser) -> Result<User, anyhow::Error> {
        let kaiin: KaiinTable = new_user.try_into()?;
        let kaiin_id = sqlx::query_as!(
            KaiinTable,
            r#"
            INSERT INTO kaiin (adana, mail_address, password, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
            kaiin.adana,
            kaiin.mail_address,
            kaiin.password,
            kaiin.created_at,
            kaiin.updated_at
        )
        .execute(self.database.pool())
        .await?
        .last_insert_id();

        Ok(kaiin.try_into().map(|user: User| user.set_id(kaiin_id))?)
    }

    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error> {
        let kaiin = sqlx::query_as!(
            KaiinTable,
            r#"
            SELECT * FROM kaiin WHERE adana = ?;
            "#,
            username.as_ref()
        )
        .fetch_optional(self.database.pool())
        .await?;

        Ok(kaiin.map(|kaiin| kaiin.try_into()).transpose()?)
    }
}
