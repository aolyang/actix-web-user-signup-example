use crate::error::ResError;
use crate::models::user::{NewUser, User};
use crate::services::crypto::Crypto;
use sqlx::PgPool;

pub async fn sign_up(pool: &PgPool, hasher: &Crypto, user: NewUser) -> Result<User, ResError> {
    let hash_pwd = hasher.hash_pwd(user.password.clone()).await.unwrap();

    let row = sqlx::query!(
        r#"
        insert into users (username, email, password, nick_name, avatar)
        values ($1, $2, $3, $4, $5)
        returning id, username, email, nick_name, avatar, create_at, update_at
        "#,
        user.username,
        user.email,
        hash_pwd,
        user.nick_name,
        "",
    )
    .fetch_one(pool)
    .await?;

    let email = match row.email {
        Some(email) => email,
        None => "".to_string(),
    };

    Ok(User {
        id: row.id,
        username: row.username,
        email,
        nick_name: row.nick_name,
        password: "".to_string(),
        avatar: row.avatar,
        create_at: row.create_at,
        update_at: row.update_at,
    })
}
