//!串口通信模块
//!
//!提供跨平台的串口通信功能。
//!
//!依赖：serial2（使用时查询最新版本：https://crates.io/crates/serial2）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!serial2 = "0.2"  # https://crates.io/crates/serial2
//!```
//!
//!# 快速开始
//!
//!## 列出可用串口
//!```rust
//!mod serial;
//!
//!fn main() {
//!    for port in serial::list_ports().unwrap() {
//!        println!("串口: {}", port);
//!    }
//!}
//!```
//!
//!## 打开串口并通信
//!```rust
//!mod serial;
//!
//!fn main() {
//!    //打开串口，设置波特率 115200
//!    let mut port = serial::SerialPort::open("/dev/ttyUSB0", 115200).unwrap();
//!
//!    //发送数据
//!    port.write_str("AT\r\n").unwrap();
//!
//!    //接收数据
//!    let response = port.read_line().unwrap();
//!    println!("收到: {}", response);
//!}
//!```
//!
//!## 使用 Builder 模式
//!```rust
//!mod serial;
//!
//!fn main() {
//!    let port = serial::SerialPort::builder()
//!        .port("/dev/ttyUSB0")
//!        .baud_rate(9600)
//!        .data_bits(serial::DataBits::Eight)
//!        .stop_bits(serial::StopBits::One)
//!        .parity(serial::Parity::None)
//!        .timeout(std::time::Duration::from_secs(1))
//!        .open()
//!        .unwrap();
//!}
//!```

use serial2::SerialPort as Serial2Port;

//========================================
//配置常量
//========================================

///默认读取超时（毫秒）
pub const DEFAULT_TIMEOUT_MS: u64 = 1000;

///默认读取缓冲区大小
pub const DEFAULT_BUFFER_SIZE: usize = 1024;

//========================================
//配置枚举
//========================================

///数据位
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

impl From<DataBits> for serial2::CharSize {
    fn from(bits: DataBits) -> Self {
        match bits {
            DataBits::Five => serial2::CharSize::Bits5,
            DataBits::Six => serial2::CharSize::Bits6,
            DataBits::Seven => serial2::CharSize::Bits7,
            DataBits::Eight => serial2::CharSize::Bits8,
        }
    }
}

///停止位
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StopBits {
    One,
    Two,
}

impl From<StopBits> for serial2::StopBits {
    fn from(bits: StopBits) -> Self {
        match bits {
            StopBits::One => serial2::StopBits::One,
            StopBits::Two => serial2::StopBits::Two,
        }
    }
}

///校验位
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parity {
    ///无校验
    None,
    ///奇校验
    Odd,
    ///偶校验
    Even,
}

impl From<Parity> for serial2::Parity {
    fn from(parity: Parity) -> Self {
        match parity {
            Parity::None => serial2::Parity::None,
            Parity::Odd => serial2::Parity::Odd,
            Parity::Even => serial2::Parity::Even,
        }
    }
}

//========================================
//串口信息
//========================================

///串口信息
#[derive(Debug, Clone)]
pub struct PortInfo {
    ///串口名称（如 /dev/ttyUSB0 或 COM1）
    pub name: String,
}

impl std::fmt::Display for PortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

//========================================
//便捷函数
//========================================

///列出所有可用串口
pub fn list_ports() -> Result<Vec<PortInfo>, String> {
    serial2::SerialPort::available_ports()
        .map(|ports| {
            ports.into_iter()
                .map(|p| PortInfo {
                    name: p.to_string_lossy().to_string(),
                })
                .collect()
        })
        .map_err(|e| format!("枚举串口失败: {}", e))
}

///快速打开串口
pub fn open(port: &str, baud_rate: u32) -> Result<SerialPort, String> {
    SerialPort::open(port, baud_rate)
}

//========================================
//SerialPort
//========================================

///串口
pub struct SerialPort {
    inner: Serial2Port,
    timeout: std::time::Duration,
}

impl SerialPort {
    ///打开串口
    ///
    ///# 参数
    ///- port: 串口名称（如 /dev/ttyUSB0 或 COM1）
    ///- baud_rate: 波特率（如 9600, 115200）
    pub fn open(port: &str, baud_rate: u32) -> Result<Self, String> {
        let inner = Serial2Port::open(port, baud_rate)
            .map_err(|e| format!("打开串口失败: {}", e))?;

        Ok(Self {
            inner,
            timeout: std::time::Duration::from_millis(DEFAULT_TIMEOUT_MS),
        })
    }

    ///获取 Builder
    pub fn builder() -> SerialPortBuilder {
        SerialPortBuilder::new()
    }

    //========================================
    //写入
    //========================================

    ///写入字节数据
    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        std::io::Write::write(&mut self.inner, data)
            .map_err(|e| format!("写入失败: {}", e))
    }

    ///写入全部字节数据
    pub fn write_all(&mut self, data: &[u8]) -> Result<(), String> {
        std::io::Write::write_all(&mut self.inner, data)
            .map_err(|e| format!("写入失败: {}", e))
    }

    ///写入字符串
    pub fn write_str(&mut self, text: &str) -> Result<(), String> {
        self.write_all(text.as_bytes())
    }

    ///写入带换行符的字符串
    pub fn write_line(&mut self, text: &str) -> Result<(), String> {
        self.write_str(text)?;
        self.write_all(b"\r\n")
    }

    ///刷新输出缓冲区
    pub fn flush(&mut self) -> Result<(), String> {
        std::io::Write::flush(&mut self.inner)
            .map_err(|e| format!("刷新失败: {}", e))
    }

    //========================================
    //读取
    //========================================

    ///读取数据到缓冲区
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        std::io::Read::read(&mut self.inner, buf)
            .map_err(|e| format!("读取失败: {}", e))
    }

    ///读取指定字节数
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), String> {
        std::io::Read::read_exact(&mut self.inner, buf)
            .map_err(|e| format!("读取失败: {}", e))
    }

    ///读取所有可用数据
    pub fn read_available(&mut self) -> Result<Vec<u8>, String> {
        let mut buf = vec![0u8; DEFAULT_BUFFER_SIZE];
        let n = self.read(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }

    ///读取一行（直到 \n 或 \r\n）
    pub fn read_line(&mut self) -> Result<String, String> {
        let mut result = Vec::new();
        let mut buf = [0u8; 1];
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > self.timeout {
                return Err("读取超时".to_string());
            }

            match self.read(&mut buf) {
                Ok(1) => {
                    if buf[0] == b'\n' {
                        break;
                    }
                    if buf[0] != b'\r' {
                        result.push(buf[0]);
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        String::from_utf8(result)
            .map_err(|e| format!("UTF-8 解码失败: {}", e))
    }

    //========================================
    //配置
    //========================================

    ///设置超时
    pub fn set_timeout(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
        let _ = self.inner.set_read_timeout(timeout);
    }

    ///设置波特率
    pub fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), String> {
        let settings = self.inner.get_configuration()
            .map_err(|e| format!("获取配置失败: {}", e))?;

        let mut settings = settings;
        settings.set_baud_rate(baud_rate)
            .map_err(|e| format!("设置波特率失败: {}", e))?;

        self.inner.set_configuration(&settings)
            .map_err(|e| format!("应用配置失败: {}", e))
    }

    //========================================
    //控制信号
    //========================================

    ///设置 DTR 信号
    pub fn set_dtr(&mut self, level: bool) -> Result<(), String> {
        self.inner.set_dtr(level)
            .map_err(|e| format!("设置 DTR 失败: {}", e))
    }

    ///设置 RTS 信号
    pub fn set_rts(&mut self, level: bool) -> Result<(), String> {
        self.inner.set_rts(level)
            .map_err(|e| format!("设置 RTS 失败: {}", e))
    }

    ///读取 CTS 信号
    pub fn read_cts(&mut self) -> Result<bool, String> {
        self.inner.read_cts()
            .map_err(|e| format!("读取 CTS 失败: {}", e))
    }

    ///读取 DSR 信号
    pub fn read_dsr(&mut self) -> Result<bool, String> {
        self.inner.read_dsr()
            .map_err(|e| format!("读取 DSR 失败: {}", e))
    }

    ///获取内部引用
    pub fn inner(&self) -> &Serial2Port {
        &self.inner
    }

    ///获取内部可变引用
    pub fn inner_mut(&mut self) -> &mut Serial2Port {
        &mut self.inner
    }
}

//========================================
//SerialPortBuilder
//========================================

///串口配置构建器
pub struct SerialPortBuilder {
    port: Option<String>,
    baud_rate: u32,
    data_bits: DataBits,
    stop_bits: StopBits,
    parity: Parity,
    timeout: std::time::Duration,
}

impl SerialPortBuilder {
    ///创建新的构建器
    pub fn new() -> Self {
        Self {
            port: None,
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            timeout: std::time::Duration::from_millis(DEFAULT_TIMEOUT_MS),
        }
    }

    ///设置串口名称
    pub fn port(mut self, port: &str) -> Self {
        self.port = Some(port.to_string());
        self
    }

    ///设置波特率
    pub fn baud_rate(mut self, baud_rate: u32) -> Self {
        self.baud_rate = baud_rate;
        self
    }

    ///设置数据位
    pub fn data_bits(mut self, data_bits: DataBits) -> Self {
        self.data_bits = data_bits;
        self
    }

    ///设置停止位
    pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    ///设置校验位
    pub fn parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    ///设置超时
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    ///打开串口
    pub fn open(self) -> Result<SerialPort, String> {
        let port_name = self.port.ok_or("未指定串口名称")?;

        let inner = Serial2Port::open(&port_name, self.baud_rate)
            .map_err(|e| format!("打开串口失败: {}", e))?;

        //获取并修改配置
        let mut settings = inner.get_configuration()
            .map_err(|e| format!("获取配置失败: {}", e))?;

        settings.set_char_size(self.data_bits.into());
        settings.set_stop_bits(self.stop_bits.into());
        settings.set_parity(self.parity.into());

        inner.set_configuration(&settings)
            .map_err(|e| format!("应用配置失败: {}", e))?;

        inner.set_read_timeout(self.timeout)
            .map_err(|e| format!("设置超时失败: {}", e))?;

        Ok(SerialPort {
            inner,
            timeout: self.timeout,
        })
    }
}

impl Default for SerialPortBuilder {
    fn default() -> Self {
        Self::new()
    }
}

//========================================
//常用波特率
//========================================

///常用波特率
pub mod baud_rates {
    pub const B9600: u32 = 9600;
    pub const B19200: u32 = 19200;
    pub const B38400: u32 = 38400;
    pub const B57600: u32 = 57600;
    pub const B115200: u32 = 115200;
    pub const B230400: u32 = 230400;
    pub const B460800: u32 = 460800;
    pub const B921600: u32 = 921600;
}
