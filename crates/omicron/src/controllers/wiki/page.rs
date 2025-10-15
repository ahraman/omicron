use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Response},
};

use crate::{AppState, Result, controllers};

#[debug_handler(state = AppState)]
pub async fn root(AppState(app): AppState) -> Result<Response> {
    controllers::root(app.to_state()).await
}

#[debug_handler(state = AppState)]
pub async fn get(AppState(_): AppState, Path(title): Path<String>) -> Result<Response> {
    Ok(format!("visited page `{title}`").into_response())
}
