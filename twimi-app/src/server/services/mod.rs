use anyhow::Context as _;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher as _,
    PasswordVerifier as _, Version,
};
use shaku::Component;
use twimi_core::domain::services::{InvalidCredentials, PasswordHasher, PasswordVerifier};

#[derive(Component, Clone, Copy)]
#[shaku(interface = PasswordHasher)]
pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn compute_password_hash(&self, password: String) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hashed = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
        Ok(hashed)
    }
}

#[derive(Component, Clone, Copy)]
#[shaku(interface = PasswordVerifier)]
pub struct Argon2PasswordVerifier;

impl PasswordVerifier for Argon2PasswordVerifier {
    fn verify_password_hash(
        &self,
        password_candidate: String,
        expected_password_hash: String,
    ) -> Result<(), InvalidCredentials> {
        let expected = PasswordHash::new(&expected_password_hash)
            .context("Failed to parse hash in PHC string format.")?;
        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected)
            .context("Invalid password.")?;
        Ok(())
    }
}
