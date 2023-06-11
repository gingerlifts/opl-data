use std::net::{Ipv4Addr, SocketAddrV4};

use axum::routing::{get, post, put};
use axum::{Router, Server};
use color_eyre::eyre::Result;

mod lifter_requests;
mod webhooks;

fn setup() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    setup()?;

    let router = Router::new()
        .route("/health", get(health))
        .route("/api/v1/webhook", post(webhooks::handler))
        .route("/api/v1/lifter-request", put(lifter_requests::handler));

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 4025).into();

    tracing::info!("Listening for requests on {addr}");

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn health() -> &'static str {
    "Everything looks good to me!"
}
