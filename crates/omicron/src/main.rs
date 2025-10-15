use axum::{Router, response::IntoResponse, routing::get};
use omicron::{Error, Result};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let router = Router::new().route("/", get(root));

    let server_url = std::env::var("SERVER_URL")?;
    let listener = TcpListener::bind(&server_url)
        .await
        .map_err(Error::Server)?;

    println!("> listening on: {server_url}");
    Ok(axum::serve(listener, router).await.map_err(Error::Server)?)
}

async fn root() -> impl IntoResponse {
    "hello, world!"
}
