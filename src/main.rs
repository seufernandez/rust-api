use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::io;

#[derive(Serialize)]
struct Message {
    content: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: "Bem-vindo à nossa API feita em Rust!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
