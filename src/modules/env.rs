use snafu::{ResultExt, Snafu};

use crate::common::env_state::EnvState;
use nanoid::nanoid;

pub struct Initer;

#[derive(Debug, Snafu)]
pub enum IniterError {
    #[snafu(display("Sqlx error: {}", source))]
    DbError { source: sqlx::Error },
}

pub enum UserExternalId {
    Email(String),
}

#[derive(serde::Deserialize)]
pub struct User {
    pub id: String,
}

impl Initer {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_or_create_user(
        &self,
        external_id: UserExternalId,
    ) -> Result<User, IniterError> {
        let UserExternalId::Email(email) = external_id;

        let maybe_user = sqlx::query_as!(User, "select id from users where email = $1", email,)
            .fetch_optional(&EnvState::get().db_reader_pool)
            .await
            .context(DbSnafu)?;

        if let Some(user) = maybe_user {
            return Ok(user);
        }

        let mut tx = EnvState::get()
            .db_writer_pool
            .begin()
            .await
            .context(DbSnafu)?;

        let dataspace_id = nanoid!();
        let user_id = nanoid!();
        let user = sqlx::query_as!(
            User,
            "insert into users (id, email, name) values ($1, $2, '') returning id",
            user_id.clone(),
            email.clone()
        )
        .fetch_one(&mut *tx)
        .await
        .context(DbSnafu)?;

        let _ = sqlx::query!(
            "insert into dataspaces (id, name) values ($1, $2)",
            dataspace_id.clone(),
            "Your dataspace"
        )
        .execute(&mut *tx)
        .await
        .context(DbSnafu)?;

        let _ = sqlx::query!(
            "insert into dataspace_members (user_id, dataspace_id) values ($1, $2)",
            user_id,
            dataspace_id
        )
        .execute(&mut *tx)
        .await
        .context(DbSnafu)?;

        let _ = tx.commit().await.context(DbSnafu)?;

        Ok(user)
    }
}
