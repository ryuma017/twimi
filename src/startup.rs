use std::str::FromStr;
use std::sync::Arc;

use actix_settings::{ApplySettings as _, Settings};
use actix_web::dev::Server;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};
use shaku::{Component, Interface};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::MySqlPool;

use crate::routes::health_check;
use crate::AppModule;

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

pub trait Database: Interface {
    fn pool(&self) -> &MySqlPool;
}

#[derive(Component)]
#[shaku(interface = Database)]
pub struct MySqlDatabase {
    pool: MySqlPool,
}

impl MySqlDatabase {
    pub fn new() -> Self {
        let options = MySqlConnectOptions::from_str(
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .unwrap();

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect_lazy_with(options);

        Self { pool }
    }
}

impl Default for MySqlDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl Database for MySqlDatabase {
    fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}
