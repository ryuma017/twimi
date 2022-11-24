use actix_web::{http::StatusCode, web::ReqData, HttpResponse, ResponseError};

use crate::server::models::UserId;

pub async fn update_authenticated_user(
    user_id: ReqData<UserId>,
) -> Result<HttpResponse, UpdateAuthnUserError> {
    let user_id: String = user_id.into_inner().into();
    Ok(HttpResponse::Ok().body(format!("update the authenticated user: {user_id}")))
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct UpdateAuthnUserError(#[from] anyhow::Error);

impl ResponseError for UpdateAuthnUserError {
    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN // FIXME
    }
}
