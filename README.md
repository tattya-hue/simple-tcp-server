## Simple Tcp server

### 概要



### コンパイル
   
   ```
   cargo build --target wasm32-wasip2
   ```
   
### Wasmtime上での動かし方
   
   ```
   wasmtime -S inherit-work=y target/wasm32-wasip2/debug/simple_tcp_server.wasm
   ```

### 確認方法
#### Linux
  ```
  nc 127.0.0.1 8080
  ```
