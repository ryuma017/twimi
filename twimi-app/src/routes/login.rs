use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use twimi_core::usecases::login::{Login, LoginInput, LoginOutput, LoginUseCaseError};

use super::Inject;

pub async fn login(
    usecase: Inject<dyn Login>,
    json: Json<LoginRequestJson>,
) -> Result<HttpResponse, LoginError> {
    Ok(usecase
        .login(json.into_inner().into())
        .await
        .map(|v| HttpResponse::Ok().json(LoginResponseJson::from(&v)))?)
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct LoginError(#[from] LoginUseCaseError);

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            LoginUseCaseError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginUseCaseError::ValidationError(_) => StatusCode::BAD_REQUEST,
            LoginUseCaseError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequestJson {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponseJson<'a> {
    user: Inner<'a>,
    access_token: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Inner<'a> {
    id: u64,
    username: &'a str,
    email: &'a str,
    created_at: &'a OffsetDateTime,
    updated_at: &'a OffsetDateTime,
}

impl<'a> From<&'a LoginOutput> for LoginResponseJson<'a> {
    fn from(output: &'a LoginOutput) -> Self {
        Self {
            user: Inner::from(&output.user),
            access_token: &output.access_token,
        }
    }
}

impl<'a> From<&'a LoginOutput> for Inner<'a> {
    fn from(user: &'a LoginOutput) -> Self {
        Self {
            id: user.user.id(),
            username: user.user.username.as_ref(),
            email: user.user.email.as_ref(),
            created_at: &user.user.created_at,
            updated_at: &user.user.updated_at,
        }
    }
}
