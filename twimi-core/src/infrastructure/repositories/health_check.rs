use std::sync::Arc;

use async_trait::async_trait;
use shaku::Component;

use crate::{domain::repositories::health_check::HealthCheckRepository, infrastructure::Database};

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
