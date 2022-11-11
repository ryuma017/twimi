use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use twimi_core::usecases::login::{Login, LoginInput, LoginOutput, LoginUseCaseError};

use super::Inject;
use crate::server::models;

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

impl From<LoginRequestJson> for LoginInput {
    fn from(value: LoginRequestJson) -> Self {
        Self {
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponseJson<'a> {
    user: models::User<'a>,
    access_token: &'a str,
}

impl<'a> From<&'a LoginOutput> for LoginResponseJson<'a> {
    fn from(output: &'a LoginOutput) -> Self {
        Self {
            user: models::User::from(&output.user),
            access_token: &output.access_token,
        }
    }
}
