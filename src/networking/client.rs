use crate::networking::constants::*;
use regex::Regex;
use std::io::{self, Write};
use std::net::Ipv4Addr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};

pub async fn discover() -> std::io::Result<Option<String>> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, Ports::DISCOVERY as u16)).await?;
    socket.join_multicast_v4(
        MULTICAST_IP.parse::<Ipv4Addr>().unwrap(),
        Ipv4Addr::UNSPECIFIED,
    )?;

    let pattern = format!(r"^{}:(\d+)$", BROADCAST_IDENTIFIER);
    let re = Regex::new(&pattern).unwrap();

    let mut buf = [0u8; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..amt]);
        println!("Discovered server: {} - {}", src.ip(), msg);

        if let Some(caps) = re.captures(msg.to_string().as_str()) {
            let port = &caps[1];
            return Ok(Some(format!("{}:{}", src.ip(), port)));
        }
    }
}

pub async fn echo_client() -> std::io::Result<()> {
    match discover().await {
        Ok(Some(server)) => {
            let mut stream = TcpStream::connect(server.clone()).await?;
            println!("Connected to server, {}", server);

            let stdin = io::stdin();
            loop {
                print!("You: ");
                io::stdout().flush()?; // Flush to show prompt

                let mut input = String::new();
                stdin.read_line(&mut input)?;

                if input.trim() == "exit" {
                    break;
                }

                stream.write_all(input.as_bytes()).await?;

                let mut buffer = [0; 512];
                let n = stream.read(&mut buffer).await?;
                println!("Echoed: {}", String::from_utf8_lossy(&buffer[..n]));
            }
        }
        Ok(None) => {
            println!("No servers found.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
