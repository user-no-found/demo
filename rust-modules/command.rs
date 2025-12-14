//!命令执行模块
//!
//!提供子进程管理、命令执行、输出捕获等功能。
//!
//!依赖：无（纯标准库实现）
//!
//!# 快速开始
//!
//!## 执行简单命令
//!```rust
//!mod command;
//!
//!fn main() {
//!    let output = command::run("ls", &["-la"]).unwrap();
//!    println!("输出: {}", output.stdout);
//!}
//!```
//!
//!## 执行 Shell 命令
//!```rust
//!mod command;
//!
//!fn main() {
//!    let output = command::shell("echo hello && ls").unwrap();
//!    println!("{}", output.stdout);
//!}
//!```

use std::process::{Command, Stdio, Child, ExitStatus};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

//========================================
//命令输出结构
//========================================

///命令执行结果
#[derive(Debug, Clone)]
pub struct Output {
    ///标准输出
    pub stdout: String,
    ///标准错误
    pub stderr: String,
    ///退出状态码
    pub status: i32,
    ///是否成功（状态码为0）
    pub success: bool,
}

impl Output {
    ///从 std::process::Output 创建
    fn from_std(output: std::process::Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            status: output.status.code().unwrap_or(-1),
            success: output.status.success(),
        }
    }

    ///获取合并的输出（stdout + stderr）
    pub fn combined(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else if self.stdout.is_empty() {
            self.stderr.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }

    ///获取去除首尾空白的 stdout
    pub fn stdout_trimmed(&self) -> &str {
        self.stdout.trim()
    }

    ///获取去除首尾空白的 stderr
    pub fn stderr_trimmed(&self) -> &str {
        self.stderr.trim()
    }
}

//========================================
//错误类型
//========================================

///命令执行错误
#[derive(Debug)]
pub enum Error {
    ///启动失败
    SpawnFailed(std::io::Error),
    ///执行超时
    Timeout,
    ///等待失败
    WaitFailed(std::io::Error),
    ///IO 错误
    IoError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SpawnFailed(e) => write!(f, "启动进程失败: {}", e),
            Error::Timeout => write!(f, "命令执行超时"),
            Error::WaitFailed(e) => write!(f, "等待进程失败: {}", e),
            Error::IoError(e) => write!(f, "IO 错误: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

//========================================
//简单命令执行
//========================================

///执行命令并获取输出
pub fn run(program: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(Error::SpawnFailed)?;

    Ok(Output::from_std(output))
}

///执行命令，仅返回成功与否
pub fn run_status(program: &str, args: &[&str]) -> Result<bool> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(Error::SpawnFailed)?;

    Ok(status.success())
}

///执行命令，忽略输出
pub fn run_silent(program: &str, args: &[&str]) -> Result<bool> {
    let status = Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(Error::SpawnFailed)?;

    Ok(status.success())
}

//========================================
//Shell 命令执行
//========================================

///通过 Shell 执行命令字符串
pub fn shell(cmd: &str) -> Result<Output> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run(shell, &[flag, cmd])
}

///通过 Shell 执行命令，仅返回成功与否
pub fn shell_status(cmd: &str) -> Result<bool> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run_status(shell, &[flag, cmd])
}

///通过 Shell 执行命令，忽略输出
pub fn shell_silent(cmd: &str) -> Result<bool> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run_silent(shell, &[flag, cmd])
}

//========================================
//超时执行
//========================================

///执行命令，带超时控制
pub fn run_with_timeout(program: &str, args: &[&str], timeout: Duration) -> Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(Error::SpawnFailed)?;

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => {
            let _ = handle.join();
            let output = result.map_err(Error::WaitFailed)?;
            Ok(Output::from_std(output))
        }
        Err(_) => {
            //超时，尝试终止进程（注意：这里无法直接访问 child）
            Err(Error::Timeout)
        }
    }
}

///通过 Shell 执行命令，带超时控制
pub fn shell_with_timeout(cmd: &str, timeout: Duration) -> Result<Output> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run_with_timeout(shell, &[flag, cmd], timeout)
}

//========================================
//后台执行
//========================================

///进程句柄
pub struct ProcessHandle {
    child: Child,
}

impl ProcessHandle {
    ///检查进程是否仍在运行
    pub fn is_running(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    ///等待进程结束
    pub fn wait(mut self) -> Result<Output> {
        let output = self.child.wait_with_output().map_err(Error::WaitFailed)?;
        Ok(Output::from_std(output))
    }

    ///终止进程
    pub fn kill(&mut self) -> Result<()> {
        self.child.kill().map_err(Error::IoError)
    }

    ///获取进程 ID
    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    ///尝试获取退出状态（非阻塞）
    pub fn try_wait(&mut self) -> Result<Option<i32>> {
        match self.child.try_wait() {
            Ok(Some(status)) => Ok(Some(status.code().unwrap_or(-1))),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::WaitFailed(e)),
        }
    }
}

///后台启动进程
pub fn spawn(program: &str, args: &[&str]) -> Result<ProcessHandle> {
    let child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(Error::SpawnFailed)?;

    Ok(ProcessHandle { child })
}

///后台启动 Shell 命令
pub fn spawn_shell(cmd: &str) -> Result<ProcessHandle> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    spawn(shell, &[flag, cmd])
}

//========================================
//带输入的执行
//========================================

///执行命令并传递输入
pub fn run_with_input(program: &str, args: &[&str], input: &str) -> Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(Error::SpawnFailed)?;

    //写入输入
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).map_err(Error::IoError)?;
    }

    let output = child.wait_with_output().map_err(Error::WaitFailed)?;
    Ok(Output::from_std(output))
}

///通过 Shell 执行命令并传递输入
pub fn shell_with_input(cmd: &str, input: &str) -> Result<Output> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    run_with_input(shell, &[flag, cmd], input)
}

//========================================
//命令构建器
//========================================

///命令构建器
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    cwd: Option<String>,
    envs: Vec<(String, String)>,
    env_clear: bool,
    stdin_data: Option<String>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    ///创建新的命令构建器
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            cwd: None,
            envs: Vec::new(),
            env_clear: false,
            stdin_data: None,
            timeout: None,
        }
    }

    ///创建 Shell 命令构建器
    pub fn shell(cmd: &str) -> Self {
        let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        Self::new(shell).arg(flag).arg(cmd)
    }

    ///添加参数
    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    ///添加多个参数
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    ///设置工作目录
    pub fn cwd(mut self, dir: &str) -> Self {
        self.cwd = Some(dir.to_string());
        self
    }

    ///设置环境变量
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.envs.push((key.to_string(), value.to_string()));
        self
    }

    ///清除所有环境变量
    pub fn env_clear(mut self) -> Self {
        self.env_clear = true;
        self
    }

    ///设置标准输入
    pub fn stdin(mut self, data: &str) -> Self {
        self.stdin_data = Some(data.to_string());
        self
    }

    ///设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    ///构建 Command 对象
    fn build(&self) -> Command {
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args);

        if let Some(ref cwd) = self.cwd {
            cmd.current_dir(cwd);
        }

        if self.env_clear {
            cmd.env_clear();
        }

        for (key, value) in &self.envs {
            cmd.env(key, value);
        }

        cmd
    }

    ///执行命令
    pub fn run(self) -> Result<Output> {
        if self.stdin_data.is_some() || self.timeout.is_some() {
            return self.run_complex();
        }

        let output = self.build()
            .output()
            .map_err(Error::SpawnFailed)?;

        Ok(Output::from_std(output))
    }

    ///复杂执行（带输入或超时）
    fn run_complex(self) -> Result<Output> {
        let mut cmd = self.build();
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if self.stdin_data.is_some() {
            cmd.stdin(Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(Error::SpawnFailed)?;

        //写入输入
        if let Some(ref input) = self.stdin_data {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes()).map_err(Error::IoError)?;
            }
        }

        //带超时等待
        if let Some(timeout) = self.timeout {
            let (tx, rx) = mpsc::channel();

            let handle = thread::spawn(move || {
                let result = child.wait_with_output();
                let _ = tx.send(result);
            });

            match rx.recv_timeout(timeout) {
                Ok(result) => {
                    let _ = handle.join();
                    let output = result.map_err(Error::WaitFailed)?;
                    Ok(Output::from_std(output))
                }
                Err(_) => Err(Error::Timeout),
            }
        } else {
            let output = child.wait_with_output().map_err(Error::WaitFailed)?;
            Ok(Output::from_std(output))
        }
    }

    ///后台启动
    pub fn spawn(self) -> Result<ProcessHandle> {
        let mut cmd = self.build();
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        if self.stdin_data.is_some() {
            cmd.stdin(Stdio::piped());
        }

        let mut child = cmd.spawn().map_err(Error::SpawnFailed)?;

        //写入输入
        if let Some(ref input) = self.stdin_data {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(input.as_bytes());
            }
        }

        Ok(ProcessHandle { child })
    }

    ///仅返回成功与否
    pub fn status(self) -> Result<bool> {
        let status = self.build()
            .status()
            .map_err(Error::SpawnFailed)?;

        Ok(status.success())
    }
}

//========================================
//便捷函数
//========================================

///快速执行命令并获取 stdout（去除首尾空白）
pub fn output(program: &str, args: &[&str]) -> Result<String> {
    let output = run(program, args)?;
    Ok(output.stdout.trim().to_string())
}

///快速执行 Shell 命令并获取 stdout（去除首尾空白）
pub fn shell_output(cmd: &str) -> Result<String> {
    let output = shell(cmd)?;
    Ok(output.stdout.trim().to_string())
}

///检查命令是否存在
pub fn exists(program: &str) -> bool {
    let check_cmd = if cfg!(target_os = "windows") {
        format!("where {}", program)
    } else {
        format!("which {}", program)
    };

    shell_status(&check_cmd).unwrap_or(false)
}

///获取当前 Shell
pub fn current_shell() -> Option<String> {
    if cfg!(target_os = "windows") {
        std::env::var("COMSPEC").ok()
    } else {
        std::env::var("SHELL").ok()
    }
}
