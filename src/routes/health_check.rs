use actix_web::{web::Data, HttpResponse};
use sqlx::{mysql::MySqlRow, MySqlPool};

pub async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

pub async fn health_check_db(pool: Data<MySqlPool>) -> HttpResponse {
    select_one(&pool)
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

async fn select_one(pool: &MySqlPool) -> Result<MySqlRow, sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await
}
