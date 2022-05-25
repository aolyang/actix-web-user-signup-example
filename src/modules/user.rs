use actix_web::web;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub nick_name: Option<String>,
    pub avatar: Option<String>,
    pub create_at: NaiveDate,
    pub update_at: NaiveDate,
}

#[derive(Serialize, Validate, Debug, Clone)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UpdateProfile {
    pub nick_name: Option<String>,
    // pub avatar: Option<String>,
}

impl From<web::Json<NewUser>> for NewUser {
    fn from(user_info: web::Json<NewUser>) -> Self {
        NewUser {
            username: user_info.username.clone(),
            email: user_info.email.clone(),
            password: user_info.password.clone(),
        }
    }
}

impl From<web::Json<UpdateProfile>> for UpdateProfile {
    fn from(info: web::Json<UpdateProfile>) -> Self {
        UpdateProfile {
            nick_name: info.nick_name.clone(),
        }
    }
}
