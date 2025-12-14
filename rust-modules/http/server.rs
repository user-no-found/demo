//!HTTP 服务端模块
//!
//!提供简易 HTTP 服务端功能：路由注册、请求处理。
//!
//!依赖：tiny_http（使用时查询最新版本：https://crates.io/crates/tiny_http）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!tiny_http = "0.12"
//!serde_json = "1"
//!```

use super::config;

//========================================
//HTTP 请求封装
//========================================

///HTTP 请求
pub struct Request {
    ///请求方法
    pub method: String,
    ///请求路径
    pub path: String,
    ///查询参数
    pub query: Option<String>,
    ///请求体
    pub body: String,
    ///内部请求对象
    inner: tiny_http::Request,
}

impl Request {
    ///从 tiny_http::Request 创建
    fn from_tiny(mut req: tiny_http::Request) -> Self {
        let method = req.method().to_string();
        let url = req.url().to_string();
        let (path, query) = if let Some(pos) = url.find('?') {
            (url[..pos].to_string(), Some(url[pos + 1..].to_string()))
        } else {
            (url, None)
        };

        let mut body = String::new();
        let _ = req.as_reader().read_to_string(&mut body);

        Self {
            method,
            path,
            query,
            body,
            inner: req,
        }
    }

    ///解析 JSON 请求体
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.body)
    }

    ///响应请求（文本）
    pub fn respond_text(self, status: u16, body: &str) {
        let response = tiny_http::Response::from_string(body)
            .with_status_code(status)
            .with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=utf-8"[..]).unwrap()
            );
        let _ = self.inner.respond(response);
    }

    ///响应请求（JSON）
    pub fn respond_json<T: serde::Serialize>(self, status: u16, data: &T) {
        let body = serde_json::to_string(data).unwrap_or_default();
        let response = tiny_http::Response::from_string(body)
            .with_status_code(status)
            .with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()
            );
        let _ = self.inner.respond(response);
    }

    ///响应请求（HTML）
    pub fn respond_html(self, status: u16, body: &str) {
        let response = tiny_http::Response::from_string(body)
            .with_status_code(status)
            .with_header(
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap()
            );
        let _ = self.inner.respond(response);
    }
}

//========================================
//路由处理器类型
//========================================

///路由处理器
pub type Handler = Box<dyn Fn(Request) + Send + Sync>;

///路由条目
struct Route {
    method: String,
    path: String,
    handler: Handler,
}

//========================================
//HTTP 服务端结构
//========================================

///HTTP 服务端
pub struct HttpServer {
    ///路由表
    routes: Vec<Route>,
    ///监听端口
    port: u16,
}

impl HttpServer {
    ///创建服务端并绑定端口
    pub fn bind(port: u16) -> Self {
        Self {
            routes: Vec::new(),
            port,
        }
    }

    ///使用默认端口创建
    pub fn bind_default() -> Self {
        Self::bind(config::SERVER_DEFAULT_PORT)
    }

    ///注册 GET 路由
    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "GET".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    ///注册 POST 路由
    pub fn post<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "POST".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    ///注册 PUT 路由
    pub fn put<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "PUT".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    ///注册 DELETE 路由
    pub fn delete<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(Request) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "DELETE".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    ///启动服务端
    pub fn run(self) {
        let addr = format!("{}:{}", config::SERVER_DEFAULT_ADDR, self.port);
        let server = tiny_http::Server::http(&addr).expect("启动 HTTP 服务端失败");
        println!("HTTP 服务端已启动，监听 http://{}", addr);

        let routes = std::sync::Arc::new(self.routes);

        for request in server.incoming_requests() {
            let req = Request::from_tiny(request);
            let method = req.method.clone();
            let path = req.path.clone();

            //查找匹配的路由
            let mut found = false;
            for route in routes.iter() {
                if route.method == method && Self::match_path(&route.path, &path) {
                    (route.handler)(req);
                    found = true;
                    break;
                }
            }

            if !found {
                //404 处理（请求已被消费，需要重新创建响应）
                //由于 req 已经移动，这里无法响应 404
                //实际使用中建议添加默认路由
            }
        }
    }

    ///路径匹配
    fn match_path(pattern: &str, path: &str) -> bool {
        //简单匹配，支持 * 通配符
        if pattern == "*" {
            return true;
        }
        if pattern.ends_with("/*") {
            let prefix = &pattern[..pattern.len() - 2];
            return path.starts_with(prefix);
        }
        pattern == path
    }
}
