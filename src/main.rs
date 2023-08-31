use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

async fn accept_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => stream.write(b"+PONG\r\n")?,
            Err(e) => {
                println!("Error occured: {}", e);
                break;
            }
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:6379";
    let listener = TcpListener::bind(address)?;
    println!("Listening at {}", address);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => accept_connection(stream).await?,
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
