use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::{
    domain::{ComputeHashError, NewUser, User, ValidationError},
    repositories::UsersRepository,
};

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    repository: Arc<dyn UsersRepository>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpError> {
        Ok(self.repository.insert_user(input.try_into()?).await?.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error(transparent)]
    DatabaseError(#[from] anyhow::Error),
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

impl TryFrom<SignUpInput> for NewUser {
    type Error = ValidationError;

    fn try_from(value: SignUpInput) -> Result<Self, Self::Error> {
        Ok(NewUser {
            username: value.username.try_into()?,
            email: value.email.try_into()?,
            password: value.password.try_into()?,
        })
    }
}

pub struct SignUpOutput {
    pub user: User,
}

impl From<User> for SignUpOutput {
    fn from(user: User) -> Self {
        Self { user }
    }
}
