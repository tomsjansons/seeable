use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

use super::env_config::EnvConfig;

#[derive(Clone)]
pub struct AppState {
    cookie_key: Key,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

impl AppState {
    pub fn new(env_config: &EnvConfig) -> Self {
        Self {
            cookie_key: Key::derive_from(env_config.cookie_secret.as_bytes()),
        }
    }
}
