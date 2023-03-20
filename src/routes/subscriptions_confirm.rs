use actix_web::{HttpResponse, web};


#[derive(serde::Deserialize)]
pub struct Parametrs {
    subscription_token: String
}


#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(_parameters)
)]
pub async fn confirm(_parameters: web::Query<Parametrs>) -> HttpResponse {
    HttpResponse::Ok().finish()
}