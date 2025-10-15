pub mod page;

use axum::{debug_handler, response::Response};

use crate::{AppState, Result, controllers};

#[debug_handler(state = AppState)]
pub async fn root(AppState(app): AppState) -> Result<Response> {
    controllers::root(app.to_state()).await
}
