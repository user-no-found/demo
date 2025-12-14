//!UDP 通信模块
//!
//!提供完整的 UDP 客户端/服务端功能，支持单播和广播通信。
//!
//!依赖：无（纯标准库）
//!
//!# 模块结构
//!- `config` - 配置项（端口、缓冲区大小等）
//!- `client` - UDP 客户端（单播、广播发送）
//!- `server` - UDP 服务端（数据报接收）
//!
//!# 快速开始
//!
//!## 服务端
//!```rust
//!mod udp;
//!
//!fn main() {
//!    let server = udp::UdpServer::bind(8081).unwrap();
//!    server.run(|data, addr, srv| {
//!        println!("[{}] 收到: {}", addr, String::from_utf8_lossy(&data));
//!        srv.send_string_to(&addr, "收到").unwrap();
//!        true
//!    });
//!}
//!```
//!
//!## 客户端
//!```rust
//!mod udp;
//!
//!fn main() {
//!    let client = udp::UdpClient::new().unwrap();
//!    client.send_string_to("127.0.0.1", 8081, "你好！").unwrap();
//!
//!    //广播
//!    let bc = udp::UdpClient::new_broadcast().unwrap();
//!    bc.broadcast_string(8081, "广播消息").unwrap();
//!}
//!```

pub mod config;
pub mod client;
pub mod server;

//========================================
//便捷重导出
//========================================

pub use client::UdpClient;
pub use server::UdpServer;
