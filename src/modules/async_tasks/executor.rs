use snafu::prelude::*;
use sqlx::Error;
use sqlxmq::{JobRegistry, JobRunnerHandle};

use super::tasks::example_job;

#[derive(Debug, Snafu)]
pub enum AsyncTaskExecutorError {
    #[snafu(display("Error creating job runner: {}", source))]
    JobRunnerError { source: Error },
}

pub async fn start(pool: &sqlx::PgPool) -> Result<JobRunnerHandle, AsyncTaskExecutorError> {
    let registry = JobRegistry::new(&[example_job]);

    let runner = registry
        .runner(pool)
        .set_concurrency(10, 20)
        .run()
        .await
        .context(JobRunnerSnafu)?;

    Ok(runner)
}
