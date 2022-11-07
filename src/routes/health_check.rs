use actix_web::HttpResponse;
use shaku_actix::Inject;

use crate::{usecases::health_check::HealthCheck, AppModule};

pub async fn health_check(usecase: Inject<AppModule, dyn HealthCheck>) -> HttpResponse {
    usecase
        .health_check()
        .await
        .map(|_| HttpResponse::NoContent())
        .unwrap_or_else(|_| HttpResponse::ServiceUnavailable())
        .finish()
}
