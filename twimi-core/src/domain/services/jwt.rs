use serde::{Deserialize, Serialize};
use shaku::Interface;

use crate::domain::models::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    name: String,
}

impl From<&User> for Claims {
    fn from(user: &User) -> Self {
        Self {
            name: user.username.as_ref().to_owned(),
        }
    }
}

pub trait JwtEncoder: Interface {
    fn encode(&self, claims: &Claims) -> Result<String, anyhow::Error>;
}
