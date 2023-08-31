use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn accept_connection(mut conn: TcpStream) -> Result<(), Box<dyn Error>> {
    conn.write("pong!".as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:6379";
    let listener = TcpListener::bind(address)?;
    println!("Listening at {}", address);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => accept_connection(stream)?,
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
