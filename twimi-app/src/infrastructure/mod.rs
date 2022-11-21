pub mod models;
pub mod repositories;
pub mod services;

use std::{str::FromStr, time::Duration};

use shaku::{Component, Interface};
use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions};

pub trait Database: Interface {
    fn pool(&self) -> &MySqlPool;
}

#[derive(Component)]
#[shaku(interface = Database)]
pub struct MySqlDatabase {
    pool: MySqlPool,
}

impl MySqlDatabase {
    pub fn new(url: &str) -> Self {
        let options = MySqlConnectOptions::from_str(url).unwrap();

        let pool = MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_lazy_with(options);

        Self { pool }
    }
}

impl Database for MySqlDatabase {
    fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}
