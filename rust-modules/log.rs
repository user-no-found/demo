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
//!    log::warn!("警告信息");
//!    log::error!("错误信息");
//!}
//!```

//=== 日志配置（在此修改） ===

///日志文件路径
const LOG_FILE_PATH: &str = "./app.log";

///终端日志级别
const TERM_LOG_LEVEL: simplelog::LevelFilter = simplelog::LevelFilter::Debug;

///文件日志级别
const FILE_LOG_LEVEL: simplelog::LevelFilter = simplelog::LevelFilter::Info;

//=== 配置结束 ===

///初始化日志系统
///
///同时启用终端输出和文件输出
pub fn init() {
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    let file = std::fs::File::create(LOG_FILE_PATH)
        .expect(&format!("无法创建日志文件: {}", LOG_FILE_PATH));

    simplelog::CombinedLogger::init(vec![
        //终端日志
        simplelog::TermLogger::new(
            TERM_LOG_LEVEL,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        //文件日志
        simplelog::WriteLogger::new(FILE_LOG_LEVEL, config, file),
    ])
    .expect("日志系统初始化失败");
}

///仅初始化终端日志
pub fn init_term_only() {
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    simplelog::TermLogger::init(
        TERM_LOG_LEVEL,
        config,
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .expect("日志系统初始化失败");
}

///仅初始化文件日志
pub fn init_file_only() {
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    let file = std::fs::File::create(LOG_FILE_PATH)
        .expect(&format!("无法创建日志文件: {}", LOG_FILE_PATH));

    simplelog::WriteLogger::init(FILE_LOG_LEVEL, config, file)
        .expect("日志系统初始化失败");
}

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
    let config = simplelog::ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();

    //终端日志
    if let Some(level) = term_level {
        loggers.push(simplelog::TermLogger::new(
            level,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ));
    }

    //文件日志
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

//重新导出 log 宏，方便使用
pub use log::{debug, error, info, trace, warn};
