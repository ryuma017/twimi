use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{User, ValidationError},
    repositories::users::UsersRepository,
};

#[async_trait]
pub trait GetAuthnUser: Interface {
    async fn get_authenticated_user(
        &self,
        input: GetAuthnUserInput,
    ) -> Result<GetAuthnUserOutput, GetAuthnUserUseCaseError>;
}

#[derive(Component)]
#[shaku(interface = GetAuthnUser)]
pub struct GetAuthnUserUseCase {
    #[shaku(inject)]
    repository: Arc<dyn UsersRepository>,
}

#[async_trait]
impl GetAuthnUser for GetAuthnUserUseCase {
    async fn get_authenticated_user(
        &self,
        input: GetAuthnUserInput,
    ) -> Result<GetAuthnUserOutput, GetAuthnUserUseCaseError> {
        Ok(self
            .repository
            .find_user_by_username(input.username.try_into()?)
            .await?
            .context("User not found.")?
            .into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetAuthnUserUseCaseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

pub struct GetAuthnUserInput {
    pub username: String,
}

pub struct GetAuthnUserOutput {
    pub user: User,
}

impl From<User> for GetAuthnUserOutput {
    fn from(value: User) -> Self {
        GetAuthnUserOutput { user: value }
    }
}
