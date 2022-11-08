use actix_web::HttpResponse;

use super::Inject;
use crate::usecases::signup::SignUp;

pub async fn signup(usecase: Inject<dyn SignUp>) -> HttpResponse {
    usecase
        .signup()
        .await
        .map(|_| HttpResponse::NoContent())
        .unwrap_or_else(|_| HttpResponse::ServiceUnavailable())
        .finish()
}
