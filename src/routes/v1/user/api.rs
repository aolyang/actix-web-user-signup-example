use crate::routes::v1::user::handler;
use crate::GlobalState;
use actix_web::{web, HttpResponse};

pub async fn sign_up(state: web::Data<GlobalState>) -> HttpResponse {
    let user = handler::sign_up(&state.db).await;
    HttpResponse::Ok().json(user)
}

pub fn sign_in() -> String {
    "in".to_string()
}
