use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorUnauthorized, InternalError},
    HttpMessage, HttpResponse,
};
use actix_web_lab::middleware::Next;
use shaku::HasComponent;

use twimi_core::domain::{models::user::Username, services::JwtService};

use crate::AppModule;

pub async fn reject_unauthenticated_user<B: MessageBody>(
    request: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, actix_web::Error> {
    let Some(token) = request.parts().0.headers().get("Authorization") else {
        return Err(ErrorUnauthorized("No token provided."))
    };
    let jwt_service: &dyn JwtService = request.app_data::<AppModule>().unwrap().resolve_ref();
    let username: Username = jwt_service
        .decode(token.to_str().unwrap())
        .map_err(|e| {
            InternalError::from_response(
                e,
                HttpResponse::Unauthorized()
                    .insert_header(("WWW-Authenticate", r#"Bearer realm="jwtssoprotectedrealm""#))
                    .finish(),
            )
        })?
        .try_into()
        .unwrap();
    request.extensions_mut().insert(username);
    next.call(request).await
}
