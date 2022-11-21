use anyhow::Context as _;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher as _,
    PasswordVerifier as _, Version,
};
use shaku::Component;

use twimi_core::domain::services::password::{
    ComputeHashError, PasswordHasher, PasswordService, PasswordVerifier, VerificationError,
};

#[derive(Component, Clone, Copy)]
#[shaku(interface = PasswordService)]
pub struct Argon2PasswordHash;

impl PasswordHasher for Argon2PasswordHash {
    fn compute_password_hash(&self, password: &str) -> Result<String, ComputeHashError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hashed = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(password.as_bytes(), &salt)
        .context("Failed to compute password hash.")?
        .to_string();
        Ok(hashed)
    }
}

impl PasswordVerifier for Argon2PasswordHash {
    fn verify_password(
        &self,
        password_candidate: &str,
        expected_password_hash: &str,
    ) -> Result<(), VerificationError> {
        let expected = PasswordHash::new(expected_password_hash)
            .context("Failed to parse hash in PHC string format.")?;
        Argon2::default()
            .verify_password(password_candidate.as_bytes(), &expected)
            .map_err(|_| VerificationError::InvalidPassword)?;
        Ok(())
    }
}
