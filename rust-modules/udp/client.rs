//!UDP 客户端模块
//!
//!提供 UDP 客户端功能：单播发送、广播发送、数据接收。

use super::config;

//========================================
//UDP 客户端结构
//========================================

///UDP 客户端
pub struct UdpClient {
    ///底层 UDP socket
    socket: std::net::UdpSocket,
}

impl UdpClient {
    //========================================
    //客户端创建方法
    //========================================

    ///创建客户端（绑定系统自动分配的端口）
    pub fn new() -> std::io::Result<Self> {
        let socket = std::net::UdpSocket::bind(config::CLIENT_BIND_ADDR)?;
        Ok(Self { socket })
    }

    ///创建客户端并绑定指定端口
    pub fn bind(port: u16) -> std::io::Result<Self> {
        let addr = format!("0.0.0.0:{}", port);
        let socket = std::net::UdpSocket::bind(&addr)?;
        Ok(Self { socket })
    }

    ///创建支持广播的客户端
    pub fn new_broadcast() -> std::io::Result<Self> {
        let socket = std::net::UdpSocket::bind(config::CLIENT_BIND_ADDR)?;
        socket.set_broadcast(true)?;
        Ok(Self { socket })
    }

    //========================================
    //单播发送方法
    //========================================

    ///向指定地址发送数据
    pub fn send_to(&self, addr: &str, port: u16, data: &[u8]) -> std::io::Result<usize> {
        let target = format!("{}:{}", addr, port);
        self.socket.send_to(data, &target)
    }

    ///向指定地址发送字符串
    pub fn send_string_to(&self, addr: &str, port: u16, content: &str) -> std::io::Result<usize> {
        self.send_to(addr, port, content.as_bytes())
    }

    ///使用默认配置发送数据
    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.send_to(config::CLIENT_DEFAULT_ADDR, config::CLIENT_DEFAULT_PORT, data)
    }

    ///使用默认配置发送字符串
    pub fn send_string(&self, content: &str) -> std::io::Result<usize> {
        self.send(content.as_bytes())
    }

    //========================================
    //广播发送方法
    //========================================

    ///广播发送数据
    pub fn broadcast(&self, port: u16, data: &[u8]) -> std::io::Result<usize> {
        let target = format!("{}:{}", config::BROADCAST_ADDR, port);
        self.socket.send_to(data, &target)
    }

    ///广播发送字符串
    pub fn broadcast_string(&self, port: u16, content: &str) -> std::io::Result<usize> {
        self.broadcast(port, content.as_bytes())
    }

    //========================================
    //数据接收方法
    //========================================

    ///接收数据报，返回数据和发送方地址
    pub fn recv(&self) -> std::io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = vec![0u8; config::RECV_BUFFER_SIZE];
        let (size, src_addr) = self.socket.recv_from(&mut buf)?;
        buf.truncate(size);
        Ok((buf, src_addr))
    }

    ///接收字符串消息
    pub fn recv_string(&self) -> std::io::Result<(String, std::net::SocketAddr)> {
        let (data, addr) = self.recv()?;
        let s = std::string::String::from_utf8_lossy(&data).to_string();
        Ok((s, addr))
    }

    //========================================
    //连接模式（伪连接）
    //========================================

    ///连接到指定地址（设置默认目标，之后可用 send_connected）
    pub fn connect(&self, addr: &str, port: u16) -> std::io::Result<()> {
        let target = format!("{}:{}", addr, port);
        self.socket.connect(&target)
    }

    ///连接到默认地址
    pub fn connect_default(&self) -> std::io::Result<()> {
        self.connect(config::CLIENT_DEFAULT_ADDR, config::CLIENT_DEFAULT_PORT)
    }

    ///向已连接的目标发送数据
    pub fn send_connected(&self, data: &[u8]) -> std::io::Result<usize> {
        self.socket.send(data)
    }

    ///向已连接的目标发送字符串
    pub fn send_string_connected(&self, content: &str) -> std::io::Result<usize> {
        self.send_connected(content.as_bytes())
    }

    //========================================
    //底层访问
    //========================================

    ///获取底层 socket 的只读引用
    pub fn socket(&self) -> &std::net::UdpSocket {
        &self.socket
    }

    ///获取本地绑定地址
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.socket.local_addr()
    }

    ///设置是否启用广播
    pub fn set_broadcast(&self, enable: bool) -> std::io::Result<()> {
        self.socket.set_broadcast(enable)
    }
}

impl Default for UdpClient {
    fn default() -> Self {
        Self::new().expect("创建 UDP 客户端失败")
    }
}
