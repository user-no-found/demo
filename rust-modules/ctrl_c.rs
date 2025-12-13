//!Ctrl+C 信号处理模块
//!
//!依赖：ctrlc（使用时查询最新版本：https://crates.io/crates/ctrlc）

///等待 Ctrl+C 信号，收到后程序退出
pub fn wait_for_exit() {
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        tx.send(()).expect("无法发送信号");
    })
    .expect("设置 Ctrl+C 处理器失败");

    println!("按 Ctrl+C 退出...");
    rx.recv().expect("接收信号失败");
    println!("正在退出...");
}
