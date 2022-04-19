mod models;

use crate::models::hello;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/v1/ping")]
async fn root_ping() -> impl Responder {
    HttpResponse::Ok().json(hello::Ping { msg: "pong".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust server start at: http://localhost:8080");

    HttpServer::new(|| {
        App::new().service(root_ping)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}