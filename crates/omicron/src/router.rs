use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    App,
    controllers::{root, wiki},
};

pub fn build_router(app: Arc<App>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/w", get(wiki::root))
        .route("/w/page", get(wiki::page::root))
        .route("/w/page/{title}", get(wiki::page::get))
        .with_state(app.clone().to_state())
}
