use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use std::str;

const TAK_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind the UDP socket to the specified address
    let addr = format!("0.0.0.0:{}", TAK_PORT);
    let socket = UdpSocket::bind(&addr).await?;
    println!("TAK server running on {}", addr);

    // Create a channel for handling messages
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(100);

    // Spawn a task to handle incoming messages
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = socket_clone.recv_from(&mut buf).await.unwrap();
            let data = buf[..len].to_vec();
            // Send the received data to the channel
            if let Err(_) = tx.send((data, addr)).await {
                eprintln!("Failed to send data to handler");
            }
        }
    });

    // Handle incoming messages
    while let Some((data, addr)) = rx.recv().await {
        handle_message(data, addr).await;
    }

    Ok(())
}

async fn handle_message(data: Vec<u8>, addr: SocketAddr) {
    if let Ok(message) = str::from_utf8(&data) {
        println!("Received message from {}: {}", addr, message);
    } else {
        eprintln!("Received invalid UTF-8 data from {}", addr);
    }

    let response = b"ACK";
    if let Err(e) = socket.send_to(response, addr).await {
        eprintln!("Failed to send response to {}: {}", addr, e);
    }
}