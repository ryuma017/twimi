use actix_web::{web::Json, HttpResponse, ResponseError};

use super::Inject;
use crate::usecases::signup::{SignUp, SignUpError, SignUpPayload};

pub async fn signup(
    usecase: Inject<dyn SignUp>,
    json: Json<SignUpPayload>,
) -> Result<HttpResponse, SignUpError> {
    usecase
        .signup(json.into_inner())
        .await
        .map(|_| HttpResponse::Ok().body("OK!!!!"))
}

impl ResponseError for SignUpError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
