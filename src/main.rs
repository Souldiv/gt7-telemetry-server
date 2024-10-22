mod gt_data;
mod socket_server;
mod server;

use crate::gt_data::GTData;
use crate::socket_server::SocketServer;
use crate::server::handler;

use std::net::SocketAddr;
use std::convert::Infallible;
use warp::Filter;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, mut rx) = broadcast::channel::<GTData>(100);

    let tx2 = tx.clone();

    // socket server task
    let recv_port = 33740;
    let send_port = 33739;
    let playstation_ip = String::from("192.168.1.85");
    let mut socket_server = SocketServer::new(recv_port, send_port, playstation_ip, tx).await?;

    tokio::spawn(async move {
        socket_server.run().await
    });

    // logs for server
    let log = warp::log::custom(|info| {
        // Use a log macro, or slog, or println, or whatever!
        println!(
            "{} {} {} {}",
            info.remote_addr().unwrap(),
            info.method(),
            info.path(),
            info.status(),
        );
    });

    // server
    let ws_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let health_route = warp::path!("health").and_then(handler::health_handler);
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || tx2.clone().subscribe()))
        .and_then(handler::ws_handler);

    let routes = health_route
        .or(ws_route)
        .with(warp::cors().allow_any_origin())
        .with(log);
    let server = warp::serve(routes).bind(ws_addr);

    println!("Server started on {}", ws_addr);
    tokio::spawn(server).await?;

    Ok(())
}
