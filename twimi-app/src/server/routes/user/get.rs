use actix_web::{http::StatusCode, web::ReqData, HttpResponse, ResponseError};
use serde::Serialize;
use twimi_core::usecases::user::get::{
    GetAuthnUser, GetAuthnUserInput, GetAuthnUserOutput, GetAuthnUserUseCaseError,
};

use super::super::Inject;
use crate::server::models::{User, UserId};

pub async fn get_authenticated_user(
    usecase: Inject<dyn GetAuthnUser>,
    user_id: ReqData<UserId>,
) -> Result<HttpResponse, GetAuthnUserError> {
    Ok(usecase
        .get_authenticated_user(user_id.into_inner().into())
        .await
        .map(|output| HttpResponse::Ok().json(GetAuthnUserResponse::from(&output)))?)
}

impl From<UserId> for GetAuthnUserInput {
    fn from(value: UserId) -> Self {
        Self {
            user_id: value.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetAuthnUserResponse<'a> {
    user: User<'a>,
}

impl<'a> From<&'a GetAuthnUserOutput> for GetAuthnUserResponse<'a> {
    fn from(value: &'a GetAuthnUserOutput) -> Self {
        Self {
            user: (&value.user).into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct GetAuthnUserError(#[from] GetAuthnUserUseCaseError);

impl ResponseError for GetAuthnUserError {
    fn status_code(&self) -> StatusCode {
        StatusCode::FORBIDDEN // FIXME
    }
}
