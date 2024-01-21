use axum::response::Response;
use axum_extra::extract::SignedCookieJar;

use crate::common::auth::Auth;

pub async fn handler(jar: SignedCookieJar) -> Response {
    let auth = Auth::new(jar).await;
    auth.destroy_session().await
}
