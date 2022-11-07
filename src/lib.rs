pub mod routes;
pub mod startup;
pub mod usecases;

use crate::usecases::health_check::HealthCheckUseCase;
use shaku::module;
use startup::MySqlDatabase;

module! {
    pub AppModule {
        components = [HealthCheckUseCase, MySqlDatabase],
        providers = []
    }
}
