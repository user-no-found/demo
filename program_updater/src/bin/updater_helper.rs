//更新助手程序：检查源目录中的主程序更新并完成替换

fn main() {
    println!("更新助手启动...");
    
    //检查配置
    if program_updater::config::SOURCE_DIR.is_empty() {
        eprintln!("错误：未配置SOURCE_DIR");
        std::process::exit(1);
    }
    
    if program_updater::config::MAIN_EXE_PATH.is_empty() {
        eprintln!("错误：未配置MAIN_EXE_PATH");
        std::process::exit(1);
    }
    
    //构建源文件路径
    let source_dir = std::path::Path::new(program_updater::config::SOURCE_DIR);
    let source_file = source_dir.join(program_updater::config::MAIN_EXE_NAME);
    
    //检查源文件是否存在
    if !source_file.exists() {
        println!("源目录中未发现主程序更新文件: {}", source_file.display());
        println!("助手退出");
        return;
    }
    
    println!("发现主程序更新文件: {}", source_file.display());
    println!("目标位置: {}", program_updater::config::MAIN_EXE_PATH);
    
    //短暂延迟确保主程序已退出
    println!("等待主程序退出...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    //复制源文件到目标位置
    println!("正在替换主程序...");
    if let Err(e) = copy_file(&source_file, program_updater::config::MAIN_EXE_PATH) {
        eprintln!("错误：替换失败: {}", e);
        std::process::exit(1);
    }
    println!("替换成功");
    
    //删除源文件
    println!("正在清理源文件...");
    if let Err(e) = std::fs::remove_file(&source_file) {
        eprintln!("警告：删除源文件失败: {}", e);
    } else {
        println!("源文件已删除");
    }
    
    //启动主程序
    println!("正在启动主程序...");
    if let Err(e) = launch_program(program_updater::config::MAIN_EXE_PATH) {
        eprintln!("错误：启动主程序失败: {}", e);
        std::process::exit(1);
    }
    
    println!("更新完成，助手退出");
}

//复制文件
fn copy_file(source: &std::path::Path, target: &str) -> std::io::Result<()> {
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
