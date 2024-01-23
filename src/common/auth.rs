use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::SignedCookieJar;
use cookie::Cookie;
use http::StatusCode;
use nanoid::nanoid;

use super::env_state::EnvState;

const SEEBABLE_SESSION_ID_COOKIE_NAME: &str = "seeable-session-id";

#[derive(Debug)]
struct User {
    id: String,
    session_id: String,
}

#[derive(Debug)]
enum Identity {
    User(User),
    Anonymous,
}

pub struct Auth {
    cookie_jar: SignedCookieJar,
    identity: Identity,
}

impl Auth {
    pub async fn new(cookie_jar: SignedCookieJar) -> Self {
        let session_cookie = cookie_jar.get(SEEBABLE_SESSION_ID_COOKIE_NAME);
        let identity = match session_cookie {
            Some(session_cookie) => {
                let session_id = session_cookie.value();
                let user_result = sqlx::query_as!(
                    User,
                    "SELECT users.id, sessions.id as session_id FROM sessions \
                    JOIN users ON users.id = sessions.user_id \
                    WHERE sessions.id = $1 AND sessions.expires_at > now()",
                    session_id,
                )
                .fetch_optional(&EnvState::get().db_reader_pool)
                .await;
                match user_result {
                    Ok(None) => Identity::Anonymous,
                    Ok(Some(user)) => Identity::User(user),
                    Err(e) => {
                        tracing::error!(msg = "error querying user", session_id = ?session_id, error = ?e);
                        Identity::Anonymous
                    }
                }
            }
            None => Identity::Anonymous,
        };
        tracing::info!(msg = "Auth created", identity = ?identity);
        Auth {
            cookie_jar,
            identity,
        }
    }

    pub fn is_anonymous(&self) -> bool {
        match self.identity {
            Identity::Anonymous => true,
            _ => false,
        }
    }

    pub fn get_response_unauthorized(&self) -> Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }

    pub fn ger_redirect_unauthorized(&self) -> Response {
        Redirect::to("/login").into_response()
    }

    pub fn get_redirect_authorized(&self) -> Response {
        Redirect::to("/app").into_response()
    }

    pub async fn destroy_session(self) -> Response {
        match self.identity {
            Identity::User(user) => {
                let delete_result =
                    sqlx::query!("DELETE FROM sessions WHERE id = $1", user.session_id)
                        .execute(&EnvState::get().db_writer_pool)
                        .await;
                match delete_result {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!(msg = "error deleting session", error = ?e, user_id = ?user.id, session_id = ?user.session_id);
                    }
                }
            }
            Identity::Anonymous => {}
        }
        (
            self.cookie_jar
                .remove(Cookie::from(SEEBABLE_SESSION_ID_COOKIE_NAME)),
            Redirect::to("/login"),
        )
            .into_response()
    }

    pub async fn create_session(self, user_id: &str, error_response: Response) -> Response {
        let session_id = nanoid!();
        let insert_result = sqlx::query!(
            "insert into sessions (id, user_id, expires_at) values ($1, $2, now() + interval '1 day')",
            session_id,
            user_id,
        ).execute(&EnvState::get().db_writer_pool).await;
        match insert_result {
            Ok(_) => {}
            Err(e) => {
                tracing::error!(msg = "error inserting session", error = ?e);
                return error_response;
            }
        }

        (
            self.cookie_jar.add(
                Cookie::build((SEEBABLE_SESSION_ID_COOKIE_NAME, session_id))
                    .http_only(true)
                    .secure(true)
                    .path("/"),
            ),
            Redirect::to("/app"),
        )
            .into_response()
    }
}
