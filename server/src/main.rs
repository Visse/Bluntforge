
use std::net::SocketAddr;

use futures_util::StreamExt;
use log::*;
use warp::{Filter, ws::WebSocket};

async fn accept_connection(socket: WebSocket) {
    match handle_connection(socket).await {
        Ok(()) => (),
        Err(e) => {
            error!("Error handling stream: {:?}", e);
        }
    }
}

async fn handle_connection(mut socket: WebSocket) -> anyhow::Result<()> {
    while let Some(msg) = socket.next().await {
        let msg = msg?;
        info!("Message resived: {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let addr = ([0, 0, 0, 0], 2000);

    let socket_path = warp::path("socket")
        .and(warp::ws())
        .and(warp::addr::remote())
        .map(|ws: warp::ws::Ws, peer: Option<SocketAddr>| {
            info!("Connection from {:?} accepted", peer);
            ws.on_upgrade(accept_connection)
        });

    warp::serve(
        socket_path.or(warp::fs::dir("dist/client"))
    ).run(addr).await;

    Ok(())
}
