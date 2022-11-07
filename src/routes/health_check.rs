use actix_web::{web::Data, HttpResponse};
use shaku_actix::Inject;
use sqlx::{mysql::MySqlRow, MySqlPool};

use crate::{AppModule, usecases::health_check::HealthCheck};

pub async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

pub async fn health_check_db(usecase: Inject<AppModule, dyn HealthCheck>) -> HttpResponse {
    usecase.health_check().await.map(|_| HttpResponse::NoContent()).unwrap_or_else(|_| HttpResponse::ServiceUnavailable()).finish()
}
