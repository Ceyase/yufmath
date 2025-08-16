//! # 缓存系统测试
//!
//! 测试多层缓存系统的功能和性能。

use yufmath::{Yufmath, ComputeConfig, CacheConfig, Expression, Number, BinaryOperator};
use std::time::Duration;
use num_bigint::BigInt;

#[test]
fn test_cache_basic_functionality() {
    // 创建启用缓存的配置
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(100)
        .with_exact_cache_size(50)
        .with_symbolic_cache_size(30);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 测试基本计算
    let result1 = yuf.compute("2 + 3").unwrap();
    assert_eq!(result1, "5");
    
    // 再次计算相同表达式，应该从缓存获取
    let result2 = yuf.compute("2 + 3").unwrap();
    assert_eq!(result2, "5");
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        assert!(stats.total_hit_rate() >= 0.0);
        println!("缓存命中率: {:.2}%", stats.total_hit_rate() * 100.0);
    }
}

#[test]
fn test_fast_cache_for_small_integers() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(1000);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 测试小整数运算
    let expressions = vec![
        "1 + 1",
        "2 * 3", 
        "10 - 5",
        "8 / 2",
        "3 ^ 2",
    ];
    
    // 第一次计算
    let mut results1 = Vec::new();
    for expr in &expressions {
        results1.push(yuf.compute(expr).unwrap());
    }
    
    // 第二次计算（应该从缓存获取）
    let mut results2 = Vec::new();
    for expr in &expressions {
        results2.push(yuf.compute(expr).unwrap());
    }
    
    // 结果应该相同
    assert_eq!(results1, results2);
    
    // 检查缓存使用情况
    if let Some(usage) = yuf.get_cache_usage() {
        println!("快速缓存使用率: {:.2}%", usage.fast_cache_usage_rate() * 100.0);
        assert!(usage.fast_cache_usage > 0);
    }
}

#[test]
fn test_symbolic_cache_for_expressions() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_symbolic_cache_size(100);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 测试符号表达式简化
    let expressions = vec![
        "x + x",
        "2 * x + 3 * x",
        "x * 1",
        "0 + y",
        "(a + b) + (a + b)",
    ];
    
    // 第一次简化
    let mut results1 = Vec::new();
    for expr_str in &expressions {
        let expr = yuf.parse(expr_str).unwrap();
        results1.push(yuf.simplify(&expr).unwrap());
    }
    
    // 第二次简化（应该从缓存获取）
    let mut results2 = Vec::new();
    for expr_str in &expressions {
        let expr = yuf.parse(expr_str).unwrap();
        results2.push(yuf.simplify(&expr).unwrap());
    }
    
    // 结果应该相同
    assert_eq!(results1, results2);
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        println!("符号缓存命中率: {:.2}%", stats.symbolic_hit_rate() * 100.0);
        assert!(stats.symbolic_hits > 0);
    }
}

#[test]
fn test_cache_with_differentiation() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_symbolic_cache_size(50);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 测试求导缓存
    let expr = yuf.parse("x^2 + 2*x + 1").unwrap();
    
    // 第一次求导
    let derivative1 = yuf.diff(&expr, "x").unwrap();
    
    // 第二次求导（应该从缓存获取）
    let derivative2 = yuf.diff(&expr, "x").unwrap();
    
    // 结果应该相同
    assert_eq!(derivative1, derivative2);
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        assert!(stats.symbolic_hits > 0);
        println!("求导缓存命中: {}", stats.symbolic_hits);
    }
}

#[test]
fn test_cache_with_integration() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_symbolic_cache_size(50);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 测试积分缓存
    let expr = yuf.parse("2*x + 1").unwrap();
    
    // 第一次积分
    let integral1 = yuf.integrate(&expr, "x").unwrap();
    
    // 第二次积分（应该从缓存获取）
    let integral2 = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该相同
    assert_eq!(integral1, integral2);
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        assert!(stats.symbolic_hits > 0);
        println!("积分缓存命中: {}", stats.symbolic_hits);
    }
}

#[test]
fn test_cache_size_limits() {
    // 创建小容量缓存配置
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(3)
        .with_exact_cache_size(2)
        .with_symbolic_cache_size(2);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 填满缓存
    for i in 1..=5 {
        let expr = format!("{} + {}", i, i + 1);
        yuf.compute(&expr).unwrap();
    }
    
    // 检查缓存使用情况
    if let Some(usage) = yuf.get_cache_usage() {
        println!("缓存使用情况:");
        println!("  快速缓存: {}/{}", usage.fast_cache_usage, usage.fast_cache_capacity);
        println!("  精确缓存: {}/{}", usage.exact_cache_usage, usage.exact_cache_capacity);
        println!("  符号缓存: {}/{}", usage.symbolic_cache_usage, usage.symbolic_cache_capacity);
        
        // 缓存应该不超过限制
        assert!(usage.fast_cache_usage <= usage.fast_cache_capacity);
        assert!(usage.exact_cache_usage <= usage.exact_cache_capacity);
        assert!(usage.symbolic_cache_usage <= usage.symbolic_cache_capacity);
    }
}

#[test]
fn test_cache_ttl_expiration() {
    // 创建短TTL的缓存配置
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_cache_ttl(Duration::from_millis(50)); // 50毫秒过期
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 计算表达式
    let result1 = yuf.compute("5 + 5").unwrap();
    assert_eq!(result1, "10");
    
    // 等待缓存过期
    std::thread::sleep(Duration::from_millis(100));
    
    // 再次计算（缓存应该已过期）
    let result2 = yuf.compute("5 + 5").unwrap();
    assert_eq!(result2, "10");
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        println!("TTL测试 - 总命中率: {:.2}%", stats.total_hit_rate() * 100.0);
    }
}

#[test]
fn test_cache_cleanup_operations() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(100)
        .with_symbolic_cache_size(50);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 填充一些缓存
    for i in 1..=10 {
        let expr = format!("x + {}", i);
        let parsed = yuf.parse(&expr).unwrap();
        yuf.simplify(&parsed).unwrap();
    }
    
    // 检查缓存使用情况
    let usage_before = yuf.get_cache_usage().unwrap();
    println!("清理前缓存使用: {:.2}%", usage_before.total_usage_rate() * 100.0);
    
    // 执行缓存清理
    yuf.cleanup_cache().unwrap();
    
    // 清空所有缓存
    yuf.clear_cache().unwrap();
    
    // 检查缓存是否被清空
    let usage_after = yuf.get_cache_usage().unwrap();
    println!("清理后缓存使用: {:.2}%", usage_after.total_usage_rate() * 100.0);
    assert_eq!(usage_after.total_usage_rate(), 0.0);
}

#[test]
fn test_cache_performance_improvement() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_symbolic_cache_size(100);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 复杂表达式
    let complex_expr = "((x + 1) * (x - 1))^2 + ((y + 2) * (y - 2))^2";
    let expr = yuf.parse(complex_expr).unwrap();
    
    // 第一次简化（建立缓存）
    let start = std::time::Instant::now();
    let result1 = yuf.simplify(&expr).unwrap();
    let duration1 = start.elapsed();
    
    // 第二次简化（从缓存获取）
    let start = std::time::Instant::now();
    let result2 = yuf.simplify(&expr).unwrap();
    let duration2 = start.elapsed();
    
    // 结果应该相同
    assert_eq!(result1, result2);
    
    // 第二次应该更快（从缓存获取）
    println!("第一次简化耗时: {:?}", duration1);
    println!("第二次简化耗时: {:?}", duration2);
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        println!("缓存统计:");
        println!("  总命中率: {:.2}%", stats.total_hit_rate() * 100.0);
        println!("  符号缓存命中: {}", stats.symbolic_hits);
        println!("  估算节省时间: {:?}", stats.total_time_saved);
        
        assert!(stats.symbolic_hits > 0);
    }
}

#[test]
fn test_cache_with_different_operations() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_symbolic_cache_size(100);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    let expr = yuf.parse("x^3 + 3*x^2 + 3*x + 1").unwrap();
    
    // 测试不同操作的缓存
    let simplified = yuf.simplify(&expr).unwrap();
    let expanded = yuf.expand(&expr).unwrap();
    let factored = yuf.factor(&expr).unwrap();
    
    // 再次执行相同操作（应该从缓存获取）
    let simplified2 = yuf.simplify(&expr).unwrap();
    let expanded2 = yuf.expand(&expr).unwrap();
    let factored2 = yuf.factor(&expr).unwrap();
    
    // 结果应该相同
    assert_eq!(simplified, simplified2);
    assert_eq!(expanded, expanded2);
    assert_eq!(factored, factored2);
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        println!("多操作缓存测试:");
        println!("  符号缓存命中: {}", stats.symbolic_hits);
        println!("  符号缓存未命中: {}", stats.symbolic_misses);
        
        assert!(stats.symbolic_hits >= 3); // 至少3次命中
    }
}

#[test]
fn test_disabled_cache() {
    // 创建禁用缓存的配置
    let cache_config = CacheConfig::new().with_enabled(false);
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 执行一些计算
    yuf.compute("2 + 3").unwrap();
    yuf.compute("2 + 3").unwrap();
    
    // 缓存统计应该显示没有命中
    if let Some(stats) = yuf.get_cache_stats() {
        assert_eq!(stats.total_hit_rate(), 0.0);
        println!("禁用缓存 - 命中率: {:.2}%", stats.total_hit_rate() * 100.0);
    }
    
    // 缓存使用情况应该为0
    if let Some(usage) = yuf.get_cache_usage() {
        assert_eq!(usage.total_usage_rate(), 0.0);
    }
}

#[test]
fn test_cache_memory_efficiency() {
    let cache_config = CacheConfig::new()
        .with_enabled(true)
        .with_fast_cache_size(1000)
        .with_exact_cache_size(500)
        .with_symbolic_cache_size(200);
    
    let config = ComputeConfig::new().with_cache(cache_config);
    let yuf = Yufmath::with_config(config);
    
    // 执行大量计算以测试内存效率
    for i in 1..=100 {
        // 小整数运算
        let expr1 = format!("{} + {}", i, i + 1);
        yuf.compute(&expr1).unwrap();
        
        // 符号表达式
        let expr2 = format!("x^{} + {}", i % 5 + 1, i);
        let parsed = yuf.parse(&expr2).unwrap();
        yuf.simplify(&parsed).unwrap();
    }
    
    // 检查最终的缓存使用情况
    if let Some(usage) = yuf.get_cache_usage() {
        println!("内存效率测试 - 缓存使用情况:");
        println!("  快速缓存: {}/{} ({:.1}%)", 
                usage.fast_cache_usage, 
                usage.fast_cache_capacity,
                usage.fast_cache_usage_rate() * 100.0);
        println!("  精确缓存: {}/{} ({:.1}%)", 
                usage.exact_cache_usage, 
                usage.exact_cache_capacity,
                usage.exact_cache_usage_rate() * 100.0);
        println!("  符号缓存: {}/{} ({:.1}%)", 
                usage.symbolic_cache_usage, 
                usage.symbolic_cache_capacity,
                usage.symbolic_cache_usage_rate() * 100.0);
        
        // 缓存不应该超过限制
        assert!(usage.fast_cache_usage <= usage.fast_cache_capacity);
        assert!(usage.exact_cache_usage <= usage.exact_cache_capacity);
        assert!(usage.symbolic_cache_usage <= usage.symbolic_cache_capacity);
    }
    
    // 检查缓存统计
    if let Some(stats) = yuf.get_cache_stats() {
        println!("缓存性能统计:");
        println!("  总命中率: {:.2}%", stats.total_hit_rate() * 100.0);
        println!("  快速缓存命中率: {:.2}%", stats.fast_hit_rate() * 100.0);
        println!("  精确缓存命中率: {:.2}%", stats.exact_hit_rate() * 100.0);
        println!("  符号缓存命中率: {:.2}%", stats.symbolic_hit_rate() * 100.0);
        println!("  缓存清理次数: {}", stats.cleanup_count);
        println!("  估算节省时间: {:?}", stats.total_time_saved);
    }
}