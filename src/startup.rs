use std::net::{TcpListener, ToSocketAddrs};
use std::str::FromStr;

use actix_web::dev::Server;
use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::MySqlPool;

use crate::routes::{health_check, health_check_db};

pub struct ApiServer {
    port: u16,
    server: Server,
}

impl ApiServer {
    pub fn build<A>(address: A) -> Result<Self, std::io::Error>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let connection_pool = {
            let db = Database::new();
            web::Data::new(db.pool())
        };

        let server = HttpServer::new(move || {
            App::new()
                .wrap(NormalizePath::default())
                .route("/health_check", web::get().to(health_check))
                .route("/health_check/database", web::get().to(health_check_db))
                .app_data(web::Data::clone(&connection_pool))
        })
        .listen(listener)?
        .run();

        Ok(Self {
            port,
            server,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub struct Database {
    pub options: MySqlConnectOptions,
}

impl Database {
    pub fn new() -> Self {
        let options = MySqlConnectOptions::from_str(
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set")
                .as_str(),
        )
        .unwrap();

        Self { options }
    }

    pub fn pool(self) -> MySqlPool {
        MySqlPoolOptions::new()
            .max_connections(5)
            .connect_lazy_with(self.options)
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
