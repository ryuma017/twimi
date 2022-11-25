use anyhow::Context as _;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use shaku::Component;

use twimi_core::domain::services::jwt::{Claims, JwtDecoder, JwtEncoder, JwtService};

#[derive(Component)]
#[shaku(interface = JwtService)]
pub struct JwtServiceImpl {
    secret: Vec<u8>,
}

impl JwtEncoder for JwtServiceImpl {
    fn encode(&self, claims: &Claims) -> Result<String, anyhow::Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_slice()),
        )
        .context("Failed to encode JWT.")
    }
}

impl JwtDecoder for JwtServiceImpl {
    fn decode(&self, token: &str) -> Result<Claims, anyhow::Error> {
        jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_slice()),
            &Validation::default(),
        )
        .context("Failed to decode JWT.")
        .map(|data| data.claims)
    }
}
