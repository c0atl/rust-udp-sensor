use tokio::net::UdpSocket;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    println!("UDP server listening on port 8080...");

    let mut buf = [0; 1024]; // create buffer for incoming data

    loop {
        let (size, addr) = socket.recv_from(&mut buf).await?;
        let received = String::from_utf8_lossy(&buf[..size]);

        println!("Received '{}' from {}", received, addr);
    }
}
