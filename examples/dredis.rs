use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpListener,
};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;

    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(err) = process_redis_connection(stream, raddr).await {
                warn!("Error processing connection with {}: {:?}", raddr, err);
            };
        });
    }
}

async fn process_redis_connection(
    mut stream: tokio::net::TcpStream,
    raddr: SocketAddr,
) -> anyhow::Result<()> {
    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("line: {:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    warn!("Connection closed: {}", raddr);

    Ok(())
}
