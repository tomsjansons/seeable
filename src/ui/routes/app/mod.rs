use askama::Template;
use axum::response::Response;
use axum_extra::extract::SignedCookieJar;
use axum_htmx::HxBoosted;
use snafu::{ResultExt, Snafu};

use crate::{
    common::{auth::Auth, env_state::EnvState},
    modules::async_tasks::tasks::example_job,
    ui::html::Html,
};

#[derive(Template)]
#[template(path = "routes/app/mod.html")]
pub struct App {}

pub async fn handler(HxBoosted(boosted): HxBoosted, jar: SignedCookieJar) -> Response {
    let auth = Auth::new(jar).await;
    if auth.is_anonymous() {
        return auth.ger_redirect_unauthorized();
    }
    Html::render_with_content("this is root title", App {}, boosted)
}

#[derive(Debug, Snafu)]
pub enum TestErr {
    #[snafu(display("Error creating job runner: {}", source))]
    JobRunnerError { source: serde_json::Error },

    #[snafu(display("Error creating job runner: {}", source))]
    JobRunnerAlsoError { source: sqlx::Error },
}

async fn trigger_test_job() -> Result<(), TestErr> {
    example_job
        .builder()
        // This is where we can override job configuration
        .set_json("John")
        .context(JobRunnerSnafu)?
        .spawn(&EnvState::get().db_writer_pool)
        .await
        .context(JobRunnerAlsoSnafu)?;
    Ok(())
}

pub async fn handler_post(HxBoosted(boosted): HxBoosted) -> Response {
    let _ = trigger_test_job().await;

    Html::render_with_content("this is root title", App {}, boosted)
}
