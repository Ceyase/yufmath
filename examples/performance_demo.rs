//! # Yufmath 性能测试示例
//!
//! 本示例展示了如何测试和优化 Yufmath 库的性能。

use yufmath::{Yufmath, ComputeConfig, PrecisionConfig};
use std::time::{Duration, Instant};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Yufmath 性能测试示例");
    println!("=====================");
    
    // 1. 基础性能测试
    basic_performance_test()?;
    
    // 2. 精度 vs 性能权衡
    precision_vs_performance_test()?;
    
    // 3. 批量处理性能
    batch_processing_test()?;
    
    // 4. 缓存效果测试
    cache_performance_test()?;
    
    // 5. 内存使用分析
    memory_usage_test()?;
    
    // 6. 复杂表达式性能
    complex_expression_test()?;
    
    Ok(())
}

/// 基础性能测试
fn basic_performance_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 1. 基础性能测试");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 测试不同类型的基本运算
    let test_cases = vec![
        ("算术运算", vec!["2 + 3", "10 * 5", "100 / 4", "2^10"]),
        ("符号运算", vec!["x + x", "x * 1", "x^2 * x^3", "(x + 1)^2"]),
        ("函数运算", vec!["sin(0)", "cos(pi/2)", "ln(e)", "sqrt(4)"]),
        ("常数运算", vec!["pi", "e", "i^2", "2*pi"]),
    ];
    
    for (category, expressions) in test_cases {
        println!("\n{} 性能测试:", category);
        
        let start_time = Instant::now();
        let mut successful = 0;
        
        for expr in &expressions {
            match yuf.compute(expr) {
                Ok(_) => successful += 1,
                Err(e) => println!("  错误 {}: {}", expr, e),
            }
        }
        
        let elapsed = start_time.elapsed();
        println!("  - 表达式数量: {}", expressions.len());
        println!("  - 成功计算: {}", successful);
        println!("  - 总耗时: {:?}", elapsed);
        println!("  - 平均耗时: {:?}", elapsed / expressions.len() as u32);
        println!("  - 吞吐量: {:.2} 表达式/秒", 
                expressions.len() as f64 / elapsed.as_secs_f64());
    }
    
    Ok(())
}

/// 精度 vs 性能权衡测试
fn precision_vs_performance_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 2. 精度 vs 性能权衡");
    println!("--------------------");
    
    let test_expr = "2^1000 + 3^500";
    
    // 不同精度配置
    let precision_configs = vec![
        ("低精度", PrecisionConfig::new().with_force_exact(false).with_approximation_threshold(1e-6)),
        ("中精度", PrecisionConfig::new().with_force_exact(true).with_max_precision(100)),
        ("高精度", PrecisionConfig::new().with_force_exact(true).with_max_precision(1000)),
        ("超高精度", PrecisionConfig::new().with_force_exact(true).with_max_precision(5000)),
    ];
    
    for (name, precision_config) in precision_configs {
        let config = ComputeConfig::new().with_precision(precision_config);
        let yuf = Yufmath::with_config(config);
        
        let start_time = Instant::now();
        match yuf.compute(test_expr) {
            Ok(result) => {
                let elapsed = start_time.elapsed();
                println!("{} 配置:", name);
                println!("  - 计算时间: {:?}", elapsed);
                println!("  - 结果长度: {} 字符", result.len());
                println!("  - 结果预览: {}...", 
                        if result.len() > 50 { &result[..50] } else { &result });
            }
            Err(e) => {
                println!("{} 配置: 计算失败 - {}", name, e);
            }
        }
    }
    
    Ok(())
}

/// 批量处理性能测试
fn batch_processing_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📦 3. 批量处理性能");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 生成测试表达式
    let batch_sizes = vec![10, 50, 100, 500, 1000];
    
    for batch_size in batch_sizes {
        let expressions: Vec<String> = (0..batch_size)
            .map(|i| format!("{}*x^{} + {}", i % 10 + 1, i % 5 + 1, i))
            .collect();
        
        let expr_refs: Vec<&str> = expressions.iter().map(|s| s.as_str()).collect();
        
        // 测试单独计算
        let start_time = Instant::now();
        let mut individual_results = vec![];
        for expr in &expr_refs {
            match yuf.compute(expr) {
                Ok(result) => individual_results.push(result),
                Err(_) => {}
            }
        }
        let individual_time = start_time.elapsed();
        
        // 测试批量计算
        let start_time = Instant::now();
        let batch_results = yuf.batch_compute(&expr_refs);
        let batch_time = start_time.elapsed();
        
        let batch_success = batch_results.iter().filter(|r| r.is_ok()).count();
        
        println!("批量大小 {} 的性能对比:", batch_size);
        println!("  - 单独计算: {:?} ({} 成功)", individual_time, individual_results.len());
        println!("  - 批量计算: {:?} ({} 成功)", batch_time, batch_success);
        
        if batch_time.as_nanos() > 0 {
            let speedup = individual_time.as_nanos() as f64 / batch_time.as_nanos() as f64;
            println!("  - 性能提升: {:.2}x", speedup);
        }
        
        println!("  - 批量吞吐量: {:.2} 表达式/秒", 
                batch_size as f64 / batch_time.as_secs_f64());
    }
    
    Ok(())
}

/// 缓存效果测试
fn cache_performance_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🗄️ 4. 缓存效果测试");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 测试重复计算的缓存效果
    let repeated_expressions = vec![
        "x^10 + x^9 + x^8",
        "sin(x) + cos(x) + tan(x)",
        "e^x + ln(x) + sqrt(x)",
        "factorial(10)",
        "fibonacci(20)",
    ];
    
    for expr in &repeated_expressions {
        println!("\n测试表达式: {}", expr);
        
        // 首次计算（冷缓存）
        let start_time = Instant::now();
        let first_result = yuf.compute(expr);
        let first_time = start_time.elapsed();
        
        // 重复计算（热缓存）
        let mut repeat_times = vec![];
        for _ in 0..5 {
            let start_time = Instant::now();
            let _ = yuf.compute(expr);
            repeat_times.push(start_time.elapsed());
        }
        
        let avg_repeat_time = repeat_times.iter().sum::<Duration>() / repeat_times.len() as u32;
        
        println!("  - 首次计算: {:?}", first_time);
        println!("  - 平均重复计算: {:?}", avg_repeat_time);
        
        if avg_repeat_time < first_time {
            let speedup = first_time.as_nanos() as f64 / avg_repeat_time.as_nanos() as f64;
            println!("  - 缓存加速: {:.2}x", speedup);
        } else {
            println!("  - 无明显缓存效果");
        }
        
        if first_result.is_ok() {
            println!("  - 计算成功");
        } else {
            println!("  - 计算失败: {:?}", first_result.err());
        }
    }
    
    Ok(())
}

/// 内存使用分析
fn memory_usage_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 5. 内存使用分析");
    println!("----------------");
    
    let mut yuf = Yufmath::new();
    
    // 执行不同复杂度的计算并监控内存使用
    let complexity_tests = vec![
        ("简单", vec!["2 + 3", "x + 1", "sin(0)"]),
        ("中等", vec!["(x + 1)^5", "integrate(x^2, x)", "diff(x^3, x)"]),
        ("复杂", vec!["(x + y + z)^10", "matrix([[1,2],[3,4]]) * matrix([[x],[y]])", "solve(x^2 - 4 = 0, x)"]),
    ];
    
    for (complexity, expressions) in complexity_tests {
        println!("\n{} 复杂度测试:", complexity);
        
        // 重置统计以获得准确的内存测量
        yuf.reset_performance_stats();
        
        let start_time = Instant::now();
        let mut successful = 0;
        
        for expr in &expressions {
            match yuf.compute(expr) {
                Ok(_) => successful += 1,
                Err(_) => {}
            }
        }
        
        let elapsed = start_time.elapsed();
        
        if let Some(stats) = yuf.get_performance_stats() {
            println!("  - 表达式数量: {}", expressions.len());
            println!("  - 成功计算: {}", successful);
            println!("  - 计算时间: {:?}", elapsed);
            println!("  - 内存使用: {} 字节", stats.memory_usage);
            println!("  - 平均内存/表达式: {} 字节", 
                    if expressions.len() > 0 { stats.memory_usage / expressions.len() } else { 0 });
            println!("  - 缓存命中率: {:.2}%", stats.cache_hit_rate * 100.0);
        }
    }
    
    Ok(())
}

/// 复杂表达式性能测试
fn complex_expression_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧮 6. 复杂表达式性能");
    println!("------------------");
    
    let yuf = Yufmath::new();
    
    // 不同类型的复杂表达式
    let complex_tests = vec![
        ("深度嵌套", "((((x + 1)^2 + 2)^2 + 3)^2 + 4)^2"),
        ("宽度展开", "(x + y + z + w + v + u + t + s + r + q)^3"),
        ("混合运算", "sin(cos(tan(x))) + ln(exp(sqrt(x))) + arctan(x^2)"),
        ("大数运算", "2^100 + 3^100 + 5^100"),
        ("矩阵运算", "det([[x, y, z], [1, 2, 3], [4, 5, 6]])"),
    ];
    
    for (test_type, expr) in complex_tests {
        println!("\n{} 测试: {}", test_type, expr);
        
        // 解析时间
        let start_time = Instant::now();
        let parse_result = yuf.parse(expr);
        let parse_time = start_time.elapsed();
        
        match parse_result {
            Ok(parsed_expr) => {
                println!("  - 解析时间: {:?}", parse_time);
                
                // 简化时间
                let start_time = Instant::now();
                let simplify_result = yuf.simplify(&parsed_expr);
                let simplify_time = start_time.elapsed();
                
                match simplify_result {
                    Ok(_) => {
                        println!("  - 简化时间: {:?}", simplify_time);
                        println!("  - 总处理时间: {:?}", parse_time + simplify_time);
                    }
                    Err(e) => {
                        println!("  - 简化失败: {}", e);
                    }
                }
                
                // 如果是数值表达式，尝试计算
                if !expr.contains(char::is_alphabetic) {
                    let start_time = Instant::now();
                    let compute_result = yuf.compute(expr);
                    let compute_time = start_time.elapsed();
                    
                    match compute_result {
                        Ok(result) => {
                            println!("  - 计算时间: {:?}", compute_time);
                            println!("  - 结果长度: {} 字符", result.len());
                        }
                        Err(e) => {
                            println!("  - 计算失败: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  - 解析失败: {}", e);
            }
        }
    }
    
    Ok(())
}

/// 辅助函数：格式化持续时间
#[allow(dead_code)]
fn format_duration(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}μs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
    }
}

/// 辅助函数：格式化字节大小
#[allow(dead_code)]
fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.2}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2}MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2}GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}