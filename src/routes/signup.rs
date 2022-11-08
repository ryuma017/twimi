use actix_web::HttpResponse;
use shaku_actix::Inject;

use crate::{usecases::signup::SignUp, AppModule};

pub async fn signup(usecase: Inject<AppModule, dyn SignUp>) -> HttpResponse {
    usecase
        .signup()
        .await
        .map(|_| HttpResponse::NoContent())
        .unwrap_or_else(|_| HttpResponse::ServiceUnavailable())
        .finish()
}
