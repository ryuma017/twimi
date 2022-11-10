use actix_web::{http::StatusCode, web::Json, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::Inject;
use crate::usecases::signup::{SignUp, SignUpError, SignUpInput, SignUpOutput};

pub async fn signup(
    usecase: Inject<dyn SignUp>,
    json: Json<SignUpRequestJson>,
) -> Result<HttpResponse, SignUpError> {
    usecase
        .signup(json.into_inner().into())
        .await
        .map(|user| HttpResponse::Ok().json(SignUpResponseJson::from(&user)))
    // .map(|user| HttpResponse::Ok().json(user.as_ref()))
}

impl ResponseError for SignUpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) | Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SignUpRequestJson {
    username: String,
    email: String,
    password: String,
}

impl From<SignUpRequestJson> for SignUpInput {
    fn from(payload: SignUpRequestJson) -> Self {
        Self {
            username: payload.username,
            email: payload.email,
            password: payload.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SignUpResponseJson<'a> {
    id: u64,
    username: &'a str,
    email: &'a str,
    created_at: &'a OffsetDateTime,
    updated_at: &'a OffsetDateTime,
}

impl<'a> From<&'a SignUpOutput> for SignUpResponseJson<'a> {
    fn from(value: &'a SignUpOutput) -> Self {
        Self {
            id: value.user.id(),
            username: value.user.username.as_ref(),
            email: value.user.email.as_ref(),
            created_at: &value.user.created_at,
            updated_at: &value.user.updated_at,
        }
    }
}

// impl<'a> AsRef<SignUpResponseJson<'a>> for SignUpOutput {
//     fn as_ref(&self) -> &SignUpResponseJson<'a> {
//         unsafe { &*(self as *const Self as *const SignUpResponseJson) }
//     }
// }
