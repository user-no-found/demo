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

//生成自我更新的临时批处理脚本（Windows）
//脚本功能：等待当前程序退出，复制新版本，删除源文件，启动新程序，删除脚本自身
#[cfg(target_os = "windows")]
pub fn generate_self_update_script(
    source_file: &std::path::Path,
) -> std::io::Result<std::path::PathBuf> {
    let current_exe = get_current_exe_path()?;
    let current_dir = current_exe.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "无法获取程序目录"))?;
    
    let exe_name = current_exe.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "无法获取程序名"))?;
    
    let target_path = current_dir.join(exe_name);
    let script_path = std::env::temp_dir().join("updater_self_update.bat");
    
    //批处理脚本内容
    //注意执行顺序：等待退出->复制->删除源文件->延迟->启动新程序->删除脚本
    let script_content = format!(
        r#"@echo off
chcp 65001 >nul
echo 正在等待程序退出...
:wait_loop
tasklist /FI "IMAGENAME eq {exe_name}" 2>nul | find /I "{exe_name}" >nul
if not errorlevel 1 (
    timeout /t 1 /nobreak >nul
    goto wait_loop
)
echo 正在更新程序...
copy /Y "{source}" "{target}"
if errorlevel 1 (
    echo 更新失败！
    pause
    exit /b 1
)
echo 正在清理源文件...
del /F /Q "{source}"
echo 等待文件系统同步...
timeout /t 2 /nobreak >nul
echo 正在启动新版本...
start "" "{target}"
echo 更新完成，清理脚本...
(goto) 2>nul & del /F /Q "%~f0"
"#,
        exe_name = exe_name,
        source = source_file.display(),
        target = target_path.display(),
    );
    
    std::fs::write(&script_path, script_content)?;
    Ok(script_path)
}

//生成自我更新的临时Shell脚本（Linux/Mac）
#[cfg(not(target_os = "windows"))]
pub fn generate_self_update_script(
    source_file: &std::path::Path,
) -> std::io::Result<std::path::PathBuf> {
    let current_exe = get_current_exe_path()?;
    let current_dir = current_exe.parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "无法获取程序目录"))?;
    
    let exe_name = current_exe.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "无法获取程序名"))?;
    
    let target_path = current_dir.join(exe_name);
    let script_path = std::env::temp_dir().join("updater_self_update.sh");
    
    let current_pid = std::process::id();
    
    //Shell脚本内容
    let script_content = format!(
        r#"#!/bin/bash
echo "正在等待程序退出..."
while kill -0 {pid} 2>/dev/null; do
    sleep 1
done
echo "正在更新程序..."
cp -f "{source}" "{target}"
if [ $? -ne 0 ]; then
    echo "更新失败！"
    exit 1
fi
chmod +x "{target}"
echo "正在清理源文件..."
rm -f "{source}"
echo "正在启动新版本..."
"{target}" &
echo "更新完成，脚本退出"
rm -f "$0"
exit 0
"#,
        pid = current_pid,
        source = source_file.display(),
        target = target_path.display(),
    );
    
    std::fs::write(&script_path, &script_content)?;
    
    //设置脚本可执行权限
    let mut perms = std::fs::metadata(&script_path)?.permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&script_path, perms)?;
    
    Ok(script_path)
}

//执行自我更新脚本（启动脚本后当前程序应退出）
#[cfg(target_os = "windows")]
pub fn execute_self_update_script(script_path: &std::path::Path) -> std::io::Result<()> {
    std::process::Command::new("cmd")
        .args(&["/C", "start", "", "/MIN", script_path.to_str().unwrap_or("")])
        .spawn()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn execute_self_update_script(script_path: &std::path::Path) -> std::io::Result<()> {
    std::process::Command::new("bash")
        .arg(script_path)
        .spawn()?;
    Ok(())
}
