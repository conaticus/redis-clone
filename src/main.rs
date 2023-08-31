use std::io;
use tokio::{net::{TcpStream, TcpListener}, io::{AsyncReadExt, AsyncWriteExt}};

async fn accept_connection(mut socket: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = [0; 512];
        match socket.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_) => socket.write_all(b"+PONG\r\n").await?,
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1:6379";
    let listener = TcpListener::bind(address).await?;
    println!("Listening at {}", address);
    
    loop {
        let socket_res = listener.accept().await;
        match socket_res {
            Ok((socket, _)) => {
                tokio::spawn(accept_connection(socket));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
