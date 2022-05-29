use crate::error::AppError;
use crate::models::user::NewUser;
use crate::routes::v1::user::handler;
use crate::GlobalState;
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn sign_up(
    state: web::Data<GlobalState>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    match new_user.validate() {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_map = e.field_errors();
            let error = if error_map.contains_key("email") {
                AppError::INVALID_USER_EMAIL.message(format!(
                    "Invalid email address \"{}\"",
                    new_user.email.clone()
                ))
            } else if error_map.contains_key("username") {
                AppError::INVALID_USER_NAME.message(format!(
                    "The username is more than 3 character but got {}",
                    new_user.username.len()
                ))
            } else {
                AppError {
                    code: AppError::INVALID_INPUT,
                    message: "Invalid user info".to_string(),
                }
            };
            Err(error)
        }
    }?;

    let user = handler::sign_up(&state.db, &state.crypto, new_user.into()).await?;
    Ok(HttpResponse::Ok().json(user))
}

// pub fn sign_in() -> String {
//     "in".to_string()
// }
