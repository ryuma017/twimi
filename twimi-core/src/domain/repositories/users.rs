use async_trait::async_trait;
use shaku::Interface;

use crate::domain::models::user::{NewUser, User};

#[async_trait]
pub trait UsersRepository: Interface {
    async fn insert_user(&self, user: NewUser) -> Result<User, anyhow::Error>;
}
