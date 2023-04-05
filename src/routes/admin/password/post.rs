use actix_web::{web, HttpResponse};
use secrecy::Secret;

use crate::routes::FormData;

#[derive(serde::Deserialize)]
pub struct FromData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(form: web::Form<FormData>) -> Result<HttpResponse, actix_web::Error> {
    todo!()
}
