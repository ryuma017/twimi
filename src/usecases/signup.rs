use std::sync::Arc;

use async_trait::async_trait;
use serde::Serialize;
use shaku::{Component, Interface};
use time::OffsetDateTime;

use super::models::Users;
use crate::{
    domain::{ComputeHashError, Email, Password, Username, ValidationError},
    startup::Database,
};

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnexpectedError(#[from] ComputeHashError),
}

pub struct SignUpInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct SignUpOutput {
    id: i64,
    username: String,
    email: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<Users> for SignUpOutput {
    fn from(user: Users) -> Self {
        Self {
            id: user.kaiin_id,
            username: user.adana,
            email: user.mail_address,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpError> {
        let username: Username = input.username.try_into()?;
        let email: Email = input.email.try_into()?;
        let password: Password = input.password.try_into()?;

        let mut transaction = self.database.pool().begin().await?;

        let result = sqlx::query!(
            r#"
            INSERT INTO kaiin (adana, mail_address, password, updated_at)
            VALUES (?, ?, ?, current_timestamp);
            "#,
            username.as_ref(),
            email.as_ref(),
            password.compute_hash()?,
        )
        .execute(&mut transaction)
        .await?;

        let user = sqlx::query_as!(
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

        Ok(user.into())
    }
}
