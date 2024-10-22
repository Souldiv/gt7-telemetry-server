use crate::server::ws;
use crate::gt_data::GTData;

use warp::{http::StatusCode, reply::json, ws::Message, Reply, Rejection};
use tokio::sync::broadcast;

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, rx: broadcast::Receiver<GTData>) -> Result<impl Reply, Rejection> {    
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, rx)))
}
