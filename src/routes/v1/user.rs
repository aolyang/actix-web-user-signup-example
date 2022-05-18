use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, sqlx::FromRow)]
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

#[derive(Serialize, Validate, Debug)]
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
