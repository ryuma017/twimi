use actix_web::HttpResponse;

pub async fn get_authenticated_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}
