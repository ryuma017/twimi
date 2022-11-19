use std::sync::Arc;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorUnauthorized, InternalError},
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
    let Some(token) = request
        .parts()
        .0 // &HttpRequest
        .headers()
        .get("Authorization")
        .map(|v| v.to_str().unwrap().strip_prefix("Bearer ").unwrap()) else {
            return Err(ErrorUnauthorized("No token provided."))
        };
    let jwt_service: &dyn JwtService = request.app_data::<Arc<AppModule>>().unwrap().resolve_ref();
    let username: Username = jwt_service
        .decode(token)
        .map(|username| username.into())
        .map_err(|e| {
            InternalError::from_response(
                e,
                HttpResponse::Unauthorized()
                    .insert_header(("WWW-Authenticate", r#"Bearer realm="jwtssoprotectedrealm""#))
                    .finish(),
            )
        })?;
    request.extensions_mut().insert(username);
    next.call(request).await
}
