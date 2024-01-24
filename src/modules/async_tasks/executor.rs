use snafu::prelude::*;
use sqlx::Error;
use sqlxmq::{JobRegistry, JobRunnerHandle};

use super::tasks::dataspace_init;

#[derive(Debug, Snafu)]
pub enum AsyncTaskExecutorError {
    #[snafu(display("Error creating job runner: {}", source))]
    JobRunnerError { source: Error },
}

pub struct AsyncTaskExecutor {
    _runner: JobRunnerHandle,
}

impl AsyncTaskExecutor {
    pub async fn start(pool: &sqlx::PgPool) -> Result<Self, AsyncTaskExecutorError> {
        let registry = JobRegistry::new(&[dataspace_init]);

        let runner = registry
            .runner(pool)
            .set_concurrency(10, 20)
            .run()
            .await
            .context(JobRunnerSnafu)?;

        Ok(Self { _runner: runner })
    }
}
