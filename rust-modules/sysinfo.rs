//!系统信息模块
//!
//!提供 CPU、内存、磁盘、网络等系统信息查询功能。
//!
//!依赖：sysinfo（使用时查询最新版本：https://crates.io/crates/sysinfo）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!sysinfo = "0.37"  # https://crates.io/crates/sysinfo
//!```
//!
//!# 快速开始
//!
//!## 获取系统概览
//!```rust
//!mod sysinfo;
//!
//!fn main() {
//!    let info = sysinfo::SystemInfo::new();
//!    println!("CPU 核心数: {}", info.cpu_count());
//!    println!("内存使用率: {:.1}%", info.memory_usage());
//!    println!("主机名: {}", info.hostname());
//!}
//!```

use sysinfo::{System, Disks, Networks, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

//========================================
//系统信息主结构
//========================================

///系统信息
pub struct SystemInfo {
    sys: System,
    disks: Disks,
    networks: Networks,
}

impl SystemInfo {
    ///创建并初始化系统信息（获取所有信息）
    pub fn new() -> Self {
        let mut sys = System::new_all();
        //刷新 CPU 使用率需要两次采样
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
        }
    }

    ///创建轻量级实例（仅基础信息，不获取磁盘和网络）
    pub fn new_light() -> Self {
        let refresh_kind = RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything());

        let mut sys = System::new_with_specifics(refresh_kind);
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        Self {
            sys,
            disks: Disks::new(),
            networks: Networks::new(),
        }
    }

    ///刷新所有信息
    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
    }

    ///刷新 CPU 信息
    pub fn refresh_cpu(&mut self) {
        self.sys.refresh_cpu_usage();
    }

    ///刷新内存信息
    pub fn refresh_memory(&mut self) {
        self.sys.refresh_memory();
    }

    ///刷新磁盘信息
    pub fn refresh_disks(&mut self) {
        self.disks.refresh(true);
    }

    ///刷新网络信息
    pub fn refresh_networks(&mut self) {
        self.networks.refresh(true);
    }
}

//========================================
//CPU 信息
//========================================

impl SystemInfo {
    ///获取 CPU 逻辑核心数
    pub fn cpu_count(&self) -> usize {
        self.sys.cpus().len()
    }

    ///获取物理核心数
    pub fn cpu_physical_count(&self) -> Option<usize> {
        self.sys.physical_core_count()
    }

    ///获取 CPU 总体使用率（0.0-100.0）
    pub fn cpu_usage(&self) -> f32 {
        let cpus = self.sys.cpus();
        if cpus.is_empty() {
            return 0.0;
        }
        cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
    }

    ///获取每个 CPU 核心的使用率
    pub fn cpu_usage_per_core(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|c| c.cpu_usage()).collect()
    }

    ///获取 CPU 品牌名称
    pub fn cpu_brand(&self) -> String {
        self.sys.cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default()
    }

    ///获取 CPU 频率（MHz）
    pub fn cpu_frequency(&self) -> u64 {
        self.sys.cpus()
            .first()
            .map(|c| c.frequency())
            .unwrap_or(0)
    }

    ///获取 CPU 详细信息
    pub fn cpu_info(&self) -> CpuInfo {
        CpuInfo {
            brand: self.cpu_brand(),
            cores: self.cpu_count(),
            physical_cores: self.cpu_physical_count(),
            frequency_mhz: self.cpu_frequency(),
            usage: self.cpu_usage(),
        }
    }
}

///CPU 详细信息
#[derive(Debug, Clone)]
pub struct CpuInfo {
    ///品牌名称
    pub brand: String,
    ///逻辑核心数
    pub cores: usize,
    ///物理核心数
    pub physical_cores: Option<usize>,
    ///频率（MHz）
    pub frequency_mhz: u64,
    ///使用率（0.0-100.0）
    pub usage: f32,
}

//========================================
//内存信息
//========================================

impl SystemInfo {
    ///获取总内存（字节）
    pub fn memory_total(&self) -> u64 {
        self.sys.total_memory()
    }

    ///获取已用内存（字节）
    pub fn memory_used(&self) -> u64 {
        self.sys.used_memory()
    }

    ///获取可用内存（字节）
    pub fn memory_available(&self) -> u64 {
        self.sys.available_memory()
    }

    ///获取内存使用率（0.0-100.0）
    pub fn memory_usage(&self) -> f64 {
        let total = self.memory_total();
        if total == 0 {
            return 0.0;
        }
        (self.memory_used() as f64 / total as f64) * 100.0
    }

    ///获取总交换区（字节）
    pub fn swap_total(&self) -> u64 {
        self.sys.total_swap()
    }

    ///获取已用交换区（字节）
    pub fn swap_used(&self) -> u64 {
        self.sys.used_swap()
    }

    ///获取内存详细信息
    pub fn memory_info(&self) -> MemoryInfo {
        MemoryInfo {
            total: self.memory_total(),
            used: self.memory_used(),
            available: self.memory_available(),
            usage: self.memory_usage(),
            swap_total: self.swap_total(),
            swap_used: self.swap_used(),
        }
    }
}

///内存详细信息
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    ///总内存（字节）
    pub total: u64,
    ///已用内存（字节）
    pub used: u64,
    ///可用内存（字节）
    pub available: u64,
    ///使用率（0.0-100.0）
    pub usage: f64,
    ///总交换区（字节）
    pub swap_total: u64,
    ///已用交换区（字节）
    pub swap_used: u64,
}

impl MemoryInfo {
    ///人性化显示总内存
    pub fn total_human(&self) -> String {
        humanize_bytes(self.total)
    }

    ///人性化显示已用内存
    pub fn used_human(&self) -> String {
        humanize_bytes(self.used)
    }

    ///人性化显示可用内存
    pub fn available_human(&self) -> String {
        humanize_bytes(self.available)
    }
}

//========================================
//磁盘信息
//========================================

impl SystemInfo {
    ///获取所有磁盘信息
    pub fn disks(&self) -> Vec<DiskInfo> {
        self.disks.iter().map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount_point: d.mount_point().to_string_lossy().to_string(),
            file_system: String::from_utf8_lossy(d.file_system()).to_string(),
            total: d.total_space(),
            available: d.available_space(),
            is_removable: d.is_removable(),
        }).collect()
    }

    ///获取指定路径所在磁盘的使用率
    pub fn disk_usage(&self, path: &str) -> Option<f64> {
        let path = std::path::Path::new(path);
        for disk in self.disks.iter() {
            if path.starts_with(disk.mount_point()) {
                let total = disk.total_space();
                if total == 0 {
                    return Some(0.0);
                }
                let used = total - disk.available_space();
                return Some((used as f64 / total as f64) * 100.0);
            }
        }
        None
    }

    ///获取磁盘总数
    pub fn disk_count(&self) -> usize {
        self.disks.iter().count()
    }
}

///磁盘信息
#[derive(Debug, Clone)]
pub struct DiskInfo {
    ///磁盘名称
    pub name: String,
    ///挂载点
    pub mount_point: String,
    ///文件系统类型
    pub file_system: String,
    ///总容量（字节）
    pub total: u64,
    ///可用空间（字节）
    pub available: u64,
    ///是否可移除
    pub is_removable: bool,
}

impl DiskInfo {
    ///获取已用空间（字节）
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }

    ///获取使用率（0.0-100.0）
    pub fn usage(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used() as f64 / self.total as f64) * 100.0
    }

    ///人性化显示总容量
    pub fn total_human(&self) -> String {
        humanize_bytes(self.total)
    }

    ///人性化显示可用空间
    pub fn available_human(&self) -> String {
        humanize_bytes(self.available)
    }

    ///人性化显示已用空间
    pub fn used_human(&self) -> String {
        humanize_bytes(self.used())
    }
}

//========================================
//网络信息
//========================================

impl SystemInfo {
    ///获取所有网络接口信息
    pub fn networks(&self) -> Vec<NetworkInfo> {
        self.networks.iter().map(|(name, data)| NetworkInfo {
            name: name.to_string(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
            packets_received: data.total_packets_received(),
            packets_transmitted: data.total_packets_transmitted(),
        }).collect()
    }

    ///获取网络接口数量
    pub fn network_count(&self) -> usize {
        self.networks.iter().count()
    }

    ///获取指定接口的信息
    pub fn network(&self, name: &str) -> Option<NetworkInfo> {
        self.networks.iter()
            .find(|(n, _)| *n == name)
            .map(|(name, data)| NetworkInfo {
                name: name.to_string(),
                received: data.total_received(),
                transmitted: data.total_transmitted(),
                packets_received: data.total_packets_received(),
                packets_transmitted: data.total_packets_transmitted(),
            })
    }
}

///网络接口信息
#[derive(Debug, Clone)]
pub struct NetworkInfo {
    ///接口名称
    pub name: String,
    ///总接收字节数
    pub received: u64,
    ///总发送字节数
    pub transmitted: u64,
    ///总接收包数
    pub packets_received: u64,
    ///总发送包数
    pub packets_transmitted: u64,
}

impl NetworkInfo {
    ///人性化显示接收数据量
    pub fn received_human(&self) -> String {
        humanize_bytes(self.received)
    }

    ///人性化显示发送数据量
    pub fn transmitted_human(&self) -> String {
        humanize_bytes(self.transmitted)
    }
}

//========================================
//系统基本信息
//========================================

impl SystemInfo {
    ///获取操作系统名称
    pub fn os_name(&self) -> String {
        System::name().unwrap_or_else(|| "Unknown".to_string())
    }

    ///获取操作系统版本
    pub fn os_version(&self) -> String {
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    }

    ///获取内核版本
    pub fn kernel_version(&self) -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    ///获取主机名
    pub fn hostname(&self) -> String {
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    }

    ///获取系统运行时间（秒）
    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    ///获取系统运行时间（人性化显示）
    pub fn uptime_human(&self) -> String {
        humanize_duration(self.uptime())
    }

    ///获取系统架构
    pub fn arch(&self) -> String {
        System::cpu_arch().unwrap_or_else(|| "Unknown".to_string())
    }

    ///获取系统基本信息
    pub fn system_info(&self) -> BasicSystemInfo {
        BasicSystemInfo {
            os_name: self.os_name(),
            os_version: self.os_version(),
            kernel_version: self.kernel_version(),
            hostname: self.hostname(),
            arch: self.arch(),
            uptime: self.uptime(),
        }
    }
}

///系统基本信息
#[derive(Debug, Clone)]
pub struct BasicSystemInfo {
    ///操作系统名称
    pub os_name: String,
    ///操作系统版本
    pub os_version: String,
    ///内核版本
    pub kernel_version: String,
    ///主机名
    pub hostname: String,
    ///系统架构
    pub arch: String,
    ///运行时间（秒）
    pub uptime: u64,
}

impl BasicSystemInfo {
    ///人性化显示运行时间
    pub fn uptime_human(&self) -> String {
        humanize_duration(self.uptime)
    }
}

//========================================
//便捷函数
//========================================

///快速获取 CPU 核心数
pub fn cpu_count() -> usize {
    System::new().cpus().len()
}

///快速获取总内存（字节）
pub fn memory_total() -> u64 {
    let sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::everything())
    );
    sys.total_memory()
}

///快速获取已用内存（字节）
pub fn memory_used() -> u64 {
    let sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::everything())
    );
    sys.used_memory()
}

///快速获取主机名
pub fn hostname() -> String {
    System::host_name().unwrap_or_else(|| "Unknown".to_string())
}

///快速获取操作系统名称
pub fn os_name() -> String {
    System::name().unwrap_or_else(|| "Unknown".to_string())
}

///快速获取系统运行时间（秒）
pub fn uptime() -> u64 {
    System::uptime()
}

//========================================
//工具函数
//========================================

///人性化显示字节数
pub fn humanize_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

///人性化显示时间（秒转换为天时分秒）
pub fn humanize_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}天{}小时{}分钟", days, hours, minutes)
    } else if hours > 0 {
        format!("{}小时{}分钟", hours, minutes)
    } else if minutes > 0 {
        format!("{}分钟{}秒", minutes, secs)
    } else {
        format!("{}秒", secs)
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
