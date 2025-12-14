//!UDP 服务端模块
//!
//!提供 UDP 服务端功能：端口监听、数据报接收、回复发送。

use super::config;

//========================================
//UDP 服务端结构
//========================================

///UDP 服务端
pub struct UdpServer {
    ///底层 UDP socket
    socket: std::net::UdpSocket,
}

impl UdpServer {
    //========================================
    //服务端启动方法
    //========================================

    ///绑定端口并启动监听
    pub fn bind(port: u16) -> std::io::Result<Self> {
        let addr = format!("{}:{}", config::SERVER_DEFAULT_ADDR, port);
        let socket = std::net::UdpSocket::bind(&addr)?;
        println!("UDP 服务端已启动，监听 {}", addr);
        Ok(Self { socket })
    }

    ///使用默认配置启动
    pub fn bind_default() -> std::io::Result<Self> {
        Self::bind(config::SERVER_DEFAULT_PORT)
    }

    ///绑定指定地址和端口
    pub fn bind_addr(addr: &str, port: u16) -> std::io::Result<Self> {
        let address = format!("{}:{}", addr, port);
        let socket = std::net::UdpSocket::bind(&address)?;
        println!("UDP 服务端已启动，监听 {}", address);
        Ok(Self { socket })
    }

    //========================================
    //数据接收方法
    //========================================

    ///接收一个数据报，返回数据和发送方地址
    pub fn recv(&self) -> std::io::Result<(Vec<u8>, std::net::SocketAddr)> {
        let mut buf = vec![0u8; config::RECV_BUFFER_SIZE];
        let (size, src_addr) = self.socket.recv_from(&mut buf)?;
        buf.truncate(size);
        Ok((buf, src_addr))
    }

    ///接收字符串消息，返回字符串和发送方地址
    pub fn recv_string(&self) -> std::io::Result<(String, std::net::SocketAddr)> {
        let (data, addr) = self.recv()?;
        let s = std::string::String::from_utf8_lossy(&data).to_string();
        Ok((s, addr))
    }

    ///阻塞式运行，为每个数据报调用回调函数
    ///
    ///参数：
    ///- handler: 数据报处理回调，参数为(数据, 发送方地址, 服务端引用)，返回 false 停止服务
    pub fn run<F>(&self, mut handler: F)
    where
        F: FnMut(Vec<u8>, std::net::SocketAddr, &Self) -> bool,
    {
        loop {
            match self.recv() {
                Ok((data, addr)) => {
                    if !handler(data, addr, self) {
                        println!("UDP 服务端停止");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("接收数据报失败: {}", e);
                }
            }
        }
    }

    //========================================
    //数据发送方法
    //========================================

    ///向指定地址发送数据
    pub fn send_to(&self, addr: &std::net::SocketAddr, data: &[u8]) -> std::io::Result<usize> {
        self.socket.send_to(data, addr)
    }

    ///向指定地址发送字符串
    pub fn send_string_to(&self, addr: &std::net::SocketAddr, content: &str) -> std::io::Result<usize> {
        self.send_to(addr, content.as_bytes())
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
}
