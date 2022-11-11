pub mod models;
pub mod repositories;
pub mod services;

use std::str::FromStr;

use shaku::{Component, Interface};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::MySqlPool;

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
                .expect("DATABASE_URL must be set.")
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

pub trait Secret: Interface {
    fn get_secret(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = Secret)]
pub struct JwtSecret {
    value: String,
}

impl Secret for JwtSecret {
    fn get_secret(&self) -> String {
        std::env::var("SECRET_KEY").expect("SECRET_KEY must be set.")
    }
}
