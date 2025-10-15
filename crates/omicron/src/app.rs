use std::{env, ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::Result;

pub struct Config {
    pub server_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            server_url: env::var("SERVER_URL")?,
        })
    }
}

pub struct App {
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn to_state(self: Arc<Self>) -> AppState {
        AppState(self)
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &App {
        &self.0
    }
}
