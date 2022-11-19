use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::InternalError,
    HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;
use shaku::HasComponent;

use twimi_core::domain::services::JwtService;

use crate::{server::models::Username, AppModule};

pub async fn reject_unauthenticated_user<B: MessageBody>(
    request: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let jwt = request
        .parts()
        .0 // &HttpRequest
        .headers()
        .get("Authorization")
        .ok_or_else(|| Unauthorized("The 'Authorization' header was missing."))?
        .to_str()
        .map_err(|_| Unauthorized("The 'Authorization' header was not a valid UTF8 string."))?
        .strip_prefix("Bearer ")
        .ok_or_else(|| Unauthorized("The authorization scheme was not `Bearer`."))?;
    let jwt_service: &dyn JwtService = request.app_data::<Arc<AppModule>>().unwrap().resolve_ref();
    let username: Username = jwt_service
        .decode(jwt)
        .map(|username| username.into())
        .map_err(Unauthorized)?;
    request.extensions_mut().insert(username);
    next.call(request).await
}

#[allow(non_snake_case)]
fn Unauthorized<T>(cause: T) -> actix_web::Error
where
    T: Display + Debug + 'static,
{
    InternalError::from_response(
        cause,
        HttpResponse::Unauthorized()
            .insert_header(("WWW-Authenticate", r#"Bearer realm="jwtssoprotectedrealm""#))
            .finish(),
    )
    .into()
}
