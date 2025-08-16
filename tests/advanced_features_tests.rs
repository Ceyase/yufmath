//! # 高级功能测试
//!
//! 测试 Yufmath 库的高级功能，包括配置系统、性能监控、异步计算等。

use yufmath::{
    Yufmath, ComputeConfig, PrecisionConfig, ParallelConfig, CacheConfig, MemoryConfig,
    ComputeProgress, ComputePhase, AsyncConfig, TaskStatus
};
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[test]
fn test_precision_config() {
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(1000)
        .with_symbolic(true)
        .with_approximation_threshold(1e-10);
    
    assert!(precision_config.force_exact);
    assert_eq!(precision_config.max_precision, Some(1000));
    assert!(precision_config.allow_symbolic);
    assert_eq!(precision_config.approximation_threshold, Some(1e-10));
}

#[test]
fn test_parallel_config() {
    let parallel_config = ParallelConfig::new()
        .with_enabled(true)
        .with_thread_count(8)
        .with_complexity_threshold(200)
        .with_max_parallel_tasks(16);
    
    assert!(parallel_config.enabled);
    assert_eq!(parallel_config.thread_count, Some(8));
    assert_eq!(parallel_config.complexity_threshold, 200);
    assert_eq!(parallel_config.max_parallel_tasks, 16);
}

#[test]
fn test_cache_config() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(2000)
        .with_exact_cache_size(1000)
        .with_symbolic_cache_size(500)
        .with_cache_ttl(Duration::from_secs(7200));
    
    assert!(cache_config.enabled);
    assert_eq!(cache_config.fast_cache_size, 2000);
    assert_eq!(cache_config.exact_cache_size, 1000);
    assert_eq!(cache_config.symbolic_cache_size, 500);
    assert_eq!(cache_config.cache_ttl, Some(Duration::from_secs(7200)));
}

#[test]
fn test_memory_config() {
    let memory_config = MemoryConfig::new()
        .with_max_memory_usage(2 * 1024 * 1024 * 1024) // 2GB
        .with_cleanup_threshold(0.9)
        .with_auto_gc(true)
        .with_gc_interval(Duration::from_secs(30));
    
    assert_eq!(memory_config.max_memory_usage, Some(2 * 1024 * 1024 * 1024));
    assert_eq!(memory_config.cleanup_threshold, 0.9);
    assert!(memory_config.auto_gc);
    assert_eq!(memory_config.gc_interval, Duration::from_secs(30));
}

#[test]
fn test_complete_compute_config() {
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(500);
    
    let parallel_config = ParallelConfig::new()
        .with_enabled(true)
        .with_thread_count(4);
    
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(1500);
    
    let memory_config = MemoryConfig::new()
        .with_max_memory_usage(1024 * 1024 * 1024); // 1GB
    
    let compute_config = ComputeConfig::new()
        .with_progress(true)
        .with_max_compute_time(Duration::from_secs(120))
        .with_precision(precision_config)
        .with_parallel(parallel_config)
        .with_cache(cache_config)
        .with_memory(memory_config);
    
    assert!(compute_config.enable_progress);
    assert_eq!(compute_config.max_compute_time, Some(Duration::from_secs(120)));
    assert!(compute_config.precision.force_exact);
    assert_eq!(compute_config.precision.max_precision, Some(500));
    assert!(compute_config.parallel.enabled);
    assert_eq!(compute_config.parallel.thread_count, Some(4));
    assert!(compute_config.cache.enabled);
    assert_eq!(compute_config.cache.fast_cache_size, 1500);
    assert_eq!(compute_config.memory.max_memory_usage, Some(1024 * 1024 * 1024));
}

#[test]
fn test_yufmath_with_advanced_config() {
    let config = ComputeConfig::new()
        .with_progress(true)
        .with_parallel(ParallelConfig::new().with_enabled(true).with_thread_count(2))
        .with_cache(CacheConfig::new().with_enabled(true))
        .with_memory(MemoryConfig::new().with_auto_gc(true));
    
    let yuf = Yufmath::with_config(config);
    
    // 测试基本功能仍然工作
    let result = yuf.compute("2 + 3");
    assert!(result.is_ok());
    
    // 测试配置访问
    let yuf_config = yuf.get_config();
    assert!(yuf_config.enable_progress);
    assert!(yuf_config.parallel.enabled);
    assert_eq!(yuf_config.parallel.thread_count, Some(2));
    assert!(yuf_config.cache.enabled);
    assert!(yuf_config.memory.auto_gc);
}

#[test]
fn test_enhanced_progress_info() {
    let progress = ComputeProgress::new("测试步骤")
        .with_progress(0.5)
        .with_phase(ComputePhase::Computation)
        .with_details("正在处理复杂表达式")
        .with_memory_usage(1024 * 1024) // 1MB
        .with_cache_hit_rate(0.85)
        .with_subtasks(5, 10);
    
    assert_eq!(progress.current_step, "测试步骤");
    assert_eq!(progress.progress, 0.5);
    assert_eq!(progress.phase, ComputePhase::Computation);
    assert_eq!(progress.details, Some("正在处理复杂表达式".to_string()));
    assert_eq!(progress.memory_usage, 1024 * 1024);
    assert_eq!(progress.cache_hit_rate, 0.85);
    assert_eq!(progress.completed_subtasks, 5);
    assert_eq!(progress.total_subtasks, 10);
}

#[test]
fn test_performance_monitor_enhanced() {
    let mut monitor = yufmath::PerformanceMonitor::new();
    
    // 记录一些操作
    monitor.record_cache_hit();
    monitor.record_cache_hit();
    monitor.record_cache_miss();
    monitor.record_parallel_computation();
    monitor.update_memory_usage(2 * 1024 * 1024); // 2MB
    
    let stats = monitor.get_stats();
    assert_eq!(stats.cache_hits, 2);
    assert_eq!(stats.cache_misses, 1);
    assert_eq!(stats.parallel_computations, 1);
    assert_eq!(stats.memory_usage, 2 * 1024 * 1024);
    
    // 测试计算的统计方法
    assert_eq!(stats.cache_hit_rate(), 2.0 / 3.0);
    
    // 测试详细报告
    let report = monitor.get_detailed_report();
    assert!(report.contains("性能统计报告"));
    assert!(report.contains("缓存命中次数: 2"));
    assert!(report.contains("缓存未命中次数: 1"));
}

#[test]
fn test_async_config() {
    let async_config = AsyncConfig::new()
        .with_max_concurrent_tasks(8)
        .with_task_timeout(Duration::from_secs(600))
        .with_progress(true)
        .with_progress_interval(Duration::from_millis(50));
    
    assert_eq!(async_config.max_concurrent_tasks, 8);
    assert_eq!(async_config.task_timeout, Duration::from_secs(600));
    assert!(async_config.enable_progress);
    assert_eq!(async_config.progress_interval, Duration::from_millis(50));
}

#[test]
fn test_async_computation_basic() {
    let yuf = Yufmath::new();
    
    // 测试异步计算
    let async_computation = yuf.compute_async("2 + 3");
    
    // 检查初始状态
    let status = async_computation.status();
    assert!(matches!(status, TaskStatus::Pending | TaskStatus::Running));
    
    // 测试批量异步计算
    let expressions = vec!["1 + 1", "2 * 3", "4 / 2"];
    let async_computations = yuf.batch_compute_async(&expressions);
    assert_eq!(async_computations.len(), 3);
    
    // 检查活跃任务数
    let active_count = yuf.active_async_tasks();
    assert!(active_count > 0);
}

#[test]
fn test_async_task_management() {
    let yuf = Yufmath::new();
    
    // 提交一些异步任务
    let _computation1 = yuf.compute_async("x^2 + 1");
    let _computation2 = yuf.compute_async("sin(x) + cos(x)");
    
    // 检查活跃任务数
    let initial_count = yuf.active_async_tasks();
    assert!(initial_count >= 2);
    
    // 取消所有任务
    yuf.cancel_all_async_tasks();
    
    // 清理已完成的任务
    yuf.cleanup_async_tasks();
    
    // 注意：由于异步执行，任务可能还在运行中
    // 这里主要测试方法调用不会出错
}

#[test]
fn test_memory_config_edge_cases() {
    // 测试无内存限制
    let config = MemoryConfig::new().without_memory_limit();
    assert!(config.max_memory_usage.is_none());
    
    // 测试清理阈值边界值
    let config = MemoryConfig::new().with_cleanup_threshold(1.5); // 超过1.0
    assert_eq!(config.cleanup_threshold, 1.0); // 应该被限制为1.0
    
    let config = MemoryConfig::new().with_cleanup_threshold(-0.5); // 小于0.0
    assert_eq!(config.cleanup_threshold, 0.0); // 应该被限制为0.0
}

#[test]
fn test_cache_config_ttl() {
    // 测试禁用缓存过期
    let config = CacheConfig::new().without_cache_ttl();
    assert!(config.cache_ttl.is_none());
    
    // 测试设置缓存过期时间
    let config = CacheConfig::new().with_cache_ttl(Duration::from_secs(3600));
    assert_eq!(config.cache_ttl, Some(Duration::from_secs(3600)));
}

#[test]
fn test_compute_phases() {
    // 测试所有计算阶段
    let phases = vec![
        ComputePhase::Parsing,
        ComputePhase::Simplification,
        ComputePhase::Computation,
        ComputePhase::Formatting,
        ComputePhase::Completed,
        ComputePhase::Error,
    ];
    
    for phase in phases {
        let progress = ComputeProgress::new_with_phase("测试", phase.clone());
        assert_eq!(progress.phase, phase);
    }
}

#[test]
fn test_performance_stats_calculations() {
    let mut stats = yufmath::PerformanceStats::new();
    
    // 手动设置一些统计数据
    stats.total_computations = 100;
    stats.successful_computations = 85;
    stats.cache_hits = 60;
    stats.cache_misses = 40;
    stats.parallel_computations = 25;
    stats.total_compute_time = Duration::from_secs(50);
    
    // 测试计算方法
    assert_eq!(stats.success_rate(), 0.85);
    assert_eq!(stats.failed_computations(), 15);
    assert_eq!(stats.cache_hit_rate(), 0.6);
    assert_eq!(stats.parallel_computation_ratio(), 0.25);
    assert_eq!(stats.throughput(), 2.0); // 100 computations / 50 seconds
}

#[test]
fn test_yufmath_config_updates() {
    let mut yuf = Yufmath::new();
    
    // 获取初始配置
    let initial_config = yuf.get_config().clone();
    assert!(initial_config.enable_progress);
    
    // 更新配置
    let new_config = ComputeConfig::new()
        .with_progress(false)
        .with_max_compute_time(Duration::from_secs(60));
    
    yuf.update_config(new_config);
    
    // 验证配置更新
    let updated_config = yuf.get_config();
    assert!(!updated_config.enable_progress);
    assert_eq!(updated_config.max_compute_time, Some(Duration::from_secs(60)));
}

#[test]
fn test_batch_async_computer() {
    let computer = yufmath::BatchAsyncComputer::new(4);
    
    let expressions = vec![
        "2 + 3".to_string(),
        "x^2 + 1".to_string(),
        "sin(pi/2)".to_string(),
    ];
    
    let computations = computer.submit_batch(expressions);
    assert_eq!(computations.len(), 3);
    
    // 检查活跃任务数
    let active_count = computer.active_task_count();
    assert!(active_count <= 3);
    
    // 取消所有任务
    computer.cancel_all();
    
    // 清理已完成任务
    computer.cleanup_completed();
}