use askama::Template;
use axum::response::Response;
use axum_extra::extract::SignedCookieJar;
use axum_htmx::HxBoosted;

use crate::{common::auth::Auth, ui::html::Html};

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

// #[derive(Debug, Snafu)]
// pub enum TestErr {
//     #[snafu(display("Error creating job runner: {}", source))]
//     JobRunnerError { source: serde_json::Error },
//
//     #[snafu(display("Error creating job runner: {}", source))]
//     JobRunnerAlsoError { source: sqlx::Error },
// }

// async fn trigger_dataspace_init_job<'a>(
//     dataspace_id: String,
//     executor: &dyn sqlx::Executor<'a>,
// ) -> Result<(), TestErr> {
//     let args = DataspaceInitArgs { database_id };
//     dataspace_init
//         .builder()
//         .set_json(&args)
//         .context(JobRunnerSnafu)?
//         .spawn(executor)
//         .await
//         .context(JobRunnerAlsoSnafu)?;
//     Ok(())
// }
