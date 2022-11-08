pub mod routes;
pub mod startup;
pub mod usecases;

use shaku::module;
use startup::MySqlDatabase;
use usecases::{signup::SignUpUseCase, health_check::HealthCheckUseCase};

module! {
    pub AppModule {
        components = [MySqlDatabase, HealthCheckUseCase, SignUpUseCase],
        providers = []
    }
}
