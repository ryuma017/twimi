use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::startup::Database;

pub enum HealthCheckError {
    DatabaseError,
    UnexpectedError,
}

#[async_trait]
pub trait HealthCheck: Interface {
    async fn health_check(&self) -> Result<(), HealthCheckError>;
}

#[derive(Component)]
#[shaku(interface = HealthCheck)]
pub struct HealthCheckUseCase {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl HealthCheck for HealthCheckUseCase {
    async fn health_check(&self) -> Result<(), HealthCheckError> {
        let result = sqlx::query("SELECT 1")
            .fetch_one(self.database.pool())
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(HealthCheckError::DatabaseError),
        }
    }
}
