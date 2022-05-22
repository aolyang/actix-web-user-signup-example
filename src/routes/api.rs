use crate::routes::v1;
use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashMap;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json(HashMap::from([("msg", "pong")]))
}

pub fn new(cfg: &mut web::ServiceConfig) {
    let api_v1 = web::scope("/api/v1")
        .service(
            web::scope("/users")
                .route("", web::post().to(v1::user::sign_up))
                .route("/login", web::post().to(v1::user::sign_in)),
        )
        .service(
            web::scope("/articles")
                .route("", web::get().to(|| "string"))
                .route("", web::post().to(|| "string")),
        );

    cfg.service(api_v1);
}