use tokio::net::UdpSocket;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?; // Bind to any available port
    socket.connect("127.0.0.1::8080").await?; // send to server

    loop {
        let mut file = File::open("/dev/netsensor").await?;
        let mut sensor_data = String::new();
        file.read_to_string(&mut sensor_data).await?;

        socket.send(sensor_data.as_bytes()).await?;
        println!("Transmitted sensor data: {}", sensor_data.trim());

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; 
    }
}
