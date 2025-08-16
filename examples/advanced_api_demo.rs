//! # Yufmath 高级 API 使用示例
//!
//! 本示例展示了 Yufmath 库的高级功能和最佳实践。

use yufmath::{Yufmath, ComputeConfig, PrecisionConfig, FormatOptions, FormatType};
use yufmath::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Yufmath 高级 API 使用示例");
    println!("==========================");
    
    // 1. 高精度计算示例
    high_precision_demo()?;
    
    // 2. 复杂表达式处理
    complex_expression_demo()?;
    
    // 3. 自定义进度监控
    custom_progress_demo()?;
    
    // 4. 性能优化技巧
    performance_optimization_demo()?;
    
    // 5. 多线程使用
    multithreading_demo()?;
    
    // 6. 错误恢复策略
    error_recovery_demo()?;
    
    // 7. 内存管理
    memory_management_demo()?;
    
    Ok(())
}

/// 高精度计算示例
fn high_precision_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔢 1. 高精度计算");
    println!("---------------");
    
    // 配置高精度计算
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(2000)
        .with_symbolic(true);
    
    let config = ComputeConfig::new()
        .with_precision(precision_config);
    
    let yuf = Yufmath::with_config(config);
    
    // 计算大数阶乘
    println!("计算 100! 的前100位数字:");
    let factorial_100 = yuf.compute("100!")?;
    let display_length = std::cmp::min(100, factorial_100.len());
    println!("{}", &factorial_100[..display_length]);
    if factorial_100.len() > 100 {
        println!("... (共 {} 位数字)", factorial_100.len());
    }
    
    // 高精度 π 计算
    println!("\n计算 π 的高精度近似:");
    let pi_expr = yuf.parse("4 * arctan(1)")?;
    let pi_result = yuf.compute("pi")?;
    println!("π ≈ {}", pi_result);
    
    // 高精度有理数运算
    println!("\n高精度有理数运算:");
    let rational_result = yuf.compute("22/7 + 355/113")?;
    println!("22/7 + 355/113 = {}", rational_result);
    
    // 精确根式计算
    println!("\n精确根式计算:");
    let sqrt_result = yuf.compute("sqrt(2) + sqrt(3)")?;
    println!("√2 + √3 = {}", sqrt_result);
    
    Ok(())
}

/// 复杂表达式处理示例
fn complex_expression_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧮 2. 复杂表达式处理");
    println!("------------------");
    
    let yuf = Yufmath::new();
    
    // 构建复杂的多项式表达式
    let complex_poly = "(x + y + z)^5";
    println!("展开复杂多项式: {}", complex_poly);
    
    let poly_expr = yuf.parse(complex_poly)?;
    let expanded = yuf.expand(&poly_expr)?;
    println!("展开结果项数: {}", count_terms(&expanded));
    
    // 三角函数恒等式验证
    println!("\n验证三角函数恒等式:");
    let identity = "sin(x)^2 + cos(x)^2";
    let identity_expr = yuf.parse(identity)?;
    let simplified_identity = yuf.simplify(&identity_expr)?;
    println!("{} 简化为: {:?}", identity, simplified_identity);
    
    // 复数表达式处理
    println!("\n复数表达式处理:");
    let complex_expr = "(3 + 4*i) * (1 - 2*i)";
    let complex_result = yuf.compute(complex_expr)?;
    println!("{} = {}", complex_expr, complex_result);
    
    // 矩阵表达式
    println!("\n矩阵表达式:");
    let matrix_expr = "[[1, 2], [3, 4]] * [[x], [y]]";
    let matrix_parsed = yuf.parse(matrix_expr)?;
    println!("矩阵表达式解析成功: {:?}", matrix_parsed);
    
    Ok(())
}

/// 自定义进度监控示例
fn custom_progress_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 3. 自定义进度监控");
    println!("------------------");
    
    let mut yuf = Yufmath::new();
    
    // 创建进度数据收集器
    let progress_history = Arc::new(Mutex::new(Vec::new()));
    let progress_history_clone = Arc::clone(&progress_history);
    
    // 设置详细的进度回调
    yuf.set_progress_callback(Box::new(move |progress| {
        // 记录进度历史
        if let Ok(mut history) = progress_history_clone.lock() {
            history.push((
                progress.current_step.clone(),
                progress.progress,
                std::time::Instant::now(),
            ));
        }
        
        // 显示进度条
        let bar_length = 40;
        let filled_length = (progress.progress * bar_length as f64) as usize;
        let bar = "█".repeat(filled_length) + &"░".repeat(bar_length - filled_length);
        
        print!("\r[{}] {:.1}% - {}", bar, progress.progress * 100.0, progress.current_step);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        // 模拟用户取消条件（这里总是继续）
        true
    }));
    
    // 执行复杂计算
    println!("开始复杂积分计算...");
    let complex_integral = "integrate(sin(x^2) * cos(x^3), x)";
    let result = yuf.compute_with_progress(complex_integral)?;
    println!("\n计算完成: {}", result);
    
    // 分析进度历史
    if let Ok(history) = progress_history.lock() {
        println!("\n进度分析:");
        println!("  - 总步骤数: {}", history.len());
        if history.len() >= 2 {
            let total_time = history.last().unwrap().2.duration_since(history.first().unwrap().2);
            println!("  - 总耗时: {:?}", total_time);
            println!("  - 平均步骤时间: {:?}", total_time / history.len() as u32);
        }
    }
    
    Ok(())
}

/// 性能优化技巧示例
fn performance_optimization_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ 4. 性能优化技巧");
    println!("----------------");
    
    let mut yuf = Yufmath::new();
    
    // 批量计算性能对比
    let expressions = vec![
        "x^2 + 2*x + 1",
        "sin(x) + cos(x)",
        "e^x - 1",
        "ln(x + 1)",
        "sqrt(x^2 + 1)",
    ];
    
    // 单独计算
    let start_time = Instant::now();
    for expr in &expressions {
        let _ = yuf.compute(expr)?;
    }
    let individual_time = start_time.elapsed();
    
    // 批量计算
    let start_time = Instant::now();
    let _batch_results = yuf.batch_compute(&expressions);
    let batch_time = start_time.elapsed();
    
    println!("性能对比:");
    println!("  - 单独计算耗时: {:?}", individual_time);
    println!("  - 批量计算耗时: {:?}", batch_time);
    println!("  - 性能提升: {:.2}x", individual_time.as_nanos() as f64 / batch_time.as_nanos() as f64);
    
    // 缓存效果演示
    println!("\n缓存效果演示:");
    let repeated_expr = "x^10 + x^9 + x^8 + x^7 + x^6";
    
    // 首次计算
    let start_time = Instant::now();
    let _ = yuf.compute(repeated_expr)?;
    let first_time = start_time.elapsed();
    
    // 重复计算（应该更快由于缓存）
    let start_time = Instant::now();
    let _ = yuf.compute(repeated_expr)?;
    let cached_time = start_time.elapsed();
    
    println!("  - 首次计算: {:?}", first_time);
    println!("  - 缓存计算: {:?}", cached_time);
    if cached_time < first_time {
        println!("  - 缓存加速: {:.2}x", first_time.as_nanos() as f64 / cached_time.as_nanos() as f64);
    }
    
    // 内存使用监控
    if let Some(stats) = yuf.get_performance_stats() {
        println!("\n当前性能统计:");
        println!("  - 内存使用: {} 字节", stats.memory_usage);
        println!("  - 缓存命中率: {:.2}%", stats.cache_hit_rate * 100.0);
        println!("  - 精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
    }
    
    Ok(())
}

/// 多线程使用示例
fn multithreading_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧵 5. 多线程使用");
    println!("---------------");
    
    let yuf = Arc::new(Yufmath::new());
    let num_threads = 4;
    let expressions_per_thread = 10;
    
    println!("启动 {} 个线程，每个线程计算 {} 个表达式", num_threads, expressions_per_thread);
    
    let start_time = Instant::now();
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let yuf_clone = Arc::clone(&yuf);
        let handle = thread::spawn(move || {
            let mut results = vec![];
            for i in 0..expressions_per_thread {
                let expr = format!("{}*x^{} + {}", thread_id + 1, i + 1, i);
                match yuf_clone.compute(&expr) {
                    Ok(result) => results.push((expr, result)),
                    Err(e) => eprintln!("线程 {} 计算错误: {}", thread_id, e),
                }
            }
            results
        });
        handles.push(handle);
    }
    
    // 收集结果
    let mut all_results = vec![];
    for (thread_id, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(results) => {
                println!("线程 {} 完成，计算了 {} 个表达式", thread_id, results.len());
                all_results.extend(results);
            }
            Err(e) => eprintln!("线程 {} 执行失败: {:?}", thread_id, e),
        }
    }
    
    let total_time = start_time.elapsed();
    println!("多线程计算完成:");
    println!("  - 总耗时: {:?}", total_time);
    println!("  - 总计算数: {}", all_results.len());
    println!("  - 平均每个计算: {:?}", total_time / all_results.len() as u32);
    
    Ok(())
}

/// 错误恢复策略示例
fn error_recovery_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚨 6. 错误恢复策略");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 定义一些可能出错的表达式
    let problematic_expressions = vec![
        ("2 + 3", "正常表达式"),
        ("2 + + 3", "语法错误"),
        ("unknown_func(x)", "未知函数"),
        ("1/0", "除零"),
        ("x^(1/0)", "复杂除零"),
        ("factorial(-1)", "无效参数"),
    ];
    
    let mut successful_count = 0;
    let mut recoverable_errors = 0;
    let mut fatal_errors = 0;
    
    for (expr, description) in &problematic_expressions {
        print!("测试 {} ({}): ", description, expr);
        
        match yuf.compute(expr) {
            Ok(result) => {
                println!("✅ 成功 -> {}", result);
                successful_count += 1;
            }
            Err(e) => {
                if e.is_recoverable() {
                    println!("⚠️  可恢复错误 -> {}", e.user_friendly_message());
                    recoverable_errors += 1;
                    
                    // 显示修复建议
                    for suggestion in e.suggestions() {
                        println!("    💡 建议: {}", suggestion);
                    }
                } else {
                    println!("❌ 严重错误 -> {}", e.user_friendly_message());
                    fatal_errors += 1;
                }
            }
        }
    }
    
    println!("\n错误恢复统计:");
    println!("  - 成功计算: {}", successful_count);
    println!("  - 可恢复错误: {}", recoverable_errors);
    println!("  - 严重错误: {}", fatal_errors);
    println!("  - 总体成功率: {:.1}%", 
            successful_count as f64 / problematic_expressions.len() as f64 * 100.0);
    
    Ok(())
}

/// 内存管理示例
fn memory_management_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💾 7. 内存管理");
    println!("-------------");
    
    let mut yuf = Yufmath::new();
    
    // 执行大量计算来观察内存使用
    println!("执行大量计算来测试内存管理...");
    
    for batch in 0..5 {
        // 每批计算100个表达式
        let expressions: Vec<String> = (0..100)
            .map(|i| format!("x^{} + {}*x + {}", i % 10 + 1, i % 5 + 1, i))
            .collect();
        
        let expr_refs: Vec<&str> = expressions.iter().map(|s| s.as_str()).collect();
        let _results = yuf.batch_compute(&expr_refs);
        
        // 获取当前内存统计
        if let Some(stats) = yuf.get_performance_stats() {
            println!("批次 {}: 内存使用 {} 字节, 总计算 {} 次", 
                    batch + 1, 
                    stats.memory_usage, 
                    stats.total_computations);
            
            // 如果内存使用过多，重置统计
            if stats.memory_usage > 1_000_000 { // 1MB
                println!("  内存使用过多，重置统计...");
                yuf.reset_performance_stats();
            }
        }
    }
    
    // 最终内存统计
    if let Some(final_stats) = yuf.get_performance_stats() {
        println!("\n最终内存统计:");
        println!("  - 内存使用: {} 字节", final_stats.memory_usage);
        println!("  - 缓存命中率: {:.2}%", final_stats.cache_hit_rate * 100.0);
        println!("  - 总计算次数: {}", final_stats.total_computations);
    }
    
    Ok(())
}

/// 辅助函数：计算表达式中的项数
fn count_terms(expr: &Expression) -> usize {
    match expr {
        Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
            count_terms(left) + count_terms(right)
        }
        _ => 1,
    }
}

/// 辅助函数：模拟用户取消条件
#[allow(dead_code)]
fn should_cancel() -> bool {
    // 在实际应用中，这里可能检查用户输入、信号或其他取消条件
    false
}