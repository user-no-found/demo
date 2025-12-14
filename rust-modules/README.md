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
