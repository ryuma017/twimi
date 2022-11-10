use time::OffsetDateTime;

use super::{Email, Id, Password, Username};

pub struct NewUser {
    pub username: Username,
    pub email: Email,
    pub password: Password,
}

pub struct User {
    pub id: Id<Self>,
    pub username: Username,
    pub email: Email,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id.value
    }
    pub fn set_id(mut self, id: u64) -> Self {
        self.id = id.into();
        self
    }
}
