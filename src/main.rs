use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <address> <port>", args[0]);
        return Ok(());
    }

    let addr = args[1].clone();
    let port = args[2].clone();
    println!("address: {:?}, port: {:?}", &addr, &port);

    let listener = TcpListener::bind(format!("{}:{}", &addr, &port))?;

    let mut client_count = 0;
    for stream in listener.incoming() {
        client_count += 1;
        println!("client{} connected", client_count);
        handle_client(stream?);
    }

    Ok(())
}