use shaku::Interface;

#[derive(Debug, thiserror::Error)]
#[error("Failed compute password hash")]
pub struct ComputeHashError(#[from] anyhow::Error);

#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    #[error("Invalid password.")]
    InvalidPassword,
    #[error("{0}")]
    InternalError(#[from] anyhow::Error),
}

pub trait PasswordHasher: Interface {
    fn compute_password_hash(&self, password: &str) -> Result<String, ComputeHashError>;
}

pub trait PasswordVerifier: Interface {
    fn verify_password(
        &self,
        password_candidate: &str,
        expected_password_hash: &str,
    ) -> Result<(), VerificationError>;
}

pub trait PasswordService: PasswordHasher + PasswordVerifier {}

impl<T: PasswordHasher + PasswordVerifier> PasswordService for T {}
