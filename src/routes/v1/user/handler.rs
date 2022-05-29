use crate::error::AppError;
use crate::models::user::{NewUser, User};
use crate::services::crypto::Crypto;
use sqlx::PgPool;

pub async fn sign_up(pool: &PgPool, hasher: &Crypto, user: NewUser) -> Result<User, AppError> {
    let hash_pwd = hasher.hash_pwd(user.password.clone()).await.unwrap();

    let row: User = sqlx::query_as!(
        User,
        r#"
        insert into users (username, email, password, nick_name, avatar)
        values ($1, $2, $3, $4, $5)
        returning id, username, password, email, nick_name, avatar, create_at, update_at
        "#,
        user.username,
        user.email,
        hash_pwd,
        user.nick_name,
        "",
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
