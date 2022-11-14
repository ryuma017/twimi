use shaku::Interface;

#[derive(Debug, thiserror::Error)]
#[error("Failed compute password hash")]
pub struct ComputeHashError(#[from] anyhow::Error);

pub trait PasswordHasher: Interface {
    fn compute_password_hash(&self, password: String) -> Result<String, anyhow::Error>;
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid credentials")]
pub struct InvalidPassword(#[from] anyhow::Error);

pub trait PasswordVerifier: Interface {
    fn verify_password_hash(
        &self,
        password_candidate: &str,
        expected_password_hash: &str,
    ) -> Result<(), InvalidPassword>;
}
