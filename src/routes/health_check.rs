use actix_web::{HttpResponse, ResponseError};

use super::Inject;
use crate::usecases::health_check::{HealthCheck, HealthCheckError};

pub async fn health_check(
    usecase: Inject<dyn HealthCheck>,
) -> Result<HttpResponse, HealthCheckError> {
    Ok(usecase
        .health_check()
        .await
        .map(|_| HttpResponse::NoContent())?
        .finish())
}

impl ResponseError for HealthCheckError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DatabaseError(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}
