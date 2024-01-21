use std::fmt::Display;

use askama::{DynTemplate, Template};
use axum::response::{Html as AxumHtml, IntoResponse, Response};
use http::StatusCode;

#[derive(Template)]
#[template(path = "html.html")]
pub struct Html<T: DynTemplate + Display> {
    pub html_title: String,
    pub html_body_content: Body<T>,
}

#[derive(Template)]
#[template(path = "body.html")]
pub struct Body<T: DynTemplate + Display> {
    pub document_title: String,
    pub document_content: T,
}

impl<T: DynTemplate + Display> Html<T> {
    pub fn render_with_content(title: &str, content: T, boosted: bool) -> Response {
        let html_string_result = if boosted {
            Body {
                document_title: title.to_string(),
                document_content: content,
            }
            .dyn_render()
        } else {
            Html {
                html_title: title.to_string(),
                html_body_content: Body {
                    document_title: title.to_string(),
                    document_content: content,
                },
            }
            .dyn_render()
        };

        match html_string_result {
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Ok(v) => (StatusCode::OK, AxumHtml(v)).into_response(),
        }
    }
}
