//! # 配置选项定义
//!
//! 定义 Yufmath 库的各种配置选项。

use std::time::Duration;

/// 并行计算配置
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// 是否启用并行计算
    pub enabled: bool,
    /// 线程池大小
    pub thread_count: Option<usize>,
    /// 并行计算的最小表达式复杂度阈值
    pub complexity_threshold: usize,
    /// 最大并行任务数
    pub max_parallel_tasks: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            thread_count: None, // 使用系统默认
            complexity_threshold: 100,
            max_parallel_tasks: 8,
        }
    }
}

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// 是否启用缓存
    pub enabled: bool,
    /// 快速缓存大小（小整数运算结果）
    pub fast_cache_size: usize,
    /// 精确缓存大小（任意精度运算结果）
    pub exact_cache_size: usize,
    /// 符号缓存大小（符号简化结果）
    pub symbolic_cache_size: usize,
    /// 缓存过期时间
    pub cache_ttl: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fast_cache_size: 1000,
            exact_cache_size: 500,
            symbolic_cache_size: 200,
            cache_ttl: Some(Duration::from_secs(3600)), // 1小时
        }
    }
}

/// 内存管理配置
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// 最大内存使用量（字节）
    pub max_memory_usage: Option<usize>,
    /// 内存清理阈值
    pub cleanup_threshold: f64,
    /// 是否启用自动垃圾回收
    pub auto_gc: bool,
    /// 垃圾回收间隔
    pub gc_interval: Duration,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_usage: Some(1024 * 1024 * 1024), // 1GB
            cleanup_threshold: 0.8, // 80%
            auto_gc: true,
            gc_interval: Duration::from_secs(60), // 1分钟
        }
    }
}

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
    /// 并行计算配置
    pub parallel: ParallelConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 内存管理配置
    pub memory: MemoryConfig,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            enable_progress: true,
            progress_interval_ms: 100, // 每100ms更新一次进度
            max_compute_time: Some(Duration::from_secs(300)), // 5分钟超时
            allow_cancellation: true,
            precision: PrecisionConfig::default(),
            parallel: ParallelConfig::default(),
            cache: CacheConfig::default(),
            memory: MemoryConfig::default(),
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
    
    /// 设置并行计算配置
    pub fn with_parallel(mut self, parallel: ParallelConfig) -> Self {
        self.parallel = parallel;
        self
    }
    
    /// 设置缓存配置
    pub fn with_cache(mut self, cache: CacheConfig) -> Self {
        self.cache = cache;
        self
    }
    
    /// 设置内存管理配置
    pub fn with_memory(mut self, memory: MemoryConfig) -> Self {
        self.memory = memory;
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

impl ParallelConfig {
    /// 创建新的并行配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置是否启用并行计算
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// 设置线程数
    pub fn with_thread_count(mut self, count: usize) -> Self {
        self.thread_count = Some(count);
        self
    }
    
    /// 设置复杂度阈值
    pub fn with_complexity_threshold(mut self, threshold: usize) -> Self {
        self.complexity_threshold = threshold;
        self
    }
    
    /// 设置最大并行任务数
    pub fn with_max_parallel_tasks(mut self, max_tasks: usize) -> Self {
        self.max_parallel_tasks = max_tasks;
        self
    }
}

impl CacheConfig {
    /// 创建新的缓存配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置是否启用缓存
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// 设置快速缓存大小
    pub fn with_fast_cache_size(mut self, size: usize) -> Self {
        self.fast_cache_size = size;
        self
    }
    
    /// 设置精确缓存大小
    pub fn with_exact_cache_size(mut self, size: usize) -> Self {
        self.exact_cache_size = size;
        self
    }
    
    /// 设置符号缓存大小
    pub fn with_symbolic_cache_size(mut self, size: usize) -> Self {
        self.symbolic_cache_size = size;
        self
    }
    
    /// 设置缓存过期时间
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = Some(ttl);
        self
    }
    
    /// 禁用缓存过期
    pub fn without_cache_ttl(mut self) -> Self {
        self.cache_ttl = None;
        self
    }
}

impl MemoryConfig {
    /// 创建新的内存配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置最大内存使用量
    pub fn with_max_memory_usage(mut self, max_bytes: usize) -> Self {
        self.max_memory_usage = Some(max_bytes);
        self
    }
    
    /// 禁用内存限制
    pub fn without_memory_limit(mut self) -> Self {
        self.max_memory_usage = None;
        self
    }
    
    /// 设置内存清理阈值
    pub fn with_cleanup_threshold(mut self, threshold: f64) -> Self {
        self.cleanup_threshold = threshold.clamp(0.0, 1.0);
        self
    }
    
    /// 设置是否启用自动垃圾回收
    pub fn with_auto_gc(mut self, auto_gc: bool) -> Self {
        self.auto_gc = auto_gc;
        self
    }
    
    /// 设置垃圾回收间隔
    pub fn with_gc_interval(mut self, interval: Duration) -> Self {
        self.gc_interval = interval;
        self
    }
}