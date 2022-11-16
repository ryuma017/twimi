use anyhow::Context as _;
use jsonwebtoken::EncodingKey;
use shaku::Component;

use twimi_core::domain::services::{Claims, JwtEncoder};

#[derive(Component)]
#[shaku(interface = JwtEncoder)]
pub struct JwtEncoderImpl {
    secret: Vec<u8>,
}

impl JwtEncoder for JwtEncoderImpl {
    fn encode(&self, claims: &Claims) -> Result<String, anyhow::Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_slice()),
        )
        .context("Failed to encode JWT.")
    }
}
