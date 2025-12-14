//!时间工具模块
//!
//!提供日期时间处理、格式化、计算等常用功能。
//!
//!依赖：chrono（使用时查询最新版本：https://crates.io/crates/chrono）
//!
//!# Cargo.toml 配置示例
//!```toml
//![dependencies]
//!chrono = "0.4"  # https://crates.io/crates/chrono
//!```
//!
//!# 快速开始
//!
//!## 获取当前时间
//!```rust
//!mod datetime;
//!
//!fn main() {
//!    let now = datetime::now();
//!    println!("当前时间: {}", datetime::format_default(&now));
//!
//!    let ts = datetime::timestamp();
//!    println!("时间戳: {}", ts);
//!}
//!```
//!
//!## 时间格式化
//!```rust
//!mod datetime;
//!
//!fn main() {
//!    let now = datetime::now();
//!    println!("{}", datetime::format(&now, "%Y年%m月%d日 %H:%M:%S"));
//!}
//!```

use chrono::{DateTime, Local, Utc, TimeZone, Duration, NaiveDateTime};

//========================================
//类型别名
//========================================

///本地时间类型
pub type LocalDateTime = DateTime<Local>;

///UTC 时间类型
pub type UtcDateTime = DateTime<Utc>;

//========================================
//获取当前时间
//========================================

///获取当前本地时间
pub fn now() -> LocalDateTime {
    Local::now()
}

///获取当前 UTC 时间
pub fn now_utc() -> UtcDateTime {
    Utc::now()
}

///获取当前时间戳（秒）
pub fn timestamp() -> i64 {
    Utc::now().timestamp()
}

///获取当前时间戳（毫秒）
pub fn timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

///获取当前时间戳（纳秒）
pub fn timestamp_nanos() -> i64 {
    Utc::now().timestamp_nanos_opt().unwrap_or(0)
}

//========================================
//时间格式化
//========================================

///常用时间格式
pub mod formats {
    ///默认格式：2024-01-15 13:45:30
    pub const DEFAULT: &str = "%Y-%m-%d %H:%M:%S";

    ///日期格式：2024-01-15
    pub const DATE: &str = "%Y-%m-%d";

    ///时间格式：13:45:30
    pub const TIME: &str = "%H:%M:%S";

    ///ISO 8601 格式
    pub const ISO8601: &str = "%Y-%m-%dT%H:%M:%S%z";

    ///中文日期：2024年01月15日
    pub const DATE_CN: &str = "%Y年%m月%d日";

    ///中文完整：2024年01月15日 13:45:30
    pub const FULL_CN: &str = "%Y年%m月%d日 %H:%M:%S";

    ///紧凑格式：20240115134530
    pub const COMPACT: &str = "%Y%m%d%H%M%S";

    ///日志格式：2024-01-15 13:45:30.123
    pub const LOG: &str = "%Y-%m-%d %H:%M:%S%.3f";
}

///使用默认格式格式化时间
pub fn format_default<Tz: TimeZone>(dt: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format(formats::DEFAULT).to_string()
}

///使用自定义格式格式化时间
pub fn format<Tz: TimeZone>(dt: &DateTime<Tz>, fmt: &str) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format(fmt).to_string()
}

///格式化为日期字符串
pub fn format_date<Tz: TimeZone>(dt: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format(formats::DATE).to_string()
}

///格式化为时间字符串
pub fn format_time<Tz: TimeZone>(dt: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format(formats::TIME).to_string()
}

///格式化为 ISO 8601 字符串
pub fn format_iso<Tz: TimeZone>(dt: &DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.to_rfc3339()
}

//========================================
//时间解析
//========================================

///解析默认格式的时间字符串
pub fn parse(s: &str) -> Result<LocalDateTime, String> {
    parse_with_format(s, formats::DEFAULT)
}

///解析自定义格式的时间字符串
pub fn parse_with_format(s: &str, fmt: &str) -> Result<LocalDateTime, String> {
    NaiveDateTime::parse_from_str(s, fmt)
        .map(|naive| Local.from_local_datetime(&naive).unwrap())
        .map_err(|e| format!("解析失败: {}", e))
}

///解析日期字符串
pub fn parse_date(s: &str) -> Result<LocalDateTime, String> {
    let with_time = format!("{} 00:00:00", s);
    parse(&with_time)
}

///解析 ISO 8601 格式
pub fn parse_iso(s: &str) -> Result<UtcDateTime, String> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| format!("解析失败: {}", e))
}

//========================================
//时间戳转换
//========================================

///从时间戳（秒）创建本地时间
pub fn from_timestamp(ts: i64) -> Option<LocalDateTime> {
    Local.timestamp_opt(ts, 0).single()
}

///从时间戳（毫秒）创建本地时间
pub fn from_timestamp_millis(ts: i64) -> Option<LocalDateTime> {
    Local.timestamp_millis_opt(ts).single()
}

///从时间戳（秒）创建 UTC 时间
pub fn from_timestamp_utc(ts: i64) -> Option<UtcDateTime> {
    Utc.timestamp_opt(ts, 0).single()
}

///转换为时间戳（秒）
pub fn to_timestamp<Tz: TimeZone>(dt: &DateTime<Tz>) -> i64 {
    dt.timestamp()
}

///转换为时间戳（毫秒）
pub fn to_timestamp_millis<Tz: TimeZone>(dt: &DateTime<Tz>) -> i64 {
    dt.timestamp_millis()
}

//========================================
//时间计算
//========================================

///加减天数
pub fn add_days<Tz: TimeZone>(dt: &DateTime<Tz>, days: i64) -> DateTime<Tz> {
    if days >= 0 {
        *dt + Duration::days(days)
    } else {
        *dt - Duration::days(-days)
    }
}

///加减小时
pub fn add_hours<Tz: TimeZone>(dt: &DateTime<Tz>, hours: i64) -> DateTime<Tz> {
    if hours >= 0 {
        *dt + Duration::hours(hours)
    } else {
        *dt - Duration::hours(-hours)
    }
}

///加减分钟
pub fn add_minutes<Tz: TimeZone>(dt: &DateTime<Tz>, minutes: i64) -> DateTime<Tz> {
    if minutes >= 0 {
        *dt + Duration::minutes(minutes)
    } else {
        *dt - Duration::minutes(-minutes)
    }
}

///加减秒数
pub fn add_seconds<Tz: TimeZone>(dt: &DateTime<Tz>, seconds: i64) -> DateTime<Tz> {
    if seconds >= 0 {
        *dt + Duration::seconds(seconds)
    } else {
        *dt - Duration::seconds(-seconds)
    }
}

///计算时间差（返回秒数）
pub fn diff_seconds<Tz1: TimeZone, Tz2: TimeZone>(
    dt1: &DateTime<Tz1>,
    dt2: &DateTime<Tz2>,
) -> i64 {
    (dt1.timestamp() - dt2.timestamp()).abs()
}

///计算时间差（返回 Duration）
pub fn diff<Tz1: TimeZone, Tz2: TimeZone>(
    dt1: &DateTime<Tz1>,
    dt2: &DateTime<Tz2>,
) -> TimeDiff {
    let secs = (dt1.timestamp() - dt2.timestamp()).abs();
    TimeDiff::from_seconds(secs)
}

//========================================
//时间差结构
//========================================

///时间差
#[derive(Debug, Clone, Copy)]
pub struct TimeDiff {
    ///总秒数
    pub total_seconds: i64,
}

impl TimeDiff {
    ///从秒数创建
    pub fn from_seconds(secs: i64) -> Self {
        Self { total_seconds: secs }
    }

    ///获取天数
    pub fn days(&self) -> i64 {
        self.total_seconds / 86400
    }

    ///获取小时数（不含天）
    pub fn hours(&self) -> i64 {
        (self.total_seconds % 86400) / 3600
    }

    ///获取分钟数（不含小时）
    pub fn minutes(&self) -> i64 {
        (self.total_seconds % 3600) / 60
    }

    ///获取秒数（不含分钟）
    pub fn seconds(&self) -> i64 {
        self.total_seconds % 60
    }

    ///获取总小时数
    pub fn total_hours(&self) -> i64 {
        self.total_seconds / 3600
    }

    ///获取总分钟数
    pub fn total_minutes(&self) -> i64 {
        self.total_seconds / 60
    }

    ///人性化显示
    pub fn humanize(&self) -> String {
        let days = self.days();
        let hours = self.hours();
        let minutes = self.minutes();
        let seconds = self.seconds();

        if days > 0 {
            format!("{}天{}小时{}分钟", days, hours, minutes)
        } else if hours > 0 {
            format!("{}小时{}分钟", hours, minutes)
        } else if minutes > 0 {
            format!("{}分钟{}秒", minutes, seconds)
        } else {
            format!("{}秒", seconds)
        }
    }
}

//========================================
//时间比较
//========================================

///判断是否是今天
pub fn is_today(dt: &LocalDateTime) -> bool {
    let today = now();
    dt.date_naive() == today.date_naive()
}

///判断是否是昨天
pub fn is_yesterday(dt: &LocalDateTime) -> bool {
    let yesterday = add_days(&now(), -1);
    dt.date_naive() == yesterday.date_naive()
}

///判断是否在指定时间之前
pub fn is_before<Tz1: TimeZone, Tz2: TimeZone>(
    dt: &DateTime<Tz1>,
    other: &DateTime<Tz2>,
) -> bool {
    dt.timestamp() < other.timestamp()
}

///判断是否在指定时间之后
pub fn is_after<Tz1: TimeZone, Tz2: TimeZone>(
    dt: &DateTime<Tz1>,
    other: &DateTime<Tz2>,
) -> bool {
    dt.timestamp() > other.timestamp()
}

//========================================
//便捷功能
//========================================

///获取今天的开始时间（00:00:00）
pub fn today_start() -> LocalDateTime {
    let today = now();
    Local.with_ymd_and_hms(
        today.year(),
        today.month(),
        today.day(),
        0, 0, 0
    ).unwrap()
}

///获取今天的结束时间（23:59:59）
pub fn today_end() -> LocalDateTime {
    let today = now();
    Local.with_ymd_and_hms(
        today.year(),
        today.month(),
        today.day(),
        23, 59, 59
    ).unwrap()
}

///人性化显示时间（如：刚刚、5分钟前、昨天）
pub fn humanize(dt: &LocalDateTime) -> String {
    let now = now();
    let diff = diff(&now, dt);

    if diff.total_seconds < 60 {
        "刚刚".to_string()
    } else if diff.total_seconds < 3600 {
        format!("{}分钟前", diff.total_minutes())
    } else if diff.total_seconds < 86400 {
        format!("{}小时前", diff.total_hours())
    } else if is_yesterday(dt) {
        format!("昨天 {}", format_time(dt))
    } else if diff.days() < 7 {
        format!("{}天前", diff.days())
    } else {
        format_date(dt)
    }
}

//需要导入年月日方法
use chrono::Datelike;
