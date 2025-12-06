//程序升级器：读取文件夹1的文件，与预设文件名对比后替换到对应路径

mod config;
mod updater;

fn main() {
    println!("程序升级器启动...");
    
    //检查配置是否有效
    if config::SOURCE_DIR.is_empty() {
        eprintln!("错误：请在config.rs中配置SOURCE_DIR路径");
        std::process::exit(1);
    }
    
    if config::STARTUP_FILE.is_empty() {
        eprintln!("错误：请在config.rs中配置STARTUP_FILE路径");
        std::process::exit(1);
    }
    
    if config::FILE_MAPPINGS.is_empty() {
        eprintln!("错误：请在config.rs中配置FILE_MAPPINGS映射表");
        std::process::exit(1);
    }
    
    //检查源目录是否存在
    if !std::path::Path::new(config::SOURCE_DIR).exists() {
        eprintln!("错误：源目录不存在: {}", config::SOURCE_DIR);
        std::process::exit(1);
    }

    //步骤1：获取源目录中的所有文件
    println!("检查源目录: {}", config::SOURCE_DIR);
    let source_files = match updater::get_source_files(config::SOURCE_DIR) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("错误：读取源目录失败: {}", e);
            std::process::exit(1);
        }
    };
    
    //步骤2：遍历源文件，与映射表对比并替换
    let mut replaced_count = 0;
    for source_file in &source_files {
        //获取文件名
        let filename = match source_file.file_name() {
            std::option::Option::Some(name) => match name.to_str() {
                std::option::Option::Some(s) => s,
                std::option::Option::None => continue,
            },
            std::option::Option::None => continue,
        };
        
        //在映射表中查找目标路径
        if let std::option::Option::Some(target_path) = updater::find_target_path(filename, config::FILE_MAPPINGS) {
            println!("发现匹配文件: {} -> {}", filename, target_path);
            
            //复制替换
            match updater::copy_file(source_file, target_path) {
                Ok(()) => {
                    println!("替换成功: {}", target_path);
                    replaced_count += 1;
                }
                Err(e) => {
                    eprintln!("错误：替换失败 {}: {}", target_path, e);
                }
            }
        }
    }

    
    println!("共替换 {} 个文件", replaced_count);
    
    //步骤3：清空源目录
    println!("正在清空源目录...");
    if let Err(e) = updater::clear_source_dir(config::SOURCE_DIR) {
        eprintln!("警告：清空源目录失败: {}", e);
    } else {
        println!("源目录已清空");
    }
    
    //步骤4：启动预设启动文件
    println!("正在启动程序: {}", config::STARTUP_FILE);
    match updater::launch_executable(config::STARTUP_FILE) {
        Ok(_) => {
            println!("程序已启动，升级器退出");
        }
        Err(e) => {
            eprintln!("错误：启动程序失败: {}", e);
            std::process::exit(1);
        }
    }
}
