use std::error::Error;

use sqlxmq::{job, CurrentJob};

use crate::env::Initer;

#[derive(snafu::Snafu, Debug)]
pub enum DataspaceInitError {
    #[snafu(display("Missing arguments"))]
    MissingArgumentsError,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataspaceInitArgs {
    pub dataspace_id: String,
}

#[job(channel_name = "env:init:dw")]
pub async fn dataspace_init(
    mut current_job: CurrentJob,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let maybe_args: Option<DataspaceInitArgs> = current_job.json()?;

    let args = match maybe_args {
        Some(a) => a,
        None => return Err(Box::new(DataspaceInitError::MissingArgumentsError)),
    };

    let result = Initer::init_data_warehouse(args.dataspace_id).await;

    match result {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };

    // Mark the job as complete
    current_job.complete().await?;

    Ok(())
}
