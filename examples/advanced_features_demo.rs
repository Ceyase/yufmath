//! # Yufmath 高级功能演示
//!
//! 本示例展示了 Yufmath 库的高级功能，包括：
//! - 高级配置系统
//! - 性能监控和统计
//! - 异步计算
//! - 内存管理
//! - 缓存优化

use yufmath::{
    Yufmath, ComputeConfig, PrecisionConfig, ParallelConfig, CacheConfig, MemoryConfig,
    ComputeProgress, ComputePhase, AsyncConfig, TaskStatus, BatchAsyncComputer
};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Yufmath 高级功能演示");
    println!("======================");
    
    // 1. 高级配置系统演示
    advanced_configuration_demo()?;
    
    // 2. 性能监控演示
    performance_monitoring_demo()?;
    
    // 3. 异步计算演示
    async_computation_demo()?;
    
    // 4. 内存管理演示
    memory_management_demo()?;
    
    // 5. 缓存优化演示
    cache_optimization_demo()?;
    
    Ok(())
}

/// 高级配置系统演示
fn advanced_configuration_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️ 1. 高级配置系统演示");
    println!("--------------------");
    
    // 创建精度配置
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(1000)
        .with_symbolic(true)
        .with_approximation_threshold(1e-12);
    
    println!("精度配置:");
    println!("  - 强制精确计算: {}", precision_config.force_exact);
    println!("  - 最大精度: {:?}", precision_config.max_precision);
    println!("  - 允许符号表示: {}", precision_config.allow_symbolic);
    println!("  - 近似阈值: {:?}", precision_config.approximation_threshold);
    
    // 创建并行配置
    let parallel_config = ParallelConfig::new()
        .with_enabled(true)
        .with_thread_count(4)
        .with_complexity_threshold(100)
        .with_max_parallel_tasks(8);
    
    println!("\n并行配置:");
    println!("  - 启用并行: {}", parallel_config.enabled);
    println!("  - 线程数: {:?}", parallel_config.thread_count);
    println!("  - 复杂度阈值: {}", parallel_config.complexity_threshold);
    println!("  - 最大并行任务: {}", parallel_config.max_parallel_tasks);
    
    // 创建缓存配置
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(2000)
        .with_exact_cache_size(1000)
        .with_symbolic_cache_size(500)
        .with_cache_ttl(Duration::from_secs(3600));
    
    println!("\n缓存配置:");
    println!("  - 启用缓存: {}", cache_config.enabled);
    println!("  - 快速缓存大小: {}", cache_config.fast_cache_size);
    println!("  - 精确缓存大小: {}", cache_config.exact_cache_size);
    println!("  - 符号缓存大小: {}", cache_config.symbolic_cache_size);
    println!("  - 缓存过期时间: {:?}", cache_config.cache_ttl);
    
    // 创建内存配置
    let memory_config = MemoryConfig::new()
        .with_max_memory_usage(512 * 1024 * 1024) // 512MB
        .with_cleanup_threshold(0.8)
        .with_auto_gc(true)
        .with_gc_interval(Duration::from_secs(60));
    
    println!("\n内存配置:");
    println!("  - 最大内存使用: {:?} 字节", memory_config.max_memory_usage);
    println!("  - 清理阈值: {}", memory_config.cleanup_threshold);
    println!("  - 自动垃圾回收: {}", memory_config.auto_gc);
    println!("  - 垃圾回收间隔: {:?}", memory_config.gc_interval);
    
    // 创建完整的计算配置
    let compute_config = ComputeConfig::new()
        .with_progress(true)
        .with_progress_interval(50)
        .with_max_compute_time(Duration::from_secs(300))
        .with_cancellation(true)
        .with_precision(precision_config)
        .with_parallel(parallel_config)
        .with_cache(cache_config)
        .with_memory(memory_config);
    
    // 使用配置创建 Yufmath 实例
    let yuf = Yufmath::with_config(compute_config);
    
    // 测试配置的效果
    println!("\n测试高级配置:");
    let result = yuf.compute("2^100")?;
    println!("  2^100 = {} (前50位)", &result[..std::cmp::min(50, result.len())]);
    
    Ok(())
}

/// 性能监控演示
fn performance_monitoring_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 2. 性能监控演示");
    println!("----------------");
    
    let mut yuf = Yufmath::new();
    
    // 设置进度回调
    yuf.set_progress_callback(Box::new(|progress| {
        println!("  进度更新: {:.1}% - {} (阶段: {:?})", 
                progress.progress * 100.0, 
                progress.current_step,
                progress.phase);
        
        if let Some(ref details) = progress.details {
            println!("    详细信息: {}", details);
        }
        
        if progress.memory_usage > 0 {
            println!("    内存使用: {} 字节", progress.memory_usage);
        }
        
        if progress.cache_hit_rate > 0.0 {
            println!("    缓存命中率: {:.2}%", progress.cache_hit_rate * 100.0);
        }
        
        true // 继续计算
    }));
    
    // 执行一些计算来生成统计数据
    println!("\n执行计算任务...");
    let expressions = vec![
        "x^2 + 2*x + 1",
        "sin(x) + cos(x)",
        "e^x - 1",
        "ln(x + 1)",
        "sqrt(x^2 + 1)",
    ];
    
    for expr in &expressions {
        let result = yuf.compute_with_progress(expr)?;
        println!("  {} = {}", expr, result);
    }
    
    // 获取性能统计
    if let Some(stats) = yuf.get_performance_stats() {
        println!("\n性能统计摘要:");
        println!("  - 总计算次数: {}", stats.total_computations);
        println!("  - 成功率: {:.2}%", stats.success_rate() * 100.0);
        println!("  - 平均计算时间: {:?}", stats.avg_compute_time);
        println!("  - 最快计算: {:?}", stats.fastest_computation);
        println!("  - 最慢计算: {:?}", stats.slowest_computation);
        println!("  - 计算吞吐量: {:.2} 计算/秒", stats.throughput());
        println!("  - 缓存命中率: {:.2}%", stats.cache_hit_rate() * 100.0);
        println!("  - 并行计算比例: {:.2}%", stats.parallel_computation_ratio() * 100.0);
        println!("  - 精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
    }
    
    Ok(())
}

/// 异步计算演示
fn async_computation_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 3. 异步计算演示");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 单个异步计算
    println!("启动异步计算...");
    let async_computation = yuf.compute_async("integrate(sin(x^2), x)");
    
    println!("  任务状态: {:?}", async_computation.status());
    
    // 批量异步计算
    let expressions = vec![
        "2^10",
        "factorial(10)",
        "sin(pi/4)",
        "e^(i*pi)",
        "sqrt(2)",
    ];
    
    println!("\n启动批量异步计算...");
    let async_computations = yuf.batch_compute_async(&expressions);
    
    println!("  提交了 {} 个异步任务", async_computations.len());
    println!("  当前活跃任务数: {}", yuf.active_async_tasks());
    
    // 监控任务进度
    println!("\n监控任务进度:");
    for (i, computation) in async_computations.iter().enumerate() {
        let status = computation.status();
        let execution_time = computation.execution_time();
        
        println!("  任务 {}: 状态={:?}, 执行时间={:?}", 
                i + 1, status, execution_time);
        
        if let Some(progress) = computation.progress() {
            println!("    进度: {:.1}% - {}", 
                    progress.progress * 100.0, 
                    progress.current_step);
        }
    }
    
    // 等待一段时间让任务完成
    thread::sleep(Duration::from_millis(500));
    
    println!("\n清理已完成的任务...");
    yuf.cleanup_async_tasks();
    println!("  清理后活跃任务数: {}", yuf.active_async_tasks());
    
    Ok(())
}

/// 内存管理演示
fn memory_management_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 4. 内存管理演示");
    println!("----------------");
    
    // 创建带内存管理的配置
    let memory_config = MemoryConfig::new()
        .with_max_memory_usage(64 * 1024 * 1024) // 64MB 限制
        .with_cleanup_threshold(0.7) // 70% 时触发清理
        .with_auto_gc(true)
        .with_gc_interval(Duration::from_secs(5));
    
    let config = ComputeConfig::new()
        .with_memory(memory_config);
    
    let mut yuf = Yufmath::with_config(config);
    
    println!("内存管理配置:");
    println!("  - 最大内存: 64MB");
    println!("  - 清理阈值: 70%");
    println!("  - 自动垃圾回收: 启用");
    println!("  - 垃圾回收间隔: 5秒");
    
    // 执行大量计算来测试内存管理
    println!("\n执行大量计算来测试内存管理...");
    for batch in 0..3 {
        println!("  批次 {}: 执行100个计算", batch + 1);
        
        for i in 0..100 {
            let expr = format!("{}*x^{} + {}", i % 10 + 1, i % 5 + 1, i);
            let _ = yuf.compute(&expr);
        }
        
        // 获取内存统计
        if let Some(stats) = yuf.get_performance_stats() {
            println!("    内存使用: {} 字节", stats.memory_usage);
            println!("    垃圾回收次数: {}", stats.gc_count);
            
            if stats.memory_usage > 32 * 1024 * 1024 { // 32MB
                println!("    内存使用较高，可能触发垃圾回收");
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
    
    // 手动重置统计来模拟内存清理
    println!("\n手动重置性能统计（模拟内存清理）...");
    yuf.reset_performance_stats();
    
    if let Some(stats) = yuf.get_performance_stats() {
        println!("  清理后内存使用: {} 字节", stats.memory_usage);
        println!("  清理后计算次数: {}", stats.total_computations);
    }
    
    Ok(())
}

/// 缓存优化演示
fn cache_optimization_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗄️ 5. 缓存优化演示");
    println!("----------------");
    
    // 创建带缓存优化的配置
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(1000)
        .with_exact_cache_size(500)
        .with_symbolic_cache_size(200)
        .with_cache_ttl(Duration::from_secs(300)); // 5分钟过期
    
    let config = ComputeConfig::new()
        .with_cache(cache_config);
    
    let yuf = Yufmath::with_config(config);
    
    println!("缓存配置:");
    println!("  - 快速缓存: 1000 项");
    println!("  - 精确缓存: 500 项");
    println!("  - 符号缓存: 200 项");
    println!("  - 缓存过期: 5分钟");
    
    // 测试缓存效果
    let test_expressions = vec![
        "2^10",
        "factorial(5)",
        "sin(pi/4)",
        "sqrt(16)",
        "ln(e)",
    ];
    
    println!("\n首次计算（冷缓存）:");
    let start_time = std::time::Instant::now();
    for expr in &test_expressions {
        let result = yuf.compute(expr)?;
        println!("  {} = {}", expr, result);
    }
    let cold_time = start_time.elapsed();
    println!("  首次计算总时间: {:?}", cold_time);
    
    println!("\n重复计算（热缓存）:");
    let start_time = std::time::Instant::now();
    for expr in &test_expressions {
        let result = yuf.compute(expr)?;
        println!("  {} = {}", expr, result);
    }
    let hot_time = start_time.elapsed();
    println!("  重复计算总时间: {:?}", hot_time);
    
    // 计算缓存效果
    if hot_time < cold_time {
        let speedup = cold_time.as_nanos() as f64 / hot_time.as_nanos() as f64;
        println!("  缓存加速比: {:.2}x", speedup);
    } else {
        println!("  缓存效果不明显（可能由于计算太简单）");
    }
    
    // 获取缓存统计
    if let Some(stats) = yuf.get_performance_stats() {
        println!("\n缓存统计:");
        println!("  - 缓存命中次数: {}", stats.cache_hits);
        println!("  - 缓存未命中次数: {}", stats.cache_misses);
        println!("  - 缓存命中率: {:.2}%", stats.cache_hit_rate() * 100.0);
    }
    
    Ok(())
}

/// 演示批量异步计算器的高级用法
#[allow(dead_code)]
fn advanced_async_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 6. 高级异步计算演示");
    println!("--------------------");
    
    // 创建自定义的批量异步计算器
    let computer = BatchAsyncComputer::new(4); // 最多4个并发任务
    
    // 准备大量表达式
    let expressions: Vec<String> = (1..=20)
        .map(|i| format!("{}^2 + {}*x + {}", i, i*2, i*3))
        .collect();
    
    println!("提交 {} 个异步计算任务...", expressions.len());
    let computations = computer.submit_batch(expressions);
    
    // 监控任务执行
    let mut completed_count = 0;
    let start_time = std::time::Instant::now();
    
    while completed_count < computations.len() {
        completed_count = 0;
        
        for (i, computation) in computations.iter().enumerate() {
            let status = computation.status();
            match status {
                TaskStatus::Completed => completed_count += 1,
                TaskStatus::Running => {
                    if let Some(progress) = computation.progress() {
                        println!("  任务 {}: {:.1}% - {}", 
                                i + 1, 
                                progress.progress * 100.0, 
                                progress.current_step);
                    }
                }
                TaskStatus::Error => {
                    println!("  任务 {} 出错", i + 1);
                    completed_count += 1; // 计入已完成
                }
                _ => {}
            }
        }
        
        println!("  已完成: {}/{}", completed_count, computations.len());
        
        if completed_count < computations.len() {
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    let total_time = start_time.elapsed();
    println!("所有任务完成，总耗时: {:?}", total_time);
    
    // 清理任务
    computer.cleanup_completed();
    println!("清理后活跃任务数: {}", computer.active_task_count());
    
    Ok(())
}