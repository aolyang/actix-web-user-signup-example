use crate::modules::user::{NewUser, User};
use sqlx::PgPool;

pub async fn sign_up(pool: &PgPool, user: NewUser) -> Result<User> {
    // TODO impl FromRow for User
    let row = sqlx::query_as::<_, User>(
        r#"
        insert into users (username, email, password, nick_name, avatar)
        values ($1, $2, $3, $4, $5)
        returning id, username, email, nick_name, avatar, create_at, update_at
        "#,
    )
    .bind(user.username)
    .bind(user.email)
    .bind(user.password)
    .bind("")
    .bind("")
    .fetch_one(pool)
    .await?;

    Ok(row)
}
