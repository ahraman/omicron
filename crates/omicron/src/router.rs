use std::sync::Arc;

use axum::{Router, debug_handler, response::IntoResponse, routing::get};
use tower_http::normalize_path::NormalizePathLayer;

use crate::app::App;

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        .route("/", get(root))
        .with_state(app.clone().to_state())
}

#[debug_handler(state = AppState)]
async fn root() -> impl IntoResponse {
    "hello, world!"
}
