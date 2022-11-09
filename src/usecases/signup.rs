use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use shaku::{Component, Interface};

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
        // usecase logic
        let _new_user = payload.parse()?;
        let _pool = self.database.pool();

        Ok(())
    }
}
