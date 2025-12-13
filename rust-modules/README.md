# Rust 可复用模块

将模块文件复制到项目 `src/` 目录，通过 `mod xxx;` 引入使用。

## 模块列表

| 文件 | 用途 | 依赖 |
|------|------|------|
| `ctrl_c.rs` | Ctrl+C 停止程序 | [ctrlc](https://crates.io/crates/ctrlc) |

> 注：使用前请到 crates.io 查询依赖的最新版本

## 使用示例

```rust
mod ctrl_c;

fn main() {
    //程序逻辑...
    ctrl_c::wait_for_exit();
}
```
