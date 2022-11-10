use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{
        user::{NewUser, User, UserCredentials},
        InvalidCredentials, ValidationError,
    },
    repositories::users::UsersRepository,
};

#[async_trait]
pub trait Login: Interface {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, LoginUseCaseError>;
}

#[derive(Component)]
#[shaku(interface = Login)]
pub struct LoginUseCase {
    #[shaku(inject)]
    repository: Arc<dyn UsersRepository>,
}

#[async_trait]
impl Login for LoginUseCase {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, LoginUseCaseError> {
        let user = self
            .repository
            .find_user_by_username(input.username.try_into()?)
            .await? // Result<User, InvalidCredential("Invalid username")>
            .verify_password(input.password.try_into()?)?; // Result<User, InvalidCredential("Invalid password")>

        let access_token = todo!("Generate access token");

        Ok(LoginOutput { user, access_token })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginUseCaseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    InvalidCredentials(#[from] InvalidCredentials),
}

pub struct LoginInput {
    pub username: String,
    pub password: String,
}

impl TryFrom<LoginInput> for UserCredentials {
    type Error = ValidationError;

    fn try_from(value: LoginInput) -> Result<Self, Self::Error> {
        Ok(UserCredentials {
            username: value.username.try_into()?,
            password: value.password.try_into()?,
        })
    }
}

pub struct LoginOutput {
    pub user: User,
    pub access_token: String,
}
