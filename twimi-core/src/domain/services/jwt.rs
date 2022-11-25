use serde::{Deserialize, Serialize};
use shaku::Interface;
use time::OffsetDateTime;

use crate::domain::models::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub name: String,
}

impl From<&User> for Claims {
    fn from(user: &User) -> Self {
        Self {
            sub: user.id.value().to_string(),
            exp: (OffsetDateTime::now_utc() + time::Duration::hours(8)).unix_timestamp(),
            name: user.username.to_string(),
        }
    }
}

pub trait JwtEncoder: Interface {
    fn encode(&self, claims: &Claims) -> Result<String, anyhow::Error>;
}

pub trait JwtDecoder: Interface {
    fn decode(&self, token: &str) -> Result<Claims, anyhow::Error>;
}

pub trait JwtService: JwtEncoder + JwtDecoder {}

impl<T: JwtEncoder + JwtDecoder> JwtService for T {}
