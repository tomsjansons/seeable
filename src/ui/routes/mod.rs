pub mod app;
pub mod login;
pub mod logout;
pub mod test;

use askama::Template;
use axum::response::Response;
use axum_htmx::HxBoosted;

use super::html::Html;

#[derive(Template)]
#[template(path = "routes/mod.html")]
pub struct Root {
    pub root_content: String,
}

pub async fn handler(HxBoosted(boosted): HxBoosted) -> Response {
    Html::render_with_content(
        "this is root title",
        Root {
            root_content: "this is dynamics root content".to_string(),
        },
        boosted,
    )
}
