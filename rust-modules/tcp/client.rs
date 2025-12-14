//!TCP 客户端模块
//!
//!提供三种连接模式：单次连接、无限重连、重试直到成功。
//!支持发送多种类型消息：字符串、字节、文件、图片、视频流。

use super::config;
use super::protocol;

//========================================
//TCP 客户端结构
//========================================

///TCP 客户端
pub struct TcpClient {
    ///底层 TCP 连接
    stream: std::net::TcpStream,
}

impl TcpClient {
    //========================================
    //连接模式1：单次连接
    //========================================

    ///单次连接，失败返回错误
    pub fn connect_once(addr: &str, port: u16) -> std::io::Result<Self> {
        let address = format!("{}:{}", addr, port);
        let stream = std::net::TcpStream::connect(&address)?;
        Self::apply_timeouts(&stream)?;
        Ok(Self { stream })
    }

    ///使用默认配置单次连接
    pub fn connect_once_default() -> std::io::Result<Self> {
        Self::connect_once(config::CLIENT_DEFAULT_ADDR, config::CLIENT_DEFAULT_PORT)
    }

    //========================================
    //连接模式2：无限重连（永不退出）
    //========================================

    ///无限重连模式，连接断开后自动重连
    ///
    ///参数：
    ///- addr: 服务器地址
    ///- port: 服务器端口
    ///- on_connected: 连接成功后的回调函数，返回 false 表示主动断开
    pub fn connect_forever<F>(addr: &str, port: u16, mut on_connected: F)
    where
        F: FnMut(&mut Self) -> bool,
    {
        let address = format!("{}:{}", addr, port);
        let mut delay_ms = config::RECONNECT_INITIAL_MS;

        loop {
            match std::net::TcpStream::connect(&address) {
                Ok(stream) => {
                    println!("已连接到 {}", address);
                    delay_ms = config::RECONNECT_INITIAL_MS;

                    if let Err(e) = Self::apply_timeouts(&stream) {
                        eprintln!("设置超时失败: {}", e);
                    }

                    let mut client = Self { stream };
                    if !on_connected(&mut client) {
                        println!("主动断开连接");
                        break;
                    }
                    println!("连接断开，准备重连...");
                }
                Err(e) => {
                    eprintln!("连接失败: {}，{}ms 后重试", e, delay_ms);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            delay_ms = ((delay_ms as f64 * config::RECONNECT_MULTIPLIER) as u64)
                .min(config::RECONNECT_MAX_MS);
        }
    }

    ///使用默认配置无限重连
    pub fn connect_forever_default<F>(on_connected: F)
    where
        F: FnMut(&mut Self) -> bool,
    {
        Self::connect_forever(config::CLIENT_DEFAULT_ADDR, config::CLIENT_DEFAULT_PORT, on_connected)
    }

    //========================================
    //连接模式3：重试直到成功
    //========================================

    ///重试连接直到成功
    pub fn connect_until_success(addr: &str, port: u16) -> Self {
        let address = format!("{}:{}", addr, port);
        let mut delay_ms = config::RECONNECT_INITIAL_MS;

        loop {
            match std::net::TcpStream::connect(&address) {
                Ok(stream) => {
                    println!("已连接到 {}", address);
                    if let Err(e) = Self::apply_timeouts(&stream) {
                        eprintln!("设置超时失败: {}", e);
                    }
                    return Self { stream };
                }
                Err(e) => {
                    eprintln!("连接失败: {}，{}ms 后重试", e, delay_ms);
                    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                    delay_ms = ((delay_ms as f64 * config::RECONNECT_MULTIPLIER) as u64)
                        .min(config::RECONNECT_MAX_MS);
                }
            }
        }
    }

    ///使用默认配置重试连接
    pub fn connect_until_success_default() -> Self {
        Self::connect_until_success(config::CLIENT_DEFAULT_ADDR, config::CLIENT_DEFAULT_PORT)
    }

    //========================================
    //超时设置
    //========================================

    ///应用超时配置
    fn apply_timeouts(stream: &std::net::TcpStream) -> std::io::Result<()> {
        if config::READ_TIMEOUT_SECS > 0 {
            stream.set_read_timeout(Some(std::time::Duration::from_secs(config::READ_TIMEOUT_SECS)))?;
        }
        if config::WRITE_TIMEOUT_SECS > 0 {
            stream.set_write_timeout(Some(std::time::Duration::from_secs(config::WRITE_TIMEOUT_SECS)))?;
        }
        Ok(())
    }

    //========================================
    //消息发送方法
    //========================================

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

    ///发送大文件（分块传输）
    pub fn send_file_chunked(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Read;

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let mut file = std::fs::File::open(path)?;
        let file_size = file.metadata()?.len();

        //先发送文件元信息
        let meta = protocol::FileMeta::new(filename);
        let meta_bytes = meta.to_bytes();

        //构造消息头
        let header = protocol::MessageHeader::new(
            protocol::MessageType::File,
            meta_bytes.len() as u64 + file_size,
        );
        self.send_raw(&header.to_bytes())?;
        self.send_raw(&meta_bytes)?;

        //分块发送文件内容
        let mut buffer = vec![0u8; config::CHUNK_SIZE];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            self.send_raw(&buffer[..bytes_read])?;
        }

        Ok(())
    }

    //========================================
    //消息接收方法
    //========================================

    ///接收一条完整消息
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

    //========================================
    //底层访问
    //========================================

    ///获取底层流的可变引用
    pub fn stream_mut(&mut self) -> &mut std::net::TcpStream {
        &mut self.stream
    }

    ///获取底层流的只读引用
    pub fn stream(&self) -> &std::net::TcpStream {
        &self.stream
    }
}
