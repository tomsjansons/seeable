use crate::{
    common::{auth::Auth, env_state::EnvState},
    ui::html::Html,
};
use askama::Template;
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::SignedCookieJar;
use nanoid::nanoid;

#[derive(Template)]
#[template(path = "routes/login/link/mod.html")]
pub struct Link {}

#[derive(serde::Deserialize)]
struct LoginLink {
    email: String,
}

#[derive(serde::Deserialize)]
pub struct User {
    id: String,
}

pub async fn handler(Path(link_id): Path<String>, jar: SignedCookieJar) -> Response {
    let auth = Auth::new(jar).await;
    if !auth.is_anonymous() {
        return auth.get_redirect_authorized();
    }

    let login_link_result = sqlx::query_as!(
        LoginLink,
        "select email from login_links where id = $1 and expires_at > now()",
        link_id,
    )
    .fetch_one(&EnvState::get().db_reader_pool)
    .await;

    let login_link = match login_link_result {
        Err(e) => {
            tracing::error!(msg = "error querying login link", error = ?e);
            return Html::render_with_content("Seeable - Login link error", Link {}, false)
                .into_response();
        }
        Ok(r) => r,
    };

    let _ = sqlx::query!(
        "insert into users (id, email) values ($1, $2) on conflict do nothing",
        nanoid!(),
        login_link.email.clone(),
    )
    .execute(&EnvState::get().db_writer_pool)
    .await;

    let user_result = sqlx::query_as!(
        User,
        "select id from users where email = $1",
        login_link.email,
    )
    .fetch_one(&EnvState::get().db_reader_pool)
    .await;

    let user = match user_result {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(msg = "error querying user", error = ?e);
            return Html::render_with_content("Seeable - Login link error", Link {}, false)
                .into_response();
        }
    };

    auth.create_session(
        &user.id,
        Html::render_with_content("Seeable - Login link error", Link {}, false).into_response(),
    )
    .await
}
