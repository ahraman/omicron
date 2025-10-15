use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error("error during TCP init: {0}")]
    Server(std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{self}").into_response()
    }
}
