pub mod infrastructure;
pub mod routes;
pub mod startup;

use shaku::module;

use infrastructure::{
    repositories::{health_check::HealthCheckRepositoryImpl, users::UsersRepositoryImpl},
    MySqlDatabase,
};
use twimi_core::usecases::{health_check::HealthCheckUseCase, signup::SignUpUseCase};

module! {
    pub AppModule {
        components = [MySqlDatabase, HealthCheckUseCase, SignUpUseCase, HealthCheckRepositoryImpl, UsersRepositoryImpl],
        providers = []
    }
}
