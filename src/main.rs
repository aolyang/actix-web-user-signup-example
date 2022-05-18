#[macro_use]
extern crate validator_derive;

mod config;
mod routes;
mod services;
mod state;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::collections::HashMap;

use crate::state::GlobalState;

#[get("/api/v1/ping")]
async fn root_ping() -> impl Responder {
    HttpResponse::Ok().json(HashMap::from([("msg", "pong".to_string())]))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::AppConfig::from_env().unwrap();

    let server = format!("{}:{}", config.server.host, config.server.port);
    let db_url = format!(
        "postgres://{}:{}@127.0.0.1:5432/blog",
        config.postgres_user, config.postgres_password
    );

    println!("Rust server start at: {}", server);

    let shared_state = web::Data::new(GlobalState {
        db: PgPoolOptions::new().connect(&db_url).await.unwrap(),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_state.clone())
            .service(root_ping)
    })
    .bind(server)?
    .run()
    .await
}
