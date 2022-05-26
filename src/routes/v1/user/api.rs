use crate::error::ResError;
use crate::models::user::NewUser;
use crate::routes::v1::user::handler;
use crate::GlobalState;
use actix_web::{web, HttpResponse};

pub async fn sign_up(
    state: web::Data<GlobalState>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, ResError> {
    let user = handler::sign_up(&state.db, &state.crypto, form.into()).await?;
    Ok(HttpResponse::Ok().json(user))
}

// pub fn sign_in() -> String {
//     "in".to_string()
// }
