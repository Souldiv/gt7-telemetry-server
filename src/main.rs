mod gt_data;
mod socket_server;
mod server;

use crate::gt_data::GTData;
use crate::socket_server::SocketServer;
use crate::server::handler;

use std::net::SocketAddr;
use std::convert::Infallible;
use std::env;
use warp::Filter;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 GT7 telemetry server version {} 🚀", env!("CARGO_PKG_VERSION"));
    // get args
    let mut playstation_ip: String;
    let mut ws_port: u16;
    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        println!("Usage: {} <playstation_ip> <ws_binding_port>\ndefaults to 192.168.1.85 and 8080 if not provided", args[0]);
        return Ok(());
    } else if args.len() == 3 {
        playstation_ip = args[1].clone();
        ws_port = args[2].parse::<u16>().unwrap();
    } else if args.len() == 2 {
        playstation_ip = args[1].clone();
        ws_port = 8080;
    } else {
        playstation_ip = String::from("192.168.1.85");
        ws_port = 8080;
    }

    let (tx, mut rx) = broadcast::channel::<GTData>(100);

    let tx2 = tx.clone();

    // socket server task
    let recv_port = 33740;
    let send_port = 33739;
    let mut socket_server = SocketServer::new(recv_port, send_port, playstation_ip, tx).await?;

    let udp_server = tokio::spawn(async move {
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
    let ws_addr = SocketAddr::from(([127, 0, 0, 1], ws_port));
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

    println!("🔗 Server started on {}...", ws_addr);
    let ws_server = tokio::spawn(server);

    let (_udp_result, _ws_result) = tokio::join!(udp_server, ws_server);

    Ok(())
}
