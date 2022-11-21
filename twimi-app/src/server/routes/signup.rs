use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::Deserialize;

use twimi_core::usecases::signup::{SignUp, SignUpInput, SignUpOutput, SignUpUseCaseError};

use super::Inject;
use crate::server::models::User;

pub async fn signup(
    usecase: Inject<dyn SignUp>,
    json: Json<SignUpRequest>,
) -> Result<HttpResponse, SignUpError> {
    Ok(usecase
        .signup(json.into_inner().into())
        .await
        .map(|output| HttpResponse::Ok().json(SignUpResponse::from(&output)))?)
}

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    username: String,
    email: String,
    password: String,
}

impl From<SignUpRequest> for SignUpInput {
    fn from(payload: SignUpRequest) -> Self {
        Self {
            username: payload.username,
            email: payload.email,
            password: payload.password,
        }
    }
}

type SignUpResponse<'a> = User<'a>;

impl<'a> From<&'a SignUpOutput> for SignUpResponse<'a> {
    fn from(value: &'a SignUpOutput) -> Self {
        User::from(&value.user)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SignUpError(#[from] SignUpUseCaseError);

impl ResponseError for SignUpError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            SignUpUseCaseError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SignUpUseCaseError::DatabaseError(_) => StatusCode::CONFLICT,
            SignUpUseCaseError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
