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
    /// 当前阶段
    pub phase: ComputePhase,
    /// 详细信息
    pub details: Option<String>,
    /// 内存使用量
    pub memory_usage: usize,
    /// 缓存命中率
    pub cache_hit_rate: f64,
}

/// 计算阶段
#[derive(Debug, Clone, PartialEq)]
pub enum ComputePhase {
    /// 解析阶段
    Parsing,
    /// 简化阶段
    Simplification,
    /// 计算阶段
    Computation,
    /// 格式化阶段
    Formatting,
    /// 完成
    Completed,
    /// 错误
    Error,
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
            phase: ComputePhase::Parsing,
            details: None,
            memory_usage: 0,
            cache_hit_rate: 0.0,
        }
    }
    
    /// 创建带阶段的进度信息
    pub fn new_with_phase(current_step: impl Into<String>, phase: ComputePhase) -> Self {
        Self {
            current_step: current_step.into(),
            progress: 0.0,
            estimated_remaining: None,
            expression_size: 0,
            completed_subtasks: 0,
            total_subtasks: 0,
            phase,
            details: None,
            memory_usage: 0,
            cache_hit_rate: 0.0,
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
    
    /// 设置计算阶段
    pub fn with_phase(mut self, phase: ComputePhase) -> Self {
        self.phase = phase;
        self
    }
    
    /// 设置详细信息
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
    
    /// 设置内存使用量
    pub fn with_memory_usage(mut self, memory_usage: usize) -> Self {
        self.memory_usage = memory_usage;
        self
    }
    
    /// 设置缓存命中率
    pub fn with_cache_hit_rate(mut self, hit_rate: f64) -> Self {
        self.cache_hit_rate = hit_rate.clamp(0.0, 1.0);
        self
    }
}

/// 进度回调函数类型
/// 返回 false 表示用户请求取消计算
pub type ProgressCallback = Box<dyn Fn(&ComputeProgress) -> bool + Send + Sync>;

/// 性能统计
#[derive(Debug, Default, Clone)]
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
    /// 最快计算时间
    pub fastest_computation: Duration,
    /// 最慢计算时间
    pub slowest_computation: Duration,
    /// 总计算时间
    pub total_compute_time: Duration,
    /// 并行计算次数
    pub parallel_computations: usize,
    /// 缓存命中次数
    pub cache_hits: usize,
    /// 缓存未命中次数
    pub cache_misses: usize,
    /// 内存清理次数
    pub gc_count: usize,
    /// 最后一次垃圾回收时间
    pub last_gc_time: Option<Instant>,
}

impl PerformanceStats {
    /// 创建新的性能统计
    pub fn new() -> Self {
        Self {
            fastest_computation: Duration::MAX,
            slowest_computation: Duration::ZERO,
            ..Default::default()
        }
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
    
    /// 获取缓存命中率
    pub fn cache_hit_rate(&self) -> f64 {
        let total_cache_requests = self.cache_hits + self.cache_misses;
        if total_cache_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total_cache_requests as f64
        }
    }
    
    /// 获取并行计算比例
    pub fn parallel_computation_ratio(&self) -> f64 {
        if self.total_computations == 0 {
            0.0
        } else {
            self.parallel_computations as f64 / self.total_computations as f64
        }
    }
    
    /// 获取平均内存使用量
    pub fn avg_memory_usage(&self) -> usize {
        self.memory_usage
    }
    
    /// 获取计算吞吐量（计算/秒）
    pub fn throughput(&self) -> f64 {
        if self.total_compute_time.as_secs_f64() == 0.0 {
            0.0
        } else {
            self.total_computations as f64 / self.total_compute_time.as_secs_f64()
        }
    }
}

/// 性能监控器
pub struct PerformanceMonitor {
    stats: PerformanceStats,
    start_time: Instant,
    progress_callback: Option<ProgressCallback>,
    total_compute_time: Duration,
    computation_count: usize,
    last_gc_check: Instant,
    memory_samples: Vec<(Instant, usize)>,
}

impl PerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            stats: PerformanceStats::new(),
            start_time: now,
            progress_callback: None,
            total_compute_time: Duration::ZERO,
            computation_count: 0,
            last_gc_check: now,
            memory_samples: Vec::new(),
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
        self.stats.total_compute_time += duration;
        
        if success {
            self.stats.successful_computations += 1;
        }
        
        // 更新最快/最慢计算时间
        if duration < self.stats.fastest_computation {
            self.stats.fastest_computation = duration;
        }
        if duration > self.stats.slowest_computation {
            self.stats.slowest_computation = duration;
        }
        
        // 更新平均计算时间
        self.stats.avg_compute_time = self.total_compute_time / self.computation_count as u32;
        
        // 更新精确计算比例
        if exact {
            self.stats.exact_computation_ratio = 
                (self.stats.exact_computation_ratio * (self.computation_count - 1) as f64 + 1.0) 
                / self.computation_count as f64;
        } else {
            self.stats.exact_computation_ratio = 
                (self.stats.exact_computation_ratio * (self.computation_count - 1) as f64) 
                / self.computation_count as f64;
        }
        
        // 记录内存样本
        self.record_memory_sample();
    }
    
    /// 记录缓存命中
    pub fn record_cache_hit(&mut self) {
        self.stats.cache_hits += 1;
        self.update_cache_hit_rate();
    }
    
    /// 记录缓存未命中
    pub fn record_cache_miss(&mut self) {
        self.stats.cache_misses += 1;
        self.update_cache_hit_rate();
    }
    
    /// 记录并行计算
    pub fn record_parallel_computation(&mut self) {
        self.stats.parallel_computations += 1;
    }
    
    /// 记录垃圾回收
    pub fn record_gc(&mut self) {
        self.stats.gc_count += 1;
        self.stats.last_gc_time = Some(Instant::now());
    }
    
    /// 更新内存使用量
    pub fn update_memory_usage(&mut self, memory_usage: usize) {
        self.stats.memory_usage = memory_usage;
        self.record_memory_sample();
    }
    
    /// 检查是否需要垃圾回收
    pub fn should_gc(&mut self, memory_config: &super::config::MemoryConfig) -> bool {
        let now = Instant::now();
        
        // 检查是否到了垃圾回收间隔
        if now.duration_since(self.last_gc_check) >= memory_config.gc_interval {
            self.last_gc_check = now;
            
            // 检查内存使用是否超过阈值
            if let Some(max_memory) = memory_config.max_memory_usage {
                let threshold = (max_memory as f64 * memory_config.cleanup_threshold) as usize;
                return self.stats.memory_usage > threshold;
            }
        }
        
        false
    }
    
    /// 记录内存样本
    fn record_memory_sample(&mut self) {
        let now = Instant::now();
        self.memory_samples.push((now, self.stats.memory_usage));
        
        // 保持最近100个样本
        if self.memory_samples.len() > 100 {
            self.memory_samples.remove(0);
        }
    }
    
    /// 更新缓存命中率
    fn update_cache_hit_rate(&mut self) {
        let total = self.stats.cache_hits + self.stats.cache_misses;
        if total > 0 {
            self.stats.cache_hit_rate = self.stats.cache_hits as f64 / total as f64;
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
        self.memory_samples.clear();
        self.last_gc_check = Instant::now();
    }
    
    /// 获取内存使用趋势
    pub fn get_memory_trend(&self) -> Vec<(Duration, usize)> {
        let start_time = self.start_time;
        self.memory_samples.iter()
            .map(|(time, usage)| (time.duration_since(start_time), *usage))
            .collect()
    }
    
    /// 获取详细的性能报告
    pub fn get_detailed_report(&self) -> String {
        format!(
            "性能统计报告:\n\
            ================\n\
            总计算次数: {}\n\
            成功计算次数: {}\n\
            失败计算次数: {}\n\
            成功率: {:.2}%\n\
            \n\
            时间统计:\n\
            --------\n\
            总计算时间: {:?}\n\
            平均计算时间: {:?}\n\
            最快计算时间: {:?}\n\
            最慢计算时间: {:?}\n\
            计算吞吐量: {:.2} 计算/秒\n\
            \n\
            缓存统计:\n\
            --------\n\
            缓存命中次数: {}\n\
            缓存未命中次数: {}\n\
            缓存命中率: {:.2}%\n\
            \n\
            并行计算:\n\
            --------\n\
            并行计算次数: {}\n\
            并行计算比例: {:.2}%\n\
            \n\
            内存统计:\n\
            --------\n\
            当前内存使用: {} 字节\n\
            垃圾回收次数: {}\n\
            \n\
            精确计算:\n\
            --------\n\
            精确计算比例: {:.2}%\n",
            self.stats.total_computations,
            self.stats.successful_computations,
            self.stats.failed_computations(),
            self.stats.success_rate() * 100.0,
            self.stats.total_compute_time,
            self.stats.avg_compute_time,
            if self.stats.fastest_computation == Duration::MAX { Duration::ZERO } else { self.stats.fastest_computation },
            self.stats.slowest_computation,
            self.stats.throughput(),
            self.stats.cache_hits,
            self.stats.cache_misses,
            self.stats.cache_hit_rate() * 100.0,
            self.stats.parallel_computations,
            self.stats.parallel_computation_ratio() * 100.0,
            self.stats.memory_usage,
            self.stats.gc_count,
            self.stats.exact_computation_ratio * 100.0
        )
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