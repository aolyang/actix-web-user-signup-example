mod config;
mod models;

use std::collections::HashMap;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[get("/api/v1/ping")]
async fn root_ping() -> impl Responder {
    HttpResponse::Ok().json(HashMap::from([("msg", "pong".to_string())]))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::AppConfig::from_env().unwrap();

    let server = format!("{}:{}", config.server.host, config.server.port);

    println!("Rust server start at: {}", server);

    HttpServer::new(|| App::new().service(root_ping))
        .bind(server)?
        .run()
        .await
}
