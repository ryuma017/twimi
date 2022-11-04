use std::net::TcpListener;

use actix_web::{middleware::NormalizePath, web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::trim())
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run()
    .await?;

    Ok(())
}

async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
