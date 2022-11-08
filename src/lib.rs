pub mod domain;
pub mod routes;
pub mod startup;
pub mod usecases;

use shaku::module;
use startup::MySqlDatabase;
use usecases::{health_check::HealthCheckUseCase, signup::SignUpUseCase};

module! {
    pub AppModule {
        components = [MySqlDatabase, HealthCheckUseCase, SignUpUseCase],
        providers = []
    }
}
