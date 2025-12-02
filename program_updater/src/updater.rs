//升级模块：处理程序替换逻辑

//获取源目录中的所有文件
pub fn get_source_files(source_dir: &str) -> std::io::Result<std::vec::Vec<std::path::PathBuf>> {
    let mut files = std::vec::Vec::new();
    let entries = std::fs::read_dir(source_dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    
    Ok(files)
}

//根据文件名在映射表中查找目标路径
pub fn find_target_path<'a>(filename: &str, mappings: &'a [(&str, &'a str)]) -> std::option::Option<&'a str> {
    for (name, target) in mappings {
        if *name == filename {
            return std::option::Option::Some(*target);
        }
    }
    std::option::Option::None
}

//复制文件到目标路径（替换）
pub fn copy_file(source: &std::path::Path, target: &str) -> std::io::Result<()> {
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
pub fn launch_executable(path: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new(path).spawn()
}

//清空源目录中的所有文件
pub fn clear_source_dir(source_dir: &str) -> std::io::Result<()> {
    let entries = std::fs::read_dir(source_dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }
    
    Ok(())
}
