pub mod models;
pub mod repositories;

use std::str::FromStr;

use shaku::Component;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::MySqlPool;

use crate::domain::repositories::Database;

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
