//!TCP 服务端模块
//!
//!提供 TCP 服务端功能：端口监听、多客户端连接处理、消息接收解析。

use super::config;
use super::protocol;

//========================================
//客户端连接句柄
//========================================

///客户端连接
pub struct ClientConnection {
    ///底层 TCP 连接
    stream: std::net::TcpStream,
    ///客户端地址
    addr: std::net::SocketAddr,
}

impl ClientConnection {
    ///获取客户端地址
    pub fn addr(&self) -> &std::net::SocketAddr {
        &self.addr
    }

    ///读取一条完整消息
    pub fn recv_message(&mut self) -> std::io::Result<protocol::Message> {
        use std::io::Read;

        let mut header_buf = [0u8; protocol::HEADER_SIZE];
        self.stream.read_exact(&mut header_buf)?;

        let header = protocol::MessageHeader::from_bytes(&header_buf)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "无效的消息头"))?;

        let mut data = vec![0u8; header.data_len as usize];
        self.stream.read_exact(&mut data)?;

        Ok(protocol::Message { header, data })
    }

    ///发送原始字节
    fn send_raw(&mut self, data: &[u8]) -> std::io::Result<()> {
        use std::io::Write;
        self.stream.write_all(data)?;
        self.stream.flush()
    }

    ///发送字符串消息
    pub fn send_string(&mut self, content: &str) -> std::io::Result<()> {
        let msg = protocol::Message::string(content);
        self.send_raw(&msg.to_bytes())
    }

    ///发送字节数据
    pub fn send_bytes(&mut self, data: Vec<u8>) -> std::io::Result<()> {
        let msg = protocol::Message::bytes(data);
        self.send_raw(&msg.to_bytes())
    }

    ///发送文件
    pub fn send_file(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let content = std::fs::read(path)?;
        let msg = protocol::Message::file(filename, content);
        self.send_raw(&msg.to_bytes())
    }

    ///发送图片
    pub fn send_image(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let content = std::fs::read(path)?;
        let msg = protocol::Message::image(filename, content);
        self.send_raw(&msg.to_bytes())
    }

    ///发送视频帧
    pub fn send_video_frame(&mut self, frame_data: Vec<u8>) -> std::io::Result<()> {
        let msg = protocol::Message::video_frame(frame_data);
        self.send_raw(&msg.to_bytes())
    }

    ///获取底层流的可变引用
    pub fn stream_mut(&mut self) -> &mut std::net::TcpStream {
        &mut self.stream
    }

    ///获取底层流的只读引用
    pub fn stream(&self) -> &std::net::TcpStream {
        &self.stream
    }
}

//========================================
//TCP 服务端结构
//========================================

///TCP 服务端
pub struct TcpServer {
    ///底层监听器
    listener: std::net::TcpListener,
}

impl TcpServer {
    //========================================
    //服务端启动方法
    //========================================

    ///绑定端口并启动监听
    pub fn bind(port: u16) -> std::io::Result<Self> {
        let addr = format!("{}:{}", config::SERVER_DEFAULT_ADDR, port);
        let listener = std::net::TcpListener::bind(&addr)?;
        println!("服务端已启动，监听 {}", addr);
        Ok(Self { listener })
    }

    ///使用默认配置启动
    pub fn bind_default() -> std::io::Result<Self> {
        Self::bind(config::SERVER_DEFAULT_PORT)
    }

    ///绑定指定地址和端口
    pub fn bind_addr(addr: &str, port: u16) -> std::io::Result<Self> {
        let address = format!("{}:{}", addr, port);
        let listener = std::net::TcpListener::bind(&address)?;
        println!("服务端已启动，监听 {}", address);
        Ok(Self { listener })
    }

    //========================================
    //客户端连接处理
    //========================================

    ///接受一个客户端连接
    pub fn accept(&self) -> std::io::Result<ClientConnection> {
        let (stream, addr) = self.listener.accept()?;
        println!("客户端连接: {}", addr);
        Ok(ClientConnection { stream, addr })
    }

    ///阻塞式运行，为每个连接调用回调函数
    ///
    ///参数：
    ///- on_client: 客户端连接回调，返回 false 表示停止服务器
    pub fn run<F>(&self, mut on_client: F)
    where
        F: FnMut(ClientConnection) -> bool,
    {
        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap_or_else(|_| {
                        std::net::SocketAddr::from(([0, 0, 0, 0], 0))
                    });
                    println!("客户端连接: {}", addr);
                    let conn = ClientConnection { stream, addr };
                    if !on_client(conn) {
                        println!("服务端停止");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("接受连接失败: {}", e);
                }
            }
        }
    }

    ///多线程运行，为每个连接创建新线程
    ///
    ///参数：
    ///- handler: 客户端处理函数（必须是 Fn + Send + Sync + 'static）
    pub fn run_threaded<F>(&self, handler: F)
    where
        F: Fn(ClientConnection) + Send + Sync + 'static,
    {
        let handler = std::sync::Arc::new(handler);

        for stream_result in self.listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let addr = stream.peer_addr().unwrap_or_else(|_| {
                        std::net::SocketAddr::from(([0, 0, 0, 0], 0))
                    });
                    println!("客户端连接: {}", addr);
                    let conn = ClientConnection { stream, addr };
                    let handler = std::sync::Arc::clone(&handler);

                    std::thread::spawn(move || {
                        handler(conn);
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
