//!WebSocket 客户端模块
//!
//!提供 WebSocket 客户端功能：连接、发送、接收消息。
//!
//!依赖：tungstenite（使用时查询最新版本：https://crates.io/crates/tungstenite）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!tungstenite = "0.21"
//!url = "2"
//!```

use super::config;

//========================================
//WebSocket 消息类型
//========================================

///WebSocket 消息
#[derive(Debug, Clone)]
pub enum WsMessage {
    ///文本消息
    Text(String),
    ///二进制消息
    Binary(Vec<u8>),
    ///Ping
    Ping(Vec<u8>),
    ///Pong
    Pong(Vec<u8>),
    ///关闭
    Close,
}

impl WsMessage {
    ///是否为文本消息
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    ///是否为二进制消息
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    ///获取文本内容
    pub fn as_text(&self) -> Option<&str> {
        if let Self::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }

    ///获取二进制内容
    pub fn as_binary(&self) -> Option<&[u8]> {
        if let Self::Binary(b) = self {
            Some(b)
        } else {
            None
        }
    }
}

//========================================
//WebSocket 客户端结构
//========================================

///WebSocket 客户端
pub struct WsClient {
    ///底层 WebSocket 连接
    socket: tungstenite::WebSocket<std::net::TcpStream>,
}

impl WsClient {
    ///连接到 WebSocket 服务端
    ///
    ///参数：
    ///- url: WebSocket URL（如 ws://127.0.0.1:9001 或 wss://example.com）
    pub fn connect(url: &str) -> Result<Self, String> {
        let (socket, _response) = tungstenite::connect(url)
            .map_err(|e| format!("连接失败: {}", e))?;
        Ok(Self { socket })
    }

    ///连接到指定地址和端口
    pub fn connect_addr(addr: &str, port: u16) -> Result<Self, String> {
        let url = format!("ws://{}:{}", addr, port);
        Self::connect(&url)
    }

    ///连接到默认地址
    pub fn connect_default() -> Result<Self, String> {
        Self::connect_addr(config::SERVER_DEFAULT_ADDR, config::SERVER_DEFAULT_PORT)
    }

    //========================================
    //发送消息
    //========================================

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

    ///发送 Ping
    pub fn send_ping(&mut self, data: &[u8]) -> Result<(), String> {
        self.socket
            .send(tungstenite::Message::Ping(data.to_vec()))
            .map_err(|e| format!("发送失败: {}", e))
    }

    //========================================
    //接收消息
    //========================================

    ///接收消息（阻塞）
    pub fn recv(&mut self) -> Result<WsMessage, String> {
        loop {
            let msg = self.socket.read().map_err(|e| format!("接收失败: {}", e))?;
            match msg {
                tungstenite::Message::Text(s) => return Ok(WsMessage::Text(s)),
                tungstenite::Message::Binary(b) => return Ok(WsMessage::Binary(b)),
                tungstenite::Message::Ping(p) => {
                    //自动回复 Pong
                    let _ = self.socket.send(tungstenite::Message::Pong(p.clone()));
                    return Ok(WsMessage::Ping(p));
                }
                tungstenite::Message::Pong(p) => return Ok(WsMessage::Pong(p)),
                tungstenite::Message::Close(_) => return Ok(WsMessage::Close),
                tungstenite::Message::Frame(_) => continue,
            }
        }
    }

    ///尝试接收消息（非阻塞，需要设置超时）
    pub fn try_recv(&mut self) -> Option<WsMessage> {
        self.recv().ok()
    }

    //========================================
    //连接控制
    //========================================

    ///关闭连接
    pub fn close(&mut self) -> Result<(), String> {
        self.socket
            .close(None)
            .map_err(|e| format!("关闭失败: {}", e))
    }

    ///检查连接是否可写
    pub fn can_write(&self) -> bool {
        self.socket.can_write()
    }
}

//========================================
//便捷函数
//========================================

///快速连接并运行消息循环
pub fn connect_and_run<F>(url: &str, mut handler: F) -> Result<(), String>
where
    F: FnMut(&mut WsClient, WsMessage) -> bool,
{
    let mut client = WsClient::connect(url)?;
    loop {
        match client.recv() {
            Ok(msg) => {
                if matches!(msg, WsMessage::Close) || !handler(&mut client, msg) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("接收错误: {}", e);
                break;
            }
        }
    }
    let _ = client.close();
    Ok(())
}
