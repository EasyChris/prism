mod handler;
mod stream;
mod utils;

use axum::{
    routing::post,
    Router,
};
use std::net::SocketAddr;
use crate::config::SharedConfigManager;
use handler::handle_messages;

pub async fn start_proxy_server(config: SharedConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/v1/messages", post(handle_messages))
        .with_state(config);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Proxy server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
