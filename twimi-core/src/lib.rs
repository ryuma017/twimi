pub mod domain;
pub mod infrastructure;
pub mod usecases;

use shaku::module;

pub use infrastructure::{
    repositories::{health_check::HealthCheckRepositoryImpl, users::UsersRepositoryImpl},
    MySqlDatabase,
};
pub use usecases::{health_check::HealthCheckUseCase, signup::SignUpUseCase};

module! {
    pub AppModule {
        components = [MySqlDatabase, HealthCheckUseCase, SignUpUseCase, HealthCheckRepositoryImpl, UsersRepositoryImpl],
        providers = []
    }
}
