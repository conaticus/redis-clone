use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::commands::execute_command;

const BUFFER_SIZE: usize = 1024;
pub type TCPBuffer = [u8; BUFFER_SIZE];

pub async fn accept_connection(mut socket: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = [0; BUFFER_SIZE];
        match socket.read(&mut buffer).await {
            Ok(0) => break,
            Ok(_) => {
                let response = execute_command(buffer);
                socket.write_all(response.as_slice()).await?;
            }
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

pub async fn listen(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port)).await?;
    println!("Listening at localhost:{}", port);

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
