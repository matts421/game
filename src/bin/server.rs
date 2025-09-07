use game::constants::*;
use std::net::Ipv4Addr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

struct ConnectionState {
    connections: usize,
    max_connections: usize,
}

async fn broadcast(state: Arc<Mutex<ConnectionState>>) -> std::io::Result<()> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await?;
    socket.set_broadcast(true)?;

    let message = format!("{}:{}", BROADCAST_IDENTIFIER, Ports::GAME as i32);
    let message_bytes = message.as_bytes();

    loop {
        let s = state.lock().await;
        if s.connections < s.max_connections {
            socket
                .send_to(message_bytes, (MULTICAST_IP, Ports::DISCOVERY as u16))
                .await?;
        }
        sleep(Duration::from_secs(BROADCAST_INTERVAL)).await;
    }
}

async fn handle_client(mut socket: TcpStream, state: Arc<Mutex<ConnectionState>>) {
    let mut buf = [0u8; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                break;
            }
        };

        if let Err(e) = socket.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            break;
        }
    }

    {
        let mut s = state.lock().await;
        s.connections -= 1;
        println!("Connections: {}", s.connections);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(ConnectionState {
        connections: 0,
        max_connections: 2,
    }));

    let broadcast_state = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(e) = broadcast(broadcast_state).await {
            eprintln!("broadcast failed: {:?}", e);
        }
    });

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, Ports::GAME as u16)).await?;
    println!("Server initialized. Waiting for connections...");

    loop {
        let (socket, _) = listener.accept().await?;
        {
            let mut s = state.lock().await;
            if s.connections < s.max_connections {
                s.connections += 1;
                println!("Connections: {}", s.connections);
            }
        }
        let client_state = Arc::clone(&state);
        tokio::spawn(handle_client(socket, client_state));
    }
}
