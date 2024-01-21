use askama::Template;
use axum::{extract::Query, response::Response};
use axum_htmx::HxBoosted;
use serde::Deserialize;

use crate::ui::html::Html;

#[derive(Template)]
#[template(path = "routes/login/sent/mod.html")]
pub struct Sent {
    pub maybe_error: String,
}

#[derive(Deserialize)]
pub struct SendResult {
    pub error: Option<String>,
}

pub async fn handler(HxBoosted(boosted): HxBoosted, send_result: Query<SendResult>) -> Response {
    let maybe_error = send_result.error.clone();
    Html::render_with_content(
        "Seeable - Login Link Sent",
        Sent {
            maybe_error: maybe_error.map_or("".to_string(), |e| e),
        },
        boosted,
    )
}
