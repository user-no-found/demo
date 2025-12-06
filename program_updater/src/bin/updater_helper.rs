//更新助手程序：等待主程序退出后完成替换

fn main() {
    println!("更新助手启动...");
    
    //解析命令行参数
    let args: std::vec::Vec<std::string::String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("错误：参数不足");
        eprintln!("用法: updater_helper <目标程序路径> <源文件路径> <目标程序PID>");
        std::process::exit(1);
    }
    
    let target_path = &args[1];
    let source_path = &args[2];
    let target_pid: u32 = match args[3].parse() {
        Ok(pid) => pid,
        Err(_) => {
            eprintln!("错误：无效的PID: {}", args[3]);
            std::process::exit(1);
        }
    };
    
    println!("目标程序: {}", target_path);
    println!("源文件: {}", source_path);
    println!("目标PID: {}", target_pid);
    
    //等待目标程序退出
    println!("等待目标程序退出...");
    wait_for_process_exit(target_pid);
    println!("目标程序已退出");
    
    //短暂延迟确保文件句柄释放
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    //复制源文件到目标位置
    println!("正在替换程序...");
    if let Err(e) = copy_file(source_path, target_path) {
        eprintln!("错误：替换失败: {}", e);
        std::process::exit(1);
    }
    println!("替换成功");
    
    //删除源文件
    println!("正在清理源文件...");
    if let Err(e) = std::fs::remove_file(source_path) {
        eprintln!("警告：删除源文件失败: {}", e);
    } else {
        println!("源文件已删除");
    }
    
    //启动目标程序
    println!("正在启动程序...");
    if let Err(e) = launch_program(target_path) {
        eprintln!("错误：启动程序失败: {}", e);
        std::process::exit(1);
    }
    
    println!("更新完成，助手退出");
}

//等待进程退出（Windows）
#[cfg(target_os = "windows")]
fn wait_for_process_exit(pid: u32) {
    //使用Windows API检测进程是否存在
    loop {
        let output = std::process::Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid), "/NH"])
            .output();
        
        match output {
            Ok(out) => {
                let stdout = std::string::String::from_utf8_lossy(&out.stdout);
                //如果输出中不包含PID，说明进程已退出
                if !stdout.contains(&pid.to_string()) {
                    break;
                }
            }
            Err(_) => {
                //命令执行失败，假设进程已退出
                break;
            }
        }
        
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

//等待进程退出（Linux/Mac）
#[cfg(not(target_os = "windows"))]
fn wait_for_process_exit(pid: u32) {
    loop {
        //使用kill -0检测进程是否存在
        let status = std::process::Command::new("kill")
            .args(&["-0", &pid.to_string()])
            .status();
        
        match status {
            Ok(s) if s.success() => {
                //进程仍存在，继续等待
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            _ => {
                //进程不存在或命令失败
                break;
            }
        }
    }
}

//复制文件
fn copy_file(source: &str, target: &str) -> std::io::Result<()> {
    //确保目标目录存在
    if let std::option::Option::Some(parent) = std::path::Path::new(target).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    
    //复制文件
    std::fs::copy(source, target)?;
    
    //在Linux上设置可执行权限
    #[cfg(not(target_os = "windows"))]
    {
        let mut perms = std::fs::metadata(target)?.permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
        std::fs::set_permissions(target, perms)?;
    }
    
    Ok(())
}

//启动程序
fn launch_program(path: &str) -> std::io::Result<()> {
    std::process::Command::new(path).spawn()?;
    Ok(())
}
