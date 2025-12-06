# 变更提案：使用updater_helper程序替代批处理脚本实现自我更新

## 概述
将原来基于批处理脚本的自我更新机制改为使用独立的`updater_helper`程序，解决Windows批处理脚本兼容性问题。

## 需求场景
### 场景 1：程序自我更新
- Given 文件夹1（SOURCE_DIR）中存在与program_updater同名的新版本程序
- When program_updater启动时检测到该文件
- Then 启动updater_helper程序完成替换，而非生成批处理脚本

### 场景 2：批处理脚本失败
- Given 当前使用批处理脚本方式进行自我更新
- When 在某些Windows环境下脚本执行失败（命令被截断）
- Then 导致更新流程中断，程序无法正常替换

## 技术方案
### 架构设计
- **程序A**（program_updater）：主程序，负责检测更新和启动助手
- **程序B**（updater_helper）：更新助手，负责等待、替换、启动

### 更新流程
1. 程序A启动，检测SOURCE_DIR中是否有同名文件（优先于其他文件检查）
2. 若检测到，程序A启动程序B，传参：
   - 程序A的完整路径
   - 源文件（新版本）的完整路径
3. 程序A退出
4. 程序B等待程序A进程关闭（通过PID检测）
5. 程序B将源文件复制到程序A的位置（替换）
6. 程序B删除SOURCE_DIR中的源文件
7. 程序B启动程序A
8. 程序B退出

### 配置项
在config.rs中新增：
- `HELPER_EXE`: updater_helper程序的完整路径

### 项目结构
```
program_updater/
├── Cargo.toml         # 定义两个bin目标
├── src/
│   ├── main.rs        # 主程序入口
│   ├── config.rs      # 配置（新增HELPER_EXE）
│   ├── updater.rs     # 文件替换逻辑
│   ├── self_update.rs # 自更新逻辑（修改为启动helper）
│   └── bin/
│       └── updater_helper.rs  # 更新助手程序
```

## 影响范围
- 修改 `Cargo.toml`：添加updater_helper二进制目标
- 修改 `config.rs`：添加HELPER_EXE配置项
- 修改 `self_update.rs`：移除脚本生成逻辑，改为启动helper
- 新增 `src/bin/updater_helper.rs`：更新助手程序
