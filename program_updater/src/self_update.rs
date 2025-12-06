//自我更新模块：处理程序自身更新逻辑

//获取当前程序的完整路径
pub fn get_current_exe_path() -> std::io::Result<std::path::PathBuf> {
    std::env::current_exe()
}

//获取当前程序的文件名
pub fn get_current_exe_name() -> std::option::Option<std::string::String> {
    match std::env::current_exe() {
        Ok(path) => {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        }
        Err(_) => std::option::Option::None,
    }
}

//检测源目录中是否存在与当前程序同名的文件，返回源文件路径
pub fn check_self_update(source_dir: &str) -> std::option::Option<std::path::PathBuf> {
    let exe_name = match get_current_exe_name() {
        std::option::Option::Some(name) => name,
        std::option::Option::None => return std::option::Option::None,
    };
    
    let entries = match std::fs::read_dir(source_dir) {
        Ok(e) => e,
        Err(_) => return std::option::Option::None,
    };
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let std::option::Option::Some(name) = path.file_name() {
                    if let std::option::Option::Some(name_str) = name.to_str() {
                        if name_str == exe_name {
                            return std::option::Option::Some(path);
                        }
                    }
                }
            }
        }
    }
    
    std::option::Option::None
}

//启动更新助手程序
//参数：helper_path=助手程序路径，source_file=新版本源文件路径
//助手程序将等待当前程序退出后完成替换
pub fn launch_update_helper(helper_path: &str, source_file: &std::path::Path) -> std::io::Result<()> {
    //检查助手程序是否存在
    if !std::path::Path::new(helper_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("更新助手程序不存在: {}", helper_path)
        ));
    }
    
    //获取当前程序路径和PID
    let current_exe = get_current_exe_path()?;
    let current_pid = std::process::id();
    
    //启动助手程序，传递参数：目标程序路径、源文件路径、目标程序PID
    std::process::Command::new(helper_path)
        .arg(current_exe.to_str().unwrap_or(""))
        .arg(source_file.to_str().unwrap_or(""))
        .arg(current_pid.to_string())
        .spawn()?;
    
    Ok(())
}
