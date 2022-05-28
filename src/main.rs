#[macro_use]
extern crate validator_derive;

mod config;
mod error;
mod middleware;
mod models;
mod routes;
mod services;
mod state;
mod utils;

use crate::services::crypto::Crypto;
use crate::state::GlobalState;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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
        crypto: Crypto::new(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_state.clone())
            .configure(routes::api::new)
    })
    .bind(server)?
    .run()
    .await
}
