use actix_web::{HttpResponse, ResponseError};

use super::Inject;
use twimi_core::usecases::health_check::{HealthCheck, HealthCheckUseCaseError};

pub async fn health_check(
    usecase: Inject<dyn HealthCheck>,
) -> Result<HttpResponse, HealthCheckError> {
    Ok(usecase
        .health_check()
        .await
        .map(|_| HttpResponse::NoContent())?
        .finish())
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct HealthCheckError(#[from] HealthCheckUseCaseError);

impl ResponseError for HealthCheckError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.0 {
            HealthCheckUseCaseError::DatabaseError(_) => {
                actix_web::http::StatusCode::SERVICE_UNAVAILABLE
            }
        }
    }
}
