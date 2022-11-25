use async_trait::async_trait;
use shaku::Interface;

use crate::domain::models::{password::Hashed, Id, NewUser, UpdatedUser, User, Username};

#[async_trait]
pub trait UsersRepository: Interface {
    async fn insert_user(&self, user: NewUser<Hashed>) -> Result<User, anyhow::Error>;

    async fn find_user_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error>;

    async fn find_user_by_username(
        &self,
        username: Username,
    ) -> Result<Option<User>, anyhow::Error>;

    async fn update_user(&self, updated_user: UpdatedUser) -> Result<User, anyhow::Error>;
}
