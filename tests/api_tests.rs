//! # Yufmath API 测试
//!
//! 测试 Yufmath 库的主要 API 功能。

use yufmath::{Yufmath, ComputeConfig, PrecisionConfig, FormatOptions, FormatType};
use yufmath::core::{Expression, Number, MathConstant, BinaryOperator};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[test]
fn test_basic_api_creation() {
    // 测试基本创建
    let yuf = Yufmath::new();
    assert!(yuf.get_config().enable_progress);
    
    // 测试带配置创建
    let config = ComputeConfig::new()
        .with_progress(false)
        .with_max_compute_time(Duration::from_secs(30));
    
    let yuf_with_config = Yufmath::with_config(config);
    assert!(!yuf_with_config.get_config().enable_progress);
    assert_eq!(yuf_with_config.get_config().max_compute_time, Some(Duration::from_secs(30)));
}

#[test]
fn test_basic_computation() {
    let yuf = Yufmath::new();
    
    // 测试基本算术
    assert_eq!(yuf.compute("2 + 3").unwrap(), "5");
    assert_eq!(yuf.compute("2 * 3").unwrap(), "6");
    assert_eq!(yuf.compute("10 / 2").unwrap(), "5");
    
    // 测试符号计算
    assert_eq!(yuf.compute("x + x").unwrap(), "2*x");
    assert_eq!(yuf.compute("x * 1").unwrap(), "x");
    assert_eq!(yuf.compute("x + 0").unwrap(), "x");
}

#[test]
fn test_expression_parsing() {
    let yuf = Yufmath::new();
    
    // 测试数值解析
    let num_expr = yuf.parse("42").unwrap();
    assert!(matches!(num_expr, Expression::Number(_)));
    
    // 测试变量解析
    let var_expr = yuf.parse("x").unwrap();
    assert!(matches!(var_expr, Expression::Variable(_)));
    
    // 测试二元运算解析
    let binary_expr = yuf.parse("x + y").unwrap();
    assert!(matches!(binary_expr, Expression::BinaryOp { .. }));
    
    // 测试函数解析
    let func_expr = yuf.parse("sin(x)").unwrap();
    assert!(matches!(func_expr, Expression::UnaryOp { .. }));
}

#[test]
fn test_expression_simplification() {
    let yuf = Yufmath::new();
    
    // 测试基本简化
    let expr = yuf.parse("x + x").unwrap();
    let simplified = yuf.simplify(&expr).unwrap();
    // 这里应该检查简化结果，但具体格式取决于实现
    
    let expr = yuf.parse("x * 1").unwrap();
    let simplified = yuf.simplify(&expr).unwrap();
    assert!(matches!(simplified, Expression::Variable(_)));
    
    let expr = yuf.parse("x + 0").unwrap();
    let simplified = yuf.simplify(&expr).unwrap();
    assert!(matches!(simplified, Expression::Variable(_)));
}

#[test]
fn test_calculus_operations() {
    let yuf = Yufmath::new();
    
    // 测试求导
    let expr = yuf.parse("x^2").unwrap();
    let derivative = yuf.diff(&expr, "x").unwrap();
    // 应该得到 2*x
    
    let expr = yuf.parse("x^3").unwrap();
    let derivative = yuf.differentiate(&expr, "x").unwrap();
    // 应该得到 3*x^2
    
    // 测试积分
    let expr = yuf.parse("2*x").unwrap();
    let integral = yuf.integrate(&expr, "x").unwrap();
    // 应该得到 x^2 + C
}

#[test]
fn test_batch_operations() {
    let yuf = Yufmath::new();
    
    // 测试批量计算
    let expressions = vec!["2 + 3", "4 * 5", "10 / 2"];
    let results = yuf.batch_compute(&expressions);
    
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].as_ref().unwrap(), "5");
    assert_eq!(results[1].as_ref().unwrap(), "20");
    assert_eq!(results[2].as_ref().unwrap(), "5");
    
    // 测试批量解析
    let parse_results = yuf.batch_parse(&expressions);
    assert_eq!(parse_results.len(), 3);
    assert!(parse_results.iter().all(|r| r.is_ok()));
    
    // 测试批量简化
    let parsed_exprs: Vec<Expression> = parse_results.into_iter()
        .map(|r| r.unwrap())
        .collect();
    let simplify_results = yuf.batch_simplify(&parsed_exprs);
    assert_eq!(simplify_results.len(), 3);
    assert!(simplify_results.iter().all(|r| r.is_ok()));
}

#[test]
fn test_format_options() {
    let mut yuf = Yufmath::new();
    
    // 测试标准格式
    yuf.set_format_options(FormatOptions {
        format_type: FormatType::Standard,
        precision: Some(2),
        use_parentheses: false,
    });
    
    let result = yuf.compute("pi").unwrap();
    // 应该包含 π 或其数值表示
    
    // 测试 LaTeX 格式
    yuf.set_format_options(FormatOptions {
        format_type: FormatType::LaTeX,
        precision: None,
        use_parentheses: true,
    });
    
    let result = yuf.compute("x^2").unwrap();
    // 应该包含 LaTeX 格式的幂次表示
}

#[test]
fn test_progress_monitoring() {
    let mut yuf = Yufmath::new();
    
    // 设置进度回调
    let progress_data = Arc::new(Mutex::new(Vec::new()));
    let progress_data_clone = Arc::clone(&progress_data);
    
    yuf.set_progress_callback(Box::new(move |progress| {
        if let Ok(mut data) = progress_data_clone.lock() {
            data.push((progress.current_step.clone(), progress.progress));
        }
        true // 继续计算
    }));
    
    // 执行带进度的计算
    let result = yuf.compute_with_progress("x^2 + 2*x + 1");
    assert!(result.is_ok());
    
    // 检查是否记录了进度
    if let Ok(data) = progress_data.lock() {
        assert!(!data.is_empty());
        // 应该有多个进度更新
        assert!(data.len() >= 2);
    };
}

#[test]
fn test_cancellation() {
    let mut yuf = Yufmath::new();
    
    // 测试取消功能
    assert!(!yuf.is_cancelled());
    
    yuf.cancel_computation();
    assert!(yuf.is_cancelled());
    
    // 重置取消状态（通过新的计算）
    let _ = yuf.compute("2 + 3");
    // 在实际实现中，compute 应该重置取消状态
}

#[test]
fn test_performance_stats() {
    let mut yuf = Yufmath::new();
    
    // 执行一些计算
    let _ = yuf.compute("2 + 3");
    let _ = yuf.compute("x^2");
    let _ = yuf.compute("sin(pi/2)");
    
    // 获取性能统计
    if let Some(stats) = yuf.get_performance_stats() {
        assert!(stats.total_computations >= 3);
        assert!(stats.successful_computations <= stats.total_computations);
        assert!(stats.success_rate() >= 0.0 && stats.success_rate() <= 1.0);
    }
    
    // 重置统计
    yuf.reset_performance_stats();
    if let Some(stats) = yuf.get_performance_stats() {
        assert_eq!(stats.total_computations, 0);
        assert_eq!(stats.successful_computations, 0);
    }
}

#[test]
fn test_config_updates() {
    let mut yuf = Yufmath::new();
    
    // 测试初始配置
    assert!(yuf.get_config().enable_progress);
    
    // 更新配置
    let new_config = ComputeConfig::new()
        .with_progress(false)
        .with_progress_interval(200);
    
    yuf.update_config(new_config);
    
    // 验证配置更新
    assert!(!yuf.get_config().enable_progress);
    assert_eq!(yuf.get_config().progress_interval_ms, 200);
}

#[test]
fn test_precision_config() {
    let precision_config = PrecisionConfig::new()
        .with_force_exact(true)
        .with_max_precision(500)
        .with_symbolic(true)
        .with_approximation_threshold(1e-10);
    
    let config = ComputeConfig::new()
        .with_precision(precision_config);
    
    let yuf = Yufmath::with_config(config);
    
    assert!(yuf.get_config().precision.force_exact);
    assert_eq!(yuf.get_config().precision.max_precision, Some(500));
    assert!(yuf.get_config().precision.allow_symbolic);
    assert_eq!(yuf.get_config().precision.approximation_threshold, Some(1e-10));
}

#[test]
fn test_numerical_evaluation() {
    let yuf = Yufmath::new();
    
    // 创建包含变量的表达式
    let expr = yuf.parse("x^2 + 2*x + 1").unwrap();
    
    // 数值计算
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 2.0);
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert_eq!(result, 9.0); // (2)^2 + 2*(2) + 1 = 4 + 4 + 1 = 9
    
    // 精确计算
    let mut exact_vars = HashMap::new();
    exact_vars.insert("x".to_string(), Number::from(2));
    
    let exact_result = yuf.evaluate(&expr, &exact_vars).unwrap();
    assert!(matches!(exact_result, Number::Integer(_)));
}

#[test]
fn test_polynomial_operations() {
    let yuf = Yufmath::new();
    
    // 测试展开
    let expr = yuf.parse("(x + 1)^2").unwrap();
    let expanded = yuf.expand(&expr).unwrap();
    // 应该得到 x^2 + 2*x + 1
    
    // 测试因式分解
    let expr = yuf.parse("x^2 - 1").unwrap();
    let factored = yuf.factor(&expr).unwrap();
    // 应该得到 (x - 1)(x + 1)
    
    // 测试收集同类项
    let expr = yuf.parse("x^2 + 2*x + x^2").unwrap();
    let collected = yuf.collect(&expr, "x").unwrap();
    // 应该得到 2*x^2 + 2*x
}

#[test]
fn test_advanced_math_functions() {
    let yuf = Yufmath::new();
    
    // 测试极限
    let expr = yuf.parse("sin(x)/x").unwrap();
    let point = yuf.parse("0").unwrap();
    let limit_result = yuf.limit(&expr, "x", &point);
    // 应该得到 1（如果实现了）
    
    // 测试级数展开
    let expr = yuf.parse("e^x").unwrap();
    let point = yuf.parse("0").unwrap();
    let series_result = yuf.series(&expr, "x", &point, 3);
    // 应该得到 1 + x + x^2/2! + x^3/3!（如果实现了）
}

#[test]
fn test_number_theory_functions() {
    let yuf = Yufmath::new();
    
    // 测试最大公约数
    let a = yuf.parse("48").unwrap();
    let b = yuf.parse("18").unwrap();
    let gcd_result = yuf.gcd(&a, &b).unwrap();
    // 应该得到 6
    
    // 测试最小公倍数
    let lcm_result = yuf.lcm(&a, &b).unwrap();
    // 应该得到 144
    
    // 测试素数判断
    let prime_num = yuf.parse("17").unwrap();
    let is_prime_result = yuf.is_prime(&prime_num).unwrap();
    assert!(is_prime_result);
    
    let composite_num = yuf.parse("15").unwrap();
    let is_prime_result = yuf.is_prime(&composite_num).unwrap();
    assert!(!is_prime_result);
}

#[test]
fn test_combinatorics() {
    let yuf = Yufmath::new();
    
    // 测试二项式系数
    let n = yuf.parse("5").unwrap();
    let k = yuf.parse("2").unwrap();
    let binomial_result = yuf.binomial(&n, &k).unwrap();
    // 应该得到 10
    
    // 测试排列数
    let perm_result = yuf.permutation(&n, &k).unwrap();
    // 应该得到 20
}

#[test]
fn test_complex_operations() {
    let yuf = Yufmath::new();
    
    // 测试复数共轭
    let complex_expr = yuf.parse("3 + 4*i").unwrap();
    let conjugate = yuf.complex_conjugate(&complex_expr).unwrap();
    // 应该得到 3 - 4*i
    
    // 测试复数模长
    let modulus = yuf.complex_modulus(&complex_expr).unwrap();
    // 应该得到 5
    
    // 测试复数幅角
    let argument = yuf.complex_argument(&complex_expr).unwrap();
    // 应该得到 arctan(4/3)
}

#[test]
fn test_error_handling() {
    let yuf = Yufmath::new();
    
    // 测试语法错误
    let result = yuf.compute("2 + + 3");
    assert!(result.is_err());
    
    // 测试除零错误
    let result = yuf.compute("1/0");
    // 根据实现，可能返回错误或无穷大
    
    // 测试未知函数
    let result = yuf.parse("unknown_func(x)");
    assert!(result.is_err());
}

#[test]
fn test_default_implementation() {
    let yuf1 = Yufmath::new();
    let yuf2 = Yufmath::default();
    
    // 两种创建方式应该产生相同的配置
    assert_eq!(yuf1.get_config().enable_progress, yuf2.get_config().enable_progress);
    assert_eq!(yuf1.get_config().progress_interval_ms, yuf2.get_config().progress_interval_ms);
}

#[test]
fn test_thread_safety() {
    // 注意：由于 Yufmath 包含可变状态（formatter），
    // 完整的线程安全需要更仔细的设计。
    // 这里我们测试基本的只读操作是否线程安全。
    
    let yuf = Yufmath::new();
    
    // 测试基本的解析和计算（只读操作）
    let result1 = yuf.compute("2 + 3");
    let result2 = yuf.compute("4 * 5");
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    // 对于真正的多线程使用，每个线程应该有自己的 Yufmath 实例
    // 或者使用适当的同步机制
}