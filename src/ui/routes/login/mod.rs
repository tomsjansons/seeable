pub mod link;
pub mod sent;

use askama::Template;
use axum::{
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::SignedCookieJar;
use axum_htmx::HxBoosted;
use nanoid::nanoid;
use serde::Deserialize;

use crate::{
    common::{auth::Auth, env_state::EnvState},
    ui::html::Html,
};

#[derive(Template)]
#[template(path = "routes/login/mod.html")]
pub struct Login {}

pub async fn handler_get(HxBoosted(boosted): HxBoosted, jar: SignedCookieJar) -> Response {
    let auth = Auth::new(jar).await;
    if !auth.is_anonymous() {
        return auth.get_redirect_authorized();
    }
    Html::render_with_content("Seeable - Login", Login {}, boosted)
}

#[derive(Deserialize)]
pub struct LoginFormData {
    pub email: String,
}
pub async fn handler_post(jar: SignedCookieJar, Form(login): Form<LoginFormData>) -> Response {
    let auth = Auth::new(jar).await;
    if !auth.is_anonymous() {
        return auth.get_redirect_authorized();
    }
    let session_id = nanoid!();
    let result = sqlx::query!(
        "insert into login_links (id, email, expires_at) values ($1, $2, now() + interval '600 seconds')",
        session_id.clone(),
        login.email.clone(),
    ).execute(&EnvState::get().db_writer_pool).await;
    tracing::info!(msg = "login link created", email = ?login.email, session_id = ?session_id.clone());
    println!("http://localhost:3000/login/link/{}", session_id);
    match result {
        Ok(_) => Redirect::to("/login/sent").into_response(),
        Err(e) => Redirect::to(format!("/login/sent?error=\"{:?}\"", e).as_str()).into_response(),
    }
}
