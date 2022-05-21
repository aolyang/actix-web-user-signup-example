use crate::config::AppConfig;
use crate::{PgPool, PgPoolOptions};
use actix_web::web;
use std::future;

pub struct GlobalState {
    pub db: PgPool,
}
