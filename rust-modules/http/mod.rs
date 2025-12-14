//!HTTP 通信模块
//!
//!提供 HTTP 客户端和服务端功能。
//!
//!依赖：
//!- ureq（使用时查询最新版本：https://crates.io/crates/ureq）
//!- tiny_http（使用时查询最新版本：https://crates.io/crates/tiny_http）
//!- serde + serde_json（使用时查询最新版本）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!ureq = { version = "2", features = ["json"] }
//!tiny_http = "0.12"
//!serde = { version = "1", features = ["derive"] }
//!serde_json = "1"
//!```
//!
//!# 模块结构
//!- `config` - 配置项（超时、端口等）
//!- `client` - HTTP 客户端
//!- `server` - HTTP 服务端
//!
//!# 快速开始
//!
//!## 客户端
//!```rust
//!mod http;
//!
//!fn main() {
//!    //简单 GET
//!    let resp = http::client::get("https://httpbin.org/get").unwrap();
//!    println!("状态: {}", resp.status);
//!
//!    //带 Header 的请求
//!    let client = http::HttpClient::new()
//!        .with_bearer_token("your-token");
//!    let resp = client.get("https://api.example.com/data").unwrap();
//!}
//!```
//!
//!## 服务端
//!```rust
//!mod http;
//!
//!fn main() {
//!    http::HttpServer::bind(8000)
//!        .get("/", |req| {
//!            req.respond_text(200, "Hello World!");
//!        })
//!        .get("/api/data", |req| {
//!            req.respond_json(200, &serde_json::json!({"status": "ok"}));
//!        })
//!        .post("/api/echo", |req| {
//!            req.respond_text(200, &req.body);
//!        })
//!        .run();
//!}
//!```

pub mod config;
pub mod client;
pub mod server;

//========================================
//便捷重导出
//========================================

pub use client::{HttpClient, Response, get, post_json};
pub use server::{HttpServer, Request};
