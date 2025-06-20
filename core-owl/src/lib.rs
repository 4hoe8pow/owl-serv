use axum::{
    http::Response,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_service::Service;
use worker::*;

use presentation_owl::handlers::handle_auth::*;

fn router(env: Arc<Env>) -> Router {
    Router::new()
        .route("/", get(hello_owl))
        .route("/auth/{auth_process_code}", post(handle_auth))
        .with_state(env)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let mut app = router(Arc::new(env));

    Ok(app.call(req).await?)
}

async fn hello_owl() -> &'static str {
    "Hello Owl!"
}
