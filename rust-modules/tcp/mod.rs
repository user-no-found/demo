//!TCP 通信模块
//!
//!提供完整的 TCP 客户端/服务端功能，支持多种连接模式和消息类型。
//!
//!依赖：无（纯标准库）
//!
//!# 模块结构
//!- `config` - 配置项（端口、超时、缓冲区等）
//!- `protocol` - 消息协议定义（消息类型、序列化）
//!- `client` - TCP 客户端（三种连接模式）
//!- `server` - TCP 服务端（单线程/多线程）
//!
//!# 快速开始
//!
//!## 服务端
//!```rust
//!mod tcp;
//!
//!fn main() {
//!    let server = tcp::server::TcpServer::bind(8080).unwrap();
//!    server.run(|mut conn| {
//!        if let Ok(msg) = conn.recv_message() {
//!            let content = tcp::protocol::parse_message_content(&msg);
//!            println!("收到: {:?}", content);
//!        }
//!        true
//!    });
//!}
//!```
//!
//!## 客户端
//!```rust
//!mod tcp;
//!
//!fn main() {
//!    //单次连接
//!    let mut client = tcp::client::TcpClient::connect_once("127.0.0.1", 8080).unwrap();
//!    client.send_string("你好！").unwrap();
//!
//!    //无限重连
//!    tcp::client::TcpClient::connect_forever("127.0.0.1", 8080, |c| {
//!        c.send_string("心跳").unwrap();
//!        true
//!    });
//!}
//!```

pub mod config;
pub mod protocol;
pub mod client;
pub mod server;

//========================================
//便捷重导出
//========================================

pub use client::TcpClient;
pub use server::{TcpServer, ClientConnection};
pub use protocol::{Message, MessageType, ParsedContent, parse_message_content};
