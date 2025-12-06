//配置模块：存放文件夹路径和文件映射

//文件夹1路径（存放新版本程序文件）
pub const SOURCE_DIR: &str = "";

//文件映射表：(源文件名, 目标完整路径)
//当源目录中发现匹配的文件名时，复制到对应的目标路径
pub const FILE_MAPPINGS: &[(&str, &str)] = &[
    //示例：("app.exe", "C:/Program Files/MyApp/app.exe"),
    //示例：("data.dll", "C:/Program Files/MyApp/data.dll"),
];

//启动文件路径（所有文件替换完成后启动此程序）
pub const STARTUP_FILE: &str = "";

//更新助手程序路径（用于自我更新时替换主程序）
//示例：Windows下为"C:/path/to/updater_helper.exe"
pub const HELPER_EXE: &str = "";
