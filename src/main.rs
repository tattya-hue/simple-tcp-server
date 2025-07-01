use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}