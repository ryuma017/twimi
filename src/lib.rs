pub mod app;
pub mod domain;
pub mod repositories;
pub mod usecases;

use shaku::module;

use repositories::{HealthCheckRepositoryImpl, MySqlDatabase, UsersRepositoryImpl};
use usecases::{HealthCheckUseCase, SignUpUseCase};

module! {
    pub AppModule {
        components = [MySqlDatabase, HealthCheckUseCase, SignUpUseCase, HealthCheckRepositoryImpl, UsersRepositoryImpl],
        providers = []
    }
}
