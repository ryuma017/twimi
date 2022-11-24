use actix_web::{
    http::StatusCode,
    web::{Json, ReqData},
    HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};

use twimi_core::usecases::user::patch::{
    UpdateAuthnUser, UpdateAuthnUserInput, UpdateAuthnUserOutput, UpdateAuthnUserUseCaseError,
};

use super::super::Inject;
use crate::server::models::{User, UserId};

pub async fn update_authenticated_user(
    usecase: Inject<dyn UpdateAuthnUser>,
    user_id: ReqData<UserId>,
    Json(body): Json<UpdateAuthnUserRequest>,
) -> Result<HttpResponse, UpdateAuthnUserError> {
    Ok(usecase
        .update_authenticated_user(UpdateAuthnUserInput::new(
            user_id.into_inner().into(),
            body.username,
            body.email,
        ))
        .await
        .map(|output| HttpResponse::Ok().json(UpdateAuthnUserResponse::from(&output)))?)
}

#[derive(Debug, Deserialize)]
pub struct UpdateAuthnUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateAuthnUserResponse<'a> {
    user: User<'a>,
}

impl<'a> From<&'a UpdateAuthnUserOutput> for UpdateAuthnUserResponse<'a> {
    fn from(value: &'a UpdateAuthnUserOutput) -> Self {
        Self {
            user: (&value.user).into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct UpdateAuthnUserError(#[from] UpdateAuthnUserUseCaseError);

impl ResponseError for UpdateAuthnUserError {
    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN // FIXME
    }
}
