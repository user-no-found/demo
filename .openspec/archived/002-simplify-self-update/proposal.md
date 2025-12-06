# 变更提案：简化自我更新流程

## 概述
简化程序A和程序B的自我更新逻辑，移除参数传递，改为硬编码配置方式。

## 需求场景
### 场景 1：程序B独立更新
- Given updater_helper启动
- When 检测到SOURCE_DIR中存在program_updater.exe
- Then 将其复制到MAIN_EXE_PATH位置，删除源文件，启动程序A

### 场景 2：程序A简化
- Given program_updater启动
- When 执行更新流程
- Then 不再检查自身更新（由程序B负责）

## 技术方案
### 程序B（updater_helper）修改
- 移除命令行参数解析
- 从config读取：
  - SOURCE_DIR：源目录路径
  - MAIN_EXE_PATH：程序A的完整路径
  - MAIN_EXE_NAME：程序A的文件名（如program_updater.exe）
- 启动流程：
  1. 检查SOURCE_DIR是否存在MAIN_EXE_NAME
  2. 若存在，复制到MAIN_EXE_PATH
  3. 删除SOURCE_DIR中的MAIN_EXE_NAME
  4. 启动程序A
  5. 退出

### 程序A（program_updater）修改
- 移除自我更新检查逻辑
- 移除self_update模块的调用

### 配置项调整
- 移除HELPER_EXE配置
- 新增MAIN_EXE_PATH配置（程序A的完整路径）
- 新增MAIN_EXE_NAME配置（程序A的文件名）

## 影响范围
- 修改 `config.rs`：调整配置项
- 修改 `main.rs`：移除自我更新检查
- 重写 `src/bin/updater_helper.rs`：改为无参数模式
- 可选：移除或保留 `self_update.rs`
