//! 时间模块
//!
//! 提供一些和时间相关的工具

mod timer;

pub use timer::ExecutionTimer;

/// 格式化时间
///
/// # 返回值
///
/// 将 Duration 格式化为 `HH:MM:SS.mmm.µµµ.nnn` 的形式
///
/// - HH: 小时
/// - MM: 分钟
/// - SS: 秒
/// - mmm: 毫秒
/// - µµµ: 微秒
/// - nnn: 纳秒
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    let millis = nanos / 1_000_000;
    let micros = (nanos / 1_000) % 1_000;
    let nanos = nanos % 1_000;
    format!("{:02}:{:02}:{:02}.{:03}.{:03}.{:03}", hours, minutes, seconds, millis, micros, nanos)
}