//! 计时器工具模块

/// 执行时间计时器
///
/// 可以用来计记录某个时间段的执行时间
pub struct ExecutionTimer {
    instant: std::time::Instant,
}

impl ExecutionTimer {
    /// 开始一个新的计时器
    pub fn start_timer() -> Self {
        Self { instant: std::time::Instant::now() }
    }

    /// 停止计时器并返回运行时间
    pub fn stop_timer(&self) -> std::time::Duration {
        self.instant.elapsed()
    }
}



