use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use super::models::Users;
use crate::{
    domain::{Email, Parse, ParseError, Password, Username},
    startup::Database,
};

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ValidationError(#[from] ParseError),
}

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), SignUpError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), SignUpError> {
        let username: Username = username.parse()?;
        let email: Email = email.parse()?;
        let password: Password = password.parse()?;

        let mut transaction = self.database.pool().begin().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO kaiin (adana, mail_address, password, updated_at)
            VALUES (?, ?, ?, current_timestamp);
            "#,
            username.as_ref(),
            email.as_ref(),
            password.as_ref(),
        )
        .execute(&mut transaction)
        .await?;

        let _users = sqlx::query_as!(
            Users,
            r#"
            SELECT kaiin_id, adana, mail_address, password, created_at, updated_at
            FROM kaiin
            WHERE kaiin_id = ?;
            "#,
            result.last_insert_id(),
        )
        .fetch_one(&mut transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
