pub mod username;

use serde::Serialize;
use time::OffsetDateTime;

use twimi_core::domain;

#[derive(Debug, Serialize)]
pub struct User<'a> {
    id: i64,
    username: &'a str,
    email: &'a str,
    created_at: &'a OffsetDateTime,
    updated_at: &'a OffsetDateTime,
}

impl<'a> From<&'a domain::models::User> for User<'a> {
    fn from(value: &'a domain::models::User) -> Self {
        Self {
            id: value.id.value(),
            username: value.username.as_ref(),
            email: value.email.as_ref(),
            created_at: &value.created_at,
            updated_at: &value.updated_at,
        }
    }
}
