//!文件监控模块
//!
//!提供文件和目录变化监控功能，支持热重载场景。
//!
//!依赖：notify（使用时查询最新版本：https://crates.io/crates/notify）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!notify = "8"        # https://crates.io/crates/notify
//!notify-debouncer-mini = "0.5"  # 可选，防抖动支持
//!```
//!
//!# 快速开始
//!
//!## 监控单个文件
//!```rust
//!mod file_watcher;
//!
//!fn main() {
//!    file_watcher::watch_file("config.toml", |event| {
//!        println!("文件变化: {:?}", event);
//!    }).unwrap();
//!}
//!```
//!
//!## 监控目录
//!```rust
//!mod file_watcher;
//!
//!fn main() {
//!    file_watcher::watch_dir("./src", |event| {
//!        match event.kind {
//!            file_watcher::EventKind::Create => println!("创建: {:?}", event.path),
//!            file_watcher::EventKind::Modify => println!("修改: {:?}", event.path),
//!            file_watcher::EventKind::Delete => println!("删除: {:?}", event.path),
//!            _ => {}
//!        }
//!    }).unwrap();
//!}
//!```
//!
//!## 使用 Builder 模式
//!```rust
//!mod file_watcher;
//!use std::time::Duration;
//!
//!fn main() {
//!    file_watcher::FileWatcher::new()
//!        .path("./src")
//!        .recursive(true)
//!        .debounce(Duration::from_millis(500))
//!        .extensions(&["rs", "toml"])
//!        .on_event(|event| {
//!            println!("{:?}", event);
//!        })
//!        .watch()
//!        .unwrap();
//!}
//!```

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc;

//========================================
//事件类型
//========================================

///文件事件类型
#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    ///文件创建
    Create,
    ///文件修改
    Modify,
    ///文件删除
    Delete,
    ///文件重命名
    Rename,
    ///其他事件
    Other,
}

///文件事件
#[derive(Debug, Clone)]
pub struct FileEvent {
    ///事件类型
    pub kind: EventKind,
    ///文件路径
    pub path: std::path::PathBuf,
    ///原路径（重命名时使用）
    pub from_path: Option<std::path::PathBuf>,
}

impl FileEvent {
    ///创建新事件
    pub fn new(kind: EventKind, path: std::path::PathBuf) -> Self {
        Self {
            kind,
            path,
            from_path: None,
        }
    }

    ///创建重命名事件
    pub fn rename(from: std::path::PathBuf, to: std::path::PathBuf) -> Self {
        Self {
            kind: EventKind::Rename,
            path: to,
            from_path: Some(from),
        }
    }
}

//========================================
//便捷函数
//========================================

///监控单个文件
///
///# 参数
///- path: 文件路径
///- callback: 事件回调函数
///
///# 注意
///此函数会阻塞当前线程
pub fn watch_file<P, F>(path: P, callback: F) -> Result<(), String>
where
    P: AsRef<std::path::Path>,
    F: Fn(FileEvent) + Send + 'static,
{
    FileWatcher::new()
        .path(path)
        .recursive(false)
        .on_event(callback)
        .watch()
}

///监控目录
///
///# 参数
///- path: 目录路径
///- callback: 事件回调函数
///
///# 注意
///此函数会阻塞当前线程
pub fn watch_dir<P, F>(path: P, callback: F) -> Result<(), String>
where
    P: AsRef<std::path::Path>,
    F: Fn(FileEvent) + Send + 'static,
{
    FileWatcher::new()
        .path(path)
        .recursive(false)
        .on_event(callback)
        .watch()
}

///递归监控目录
///
///# 参数
///- path: 目录路径
///- callback: 事件回调函数
///
///# 注意
///此函数会阻塞当前线程
pub fn watch_dir_recursive<P, F>(path: P, callback: F) -> Result<(), String>
where
    P: AsRef<std::path::Path>,
    F: Fn(FileEvent) + Send + 'static,
{
    FileWatcher::new()
        .path(path)
        .recursive(true)
        .on_event(callback)
        .watch()
}

//========================================
//FileWatcher Builder
//========================================

///文件监控器
pub struct FileWatcher<F>
where
    F: Fn(FileEvent) + Send + 'static,
{
    ///监控路径
    paths: Vec<std::path::PathBuf>,
    ///是否递归
    recursive: bool,
    ///防抖动延迟
    debounce: Option<std::time::Duration>,
    ///文件扩展名过滤
    extensions: Option<Vec<String>>,
    ///文件名模式过滤
    pattern: Option<String>,
    ///事件回调
    callback: Option<F>,
}

impl<F> FileWatcher<F>
where
    F: Fn(FileEvent) + Send + 'static,
{
    ///创建新的监控器
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            recursive: false,
            debounce: None,
            extensions: None,
            pattern: None,
            callback: None,
        }
    }

    ///添加监控路径
    pub fn path<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.paths.push(path.as_ref().to_path_buf());
        self
    }

    ///添加多个监控路径
    pub fn paths<P: AsRef<std::path::Path>>(mut self, paths: &[P]) -> Self {
        for p in paths {
            self.paths.push(p.as_ref().to_path_buf());
        }
        self
    }

    ///设置是否递归监控
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    ///设置防抖动延迟
    pub fn debounce(mut self, duration: std::time::Duration) -> Self {
        self.debounce = Some(duration);
        self
    }

    ///设置文件扩展名过滤
    pub fn extensions(mut self, exts: &[&str]) -> Self {
        self.extensions = Some(exts.iter().map(|s| s.to_string()).collect());
        self
    }

    ///设置文件名模式过滤（简单通配符，支持 * 和 ?）
    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    ///设置事件回调
    pub fn on_event(mut self, callback: F) -> Self {
        self.callback = Some(callback);
        self
    }

    ///开始监控（阻塞）
    pub fn watch(self) -> Result<(), String> {
        if self.paths.is_empty() {
            return Err("未指定监控路径".to_string());
        }

        let callback = self.callback.ok_or("未设置回调函数")?;

        let (tx, rx) = mpsc::channel();

        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        }).map_err(|e| format!("创建监控器失败: {}", e))?;

        let mode = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        for path in &self.paths {
            watcher.watch(path, mode)
                .map_err(|e| format!("监控路径失败: {}", e))?;
        }

        //事件处理循环
        loop {
            match rx.recv() {
                Ok(event) => {
                    for path in event.paths {
                        //扩展名过滤
                        if let Some(ref exts) = self.extensions {
                            if let Some(ext) = path.extension() {
                                let ext_str = ext.to_string_lossy().to_lowercase();
                                if !exts.iter().any(|e| e.to_lowercase() == ext_str) {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }

                        //模式过滤
                        if let Some(ref pattern) = self.pattern {
                            if let Some(name) = path.file_name() {
                                if !match_pattern(pattern, &name.to_string_lossy()) {
                                    continue;
                                }
                            }
                        }

                        let kind = convert_event_kind(&event.kind);
                        let file_event = FileEvent::new(kind, path);
                        callback(file_event);
                    }
                }
                Err(_) => break,
            }
        }

        Ok(())
    }

    ///启动监控（非阻塞，返回句柄）
    pub fn watch_async(self) -> Result<WatchHandle, String> {
        if self.paths.is_empty() {
            return Err("未指定监控路径".to_string());
        }

        let callback = self.callback.ok_or("未设置回调函数")?;
        let paths = self.paths.clone();
        let recursive = self.recursive;
        let extensions = self.extensions.clone();
        let pattern = self.pattern.clone();

        let (stop_tx, stop_rx) = mpsc::channel();

        let handle = std::thread::spawn(move || {
            let (tx, rx) = mpsc::channel();

            let mut watcher: RecommendedWatcher = match notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            }) {
                Ok(w) => w,
                Err(_) => return,
            };

            let mode = if recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };

            for path in &paths {
                if watcher.watch(path, mode).is_err() {
                    return;
                }
            }

            loop {
                //检查停止信号
                if stop_rx.try_recv().is_ok() {
                    break;
                }

                //处理事件（带超时）
                match rx.recv_timeout(std::time::Duration::from_millis(100)) {
                    Ok(event) => {
                        for path in event.paths {
                            //扩展名过滤
                            if let Some(ref exts) = extensions {
                                if let Some(ext) = path.extension() {
                                    let ext_str = ext.to_string_lossy().to_lowercase();
                                    if !exts.iter().any(|e| e.to_lowercase() == ext_str) {
                                        continue;
                                    }
                                } else {
                                    continue;
                                }
                            }

                            //模式过滤
                            if let Some(ref pat) = pattern {
                                if let Some(name) = path.file_name() {
                                    if !match_pattern(pat, &name.to_string_lossy()) {
                                        continue;
                                    }
                                }
                            }

                            let kind = convert_event_kind(&event.kind);
                            let file_event = FileEvent::new(kind, path);
                            callback(file_event);
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        Ok(WatchHandle {
            stop_sender: stop_tx,
            thread: Some(handle),
        })
    }
}

impl<F> Default for FileWatcher<F>
where
    F: Fn(FileEvent) + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

//========================================
//监控句柄
//========================================

///监控句柄，用于控制异步监控
pub struct WatchHandle {
    stop_sender: mpsc::Sender<()>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl WatchHandle {
    ///停止监控
    pub fn stop(mut self) {
        let _ = self.stop_sender.send(());
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }

    ///检查是否仍在运行
    pub fn is_running(&self) -> bool {
        self.thread.as_ref().map_or(false, |h| !h.is_finished())
    }
}

impl Drop for WatchHandle {
    fn drop(&mut self) {
        let _ = self.stop_sender.send(());
    }
}

//========================================
//辅助函数
//========================================

///转换事件类型
fn convert_event_kind(kind: &notify::EventKind) -> EventKind {
    match kind {
        notify::EventKind::Create(_) => EventKind::Create,
        notify::EventKind::Modify(_) => EventKind::Modify,
        notify::EventKind::Remove(_) => EventKind::Delete,
        notify::EventKind::Other => EventKind::Other,
        _ => EventKind::Other,
    }
}

///简单模式匹配（支持 * 和 ?）
fn match_pattern(pattern: &str, text: &str) -> bool {
    let mut pattern_chars = pattern.chars().peekable();
    let mut text_chars = text.chars().peekable();

    while let Some(p) = pattern_chars.next() {
        match p {
            '*' => {
                //跳过连续的 *
                while pattern_chars.peek() == Some(&'*') {
                    pattern_chars.next();
                }

                //* 在末尾，匹配所有剩余
                if pattern_chars.peek().is_none() {
                    return true;
                }

                //尝试匹配剩余部分
                let remaining_pattern: String = pattern_chars.collect();
                let mut remaining_text: String = text_chars.collect();

                while !remaining_text.is_empty() {
                    if match_pattern(&remaining_pattern, &remaining_text) {
                        return true;
                    }
                    remaining_text = remaining_text.chars().skip(1).collect();
                }

                return match_pattern(&remaining_pattern, "");
            }
            '?' => {
                if text_chars.next().is_none() {
                    return false;
                }
            }
            c => {
                if text_chars.next() != Some(c) {
                    return false;
                }
            }
        }
    }

    text_chars.peek().is_none()
}
