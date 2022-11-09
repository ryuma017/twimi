use actix_web::{web::Json, HttpResponse, ResponseError};
use serde::Deserialize;

use super::Inject;
use crate::usecases::signup::{SignUp, SignUpError};

pub async fn signup(
    usecase: Inject<dyn SignUp>,
    json: Json<SignUpPayload>,
) -> Result<HttpResponse, SignUpError> {
    let SignUpPayload {
        username,
        email,
        password,
    } = json.into_inner();
    usecase
        .signup(username, email, password)
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
