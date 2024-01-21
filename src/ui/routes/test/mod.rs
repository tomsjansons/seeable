use askama::Template;
use axum::response::Response;
use axum_htmx::HxBoosted;

use crate::ui::html::Html;

#[derive(Template)]
#[template(path = "routes/test/mod.html")]
pub struct Test {
    pub test_content: String,
}

pub async fn handler(HxBoosted(boosted): HxBoosted) -> Response {
    Html::render_with_content(
        "this is test title",
        Test {
            test_content: "this is test XXX content".to_string(),
        },
        boosted,
    )
}
