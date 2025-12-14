//!TCP 配置模块
//!
//!统一管理 TCP 通信相关的所有配置项。
//!修改此文件中的常量即可自定义 TCP 行为。

//========================================
//服务端配置
//========================================

///服务端默认监听端口
pub const SERVER_DEFAULT_PORT: u16 = 8080;

///服务端默认绑定地址
pub const SERVER_DEFAULT_ADDR: &str = "0.0.0.0";

///接收缓冲区大小（字节）
pub const RECV_BUFFER_SIZE: usize = 65536;

//========================================
//客户端配置
//========================================

///客户端默认连接端口
pub const CLIENT_DEFAULT_PORT: u16 = 8080;

///客户端默认连接地址
pub const CLIENT_DEFAULT_ADDR: &str = "127.0.0.1";

//========================================
//重连配置
//========================================

///初始重连间隔（毫秒）
pub const RECONNECT_INITIAL_MS: u64 = 1000;

///最大重连间隔（毫秒）
pub const RECONNECT_MAX_MS: u64 = 30000;

///重连间隔倍数（指数退避）
pub const RECONNECT_MULTIPLIER: f64 = 1.5;

//========================================
//传输配置
//========================================

///文件分块大小（字节）
pub const CHUNK_SIZE: usize = 8192;

///连接超时时间（秒）
pub const CONNECT_TIMEOUT_SECS: u64 = 10;

///读取超时时间（秒），0 表示无超时
pub const READ_TIMEOUT_SECS: u64 = 0;

///写入超时时间（秒），0 表示无超时
pub const WRITE_TIMEOUT_SECS: u64 = 0;
