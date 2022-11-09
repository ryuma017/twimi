use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::repositories::Database;

#[derive(thiserror::Error, Debug)]
pub enum HealthCheckError {
    #[error("Failed to acquire a connection from the pool")]
    DatabaseError(#[from] sqlx::Error),
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
        sqlx::query("SELECT 1")
            .fetch_one(self.database.pool())
            .await?;
        Ok(())
    }
}
