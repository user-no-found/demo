# 任务清单

## 阶段 1：修改配置
- [x] 修改config.rs：
  - [x] 移除HELPER_EXE
  - [x] 新增MAIN_EXE_PATH（程序A完整路径）
  - [x] 新增MAIN_EXE_NAME（程序A文件名）

## 阶段 2：修改程序A
- [x] 修改main.rs：移除自我更新检查逻辑
- [x] 删除self_update.rs
- [x] 新增lib.rs（共享模块）

## 阶段 3：重写程序B
- [x] 重写updater_helper.rs：
  - [x] 移除命令行参数解析
  - [x] 使用program_updater库的config模块
  - [x] 实现：检查SOURCE_DIR中的MAIN_EXE_NAME
  - [x] 实现：复制到MAIN_EXE_PATH
  - [x] 实现：删除源文件
  - [x] 实现：启动程序A

## 阶段 4：编译验证
- [x] 编译两个程序
