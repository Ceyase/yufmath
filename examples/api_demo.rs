//! # Yufmath API 使用示例
//!
//! 本示例展示了如何使用 Yufmath 库的各种 API 功能。

use yufmath::{Yufmath, ComputeConfig, PrecisionConfig, FormatOptions, FormatType};
use yufmath::core::{Expression, Number, MathConstant};
use std::collections::HashMap;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧮 Yufmath API 使用示例");
    println!("===================");
    
    // 1. 基本使用
    basic_usage_demo()?;
    
    // 2. 配置使用
    config_demo()?;
    
    // 3. 进度监控
    progress_demo()?;
    
    // 4. 批量计算
    batch_demo()?;
    
    // 5. 高级数学功能
    advanced_math_demo()?;
    
    // 6. 性能统计
    performance_demo()?;
    
    Ok(())
}

/// 基本使用示例
fn basic_usage_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📚 1. 基本使用");
    println!("-------------");
    
    // 创建 Yufmath 实例
    let yuf = Yufmath::new();
    
    // 基本计算
    let result = yuf.compute("2 + 3 * 4")?;
    println!("计算 '2 + 3 * 4' = {}", result);
    
    // 符号计算
    let result = yuf.compute("x + x")?;
    println!("简化 'x + x' = {}", result);
    
    // 解析表达式
    let expr = yuf.parse("x^2 + 2*x + 1")?;
    println!("解析表达式: {:?}", expr);
    
    // 简化表达式
    let simplified = yuf.simplify(&expr)?;
    println!("简化结果: {:?}", simplified);
    
    // 求导
    let derivative = yuf.diff(&expr, "x")?;
    println!("对 x 求导: {:?}", derivative);
    
    // 积分
    let integral = yuf.integrate(&expr, "x")?;
    println!("对 x 积分: {:?}", integral);
    
    Ok(())
}

/// 配置使用示例
fn config_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️ 2. 配置使用");
    println!("-------------");
    
    // 创建自定义配置
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(1000)
        .with_symbolic(true);
    
    let compute_config = ComputeConfig::new()
        .with_progress(true)
        .with_progress_interval(50)
        .with_max_compute_time(Duration::from_secs(60))
        .with_precision(precision_config);
    
    // 使用配置创建实例
    let yuf = Yufmath::with_config(compute_config);
    
    // 测试高精度计算
    let result = yuf.compute("2^1000")?;
    println!("高精度计算 2^1000 的前50位: {}", 
             if result.len() > 50 { &result[..50] } else { &result });
    
    // 设置格式化选项
    let mut yuf = yuf;
    yuf.set_format_options(FormatOptions {
        format_type: FormatType::LaTeX,
        precision: Some(10),
        use_parentheses: true,
    });
    
    let expr = yuf.parse("sqrt(x^2 + y^2)")?;
    let formatted = yuf.compute("sqrt(x^2 + y^2)")?;
    println!("LaTeX 格式输出: {}", formatted);
    
    Ok(())
}

/// 进度监控示例
fn progress_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 3. 进度监控");
    println!("-------------");
    
    let mut yuf = Yufmath::new();
    
    // 设置进度回调
    yuf.set_progress_callback(Box::new(|progress| {
        println!("进度: {:.1}% - {}", 
                progress.progress * 100.0, 
                progress.current_step);
        
        // 返回 true 继续计算，false 取消计算
        true
    }));
    
    // 带进度的计算
    println!("开始带进度监控的计算...");
    let result = yuf.compute_with_progress("integrate(sin(x^2), x)")?;
    println!("计算结果: {}", result);
    
    // 获取性能统计
    if let Some(stats) = yuf.get_performance_stats() {
        println!("性能统计:");
        println!("  - 总计算次数: {}", stats.total_computations);
        println!("  - 成功率: {:.2}%", stats.success_rate() * 100.0);
        println!("  - 平均计算时间: {:?}", stats.avg_compute_time);
        println!("  - 精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
    }
    
    Ok(())
}

/// 批量计算示例
fn batch_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📦 4. 批量计算");
    println!("-------------");
    
    let yuf = Yufmath::new();
    
    // 批量计算表达式
    let expressions = vec![
        "2 + 3",
        "x^2 + 1",
        "sin(pi/2)",
        "e^(i*pi)",
        "sqrt(4)",
    ];
    
    println!("批量计算结果:");
    let results = yuf.batch_compute(&expressions);
    for (expr, result) in expressions.iter().zip(results.iter()) {
        match result {
            Ok(value) => println!("  {} = {}", expr, value),
            Err(e) => println!("  {} -> 错误: {}", expr, e),
        }
    }
    
    // 批量解析
    let parsed_results = yuf.batch_parse(&expressions);
    println!("\n批量解析结果:");
    for (expr, result) in expressions.iter().zip(parsed_results.iter()) {
        match result {
            Ok(parsed) => println!("  {} -> 解析成功", expr),
            Err(e) => println!("  {} -> 解析错误: {}", expr, e),
        }
    }
    
    Ok(())
}

/// 高级数学功能示例
fn advanced_math_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔬 5. 高级数学功能");
    println!("----------------");
    
    let yuf = Yufmath::new();
    
    // 多项式运算
    let poly = yuf.parse("(x + 1)^3")?;
    let expanded = yuf.expand(&poly)?;
    println!("展开 (x + 1)^3: {:?}", expanded);
    
    let factored_expr = yuf.parse("x^2 - 4")?;
    let factored = yuf.factor(&factored_expr)?;
    println!("因式分解 x^2 - 4: {:?}", factored);
    
    // 极限计算
    let expr = yuf.parse("sin(x)/x")?;
    let point = yuf.parse("0")?;
    let limit_result = yuf.limit(&expr, "x", &point)?;
    println!("lim(x->0) sin(x)/x: {:?}", limit_result);
    
    // 级数展开
    let series_result = yuf.series(&yuf.parse("e^x")?, "x", &yuf.parse("0")?, 5)?;
    println!("e^x 在 x=0 处的5阶级数展开: {:?}", series_result);
    
    // 数论函数
    let gcd_result = yuf.gcd(&yuf.parse("48")?, &yuf.parse("18")?)?;
    println!("gcd(48, 18): {:?}", gcd_result);
    
    let is_prime_result = yuf.is_prime(&yuf.parse("17")?)?;
    println!("17 是素数吗? {}", is_prime_result);
    
    // 组合数学
    let binomial_result = yuf.binomial(&yuf.parse("5")?, &yuf.parse("2")?)?;
    println!("C(5,2): {:?}", binomial_result);
    
    Ok(())
}

/// 性能统计示例
fn performance_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 6. 性能统计");
    println!("-------------");
    
    let mut yuf = Yufmath::new();
    
    // 执行一些计算来生成统计数据
    let test_expressions = vec![
        "2 + 3",
        "x^2 + 2*x + 1",
        "sin(pi/4)",
        "integrate(x^2, x)",
        "diff(x^3, x)",
    ];
    
    println!("执行测试计算...");
    for expr in &test_expressions {
        let _ = yuf.compute(expr);
    }
    
    // 获取并显示性能统计
    if let Some(stats) = yuf.get_performance_stats() {
        println!("\n性能统计报告:");
        println!("  📊 总计算次数: {}", stats.total_computations);
        println!("  ✅ 成功计算次数: {}", stats.successful_computations);
        println!("  ❌ 失败计算次数: {}", stats.failed_computations());
        println!("  🎯 成功率: {:.2}%", stats.success_rate() * 100.0);
        println!("  ⏱️  平均计算时间: {:?}", stats.avg_compute_time);
        println!("  🔢 精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
        println!("  💾 内存使用量: {} 字节", stats.memory_usage);
        
        if let Some(ref progress) = stats.last_progress {
            println!("  📋 最后进度: {} ({:.1}%)", 
                    progress.current_step, 
                    progress.progress * 100.0);
        }
    }
    
    // 重置统计
    yuf.reset_performance_stats();
    println!("\n统计信息已重置");
    
    Ok(())
}

/// 错误处理示例
#[allow(dead_code)]
fn error_handling_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚨 7. 错误处理");
    println!("-------------");
    
    let yuf = Yufmath::new();
    
    // 故意制造一些错误来演示错误处理
    let invalid_expressions = vec![
        "2 + + 3",           // 语法错误
        "unknown_func(x)",   // 未知函数
        "1/0",              // 除零错误
    ];
    
    for expr in &invalid_expressions {
        match yuf.compute(expr) {
            Ok(result) => println!("  {} = {}", expr, result),
            Err(e) => {
                println!("  {} -> 错误类型: {:?}", expr, e);
                println!("    用户友好消息: {}", e.user_friendly_message());
                println!("    修复建议: {:?}", e.suggestions());
                println!("    是否可恢复: {}", e.is_recoverable());
                println!("    完整错误报告:");
                println!("{}", e.format_error_report(Some(expr)));
            }
        }
    }
    
    Ok(())
}

/// 数值计算示例
#[allow(dead_code)]
fn numerical_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔢 8. 数值计算");
    println!("-------------");
    
    let yuf = Yufmath::new();
    
    // 创建包含变量的表达式
    let expr = yuf.parse("x^2 + 2*x + 1")?;
    
    // 设置变量值
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 3.0);
    
    // 数值计算
    let numerical_result = yuf.numerical_evaluate(&expr, &vars)?;
    println!("当 x = 3 时，x^2 + 2*x + 1 = {}", numerical_result);
    
    // 精确计算
    let mut exact_vars = HashMap::new();
    exact_vars.insert("x".to_string(), Number::from(3));
    
    let exact_result = yuf.evaluate(&expr, &exact_vars)?;
    println!("精确计算结果: {:?}", exact_result);
    
    Ok(())
}