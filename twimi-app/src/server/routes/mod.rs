mod health_check;
mod login;
mod signup;

pub use health_check::health_check;
pub use login::login;
pub use signup::signup;

pub type Inject<T> = shaku_actix::Inject<crate::AppModule, T>;
