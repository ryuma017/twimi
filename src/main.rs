use std::{net::TcpListener, str::FromStr};

use actix_web::{
    middleware::NormalizePath,
    web::{Data, get, scope},
    App, HttpResponse, HttpServer,
};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    MySqlPool,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    let db_pool = Data::new(get_connection_pool());

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .service(
                scope("/health_check")
                    .route("", get().to(health_check))
                    .route("/database", get().to(health_check)),
            )
            .route("/health_check", get().to(health_check))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}

async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

async fn health_check_db(pool: Data<MySqlPool>) -> HttpResponse {
    sqlx::query("SELECT 1")
        .fetch_one(pool.as_ref())
        .await
        .map(|_| HttpResponse::NoContent().finish())
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

pub fn get_connection_pool() -> MySqlPool {
    let option = MySqlConnectOptions::from_str(
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set")
            .as_str(),
    )
    .unwrap();

    MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(option)
}
