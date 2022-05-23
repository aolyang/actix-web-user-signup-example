use crate::error::ResError;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub fn sign_up(pool: &PgPool) -> Result<HttpResponse, ResError> {
    todo!()
}

pub fn sign_in() -> String {
    "in".to_string()
}
