//!HTTP 配置模块
//!
//!统一管理 HTTP 通信相关的所有配置项。
//!修改此文件中的常量即可自定义 HTTP 行为。

//========================================
//客户端配置
//========================================

///请求超时时间（秒）
pub const REQUEST_TIMEOUT_SECS: u64 = 30;

///连接超时时间（秒）
pub const CONNECT_TIMEOUT_SECS: u64 = 10;

///默认 User-Agent
pub const DEFAULT_USER_AGENT: &str = "rust-http-client/1.0";

///默认 Content-Type
pub const DEFAULT_CONTENT_TYPE: &str = "application/json";

//========================================
//服务端配置
//========================================

///服务端默认监听端口
pub const SERVER_DEFAULT_PORT: u16 = 8000;

///服务端默认绑定地址
pub const SERVER_DEFAULT_ADDR: &str = "0.0.0.0";

///工作线程数（0 表示使用 CPU 核心数）
pub const WORKER_THREADS: usize = 4;

//========================================
//响应配置
//========================================

///最大响应体大小（字节）
pub const MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024; //10MB
