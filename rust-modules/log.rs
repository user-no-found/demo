//!日志配置模块
//!
//!依赖：
//!- simplelog（使用时查询最新版本：https://crates.io/crates/simplelog）
//!- log（使用时查询最新版本：https://crates.io/crates/log）
//!
//!Cargo.toml 添加：
//!```toml
//!simplelog = "x.x"
//!log = "x.x"
//!```
//!
//!使用示例：
//!```rust
//!mod log;
//!
//!fn main() {
//!    log::init();
//!    log::info!("程序启动");
//!    log::debug!("调试信息");
//!}
//!```
//!
//!# 扩展说明
//!添加新日志输出需要两步：
//!1. 在配置区添加新的常量（路径、级别等）
//!2. 在 init() 函数的 CombinedLogger 中添加对应的 Logger

//========================================
//配置1：日志文件路径
//========================================
const LOG_FILE_PATH: &str = "./app.log";

//========================================
//配置2：终端日志级别
//可选值：Off, Error, Warn, Info, Debug, Trace
//========================================
const TERM_LOG_LEVEL: simplelog::LevelFilter = simplelog::LevelFilter::Debug;

//========================================
//配置3：文件日志级别
//可选值：Off, Error, Warn, Info, Debug, Trace
//========================================
const FILE_LOG_LEVEL: simplelog::LevelFilter = simplelog::LevelFilter::Info;

//========================================
//配置4：第二个日志文件（示例，取消注释启用）
//用于分离不同类型的日志，如错误日志单独存放
//========================================
//const ERROR_LOG_PATH: &str = "./error.log";
//const ERROR_LOG_LEVEL: simplelog::LevelFilter = simplelog::LevelFilter::Error;

//========================================
//初始化函数：同时输出到终端和文件
//========================================
///初始化日志系统（终端+文件）
pub fn init() {
    let config = build_config();

    let file = std::fs::File::create(LOG_FILE_PATH)
        .expect(&format!("无法创建日志文件: {}", LOG_FILE_PATH));

    simplelog::CombinedLogger::init(vec![
        //========================================
        //输出1：终端日志（对应配置2）
        //========================================
        simplelog::TermLogger::new(
            TERM_LOG_LEVEL,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        //========================================
        //输出2：文件日志（对应配置1、3）
        //========================================
        simplelog::WriteLogger::new(FILE_LOG_LEVEL, config.clone(), file),
        //========================================
        //输出3：错误日志文件（示例，取消注释启用）
        //需要同时取消上方配置4的注释
        //========================================
        //simplelog::WriteLogger::new(
        //    ERROR_LOG_LEVEL,
        //    config.clone(),
        //    std::fs::File::create(ERROR_LOG_PATH)
        //        .expect(&format!("无法创建日志文件: {}", ERROR_LOG_PATH)),
        //),
    ])
    .expect("日志系统初始化失败");
}

//========================================
//初始化函数：仅终端
//========================================
///初始化日志系统（仅终端）
pub fn init_term_only() {
    simplelog::TermLogger::init(
        TERM_LOG_LEVEL,
        build_config(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("日志系统初始化失败");
}

//========================================
//初始化函数：仅文件
//========================================
///初始化日志系统（仅文件）
pub fn init_file_only() {
    let file = std::fs::File::create(LOG_FILE_PATH)
        .expect(&format!("无法创建日志文件: {}", LOG_FILE_PATH));

    simplelog::WriteLogger::init(FILE_LOG_LEVEL, build_config(), file)
        .expect("日志系统初始化失败");
}

//========================================
//初始化函数：自定义配置
//========================================
///自定义初始化日志系统
///
///# 参数
///- `term_level`: 终端日志级别（None 表示不启用）
///- `file_level`: 文件日志级别（None 表示不启用）
///- `file_path`: 日志文件路径（仅当 file_level 为 Some 时有效）
pub fn init_custom(
    term_level: Option<simplelog::LevelFilter>,
    file_level: Option<simplelog::LevelFilter>,
    file_path: Option<&str>,
) {
    let config = build_config();
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();

    if let Some(level) = term_level {
        loggers.push(simplelog::TermLogger::new(
            level,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ));
    }

    if let Some(level) = file_level {
        let path = file_path.unwrap_or(LOG_FILE_PATH);
        let file = std::fs::File::create(path)
            .expect(&format!("无法创建日志文件: {}", path));
        loggers.push(simplelog::WriteLogger::new(level, config.clone(), file));
    }

    if loggers.is_empty() {
        panic!("至少需要启用一个日志输出");
    }

    simplelog::CombinedLogger::init(loggers).expect("日志系统初始化失败");
}

//========================================
//内部函数：构建日志配置
//========================================
fn build_config() -> simplelog::Config {
    simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        //--- 可选配置（取消注释启用）---
        //.set_target_level(simplelog::LevelFilter::Off)  //隐藏目标模块名
        //.set_location_level(simplelog::LevelFilter::Debug)  //显示代码位置
        //.set_thread_level(simplelog::LevelFilter::Off)  //隐藏线程信息
        .build()
}

//========================================
//重新导出 log 宏
//========================================
pub use log::{debug, error, info, trace, warn};
