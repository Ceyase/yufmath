//! # 配置选项定义
//!
//! 定义 Yufmath 库的各种配置选项。

use std::time::Duration;

/// 计算配置
#[derive(Debug, Clone)]
pub struct ComputeConfig {
    /// 是否启用进度报告
    pub enable_progress: bool,
    /// 进度更新间隔（毫秒）
    pub progress_interval_ms: u64,
    /// 最大计算时间限制
    pub max_compute_time: Option<Duration>,
    /// 是否允许用户取消计算
    pub allow_cancellation: bool,
    /// 精度配置
    pub precision: PrecisionConfig,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            enable_progress: true,
            progress_interval_ms: 100, // 每100ms更新一次进度
            max_compute_time: Some(Duration::from_secs(300)), // 5分钟超时
            allow_cancellation: true,
            precision: PrecisionConfig::default(),
        }
    }
}

/// 精确计算配置
#[derive(Debug, Clone)]
pub struct PrecisionConfig {
    /// 是否强制使用精确计算
    pub force_exact: bool,
    /// 任意精度计算的最大位数限制
    pub max_precision: Option<usize>,
    /// 是否允许符号表示
    pub allow_symbolic: bool,
    /// 数值近似的阈值
    pub approximation_threshold: Option<f64>,
}

impl Default for PrecisionConfig {
    fn default() -> Self {
        Self {
            force_exact: true,
            max_precision: None, // 无限制
            allow_symbolic: true,
            approximation_threshold: None,
        }
    }
}

impl ComputeConfig {
    /// 创建新的计算配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置是否启用进度报告
    pub fn with_progress(mut self, enable: bool) -> Self {
        self.enable_progress = enable;
        self
    }
    
    /// 设置进度更新间隔
    pub fn with_progress_interval(mut self, interval_ms: u64) -> Self {
        self.progress_interval_ms = interval_ms;
        self
    }
    
    /// 设置最大计算时间
    pub fn with_max_compute_time(mut self, duration: Duration) -> Self {
        self.max_compute_time = Some(duration);
        self
    }
    
    /// 设置是否允许取消
    pub fn with_cancellation(mut self, allow: bool) -> Self {
        self.allow_cancellation = allow;
        self
    }
    
    /// 设置精度配置
    pub fn with_precision(mut self, precision: PrecisionConfig) -> Self {
        self.precision = precision;
        self
    }
}

impl PrecisionConfig {
    /// 创建新的精度配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置是否强制精确计算
    pub fn with_force_exact(mut self, force: bool) -> Self {
        self.force_exact = force;
        self
    }
    
    /// 设置最大精度限制
    pub fn with_max_precision(mut self, max_precision: usize) -> Self {
        self.max_precision = Some(max_precision);
        self
    }
    
    /// 设置是否允许符号表示
    pub fn with_symbolic(mut self, allow: bool) -> Self {
        self.allow_symbolic = allow;
        self
    }
    
    /// 设置数值近似阈值
    pub fn with_approximation_threshold(mut self, threshold: f64) -> Self {
        self.approximation_threshold = Some(threshold);
        self
    }
}