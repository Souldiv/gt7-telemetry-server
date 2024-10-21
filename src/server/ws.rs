use crate::gt_data::GTData;

use futures::{StreamExt, FutureExt, TryStreamExt};
use tokio_stream::wrappers::BroadcastStream;
use warp::ws::{Message, WebSocket};
use serde_json;
use tokio::sync::broadcast;

pub async fn client_connection(ws: WebSocket, rx: broadcast::Receiver<GTData>) {
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let bs = BroadcastStream::new(rx);

    bs.map(|data| {
        Ok(Message::text(serde_json::to_string(&data.unwrap()).unwrap()))
    }).forward(client_ws_sender).await;
}
