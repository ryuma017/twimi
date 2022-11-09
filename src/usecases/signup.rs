use std::sync::Arc;

use async_trait::async_trait;
use serde::Serialize;
use shaku::{Component, Interface};
use time::OffsetDateTime;

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
    id: u64,
    username: String,
    email: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
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
            timestamp
        )
        .execute(self.database.pool())
        .await?;

        Ok(SignUpOutput {
            id: result.last_insert_id(),
            username: username.as_ref().to_owned(),
            email: email.as_ref().to_owned(),
            created_at: timestamp,
            updated_at: timestamp,
        })
    }
}
