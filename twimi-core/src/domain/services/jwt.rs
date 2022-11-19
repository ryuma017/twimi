use serde::{Deserialize, Serialize};
use shaku::Interface;

use crate::domain::models::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub name: String,
}

impl From<&User> for Claims {
    fn from(user: &User) -> Self {
        Self {
            name: user.username.to_string(),
        }
    }
}

pub trait JwtEncoder: Interface {
    fn encode(&self, claims: &Claims) -> Result<String, anyhow::Error>;
}

pub trait JwtDecoder: Interface {
    fn decode(&self, token: &str) -> Result<String, anyhow::Error>;
}

pub trait JwtService: JwtEncoder + JwtDecoder {}

impl<T: JwtEncoder + JwtDecoder> JwtService for T {}
