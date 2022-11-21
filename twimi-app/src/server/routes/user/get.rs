use std::ops::Deref;

use actix_web::{web::ReqData, HttpResponse};

use crate::server::models;

pub async fn get_authenticated_user(username: ReqData<models::Username>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", username.into_inner().deref()))
}
