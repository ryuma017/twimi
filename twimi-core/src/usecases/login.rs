use std::sync::Arc;

use anyhow::Context as _;
use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::{
    models::{User, ValidationError},
    repositories::users::UsersRepository,
    services::{
        jwt::JwtService,
        password::{PasswordService, VerificationError},
    },
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
    password_verifier: Arc<dyn PasswordService>,
    #[shaku(inject)]
    jwt_encoder: Arc<dyn JwtService>,
}

#[async_trait]
impl Login for LoginUseCase {
    async fn login(&self, input: LoginInput) -> Result<LoginOutput, LoginUseCaseError> {
        let user = self
            .repository
            .find_user_by_username(input.username.try_into()?)
            .await?
            .context("User not Found.")?;
        self.password_verifier
            .verify_password(input.password.as_str(), user.password_hash.as_ref())?;

        let access_token = self.jwt_encoder.encode(&(&user).into())?;

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
    VerificationError(#[from] VerificationError),
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
    fn from((user, access_token): (User, String)) -> Self {
        Self { user, access_token }
    }
}
