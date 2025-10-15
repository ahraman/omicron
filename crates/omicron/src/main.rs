use std::{net::SocketAddr, sync::Arc};

use omicron::{
    Error, Result,
    app::{App, Config},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let config = Config::from_env()?;
    let app = Arc::new(App::new(config)?);
    let router = omicron::build_router(app.clone());

    let listener = TcpListener::bind(&app.config.server_url)
        .await
        .map_err(Error::Server)?;
    println!("> listening on: {}", &app.config.server_url);

    Ok(axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .map_err(Error::Server)?)
}
