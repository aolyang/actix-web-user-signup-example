use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Deserialize, Validate, Debug, Clone)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub nick_name: Option<String>,
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct UpdateProfile {
    pub nick_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub avatar: Option<String>,
}

impl From<web::Json<NewUser>> for NewUser {
    fn from(user_info: web::Json<NewUser>) -> Self {
        NewUser {
            username: user_info.username.clone(),
            email: user_info.email.clone(),
            nick_name: user_info.nick_name.clone(),
            password: user_info.password.clone(),
        }
    }
}

impl From<web::Json<UpdateProfile>> for UpdateProfile {
    fn from(info: web::Json<UpdateProfile>) -> Self {
        UpdateProfile {
            nick_name: info.nick_name.clone(),
            email: info.email.clone(),
            avatar: info.avatar.clone(),
        }
    }
}
