#![allow(unused_imports)]

use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await;
}