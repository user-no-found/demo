//!JSON 配置模块
//!
//!提供 JSON 配置文件的读取、写入、修改功能。
//!
//!依赖：serde_json（使用时查询最新版本：https://crates.io/crates/serde_json）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!serde = { version = "1", features = ["derive"] }
//!serde_json = "1"
//!```
//!
//!# 快速开始
//!
//!## 读取配置
//!```rust
//!mod json_config;
//!
//!fn main() {
//!    //读取为动态 JSON
//!    let config = json_config::load("config.json").unwrap();
//!    let name = config.get_str("name").unwrap_or("default");
//!
//!    //读取为结构体
//!    #[derive(serde::Deserialize)]
//!    struct Config { name: String, port: u16 }
//!    let config: Config = json_config::load_as("config.json").unwrap();
//!}
//!```
//!
//!## 保存配置
//!```rust
//!mod json_config;
//!
//!#[derive(serde::Serialize)]
//!struct Config { name: String, port: u16 }
//!
//!fn main() {
//!    let config = Config { name: "app".to_string(), port: 8080 };
//!    json_config::save_pretty("config.json", &config).unwrap();
//!}
//!```

//========================================
//JSON 配置包装器
//========================================

///JSON 配置
pub struct JsonConfig {
    ///内部 JSON 值
    data: serde_json::Value,
}

impl JsonConfig {
    ///从 JSON 值创建
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }

    ///创建空配置
    pub fn empty() -> Self {
        Self {
            data: serde_json::json!({}),
        }
    }

    ///获取内部值的引用
    pub fn inner(&self) -> &serde_json::Value {
        &self.data
    }

    ///获取内部值的可变引用
    pub fn inner_mut(&mut self) -> &mut serde_json::Value {
        &mut self.data
    }

    //========================================
    //获取值
    //========================================

    ///获取指定路径的值（支持点分隔路径，如 "server.port"）
    pub fn get(&self, path: &str) -> Option<&serde_json::Value> {
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
        self.get(path)?.as_i64()
    }

    ///获取浮点数值
    pub fn get_f64(&self, path: &str) -> Option<f64> {
        self.get(path)?.as_f64()
    }

    ///获取布尔值
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.get(path)?.as_bool()
    }

    ///获取数组
    pub fn get_array(&self, path: &str) -> Option<&Vec<serde_json::Value>> {
        self.get(path)?.as_array()
    }

    //========================================
    //设置值
    //========================================

    ///设置指定路径的值（支持点分隔路径）
    pub fn set<T: serde::Serialize>(&mut self, path: &str, value: T) -> Result<(), String> {
        let json_value = serde_json::to_value(value).map_err(|e| format!("序列化失败: {}", e))?;
        let keys: Vec<&str> = path.split('.').collect();
        self.set_nested(&keys, json_value)
    }

    ///设置嵌套值
    fn set_nested(&mut self, keys: &[&str], value: serde_json::Value) -> Result<(), String> {
        if keys.is_empty() {
            return Err("路径不能为空".to_string());
        }

        let mut current = &mut self.data;
        for (i, key) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                if let Some(obj) = current.as_object_mut() {
                    obj.insert(key.to_string(), value);
                    return Ok(());
                }
                return Err("父路径不是对象".to_string());
            }

            if current.get(key).is_none() {
                if let Some(obj) = current.as_object_mut() {
                    obj.insert(key.to_string(), serde_json::json!({}));
                }
            }
            current = current.get_mut(key).ok_or("路径无效".to_string())?;
        }
        Ok(())
    }

    ///删除指定路径的值
    pub fn remove(&mut self, path: &str) -> Option<serde_json::Value> {
        let keys: Vec<&str> = path.split('.').collect();
        if keys.is_empty() {
            return None;
        }

        let mut current = &mut self.data;
        for (i, key) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                return current.as_object_mut()?.remove(*key);
            }
            current = current.get_mut(key)?;
        }
        None
    }

    //========================================
    //文件操作
    //========================================

    ///保存到文件
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let content = serde_json::to_string(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }

    ///保存到文件（美化格式）
    pub fn save_pretty(&self, path: &str) -> std::io::Result<()> {
        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, content)
    }
}

//========================================
//便捷函数
//========================================

///加载 JSON 配置文件
pub fn load(path: &str) -> std::io::Result<JsonConfig> {
    let content = std::fs::read_to_string(path)?;
    let data: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(JsonConfig::new(data))
}

///加载 JSON 配置文件为指定类型
pub fn load_as<T: serde::de::DeserializeOwned>(path: &str) -> std::io::Result<T> {
    let content = std::fs::read_to_string(path)?;
    serde_json::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

///保存数据到 JSON 文件
pub fn save<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let content = serde_json::to_string(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, content)
}

///保存数据到 JSON 文件（美化格式）
pub fn save_pretty<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    std::fs::write(path, content)
}

///从字符串解析 JSON 配置
pub fn from_str(json: &str) -> Result<JsonConfig, serde_json::Error> {
    let data: serde_json::Value = serde_json::from_str(json)?;
    Ok(JsonConfig::new(data))
}

///创建新的空配置
pub fn new() -> JsonConfig {
    JsonConfig::empty()
}
