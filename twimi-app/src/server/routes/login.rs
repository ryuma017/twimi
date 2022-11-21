use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use twimi_core::usecases::login::{Login, LoginInput, LoginOutput, LoginUseCaseError};

use super::Inject;
use crate::server::models::User;

pub async fn login(
    usecase: Inject<dyn Login>,
    json: Json<LoginRequest>,
) -> Result<HttpResponse, LoginError> {
    Ok(usecase
        .login(json.into_inner().into())
        .await
        .map(|v| HttpResponse::Ok().json(LoginResponse::from(&v)))?)
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct LoginError(#[from] LoginUseCaseError);

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            LoginUseCaseError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginUseCaseError::ValidationError(_) => StatusCode::BAD_REQUEST,
            LoginUseCaseError::VerificationError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl From<LoginRequest> for LoginInput {
    fn from(value: LoginRequest) -> Self {
        Self {
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse<'a> {
    user: User<'a>,
    access_token: &'a str,
}

impl<'a> From<&'a LoginOutput> for LoginResponse<'a> {
    fn from(output: &'a LoginOutput) -> Self {
        Self {
            user: User::from(&output.user),
            access_token: &output.access_token,
        }
    }
}
