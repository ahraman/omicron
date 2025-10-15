pub mod wiki;

use axum::{
    debug_handler,
    response::{IntoResponse, Redirect, Response},
};

use crate::{AppState, Result};

#[debug_handler(state = AppState)]
pub async fn root(AppState(app): AppState) -> Result<Response> {
    let main_page_dir = format!("/w/page/{}", &app.config.main_page);
    Ok(Redirect::to(&main_page_dir).into_response())
}
