pub mod app;
pub mod controllers;
pub mod error;
mod router;

pub use self::{
    app::{App, AppState, Config},
    error::Error,
    result::Result,
    router::build_router,
};

pub mod result {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;
}
