use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    stream.write(&buf).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <address> <port>", args[0]);
        std::process::exit(1);
    }

    let addr = args[1].clone();
    let port = args[2].clone();
    println!("address: {:?}, port: {:?}", &addr, &port);

    // Bind the TCP listener to the specified address and port
    let listener = TcpListener::bind(format!("{}:{}", &addr, &port)).unwrap();

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("connection established");
        handle_client(_stream);
    }
}