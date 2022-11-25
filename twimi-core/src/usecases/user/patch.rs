use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{UpdatedUser, User, ValidationError},
    repositories::users::UsersRepository,
};

#[async_trait]
pub trait UpdateAuthnUser: Interface {
    async fn update_authenticated_user(
        &self,
        input: UpdateAuthnUserInput,
    ) -> Result<UpdateAuthnUserOutput, UpdateAuthnUserUseCaseError>;
}

#[derive(Component)]
#[shaku(interface = UpdateAuthnUser)]
pub struct UpdateAuthnUserUseCase {
    #[shaku(inject)]
    repository: Arc<dyn UsersRepository>,
}

#[async_trait]
impl UpdateAuthnUser for UpdateAuthnUserUseCase {
    async fn update_authenticated_user(
        &self,
        input: UpdateAuthnUserInput,
    ) -> Result<UpdateAuthnUserOutput, UpdateAuthnUserUseCaseError> {
        Ok(self.repository.update_user(input.try_into()?).await?.into())
    }
}

pub struct UpdateAuthnUserInput {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl UpdateAuthnUserInput {
    pub fn new(user_id: String, username: Option<String>, email: Option<String>) -> Self {
        UpdateAuthnUserInput {
            user_id,
            username,
            email,
        }
    }
}

impl TryFrom<UpdateAuthnUserInput> for UpdatedUser {
    type Error = ValidationError;

    fn try_from(value: UpdateAuthnUserInput) -> Result<Self, Self::Error> {
        Ok(UpdatedUser {
            id: value.user_id.into(),
            username: value.username.map(TryInto::try_into).transpose()?,
            email: value.email.map(TryInto::try_into).transpose()?,
        })
    }
}

pub struct UpdateAuthnUserOutput {
    pub user: User,
}

impl From<User> for UpdateAuthnUserOutput {
    fn from(value: User) -> Self {
        UpdateAuthnUserOutput { user: value }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateAuthnUserUseCaseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}
