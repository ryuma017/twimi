use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{
        user::{NewUser, Plain, User},
        ValidationError,
    },
    repositories::users::{InsertionError, UsersRepository},
    services::{ComputeHashError, PasswordService},
};

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpUseCaseError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    repository: Arc<dyn UsersRepository>,
    #[shaku(inject)]
    password_hasher: Arc<dyn PasswordService>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(&self, input: SignUpInput) -> Result<SignUpOutput, SignUpUseCaseError> {
        Ok(self
            .repository
            .insert_user(
                NewUser::try_from(input.clone())?.with_password_hash(
                    self.password_hasher
                        .compute_password_hash(&input.password)?,
                ),
            )
            .await?
            .into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SignUpUseCaseError {
    #[error(transparent)]
    DatabaseError(#[from] InsertionError),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    UnexpectedError(#[from] ComputeHashError),
}

#[derive(Clone)]
pub struct SignUpInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<SignUpInput> for NewUser<Plain> {
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
