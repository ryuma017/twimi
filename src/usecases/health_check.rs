use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::repositories::HealthCheckRepository;

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
    repository: Arc<dyn HealthCheckRepository>,
}

#[async_trait]
impl HealthCheck for HealthCheckUseCase {
    async fn health_check(&self) -> Result<(), HealthCheckError> {
        self.repository.health_check().await?;
        Ok(())
    }
}
