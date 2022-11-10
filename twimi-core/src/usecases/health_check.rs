use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::domain::repositories::health_check::HealthCheckRepository;

#[derive(thiserror::Error, Debug)]
pub enum HealthCheckUseCaseError {
    #[error("Failed to acquire a connection from the pool")]
    DatabaseError(#[from] sqlx::Error),
}

#[async_trait]
pub trait HealthCheck: Interface {
    async fn health_check(&self) -> Result<(), HealthCheckUseCaseError>;
}

#[derive(Component)]
#[shaku(interface = HealthCheck)]
pub struct HealthCheckUseCase {
    #[shaku(inject)]
    repository: Arc<dyn HealthCheckRepository>,
}

#[async_trait]
impl HealthCheck for HealthCheckUseCase {
    async fn health_check(&self) -> Result<(), HealthCheckUseCaseError> {
        self.repository.health_check().await?;
        Ok(())
    }
}
