use std::sync::Arc;

use actix_settings::{ApplySettings as _, Settings};
use actix_web::dev::Server;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};

use crate::repositories::Database;
use crate::routes::{health_check, signup};
use crate::{AppModule, MySqlDatabase};

pub struct ApiServer {
    port: u16,
    server: Server,
}

impl ApiServer {
    pub fn build() -> Result<Self, std::io::Error> {
        let settings = Settings::parse_toml("config/config.toml")
            .expect("Failed to parse settings from toml file.");
        let port = settings.actix.hosts[0].port;
        let module = Arc::new(
            AppModule::builder()
                .with_component_override::<dyn Database>(Box::new(MySqlDatabase::new()))
                .build(),
        );

        let server = HttpServer::new(move || {
            App::new()
                .wrap(NormalizePath::default())
                .route("/health_check", web::get().to(health_check))
                .route("/signup", web::post().to(signup))
                .app_data(Arc::clone(&module))
        })
        .apply_settings(&settings)
        .run();

        Ok(Self { port, server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
