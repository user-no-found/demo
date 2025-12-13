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
//!    cfg.debug_println("调试信息");
//!}
//!```
//!
//!# 扩展说明
//!添加新参数需要两步：
//!1. 在 Config 结构体中添加字段（带 #[arg(...)] 属性）
//!2. 在 impl Config 中添加对应的访问方法（可选但推荐）

///命令行配置结构体
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    //========================================
    //参数1：调试模式
    //用法：./program -d 或 ./program --debug
    //========================================
    ///启用调试模式
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,

    //========================================
    //参数2：配置文件路径（示例，取消注释启用）
    //用法：./program -c config.toml 或 ./program --config config.toml
    //========================================
    /////配置文件路径
    //#[arg(short = 'c', long = "config", default_value = "config.toml")]
    //pub config_path: String,

    //========================================
    //参数3：端口号（示例，取消注释启用）
    //用法：./program -p 3000 或 ./program --port 3000
    //========================================
    /////监听端口
    //#[arg(short = 'p', long = "port", default_value_t = 8080)]
    //pub port: u16,

    //========================================
    //参数4：详细程度（示例，取消注释启用）
    //用法：./program -v（1级）-vv（2级）-vvv（3级）
    //========================================
    /////详细输出级别（可多次指定）
    //#[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    //pub verbose: u8,
}

impl Config {
    ///解析命令行参数（必须保留）
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    //========================================
    //参数1对应方法：调试模式
    //========================================
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

    //========================================
    //参数2对应方法：配置文件路径（示例，取消注释启用）
    //========================================
    /////获取配置文件路径
    //pub fn config_path(&self) -> &str {
    //    &self.config_path
    //}

    //========================================
    //参数3对应方法：端口号（示例，取消注释启用）
    //========================================
    /////获取端口号
    //pub fn port(&self) -> u16 {
    //    self.port
    //}

    //========================================
    //参数4对应方法：详细程度（示例，取消注释启用）
    //========================================
    /////获取详细程度级别
    //pub fn verbose_level(&self) -> u8 {
    //    self.verbose
    //}
    //
    /////检查是否启用详细输出
    //pub fn is_verbose(&self) -> bool {
    //    self.verbose > 0
    //}
}
