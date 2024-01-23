use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;

use super::env_config::EnvConfig;

static ENV_STATE: OnceCell<EnvState> = OnceCell::const_new();

pub struct EnvState {
    pub db_writer_pool: PgPool,
    pub db_reader_pool: PgPool,
    pub env_config: EnvConfig,
}

impl EnvState {
    pub async fn init(env_config: EnvConfig) -> () {
        let db_writer_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(env_config.database_writer.get_url().as_str())
            .await
            .expect("Unable to connect to writer database");
        let db_reader_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env_config.database_reader.get_url())
            .await
            .expect("Unable to connect to reader database");

        let state = EnvState {
            db_writer_pool,
            db_reader_pool,
            env_config,
        };

        if !ENV_STATE.initialized() {
            ENV_STATE
                .set(state)
                .unwrap_or_else(|_| panic!("Unable to set EnvState"));
        }
    }

    pub fn get() -> &'static Self {
        ENV_STATE.get().expect("EnvState not initialized")
    }
}
