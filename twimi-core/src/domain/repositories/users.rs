use async_trait::async_trait;
use shaku::Interface;

use crate::domain::models::user::{NewUser, User, Username};

#[derive(Debug, thiserror::Error)]
#[error("Failed to insert a new user: {0}")]
pub struct InsertionError(#[from] anyhow::Error);

#[async_trait]
pub trait UsersRepository: Interface {
    async fn insert_user(&self, user: NewUser) -> Result<User, InsertionError>;
    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error>;
}
