# Rust 可复用模块

将模块文件复制到项目 `src/` 目录，通过 `mod xxx;` 引入使用。

## 模块列表

| 文件 | 用途 | 依赖 |
|------|------|------|
| `ctrl_c.rs` | Ctrl+C 停止程序 | [ctrlc](https://crates.io/crates/ctrlc) |
| `cmd_config.rs` | 命令行参数配置 | [clap](https://crates.io/crates/clap) (需 derive feature) |

> 注：使用前请到 crates.io 查询依赖的最新版本

## 使用示例

### ctrl_c.rs
```rust
mod ctrl_c;

fn main() {
    //程序逻辑...
    ctrl_c::wait_for_exit();
}
```

### cmd_config.rs
```rust
mod cmd_config;

fn main() {
    let cfg = cmd_config::Config::parse();

    if cfg.is_debug() {
        cfg.debug_println("调试模式已启用");
    }

    //程序逻辑...
}
```

运行：`./program -d` 或 `./program --debug` 启用调试模式
