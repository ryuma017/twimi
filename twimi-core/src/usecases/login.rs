use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{user::User, ValidationError},
    repositories::users::UsersRepository,
    services::{InvalidPassword, JwtEncoder, PasswordVerifier},
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
    #[shaku(inject)]
    password_service: Arc<dyn PasswordVerifier>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JwtEncoder>,
}

#[async_trait]
impl Login for LoginUseCase {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, LoginUseCaseError> {
        let user = self
            .repository
            .find_user_by_username(input.username.try_into()?)
            .await?
            .context("User not Found.")?;
        self.password_service
            .verify_password_hash(input.password.as_str(), user.password_hash.as_str())?;

        let access_token = self.jwt_service.encode(&(&user).into())?;

        Ok((user, access_token).into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoginUseCaseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] ValidationError),
    #[error(transparent)]
    InvalidPassword(#[from] InvalidPassword),
}

pub struct LoginInput {
    pub username: String,
    pub password: String,
}

pub struct LoginOutput {
    pub user: User,
    pub access_token: String,
}

impl From<(User, String)> for LoginOutput {
    fn from(value: (User, String)) -> Self {
        Self {
            user: value.0,
            access_token: value.1,
        }
    }
}
