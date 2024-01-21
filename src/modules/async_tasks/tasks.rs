use std::error::Error;

use sqlxmq::{job, CurrentJob};

// Arguments to the `#[job]` attribute allow setting default job options.
#[job(channel_name = "foo")]
pub async fn example_job(
    // The first argument should always be the current job.
    mut current_job: CurrentJob,
    // Additional arguments are optional, but can be used to access context
    // provided via [`JobRegistry::set_context`].
    message: &'static str,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Decode a JSON payload
    let who: Option<String> = current_job.json()?;

    // Do some work
    tracing::trace!("{} {}", message, who.as_deref().unwrap_or("world"));

    // Mark the job as complete
    current_job.complete().await?;

    Ok(())
}
