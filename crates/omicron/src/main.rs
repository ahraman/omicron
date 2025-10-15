use std::{net::SocketAddr, sync::Arc};

use axum::{ServiceExt, extract::Request};
use omicron::{App, Config, Error, Result};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let config = Config::from_env()?;
    let app = Arc::new(App::new(config)?);
    let router = omicron::build_router(app.clone());

    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let listener = TcpListener::bind(&app.config.server_url)
        .await
        .map_err(Error::Server)?;
    println!("> listening on: {}", &app.config.server_url);

    Ok(axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service_with_connect_info::<SocketAddr>(router),
    )
    .await
    .map_err(Error::Server)?)
}
