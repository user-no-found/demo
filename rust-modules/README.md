# Rust 可复用模块

将模块文件复制到项目 `src/` 目录，通过 `mod xxx;` 引入使用。

## 模块列表

| 文件/目录 | 用途 | 依赖 |
|------|------|------|
| `ctrl_c.rs` | Ctrl+C 停止程序 | [ctrlc](https://crates.io/crates/ctrlc) |
| `cmd_config.rs` | 命令行参数配置 | [clap](https://crates.io/crates/clap) (需 derive feature) |
| `log.rs` | 日志配置（终端+文件） | [simplelog](https://crates.io/crates/simplelog) + [log](https://crates.io/crates/log) |
| `tcp/` | TCP 通信模块（客户端+服务端） | 无（纯标准库） |
| `udp/` | UDP 通信模块（单播+广播） | 无（纯标准库） |
| `http/` | HTTP 通信模块（客户端+服务端） | [ureq](https://crates.io/crates/ureq) + [tiny_http](https://crates.io/crates/tiny_http) |
| `websocket/` | WebSocket 双向通信 | [tungstenite](https://crates.io/crates/tungstenite) |
| `json_config/` | JSON 配置文件读写 | [serde_json](https://crates.io/crates/serde_json) |

> 注：使用前请到 crates.io 查询依赖的最新版本

## 使用示例

### ctrl_c.rs
```rust
mod ctrl_c;

fn main() {
    //程序逻辑...
    ctrl_c::wait_for_exit();
}
```

### cmd_config.rs
```rust
mod cmd_config;

fn main() {
    let cfg = cmd_config::Config::parse();

    if cfg.is_debug() {
        cfg.debug_println("调试模式已启用");
    }

    //程序逻辑...
}
```

运行：`./program -d` 或 `./program --debug` 启用调试模式

### log.rs
```rust
mod log;

fn main() {
    //初始化日志（终端+文件）
    log::init();

    //使用日志宏
    log::info!("程序启动");
    log::debug!("调试信息");
    log::warn!("警告信息");
    log::error!("错误信息");
}
```

日志文件路径在 log.rs 顶部 `LOG_FILE_PATH` 常量中配置

### tcp/ （TCP 通信模块）

复制整个 `tcp/` 目录到项目 `src/` 目录。

**目录结构：**
```
tcp/
├── mod.rs       # 模块入口
├── config.rs    # 配置项（端口、超时等）
├── protocol.rs  # 消息协议定义
├── client.rs    # 客户端
└── server.rs    # 服务端
```

**服务端示例：**
```rust
mod tcp;

fn main() {
    //使用默认端口（config.rs 中配置）
    let server = tcp::TcpServer::bind_default().unwrap();

    //或指定端口
    //let server = tcp::TcpServer::bind(9000).unwrap();

    //单线程处理
    server.run(|mut conn| {
        println!("客户端: {}", conn.addr());

        loop {
            match conn.recv_message() {
                Ok(msg) => {
                    let content = tcp::parse_message_content(&msg);
                    println!("收到: {:?}", content);
                    conn.send_string("收到").unwrap();
                }
                Err(_) => break,
            }
        }
        true //继续接受新连接
    });

    //多线程处理（高并发场景）
    //server.run_threaded(|mut conn| {
    //    loop {
    //        match conn.recv_message() {
    //            Ok(msg) => println!("{:?}", tcp::parse_message_content(&msg)),
    //            Err(_) => break,
    //        }
    //    }
    //});
}
```

**客户端示例：**
```rust
mod tcp;

fn main() {
    //方式1：单次连接
    if let Ok(mut client) = tcp::TcpClient::connect_once("127.0.0.1", 8080) {
        client.send_string("你好！").unwrap();
    }

    //方式2：无限重连（永不退出，适合长连接场景）
    tcp::TcpClient::connect_forever("127.0.0.1", 8080, |client| {
        client.send_string("心跳").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        true //返回 false 可主动断开
    });

    //方式3：重试直到成功（适合启动时必须连接的场景）
    let mut client = tcp::TcpClient::connect_until_success("127.0.0.1", 8080);
    client.send_file(std::path::Path::new("test.txt")).unwrap();
}
```

**配置修改（tcp/config.rs）：**
```rust
//修改默认端口
pub const SERVER_DEFAULT_PORT: u16 = 9000;

//修改重连间隔
pub const RECONNECT_INITIAL_MS: u64 = 500;
pub const RECONNECT_MAX_MS: u64 = 60000;
```

**支持的消息类型：**
- `send_string()` - 字符串消息
- `send_bytes()` - 原始字节数据
- `send_file()` - 文件传输
- `send_image()` - 图片传输
- `send_video_frame()` - 视频帧
- `send_file_chunked()` - 大文件分块传输

### udp/ （UDP 通信模块）

复制整个 `udp/` 目录到项目 `src/` 目录。

**目录结构：**
```
udp/
├── mod.rs       # 模块入口
├── config.rs    # 配置项（端口、缓冲区等）
├── client.rs    # 客户端
└── server.rs    # 服务端
```

**服务端示例：**
```rust
mod udp;

fn main() {
    let server = udp::UdpServer::bind(8081).unwrap();

    server.run(|data, addr, srv| {
        println!("[{}] 收到: {}", addr, String::from_utf8_lossy(&data));
        //回复客户端
        srv.send_string_to(&addr, "收到").unwrap();
        true //继续监听
    });
}
```

**客户端示例：**
```rust
mod udp;

fn main() {
    //单播发送
    let client = udp::UdpClient::new().unwrap();
    client.send_string_to("127.0.0.1", 8081, "你好！").unwrap();

    //广播发送（需启用广播）
    let bc = udp::UdpClient::new_broadcast().unwrap();
    bc.broadcast_string(8081, "广播消息").unwrap();

    //伪连接模式（设置默认目标后简化发送）
    let client = udp::UdpClient::new().unwrap();
    client.connect("127.0.0.1", 8081).unwrap();
    client.send_string_connected("简化发送").unwrap();
}
```

**配置修改（udp/config.rs）：**
```rust
//修改默认端口
pub const SERVER_DEFAULT_PORT: u16 = 9000;

//修改缓冲区大小
pub const RECV_BUFFER_SIZE: usize = 4096;
```

**支持的方法：**
- `send_to()` / `send_string_to()` - 单播发送
- `broadcast()` / `broadcast_string()` - 广播发送
- `recv()` / `recv_string()` - 接收数据
- `connect()` + `send_connected()` - 伪连接模式

### http/ （HTTP 通信模块）

复制整个 `http/` 目录到项目 `src/` 目录。

**目录结构：**
```
http/
├── mod.rs       # 模块入口
├── config.rs    # 配置项（超时、端口等）
├── client.rs    # HTTP 客户端
└── server.rs    # HTTP 服务端
```

**Cargo.toml 依赖：**
```toml
[dependencies]
ureq = { version = "2", features = ["json"] }
tiny_http = "0.12"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**客户端示例：**
```rust
mod http;

fn main() {
    //简单 GET 请求
    let resp = http::client::get("https://httpbin.org/get").unwrap();
    println!("状态码: {}", resp.status);
    println!("响应: {}", resp.text());

    //带 Header 的请求
    let client = http::HttpClient::new()
        .with_header("X-Custom", "value")
        .with_bearer_token("your-token");
    let resp = client.get("https://api.example.com/data").unwrap();

    //POST JSON
    let data = serde_json::json!({"name": "test"});
    let resp = http::client::post_json("https://httpbin.org/post", &data).unwrap();
}
```

**服务端示例：**
```rust
mod http;

fn main() {
    http::HttpServer::bind(8000)
        .get("/", |req| {
            req.respond_text(200, "Hello World!");
        })
        .get("/api/status", |req| {
            req.respond_json(200, &serde_json::json!({"status": "ok"}));
        })
        .post("/api/echo", |req| {
            println!("收到: {}", req.body);
            req.respond_text(200, &req.body);
        })
        .run();
}
```

**支持的方法：**
- 客户端：`get()`, `post_json()`, `post_form()`, `put_json()`, `delete()`
- 服务端：`.get()`, `.post()`, `.put()`, `.delete()` 路由注册
- 响应：`respond_text()`, `respond_json()`, `respond_html()`

### websocket/ （WebSocket 通信模块）

复制整个 `websocket/` 目录到项目 `src/` 目录。

**Cargo.toml 依赖：**
```toml
[dependencies]
tungstenite = "0.21"
```

**客户端示例：**
```rust
mod websocket;

fn main() {
    let mut client = websocket::WsClient::connect("ws://127.0.0.1:9001").unwrap();

    //发送消息
    client.send_text("你好！").unwrap();

    //接收消息
    loop {
        match client.recv() {
            Ok(websocket::WsMessage::Text(s)) => println!("收到: {}", s),
            Ok(websocket::WsMessage::Close) => break,
            Err(_) => break,
            _ => {}
        }
    }
}
```

**服务端示例：**
```rust
mod websocket;

fn main() {
    let server = websocket::WsServer::bind(9001).unwrap();

    //多线程处理连接
    server.run_threaded(|mut conn| {
        println!("客户端连接: {}", conn.addr);

        loop {
            match conn.recv() {
                Ok(websocket::WsMessage::Text(s)) => {
                    println!("收到: {}", s);
                    conn.send_text(&format!("回复: {}", s)).unwrap();
                }
                Ok(websocket::WsMessage::Close) | Err(_) => break,
                _ => {}
            }
        }
    });
}
```

**支持的方法：**
- 客户端：`connect()`, `send_text()`, `send_binary()`, `recv()`
- 服务端：`bind()`, `run()`, `run_threaded()`
- 消息类型：`Text`, `Binary`, `Ping`, `Pong`, `Close`

### json_config/ （JSON 配置模块）

复制整个 `json_config/` 目录到项目 `src/` 目录。

**Cargo.toml 依赖：**
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**读取配置示例：**
```rust
mod json_config;

fn main() {
    //读取为动态 JSON
    let config = json_config::load("config.json").unwrap();
    let name = config.get_str("name").unwrap_or("default");
    let port = config.get_i64("server.port").unwrap_or(8080);

    //读取为结构体
    #[derive(serde::Deserialize)]
    struct Config { name: String, port: u16 }
    let config: Config = json_config::load_as("config.json").unwrap();
}
```

**保存配置示例：**
```rust
mod json_config;

#[derive(serde::Serialize)]
struct Config { name: String, port: u16 }

fn main() {
    //保存结构体
    let config = Config { name: "app".to_string(), port: 8080 };
    json_config::save_pretty("config.json", &config).unwrap();

    //动态构建并保存
    let mut config = json_config::new();
    config.set("name", "myapp").unwrap();
    config.set("server.port", 9000).unwrap();
    config.save_pretty("config.json").unwrap();
}
```

**支持的方法：**
- 读取：`load()`, `load_as::<T>()`, `from_str()`
- 保存：`save()`, `save_pretty()`
- 操作：`get()`, `get_str()`, `get_i64()`, `set()`, `remove()`
