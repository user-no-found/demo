//!环境变量配置模块
//!
//!提供 .env 文件加载和环境变量读取功能。
//!
//!依赖：dotenvy（使用时查询最新版本：https://crates.io/crates/dotenvy）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!dotenvy = "0.15"  # https://crates.io/crates/dotenvy
//!```
//!
//!# 快速开始
//!
//!## 加载 .env 文件
//!```rust
//!mod env_config;
//!
//!fn main() {
//!    //加载 .env 文件
//!    env_config::load().ok();
//!
//!    //读取变量
//!    let db_url = env_config::get("DATABASE_URL").unwrap();
//!    let port = env_config::get_int("PORT").unwrap_or(8080);
//!}
//!```
//!
//!## 使用 EnvReader
//!```rust
//!mod env_config;
//!
//!fn main() {
//!    let env = env_config::EnvReader::new()
//!        .prefix("APP_")
//!        .load_dotenv();
//!
//!    let name = env.require("NAME").unwrap();  //读取 APP_NAME
//!    let debug = env.get_bool("DEBUG").unwrap_or(false);
//!}
//!```

//========================================
//加载函数
//========================================

///加载 .env 文件
///
///从当前目录及父目录查找 .env 文件并加载
pub fn load() -> Result<(), String> {
    dotenvy::dotenv()
        .map(|_| ())
        .map_err(|e| format!("加载 .env 失败: {}", e))
}

///加载指定路径的 .env 文件
pub fn load_from(path: &str) -> Result<(), String> {
    dotenvy::from_filename(path)
        .map(|_| ())
        .map_err(|e| format!("加载 {} 失败: {}", path, e))
}

///可选加载 .env 文件（文件不存在不报错）
pub fn load_optional() {
    let _ = dotenvy::dotenv();
}

///可选加载指定路径的 .env 文件
pub fn load_from_optional(path: &str) {
    let _ = dotenvy::from_filename(path);
}

//========================================
//读取函数
//========================================

///读取环境变量
pub fn get(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

///读取必需的环境变量
pub fn require(key: &str) -> Result<String, String> {
    std::env::var(key)
        .map_err(|_| format!("环境变量 {} 未设置", key))
}

///读取环境变量，不存在返回默认值
pub fn get_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

///读取整数类型环境变量
pub fn get_int(key: &str) -> Option<i64> {
    std::env::var(key).ok()?.parse().ok()
}

///读取必需的整数类型环境变量
pub fn require_int(key: &str) -> Result<i64, String> {
    let value = require(key)?;
    value.parse()
        .map_err(|_| format!("环境变量 {} 不是有效的整数: {}", key, value))
}

///读取整数类型环境变量，不存在返回默认值
pub fn get_int_or(key: &str, default: i64) -> i64 {
    get_int(key).unwrap_or(default)
}

///读取布尔类型环境变量
///
///支持的值：true, false, 1, 0, yes, no（不区分大小写）
pub fn get_bool(key: &str) -> Option<bool> {
    let value = std::env::var(key).ok()?;
    parse_bool(&value)
}

///读取必需的布尔类型环境变量
pub fn require_bool(key: &str) -> Result<bool, String> {
    let value = require(key)?;
    parse_bool(&value)
        .ok_or_else(|| format!("环境变量 {} 不是有效的布尔值: {}", key, value))
}

///读取布尔类型环境变量，不存在返回默认值
pub fn get_bool_or(key: &str, default: bool) -> bool {
    get_bool(key).unwrap_or(default)
}

///读取浮点数类型环境变量
pub fn get_float(key: &str) -> Option<f64> {
    std::env::var(key).ok()?.parse().ok()
}

///读取浮点数类型环境变量，不存在返回默认值
pub fn get_float_or(key: &str, default: f64) -> f64 {
    get_float(key).unwrap_or(default)
}

//========================================
//辅助函数
//========================================

///解析布尔值
fn parse_bool(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

///设置环境变量
pub fn set(key: &str, value: &str) {
    std::env::set_var(key, value);
}

///删除环境变量
pub fn remove(key: &str) {
    std::env::remove_var(key);
}

///检查环境变量是否存在
pub fn exists(key: &str) -> bool {
    std::env::var(key).is_ok()
}

//========================================
//EnvReader（带前缀支持）
//========================================

///环境变量读取器
pub struct EnvReader {
    ///变量前缀
    prefix: String,
}

impl EnvReader {
    ///创建新的读取器
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
        }
    }

    ///设置变量前缀
    ///
    ///# 示例
    ///```rust
    ///let env = EnvReader::new().prefix("APP_");
    ///env.get("NAME"); //读取 APP_NAME
    ///```
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    ///加载 .env 文件
    pub fn load_dotenv(self) -> Self {
        load_optional();
        self
    }

    ///加载指定路径的 .env 文件
    pub fn load_from(self, path: &str) -> Self {
        load_from_optional(path);
        self
    }

    ///获取完整键名
    fn full_key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }

    ///读取环境变量
    pub fn get(&self, key: &str) -> Option<String> {
        get(&self.full_key(key))
    }

    ///读取必需的环境变量
    pub fn require(&self, key: &str) -> Result<String, String> {
        require(&self.full_key(key))
    }

    ///读取环境变量，不存在返回默认值
    pub fn get_or(&self, key: &str, default: &str) -> String {
        get_or(&self.full_key(key), default)
    }

    ///读取整数类型
    pub fn get_int(&self, key: &str) -> Option<i64> {
        get_int(&self.full_key(key))
    }

    ///读取整数类型，不存在返回默认值
    pub fn get_int_or(&self, key: &str, default: i64) -> i64 {
        get_int_or(&self.full_key(key), default)
    }

    ///读取布尔类型
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        get_bool(&self.full_key(key))
    }

    ///读取布尔类型，不存在返回默认值
    pub fn get_bool_or(&self, key: &str, default: bool) -> bool {
        get_bool_or(&self.full_key(key), default)
    }

    ///读取浮点数类型
    pub fn get_float(&self, key: &str) -> Option<f64> {
        get_float(&self.full_key(key))
    }

    ///读取浮点数类型，不存在返回默认值
    pub fn get_float_or(&self, key: &str, default: f64) -> f64 {
        get_float_or(&self.full_key(key), default)
    }

    ///检查变量是否存在
    pub fn exists(&self, key: &str) -> bool {
        exists(&self.full_key(key))
    }
}

impl Default for EnvReader {
    fn default() -> Self {
        Self::new()
    }
}

//========================================
//批量读取
//========================================

///读取所有以指定前缀开头的环境变量
pub fn get_all_with_prefix(prefix: &str) -> Vec<(String, String)> {
    std::env::vars()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect()
}

///读取所有环境变量
pub fn get_all() -> Vec<(String, String)> {
    std::env::vars().collect()
}
