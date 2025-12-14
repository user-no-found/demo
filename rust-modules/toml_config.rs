//!TOML 配置模块
//!
//!提供 TOML 配置文件的读取、写入功能。
//!TOML 是 Rust 生态系统的标准配置格式。
//!
//!依赖：toml（使用时查询最新版本：https://crates.io/crates/toml）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!serde = { version = "1", features = ["derive"] }
//!toml = "0.8"
//!```
//!
//!# 快速开始
//!
//!## 读取配置
//!```rust
//!mod toml_config;
//!
//!#[derive(serde::Deserialize)]
//!struct Config {
//!    name: String,
//!    server: ServerConfig,
//!}
//!
//!#[derive(serde::Deserialize)]
//!struct ServerConfig {
//!    port: u16,
//!}
//!
//!fn main() {
//!    let config: Config = toml_config::load_as("config.toml").unwrap();
//!    println!("端口: {}", config.server.port);
//!}
//!```
//!
//!## 保存配置
//!```rust
//!mod toml_config;
//!
//!#[derive(serde::Serialize)]
//!struct Config { name: String, port: u16 }
//!
//!fn main() {
//!    let config = Config { name: "app".to_string(), port: 8080 };
//!    toml_config::save("config.toml", &config).unwrap();
//!}
//!```

//========================================
//TOML 配置包装器
//========================================

///TOML 配置
pub struct TomlConfig {
    ///内部 TOML 值
    data: toml::Value,
}

impl TomlConfig {
    ///从 TOML 值创建
    pub fn new(data: toml::Value) -> Self {
        Self { data }
    }

    ///创建空配置
    pub fn empty() -> Self {
        Self {
            data: toml::Value::Table(toml::map::Map::new()),
        }
    }

    ///获取内部值的引用
    pub fn inner(&self) -> &toml::Value {
        &self.data
    }

    //========================================
    //获取值
    //========================================

    ///获取指定路径的值（支持点分隔路径）
    pub fn get(&self, path: &str) -> Option<&toml::Value> {
        let mut current = &self.data;
        for key in path.split('.') {
            current = current.get(key)?;
        }
        Some(current)
    }

    ///获取字符串值
    pub fn get_str(&self, path: &str) -> Option<&str> {
        self.get(path)?.as_str()
    }

    ///获取整数值
    pub fn get_i64(&self, path: &str) -> Option<i64> {
        self.get(path)?.as_integer()
    }

    ///获取浮点数值
    pub fn get_f64(&self, path: &str) -> Option<f64> {
        self.get(path)?.as_float()
    }

    ///获取布尔值
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.get(path)?.as_bool()
    }

    ///获取数组
    pub fn get_array(&self, path: &str) -> Option<&Vec<toml::Value>> {
        self.get(path)?.as_array()
    }

    //========================================
    //文件操作
    //========================================

    ///保存到文件
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let content = toml::to_string_pretty(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }
}

//========================================
//便捷函数
//========================================

///加载 TOML 配置文件
pub fn load(path: &str) -> std::io::Result<TomlConfig> {
    let content = std::fs::read_to_string(path)?;
    let data: toml::Value = toml::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(TomlConfig::new(data))
}

///加载 TOML 配置文件为指定类型
pub fn load_as<T: serde::de::DeserializeOwned>(path: &str) -> std::io::Result<T> {
    let content = std::fs::read_to_string(path)?;
    toml::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

///保存数据到 TOML 文件
pub fn save<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let content = toml::to_string_pretty(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, content)
}

///从字符串解析 TOML 配置
pub fn from_str(toml_str: &str) -> Result<TomlConfig, toml::de::Error> {
    let data: toml::Value = toml::from_str(toml_str)?;
    Ok(TomlConfig::new(data))
}

///创建新的空配置
pub fn new() -> TomlConfig {
    TomlConfig::empty()
}
