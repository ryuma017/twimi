mod health_check;
mod login;
mod signup;
mod user;

pub use health_check::health_check;
pub use login::login;
pub use signup::signup;
pub use user::{get_authenticated_user, update_authenticated_user};

pub(self) type Inject<T> = shaku_actix::Inject<crate::AppModule, T>;
