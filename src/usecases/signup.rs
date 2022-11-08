use std::sync::Arc;

use async_trait::async_trait;
use shaku::{Component, Interface};

use crate::startup::Database;

#[derive(thiserror::Error, Debug)]
pub enum SignUpError {
    #[error("Failed to acquire a connection from the pool")]
    DatabaseError(#[from] sqlx::Error),
}

#[async_trait]
pub trait SignUp: Interface {
    async fn signup(&self) -> Result<(), SignUpError>;
}

#[derive(Component)]
#[shaku(interface = SignUp)]
pub struct SignUpUseCase {
    #[shaku(inject)]
    database: Arc<dyn Database>,
}

#[async_trait]
impl SignUp for SignUpUseCase {
    async fn signup(&self) -> Result<(), SignUpError> {
        // usecase logic
        Ok(())
    }
}
