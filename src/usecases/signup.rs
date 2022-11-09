use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shaku::{Component, Interface};

use super::models::Users;
use crate::{
    domain::{NewUser, Parse, ParseError},
    startup::Database,
};

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ValidationError(#[from] ParseError),
}

#[derive(Debug, Deserialize)]
pub struct SignUpPayload {
    username: String,
    email: String,
    password: String,
}

impl Parse<NewUser> for SignUpPayload {
    fn parse(self) -> Result<NewUser, ParseError> {
        Ok(NewUser {
            username: self.username.parse()?,
            email: self.email.parse()?,
            password: self.password.parse()?,
        })
    }
}

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(&self, payload: SignUpPayload) -> Result<(), SignUpError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(&self, payload: SignUpPayload) -> Result<(), SignUpError> {
        let new_user = payload.parse()?;
        let mut transaction = self.database.pool().begin().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO kaiin (adana, mail_address, password, updated_at)
            VALUES (?, ?, ?, current_timestamp);
            "#,
            new_user.username.as_ref(),
            new_user.email.as_ref(),
            new_user.password.as_ref(),
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
