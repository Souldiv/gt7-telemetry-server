use byteorder::{ByteOrder, LittleEndian};
use tokio::io::{self, Interest};
use tokio::net::UdpSocket;
use tokio::time::{self, Duration};
use tokio::sync::broadcast;
use crate::gt_data::GTData;
use crate::socket_server::helper;

pub struct SocketServer {
    recv_port: u16,
    send_port: u16,
    playstation_ip: String,
    socket: Option<UdpSocket>,
    tx: broadcast::Sender<GTData>,
}

impl Drop for SocketServer {
    fn drop(&mut self) {
        println!("{}", self.socket.as_ref().unwrap().local_addr().unwrap());
    }
}

impl SocketServer {
    pub async fn new(
        recv_port: u16,
        send_port: u16,
        playstation_ip: String,
        tx: broadcast::Sender<GTData>
    ) -> std::io::Result<SocketServer> {
        let socket = Some(UdpSocket::bind(format!("0.0.0.0:{}", recv_port)).await?);
        println!("📡 UDP socket bound to port {} with ip {}", recv_port, playstation_ip);
        Ok(SocketServer {
            recv_port,
            send_port,
            playstation_ip,
            socket,
            tx
        })
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let mut package_id = 0;
        let mut package_nr = 0;
        let timeout_duration = Duration::from_secs(5);
        loop {
            // Use select! to wait for either readiness or timeout
            tokio::select! {
                // Check if the socket is ready
                Ok(ready) = self.socket.as_ref().unwrap().ready(Interest::READABLE) => {
                    if package_nr == 0 {
                        self.send_heartbeat().await;
                    }

                    if ready.is_readable() {
                        let mut buf = [0; 4096];

                        let amt = match self.socket.as_ref().unwrap().try_recv_from(&mut buf) {
                            Ok((amt, _src)) => {
                                // Increment package nr
                                package_nr += 1;
                                if package_nr > 100 {
                                    package_nr = 0;
                                }
                                amt
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {0}
                            Err(e) => {
                                println!("Error receiving: {}", e);
                                0
                            }
                        };

                        if amt == 0 {
                            continue;
                        }

                        let buf = &mut buf[..amt];

                        let decrypted_data = helper::salsa20_dec(buf);
                        if !decrypted_data.is_empty() && LittleEndian::read_i32(&decrypted_data[0x70..0x74]) > package_id {
                            // Your logic here
                            // println!("Package ID: {} Package nr: {}", LittleEndian::read_i32(&decrypted_data[0x70..0x74]), package_nr);
                            package_id = LittleEndian::read_i32(&decrypted_data[0x70..0x74]);
                            let gt_data = GTData::new(&decrypted_data);
                            // print!("\r{:?}", gt_data);
                            let _ = self.tx.send(gt_data);
                        }
                    }
                }

                // Timeout case
                _ = time::sleep(timeout_duration) => {
                    println!("Timeout occurred, no data received.");
                    // Handle timeout logic here, if needed
                    self.reset_socket().await?;
                    self.send_heartbeat().await;
                }
            }
        }
    }

    async fn reset_socket(&mut self) -> std::io::Result<()> {
        // Create a new UdpSocket
        self.socket = None;
        self.socket = Some(UdpSocket::bind(format!("0.0.0.0:{}", self.recv_port)).await?);
        println!("UDP socket reset and bound to port {}", self.recv_port);
        Ok(())
    }

    async fn send_heartbeat(&self) {
        let _ = match self
            .socket
            .as_ref()
            .unwrap()
            .send_to(b"A", format!("{}:{}", self.playstation_ip, self.send_port))
            .await
        {
            Ok(r_size) => {
                // println!("Heartbeat sent, size: {}", r_size);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => {
                println!("Error sending heartbeat: {}", e);
            }
        };
    }
}
