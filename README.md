# Simple TCP server

## Overview
This TCP server displays the connected clients in order and returns the messages sent by the clients.

## Build
You can build targeting native :
```
cargo build
```
And targeting wasi preview2 :
```
cargo build --target wasm32-wasip2
```
   
## Run
If you run native :
```
cargo run 127.0.0.1 8080
```
You can run Wasm Component (built for wasi preview2) on wasmtime :
```
wasmtime -S inherit-network=y target/wasm32-wasip2/debug/simple_tcp_server.wasm 127.0.0.1 8080
```

## Connect to server
Here we use netcat command :
```
> nc 127.0.0.1 8080
hello
hello
```
