mod config;
mod models;
mod state;

use std::collections::HashMap;

use crate::state::GlobalState;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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
        "postgres://postgres:{}@127.0.0.1:5432/blog",
        config.postgres_password
    );

    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    println!("Rust server start at: {}", server);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(GlobalState { db: db_pool })
            .service(root_ping)
    })
    .bind(server)?
    .run()
    .await
}
