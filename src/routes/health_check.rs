use actix_web::HttpResponse;

use super::Inject;
use crate::usecases::health_check::HealthCheck;

pub async fn health_check(usecase: Inject<dyn HealthCheck>) -> HttpResponse {
    usecase
        .health_check()
        .await
        .map(|_| HttpResponse::NoContent())
        .unwrap_or_else(|_| HttpResponse::ServiceUnavailable())
        .finish()
}
