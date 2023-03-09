use std::net::TcpListener;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> std::io::Result<Server>{
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}