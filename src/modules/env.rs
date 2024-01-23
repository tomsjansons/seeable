use snafu::{ResultExt, Snafu};
use sqlx::postgres::PgPoolOptions;

use crate::{
    async_tasks::tasks::{dataspace_init, DataspaceInitArgs},
    common::env_state::EnvState,
};
use nanoid::nanoid;

pub struct Initer;

#[derive(Debug, Snafu)]
pub enum IniterError {
    #[snafu(display("Sqlx error: {}", source))]
    DbError { source: sqlx::Error },

    #[snafu(display("Error creating job runner: {}", source))]
    JobRunnerError { source: serde_json::Error },
}

pub enum UserExternalId {
    Email(String),
}

#[derive(serde::Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct Database {
    pub datname: String,
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

        let maybe_user = sqlx::query_as!(User, "SELECT id FROM users WHERE email = $1", email,)
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
            "INSERT INTO users (id, email, name) VALUES ($1, $2, '') returning id",
            user_id.clone(),
            email.clone()
        )
        .fetch_one(&mut *tx)
        .await
        .context(DbSnafu)?;

        let _ = sqlx::query!(
            "INSERT INTO dataspaces (id, name) VALUES ($1, $2)",
            dataspace_id.clone(),
            "Your dataspace"
        )
        .execute(&mut *tx)
        .await
        .context(DbSnafu)?;

        let _ = sqlx::query!(
            "INSERT INTO dataspace_members (user_id, dataspace_id) VALUES ($1, $2)",
            user_id,
            dataspace_id.clone()
        )
        .execute(&mut *tx)
        .await
        .context(DbSnafu)?;

        let args = DataspaceInitArgs { dataspace_id };
        dataspace_init
            .builder()
            .set_json(&args)
            .context(JobRunnerSnafu)?
            .spawn(&mut *tx)
            .await
            .context(DbSnafu)?;

        let _ = tx.commit().await.context(DbSnafu)?;

        Ok(user)
    }

    pub async fn init_data_warehouse(dataspace_id: String) -> Result<(), IniterError> {
        let dw_con = PgPoolOptions::new()
            .max_connections(1)
            .connect(&EnvState::get().env_config.data_warehouse.get_url_admin())
            .await
            .context(DbSnafu)?;

        let database = format!("dataspace_{dataspace_id}");
        let model_schema_name = format!("model_{dataspace_id}");
        let role_name = format!("username_{dataspace_id}");

        let maybe_db = sqlx::query_as!(
            Database,
            "SELECT datname FROM pg_database WHERE datname = $1",
            database
        )
        .fetch_optional(&dw_con)
        .await
        .context(DbSnafu)?;

        if maybe_db.is_none() {
            let _ = sqlx::query!("CREATE DATABASE \"{database}\";")
                .execute(&dw_con)
                .await
                .context(DbSnafu);
        }

        let mut tx = dw_con.begin().await.context(DbSnafu)?;
        let statements = vec![
            format!("CREATE ROLE \"{role_name}\";"),
            format!("CREATE SCHEMA \"{model_schema_name}\";"),
            format!("GRANT CONNECT ON DATABASE \"{database}\" TO \"{role_name}\";"),
            format!("ALTER DEFAULT PRIVILEGES GRANT USAGE ON SCHEMAS TO \"{role_name}\";"),
            format!("ALTER DEFAULT PRIVILEGES GRANT SELECT ON TABLES TO \"{role_name}\";"),
            format!("GRANT SELECT, INSERT, UPDATE, DELETE, TRUNCATE, REFERENCES ON ALL TABLES IN SCHEMA \"{model_schema_name}\" TO \"{role_name}\";"),        
        ];

        for statement in statements {
            let _ = sqlx::query(statement.as_str())
                .execute(&mut *tx)
                .await
                .context(DbSnafu)?;
        }

        tx.commit().await.context(DbSnafu)?;

        Ok(())
    }
}
