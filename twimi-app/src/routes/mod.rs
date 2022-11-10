mod health_check;
mod signup;

pub use health_check::health_check;
pub use signup::signup;

pub type Inject<T> = shaku_actix::Inject<twimi_core::AppModule, T>;
