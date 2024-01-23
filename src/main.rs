mod common;
mod modules;

mod ui;

use crate::modules::*;
use crate::{
    common::{app_state::AppState, env_state::EnvState},
    ui::routes,
};
use axum::{
    body::Body,
    extract::Request,
    routing::{get, post},
    Router,
};
use common::env_config::{self, Env};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_request_id::{RequestId, RequestIdLayer};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let env_config = env_config::EnvConfig::new();
    EnvState::init(env_config).await;
    let app_state = AppState::new(&EnvState::get().env_config);

    tracing_subscriber::fmt::init();

    tracing::info!(
        "starting server from {:?}",
        std::env::current_dir().expect("failed to get current dir")
    );

    let paths = std::fs::read_dir("static").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    if EnvState::get().env_config.env != Env::Local {
        sqlx::migrate!()
            .run(&EnvState::get().db_writer_pool)
            .await
            .expect("migrations failed");
    }

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(routes::handler))
        .route("/login", get(routes::login::handler_get))
        .route("/login", post(routes::login::handler_post))
        .route("/login/sent", get(routes::login::sent::handler))
        .route("/login/link/:link_id", get(routes::login::link::handler))
        .route("/logout", get(routes::logout::handler))
        .route("/app", get(routes::app::handler))
        .route("/test", get(routes::test::handler))
        .route("/user/:id", get(get_user))
        .route("/_sys/version", get(|| async { VERSION }))
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                tracing::error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // This layer creates a new id for each request and puts it into the request extensions.
        // Note that it should be added after the Trace layer.
        .layer(RequestIdLayer)
        .with_state(app_state);

    tracing::info!("starting async processing");
    let _ = async_tasks::executor::start(&EnvState::get().db_writer_pool)
        .await
        .expect("async processing failed");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("web serving failed");
    tracing::info!("done");
}

async fn get_user() -> impl axum::response::IntoResponse {
    "Hello, world!"
}
