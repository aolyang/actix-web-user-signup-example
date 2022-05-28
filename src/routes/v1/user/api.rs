use crate::error::AppError;
use crate::models::user::NewUser;
use crate::routes::v1::user::handler;
use crate::utils::defaults::default_string;
use crate::GlobalState;
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn sign_up(
    state: web::Data<GlobalState>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    match new_user.validate() {
        Ok(_) => {}
        Err(e) => {
            let error_map = e.field_errors();
            let error_msg = if error_map.contains_key("email") {
                format!(
                    "Invalid email address \"{}\"",
                    default_string(new_user.email.clone())
                )
            } else {
                "".to_string()
            };
            todo!("fix err");
            Err(error_msg)
        }
    }
    let user = handler::sign_up(&state.db, &state.crypto, new_user.into()).await?;
    Ok(HttpResponse::Ok().json(user))
}

// pub fn sign_in() -> String {
//     "in".to_string()
// }
