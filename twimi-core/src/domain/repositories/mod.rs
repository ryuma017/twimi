pub mod health_check;
pub mod users;

use shaku::Interface;
use sqlx::MySqlPool;

pub trait Database: Interface {
    fn pool(&self) -> &MySqlPool;
}
