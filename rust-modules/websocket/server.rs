//!WebSocket 服务端模块
//!
//!提供 WebSocket 服务端功能：监听连接、处理消息、广播。
//!
//!依赖：tungstenite（使用时查询最新版本：https://crates.io/crates/tungstenite）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!tungstenite = "0.21"
//!```

use super::config;
use super::client::WsMessage;

//========================================
//客户端连接句柄
//========================================

///客户端连接
pub struct WsConnection {
    ///底层 WebSocket
    socket: tungstenite::WebSocket<std::net::TcpStream>,
    ///客户端地址
    pub addr: std::net::SocketAddr,
}

impl WsConnection {
    ///发送文本消息
    pub fn send_text(&mut self, message: &str) -> Result<(), String> {
        self.socket
            .send(tungstenite::Message::Text(message.to_string()))
            .map_err(|e| format!("发送失败: {}", e))
    }

    ///发送二进制消息
    pub fn send_binary(&mut self, data: &[u8]) -> Result<(), String> {
        self.socket
            .send(tungstenite::Message::Binary(data.to_vec()))
            .map_err(|e| format!("发送失败: {}", e))
    }

    ///接收消息
    pub fn recv(&mut self) -> Result<WsMessage, String> {
        loop {
            let msg = self.socket.read().map_err(|e| format!("接收失败: {}", e))?;
            match msg {
                tungstenite::Message::Text(s) => return Ok(WsMessage::Text(s)),
                tungstenite::Message::Binary(b) => return Ok(WsMessage::Binary(b)),
                tungstenite::Message::Ping(p) => {
                    let _ = self.socket.send(tungstenite::Message::Pong(p.clone()));
                    return Ok(WsMessage::Ping(p));
                }
                tungstenite::Message::Pong(p) => return Ok(WsMessage::Pong(p)),
                tungstenite::Message::Close(_) => return Ok(WsMessage::Close),
                tungstenite::Message::Frame(_) => continue,
            }
        }
    }

    ///关闭连接
    pub fn close(&mut self) -> Result<(), String> {
        self.socket
            .close(None)
            .map_err(|e| format!("关闭失败: {}", e))
    }
}

//========================================
//WebSocket 服务端结构
//========================================

///WebSocket 服务端
pub struct WsServer {
    ///TCP 监听器
    listener: std::net::TcpListener,
}

impl WsServer {
    ///绑定端口并启动监听
    pub fn bind(port: u16) -> std::io::Result<Self> {
        let addr = format!("{}:{}", config::SERVER_DEFAULT_ADDR, port);
        let listener = std::net::TcpListener::bind(&addr)?;
        println!("WebSocket 服务端已启动，监听 ws://{}", addr);
        Ok(Self { listener })
    }

    ///使用默认端口启动
    pub fn bind_default() -> std::io::Result<Self> {
        Self::bind(config::SERVER_DEFAULT_PORT)
    }

    ///绑定指定地址和端口
    pub fn bind_addr(addr: &str, port: u16) -> std::io::Result<Self> {
        let address = format!("{}:{}", addr, port);
        let listener = std::net::TcpListener::bind(&address)?;
        println!("WebSocket 服务端已启动，监听 ws://{}", address);
        Ok(Self { listener })
    }

    ///接受一个连接（阻塞）
    pub fn accept(&self) -> Result<WsConnection, String> {
        let (stream, addr) = self.listener.accept().map_err(|e| format!("接受连接失败: {}", e))?;
        let socket = tungstenite::accept(stream).map_err(|e| format!("WebSocket 握手失败: {}", e))?;
        println!("客户端连接: {}", addr);
        Ok(WsConnection { socket, addr })
    }

    ///运行服务端，为每个连接调用处理函数
    ///
    ///参数：
    ///- handler: 连接处理函数，返回 false 停止服务
    pub fn run<F>(&self, mut handler: F)
    where
        F: FnMut(WsConnection) -> bool,
    {
        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap_or_else(|_| {
                        std::net::SocketAddr::from(([0, 0, 0, 0], 0))
                    });
                    match tungstenite::accept(stream) {
                        Ok(socket) => {
                            println!("客户端连接: {}", addr);
                            let conn = WsConnection { socket, addr };
                            if !handler(conn) {
                                println!("服务端停止");
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("WebSocket 握手失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("接受连接失败: {}", e);
                }
            }
        }
    }

    ///多线程运行，为每个连接创建新线程
    pub fn run_threaded<F>(&self, handler: F)
    where
        F: Fn(WsConnection) + Send + Sync + 'static,
    {
        let handler = std::sync::Arc::new(handler);

        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap_or_else(|_| {
                        std::net::SocketAddr::from(([0, 0, 0, 0], 0))
                    });
                    let handler = std::sync::Arc::clone(&handler);

                    std::thread::spawn(move || {
                        match tungstenite::accept(stream) {
                            Ok(socket) => {
                                println!("客户端连接: {}", addr);
                                let conn = WsConnection { socket, addr };
                                handler(conn);
                            }
                            Err(e) => {
                                eprintln!("WebSocket 握手失败: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("接受连接失败: {}", e);
                }
            }
        }
    }

    ///获取本地绑定地址
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
}
