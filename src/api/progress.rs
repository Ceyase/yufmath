//! # 进度监控和性能统计
//!
//! 提供计算进度监控和性能统计功能。

use std::time::{Duration, Instant};

/// 计算进度信息
#[derive(Debug, Clone)]
pub struct ComputeProgress {
    /// 当前步骤描述
    pub current_step: String,
    /// 完成百分比 (0.0 - 1.0)
    pub progress: f64,
    /// 预估剩余时间
    pub estimated_remaining: Option<Duration>,
    /// 当前处理的表达式大小
    pub expression_size: usize,
    /// 已完成的子任务数
    pub completed_subtasks: usize,
    /// 总子任务数
    pub total_subtasks: usize,
}

impl ComputeProgress {
    /// 创建新的进度信息
    pub fn new(current_step: impl Into<String>) -> Self {
        Self {
            current_step: current_step.into(),
            progress: 0.0,
            estimated_remaining: None,
            expression_size: 0,
            completed_subtasks: 0,
            total_subtasks: 0,
        }
    }
    
    /// 更新进度百分比
    pub fn with_progress(mut self, progress: f64) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }
    
    /// 设置预估剩余时间
    pub fn with_estimated_remaining(mut self, duration: Duration) -> Self {
        self.estimated_remaining = Some(duration);
        self
    }
    
    /// 设置表达式大小
    pub fn with_expression_size(mut self, size: usize) -> Self {
        self.expression_size = size;
        self
    }
    
    /// 设置子任务信息
    pub fn with_subtasks(mut self, completed: usize, total: usize) -> Self {
        self.completed_subtasks = completed;
        self.total_subtasks = total;
        if total > 0 {
            self.progress = completed as f64 / total as f64;
        }
        self
    }
}

/// 进度回调函数类型
/// 返回 false 表示用户请求取消计算
pub type ProgressCallback = Box<dyn Fn(&ComputeProgress) -> bool + Send + Sync>;

/// 性能统计
#[derive(Debug, Default)]
pub struct PerformanceStats {
    /// 缓存命中率
    pub cache_hit_rate: f64,
    /// 平均计算时间
    pub avg_compute_time: Duration,
    /// 内存使用量（字节）
    pub memory_usage: usize,
    /// 精确计算比例
    pub exact_computation_ratio: f64,
    /// 最近的进度信息
    pub last_progress: Option<ComputeProgress>,
    /// 总计算次数
    pub total_computations: usize,
    /// 成功计算次数
    pub successful_computations: usize,
}

impl PerformanceStats {
    /// 创建新的性能统计
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_computations == 0 {
            0.0
        } else {
            self.successful_computations as f64 / self.total_computations as f64
        }
    }
    
    /// 获取失败次数
    pub fn failed_computations(&self) -> usize {
        self.total_computations.saturating_sub(self.successful_computations)
    }
}

/// 性能监控器
pub struct PerformanceMonitor {
    stats: PerformanceStats,
    start_time: Instant,
    progress_callback: Option<ProgressCallback>,
    total_compute_time: Duration,
    computation_count: usize,
}

impl PerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new() -> Self {
        Self {
            stats: PerformanceStats::new(),
            start_time: Instant::now(),
            progress_callback: None,
            total_compute_time: Duration::ZERO,
            computation_count: 0,
        }
    }
    
    /// 设置进度回调函数
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }
    
    /// 更新计算进度
    /// 返回 false 表示用户请求取消计算
    pub fn update_progress(&mut self, progress: ComputeProgress) -> bool {
        self.stats.last_progress = Some(progress.clone());
        
        if let Some(ref callback) = self.progress_callback {
            callback(&progress)
        } else {
            true
        }
    }
    
    /// 记录计算开始
    pub fn start_computation(&mut self) -> ComputationTimer {
        ComputationTimer::new()
    }
    
    /// 记录计算完成
    pub fn record_computation(&mut self, timer: ComputationTimer, exact: bool, success: bool) {
        let duration = timer.elapsed();
        self.total_compute_time += duration;
        self.computation_count += 1;
        
        // 更新统计信息
        self.stats.total_computations += 1;
        if success {
            self.stats.successful_computations += 1;
        }
        
        // 更新平均计算时间
        self.stats.avg_compute_time = self.total_compute_time / self.computation_count as u32;
        
        // 更新精确计算比例（这里需要更复杂的逻辑来跟踪）
        if exact {
            // 简化的实现，实际应该维护更详细的统计
            self.stats.exact_computation_ratio = 
                (self.stats.exact_computation_ratio * (self.computation_count - 1) as f64 + 1.0) 
                / self.computation_count as f64;
        } else {
            self.stats.exact_computation_ratio = 
                (self.stats.exact_computation_ratio * (self.computation_count - 1) as f64) 
                / self.computation_count as f64;
        }
    }
    
    /// 获取性能统计
    pub fn get_stats(&self) -> &PerformanceStats {
        &self.stats
    }
    
    /// 重置统计信息
    pub fn reset_stats(&mut self) {
        self.stats = PerformanceStats::new();
        self.total_compute_time = Duration::ZERO;
        self.computation_count = 0;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// 计算计时器
pub struct ComputationTimer {
    start_time: Instant,
}

impl ComputationTimer {
    /// 创建新的计时器
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
    
    /// 获取已经过的时间
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}