pub mod infrastructure;
pub mod server;

use shaku::module;

use infrastructure::repositories::{
    health_check::HealthCheckRepositoryImpl, users::UsersRepositoryImpl,
};
use infrastructure::services::{Argon2PasswordHash, JwtServiceImpl, MySqlDatabase};
use twimi_core::usecases::{
    health_check::HealthCheckUseCase, login::LoginUseCase, signup::SignUpUseCase,
};

module! {
    pub AppModule {
        components = [
            MySqlDatabase,
            HealthCheckRepositoryImpl,
            UsersRepositoryImpl,
            HealthCheckUseCase,
            SignUpUseCase,
            LoginUseCase,
            Argon2PasswordHash,
            JwtServiceImpl,
        ],
        providers = []
    }
}
