//!HTTP 客户端模块
//!
//!提供 HTTP 客户端功能：GET/POST/PUT/DELETE 请求。
//!
//!依赖：ureq（使用时查询最新版本：https://crates.io/crates/ureq）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!ureq = { version = "2", features = ["json"] }
//!serde = { version = "1", features = ["derive"] }
//!serde_json = "1"
//!```

use super::config;

//========================================
//HTTP 响应结构
//========================================

///HTTP 响应
pub struct Response {
    ///状态码
    pub status: u16,
    ///响应体
    body: String,
}

impl Response {
    ///获取响应文本
    pub fn text(&self) -> &str {
        &self.body
    }

    ///解析为 JSON
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    ///检查是否成功（2xx）
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

//========================================
//HTTP 客户端结构
//========================================

///HTTP 客户端
pub struct HttpClient {
    ///自定义请求头
    headers: Vec<(String, String)>,
}

impl HttpClient {
    ///创建新的 HTTP 客户端
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    ///添加请求头
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    ///添加 Bearer Token
    pub fn with_bearer_token(self, token: &str) -> Self {
        self.with_header("Authorization", &format!("Bearer {}", token))
    }

    //========================================
    //GET 请求
    //========================================

    ///发送 GET 请求
    pub fn get(&self, url: &str) -> Result<Response, String> {
        let mut request = ureq::get(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS));

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        match request.call() {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }

    //========================================
    //POST 请求
    //========================================

    ///发送 POST 请求（JSON 数据）
    pub fn post_json<T: serde::Serialize>(&self, url: &str, data: &T) -> Result<Response, String> {
        let mut request = ureq::post(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
            .set("Content-Type", "application/json");

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        match request.send_json(data) {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }

    ///发送 POST 请求（表单数据）
    pub fn post_form(&self, url: &str, data: &[(&str, &str)]) -> Result<Response, String> {
        let mut request = ureq::post(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
            .set("Content-Type", "application/x-www-form-urlencoded");

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        let body: String = data
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        match request.send_string(&body) {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }

    ///发送 POST 请求（原始字符串）
    pub fn post_string(&self, url: &str, body: &str) -> Result<Response, String> {
        let mut request = ureq::post(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS));

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        match request.send_string(body) {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }

    //========================================
    //PUT 请求
    //========================================

    ///发送 PUT 请求（JSON 数据）
    pub fn put_json<T: serde::Serialize>(&self, url: &str, data: &T) -> Result<Response, String> {
        let mut request = ureq::put(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS))
            .set("Content-Type", "application/json");

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        match request.send_json(data) {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }

    //========================================
    //DELETE 请求
    //========================================

    ///发送 DELETE 请求
    pub fn delete(&self, url: &str) -> Result<Response, String> {
        let mut request = ureq::delete(url)
            .timeout(std::time::Duration::from_secs(config::REQUEST_TIMEOUT_SECS));

        for (key, value) in &self.headers {
            request = request.set(key, value);
        }

        match request.call() {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status, body })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let body = resp.into_string().unwrap_or_default();
                Ok(Response { status: code, body })
            }
            Err(e) => Err(format!("请求失败: {}", e)),
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

//========================================
//便捷函数
//========================================

///快速 GET 请求
pub fn get(url: &str) -> Result<Response, String> {
    HttpClient::new().get(url)
}

///快速 POST JSON 请求
pub fn post_json<T: serde::Serialize>(url: &str, data: &T) -> Result<Response, String> {
    HttpClient::new().post_json(url, data)
}
