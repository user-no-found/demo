# 任务清单

## 阶段 1：项目配置
- [x] 修改Cargo.toml，添加updater_helper二进制目标
- [x] 修改config.rs，添加HELPER_EXE路径配置

## 阶段 2：创建updater_helper程序
- [x] 创建src/bin目录
- [x] 创建updater_helper.rs，实现：
  - [x] 解析命令行参数（程序A路径、源文件路径）
  - [x] 等待程序A进程退出（通过PID）
  - [x] 复制源文件到目标位置
  - [x] 删除源文件
  - [x] 启动程序A
  - [x] 退出

## 阶段 3：修改自更新逻辑
- [x] 修改self_update.rs：
  - [x] 移除generate_self_update_script函数
  - [x] 移除execute_self_update_script函数
  - [x] 新增launch_update_helper函数（启动helper并传参）
- [x] 修改main.rs：
  - [x] 调整自更新流程调用

## 阶段 4：编译验证
- [x] 编译两个程序（cargo build --release）
