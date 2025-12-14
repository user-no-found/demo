//!WebSocket 通信模块
//!
//!提供 WebSocket 客户端和服务端功能，支持双向实时通信。
//!
//!依赖：tungstenite（使用时查询最新版本：https://crates.io/crates/tungstenite）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!tungstenite = "0.21"
//!url = "2"
//!```
//!
//!# 模块结构
//!- `config` - 配置项（超时、端口等）
//!- `client` - WebSocket 客户端
//!- `server` - WebSocket 服务端
//!
//!# 快速开始
//!
//!## 客户端
//!```rust
//!mod websocket;
//!
//!fn main() {
//!    let mut client = websocket::WsClient::connect("ws://127.0.0.1:9001").unwrap();
//!    client.send_text("你好！").unwrap();
//!
//!    loop {
//!        match client.recv() {
//!            Ok(msg) => println!("收到: {:?}", msg),
//!            Err(_) => break,
//!        }
//!    }
//!}
//!```
//!
//!## 服务端
//!```rust
//!mod websocket;
//!
//!fn main() {
//!    let server = websocket::WsServer::bind(9001).unwrap();
//!    server.run_threaded(|mut conn| {
//!        loop {
//!            match conn.recv() {
//!                Ok(websocket::WsMessage::Text(s)) => {
//!                    println!("[{}] {}", conn.addr, s);
//!                    conn.send_text(&format!("回复: {}", s)).unwrap();
//!                }
//!                Ok(websocket::WsMessage::Close) | Err(_) => break,
//!                _ => {}
//!            }
//!        }
//!    });
//!}
//!```

pub mod config;
pub mod client;
pub mod server;

//========================================
//便捷重导出
//========================================

pub use client::{WsClient, WsMessage, connect_and_run};
pub use server::{WsServer, WsConnection};
