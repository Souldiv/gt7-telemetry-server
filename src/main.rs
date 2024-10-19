mod socket_server;

use crate::socket_server::SocketServer;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    {   
        let recv_port = 33740;
        let send_port = 33739;
        let playstation_ip = String::from("192.168.1.85");
        let mut socket_server = SocketServer::new(recv_port, send_port, playstation_ip).await?;
        socket_server.run().await?;
    } // the socket is closed here
    Ok(())
}
