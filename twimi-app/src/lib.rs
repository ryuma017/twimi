pub mod infrastructure;
pub mod routes;
pub mod startup;

use shaku::module;

use infrastructure::{
    repositories::{health_check::HealthCheckRepositoryImpl, users::UsersRepositoryImpl},
    MySqlDatabase,
};
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
        ],
        providers = []
    }
}
