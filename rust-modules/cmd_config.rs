//!命令行配置模块
//!
//!依赖：clap（使用时查询最新版本：https://crates.io/crates/clap）
//!Cargo.toml 添加：clap = { version = "x.x", features = ["derive"] }
//!
//!使用示例：
//!```rust
//!mod cmd_config;
//!
//!fn main() {
//!    let cfg = cmd_config::Config::parse();
//!    if cfg.debug {
//!        println!("[DEBUG] 调试模式已启用");
//!    }
//!}
//!```

///命令行配置结构体
///
///根据需要添加更多字段，使用 clap 属性配置参数
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    ///启用调试模式，输出详细信息
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,

    //--- 在下方添加你的自定义参数 ---

    //示例：配置文件路径
    //#[arg(short = 'c', long = "config", default_value = "config.toml")]
    //pub config_path: String,

    //示例：日志级别
    //#[arg(short = 'l', long = "log-level", default_value = "info")]
    //pub log_level: String,

    //示例：端口号
    //#[arg(short = 'p', long = "port", default_value_t = 8080)]
    //pub port: u16,

    //示例：详细输出（可多次使用 -v -v -v）
    //#[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    //pub verbose: u8,
}

impl Config {
    ///解析命令行参数
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    ///检查是否为调试模式
    pub fn is_debug(&self) -> bool {
        self.debug
    }

    ///调试模式下打印信息
    pub fn debug_println(&self, msg: &str) {
        if self.debug {
            println!("[DEBUG] {}", msg);
        }
    }
}
