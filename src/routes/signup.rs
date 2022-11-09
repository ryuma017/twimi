use actix_web::{web::Json, HttpResponse, ResponseError};
use serde::Deserialize;

use super::Inject;
use crate::usecases::signup::{SignUp, SignUpError, SignUpInput};

pub async fn signup(
    usecase: Inject<dyn SignUp>,
    json: Json<SignUpPayload>,
) -> Result<HttpResponse, SignUpError> {
    usecase
        .signup(json.into_inner().into())
        .await
        .map(|_| HttpResponse::Ok().body("OK!!!!"))
}

#[derive(Debug, Deserialize)]
pub struct SignUpPayload {
    username: String,
    email: String,
    password: String,
}

impl ResponseError for SignUpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DatabaseError(_) | Self::UnexpectedError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}

impl From<SignUpPayload> for SignUpInput {
    fn from(payload: SignUpPayload) -> Self {
        Self {
            username: payload.username,
            email: payload.email,
            password: payload.password,
        }
    }
}
