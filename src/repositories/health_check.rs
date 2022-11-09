use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use super::Database;

#[async_trait]
pub trait HealthCheckRepository: Interface {
    async fn health_check(&self) -> Result<(), sqlx::Error>;
}

#[derive(Component)]
#[shaku(interface = HealthCheckRepository)]
pub struct HealthCheckRepositoryImpl {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(self.database.pool())
            .await?;
        Ok(())
    }
}
