//!进度显示模块
//!
//!提供终端进度条和 Spinner 动画功能。
//!
//!依赖：indicatif（使用时查询最新版本：https://crates.io/crates/indicatif）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!indicatif = "0.17"  # https://crates.io/crates/indicatif
//!```
//!
//!# 快速开始
//!
//!## 进度条
//!```rust
//!mod progress;
//!
//!fn main() {
//!    let pb = progress::ProgressBar::new(100);
//!    for i in 0..100 {
//!        pb.inc(1);
//!        std::thread::sleep(std::time::Duration::from_millis(50));
//!    }
//!    pb.finish_with_message("完成！");
//!}
//!```
//!
//!## Spinner
//!```rust
//!mod progress;
//!
//!fn main() {
//!    let spinner = progress::Spinner::new("处理中...");
//!    std::thread::sleep(std::time::Duration::from_secs(3));
//!    spinner.finish_with_success("处理完成！");
//!}
//!```

//========================================
//进度条
//========================================

///进度条
pub struct ProgressBar {
    inner: indicatif::ProgressBar,
}

impl ProgressBar {
    ///创建新的进度条
    ///
    ///# 参数
    ///- total: 总量
    pub fn new(total: u64) -> Self {
        let pb = indicatif::ProgressBar::new(total);
        pb.set_style(default_progress_style());
        Self { inner: pb }
    }

    ///创建带消息的进度条
    pub fn new_with_message(total: u64, msg: &str) -> Self {
        let pb = Self::new(total);
        pb.inner.set_message(msg.to_string());
        pb
    }

    ///增加进度
    pub fn inc(&self, delta: u64) {
        self.inner.inc(delta);
    }

    ///设置进度
    pub fn set(&self, pos: u64) {
        self.inner.set_position(pos);
    }

    ///设置消息
    pub fn set_message(&self, msg: &str) {
        self.inner.set_message(msg.to_string());
    }

    ///设置前缀
    pub fn set_prefix(&self, prefix: &str) {
        self.inner.set_prefix(prefix.to_string());
    }

    ///完成进度条
    pub fn finish(&self) {
        self.inner.finish();
    }

    ///带消息完成
    pub fn finish_with_message(&self, msg: &str) {
        self.inner.finish_with_message(msg.to_string());
    }

    ///清除进度条
    pub fn finish_and_clear(&self) {
        self.inner.finish_and_clear();
    }

    ///放弃进度条（显示失败状态）
    pub fn abandon(&self) {
        self.inner.abandon();
    }

    ///带消息放弃
    pub fn abandon_with_message(&self, msg: &str) {
        self.inner.abandon_with_message(msg.to_string());
    }

    ///设置样式
    pub fn set_style(&self, template: &str) {
        if let Ok(style) = indicatif::ProgressStyle::default_bar()
            .template(template)
        {
            self.inner.set_style(style);
        }
    }

    ///获取内部引用（用于高级操作）
    pub fn inner(&self) -> &indicatif::ProgressBar {
        &self.inner
    }
}

//========================================
//Spinner
//========================================

///Spinner 动画
pub struct Spinner {
    inner: indicatif::ProgressBar,
}

impl Spinner {
    ///创建新的 Spinner
    pub fn new(msg: &str) -> Self {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(default_spinner_style());
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Self { inner: pb }
    }

    ///设置消息
    pub fn set_message(&self, msg: &str) {
        self.inner.set_message(msg.to_string());
    }

    ///完成
    pub fn finish(&self) {
        self.inner.finish();
    }

    ///带消息完成
    pub fn finish_with_message(&self, msg: &str) {
        self.inner.finish_with_message(msg.to_string());
    }

    ///成功完成（带 ✓ 图标）
    pub fn finish_with_success(&self, msg: &str) {
        self.inner.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{prefix:.green} {msg}")
                .unwrap()
        );
        self.inner.set_prefix("✓");
        self.inner.finish_with_message(msg.to_string());
    }

    ///失败完成（带 ✗ 图标）
    pub fn finish_with_error(&self, msg: &str) {
        self.inner.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{prefix:.red} {msg}")
                .unwrap()
        );
        self.inner.set_prefix("✗");
        self.inner.finish_with_message(msg.to_string());
    }

    ///清除
    pub fn finish_and_clear(&self) {
        self.inner.finish_and_clear();
    }

    ///设置样式
    pub fn set_style(&self, style: SpinnerStyle) {
        let chars = match style {
            SpinnerStyle::Dots => "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏",
            SpinnerStyle::Line => "-\\|/",
            SpinnerStyle::Arrow => "←↖↑↗→↘↓↙",
            SpinnerStyle::Circle => "◐◓◑◒",
            SpinnerStyle::Square => "◰◳◲◱",
            SpinnerStyle::Bounce => "⠁⠂⠄⠂",
        };

        self.inner.set_style(
            indicatif::ProgressStyle::default_spinner()
                .tick_chars(chars)
                .template("{spinner} {msg}")
                .unwrap()
        );
    }

    ///获取内部引用
    pub fn inner(&self) -> &indicatif::ProgressBar {
        &self.inner
    }
}

///Spinner 样式
#[derive(Debug, Clone, Copy)]
pub enum SpinnerStyle {
    ///点阵（默认）
    Dots,
    ///线条
    Line,
    ///箭头
    Arrow,
    ///圆形
    Circle,
    ///方形
    Square,
    ///弹跳
    Bounce,
}

//========================================
//多进度条
//========================================

///多进度条管理器
pub struct MultiProgress {
    inner: indicatif::MultiProgress,
}

impl MultiProgress {
    ///创建新的多进度条管理器
    pub fn new() -> Self {
        Self {
            inner: indicatif::MultiProgress::new(),
        }
    }

    ///添加进度条
    pub fn add(&self, total: u64) -> ProgressBar {
        let pb = indicatif::ProgressBar::new(total);
        pb.set_style(default_progress_style());
        let pb = self.inner.add(pb);
        ProgressBar { inner: pb }
    }

    ///添加带消息的进度条
    pub fn add_with_message(&self, total: u64, msg: &str) -> ProgressBar {
        let pb = self.add(total);
        pb.set_message(msg);
        pb
    }

    ///添加 Spinner
    pub fn add_spinner(&self, msg: &str) -> Spinner {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(default_spinner_style());
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        let pb = self.inner.add(pb);
        Spinner { inner: pb }
    }

    ///清除所有
    pub fn clear(&self) -> std::io::Result<()> {
        self.inner.clear()
    }

    ///获取内部引用
    pub fn inner(&self) -> &indicatif::MultiProgress {
        &self.inner
    }
}

impl Default for MultiProgress {
    fn default() -> Self {
        Self::new()
    }
}

//========================================
//便捷函数
//========================================

///快速创建进度条
pub fn bar(total: u64) -> ProgressBar {
    ProgressBar::new(total)
}

///快速创建带消息的进度条
pub fn bar_with_message(total: u64, msg: &str) -> ProgressBar {
    ProgressBar::new_with_message(total, msg)
}

///快速创建 Spinner
pub fn spinner(msg: &str) -> Spinner {
    Spinner::new(msg)
}

///快速创建多进度条管理器
pub fn multi() -> MultiProgress {
    MultiProgress::new()
}

//========================================
//默认样式
//========================================

///默认进度条样式
fn default_progress_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_bar()
        .template("{prefix:.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("█▓░")
}

///默认 Spinner 样式
fn default_spinner_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_spinner()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
        .template("{spinner} {msg}")
        .unwrap()
}

//========================================
//预设样式模板
//========================================

///进度条样式模板
pub mod templates {
    ///简洁样式
    pub const SIMPLE: &str = "[{bar:40}] {pos}/{len}";

    ///带百分比
    pub const WITH_PERCENT: &str = "[{bar:40}] {percent}%";

    ///带速度
    pub const WITH_SPEED: &str = "[{bar:40}] {pos}/{len} ({per_sec})";

    ///带预估时间
    pub const WITH_ETA: &str = "[{bar:40}] {pos}/{len} ETA: {eta}";

    ///完整样式
    pub const FULL: &str = "{prefix:.cyan} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {per_sec} ETA: {eta} {msg}";

    ///下载样式
    pub const DOWNLOAD: &str = "{prefix:.cyan} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) ETA: {eta}";
}
