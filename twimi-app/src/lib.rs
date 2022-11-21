pub mod infrastructure;
pub mod server;

use shaku::module;

use infrastructure::{
    repositories::{health_check::HealthCheckRepositoryImpl, users::UsersRepositoryImpl},
    services::{jwt::JwtServiceImpl, password::Argon2PasswordHash},
    MySqlDatabase,
};

use twimi_core::usecases::{
    health_check::HealthCheckUseCase, login::LoginUseCase, signup::SignUpUseCase,
    user::get::GetAuthnUserUseCase,
};

module! {
    pub AppModule {
        components = [
            MySqlDatabase,
            HealthCheckRepositoryImpl,
            UsersRepositoryImpl,
            Argon2PasswordHash,
            JwtServiceImpl,
            HealthCheckUseCase,
            SignUpUseCase,
            LoginUseCase,
            GetAuthnUserUseCase,
        ],
        providers = []
    }
}
