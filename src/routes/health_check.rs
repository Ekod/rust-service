use actix_web::{web, HttpResponse};

use super::FormData;

pub async fn health_check(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}