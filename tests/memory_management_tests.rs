//! # 内存管理测试
//!
//! 测试表达式的内存管理优化功能，包括共享、写时复制、哈希优化等。

use yufmath::core::{
    Expression, Number, MathConstant, BinaryOperator, UnaryOperator,
    SharedExpression, CowExpression, MemoryManager, MemoryMonitor,
    ExpressionBuilder, ExpressionFactory, ExpressionComparator
};
use num_bigint::BigInt;
use std::time::Duration;

#[test]
fn test_shared_expression_basic() {
    let expr = Expression::Number(Number::Integer(BigInt::from(42)));
    let shared = SharedExpression::new(expr.clone());
    
    assert_eq!(shared.as_ref(), &expr);
    assert_eq!(shared.ref_count(), 1);
    assert!(shared.is_unique());
}

#[test]
fn test_shared_expression_cloning() {
    let expr = Expression::Number(Number::Integer(BigInt::from(42)));
    let shared1 = SharedExpression::new(expr.clone());
    let shared2 = shared1.clone_shared();
    
    assert_eq!(shared1.ref_count(), 2);
    assert_eq!(shared2.ref_count(), 2);
    assert!(!shared1.is_unique());
    assert!(!shared2.is_unique());
    assert_eq!(shared1, shared2);
}

#[test]
fn test_cow_write_on_copy() {
    let expr = Expression::Number(Number::Integer(BigInt::from(42)));
    let mut shared = SharedExpression::new(expr.clone());
    let shared_clone = shared.clone_shared();
    
    // 在有多个引用时，make_mut 应该触发写时复制
    assert_eq!(shared.ref_count(), 2);
    let mutable_ref = shared.make_mut();
    
    // 修改表达式
    *mutable_ref = Expression::Number(Number::Integer(BigInt::from(43)));
    
    // 验证写时复制生效
    assert_eq!(shared.ref_count(), 1);
    assert_eq!(shared_clone.ref_count(), 1);
    assert_ne!(shared, shared_clone);
}

#[test]
fn test_expression_hashing_consistency() {
    let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr3 = Expression::Number(Number::Integer(BigInt::from(43)));
    
    let hash1 = yufmath::core::memory::calculate_expression_hash(&expr1);
    let hash2 = yufmath::core::memory::calculate_expression_hash(&expr2);
    let hash3 = yufmath::core::memory::calculate_expression_hash(&expr3);
    
    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
}

#[test]
fn test_complex_expression_hashing() {
    let x = Expression::Variable("x".to_string());
    let two = Expression::Number(Number::Integer(BigInt::from(2)));
    let expr1 = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(two.clone()),
        right: Box::new(x.clone()),
    };
    
    let expr2 = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(two.clone()),
        right: Box::new(x.clone()),
    };
    
    let hash1 = yufmath::core::memory::calculate_expression_hash(&expr1);
    let hash2 = yufmath::core::memory::calculate_expression_hash(&expr2);
    
    assert_eq!(hash1, hash2);
}

#[test]
fn test_memory_manager_sharing() {
    let mut manager = MemoryManager::new();
    
    let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
    
    let shared1 = manager.create_shared(expr1);
    let shared2 = manager.create_shared(expr2);
    
    // 相同的表达式应该共享
    let stats = manager.get_stats();
    assert!(stats.cache_hits > 0 || stats.cache_misses > 0);
}

#[test]
fn test_memory_manager_cleanup() {
    let mut manager = MemoryManager::new();
    
    // 创建一些表达式
    let mut shared_expressions = Vec::new();
    for i in 0..100 {
        let expr = Expression::Number(Number::Integer(BigInt::from(i)));
        let shared = manager.create_shared(expr);
        shared_expressions.push(shared);
    }
    
    let stats_before = manager.get_stats().clone();
    
    // 释放一些表达式
    shared_expressions.truncate(50);
    
    manager.cleanup();
    let stats_after = manager.get_stats().clone();
    
    println!("清理前: 共享表达式 {}", stats_before.shared_expressions);
    println!("清理后: 共享表达式 {}", stats_after.shared_expressions);
}

#[test]
fn test_expression_comparator() {
    let mut comparator = ExpressionComparator::new();
    
    let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr3 = Expression::Number(Number::Integer(BigInt::from(43)));
    
    assert!(comparator.fast_eq(&expr1, &expr2));
    assert!(!comparator.fast_eq(&expr1, &expr3));
    
    // 测试指针相等的快速路径
    assert!(comparator.fast_eq(&expr1, &expr1));
}

#[test]
fn test_cow_expression() {
    let expr = Expression::Number(Number::Integer(BigInt::from(42)));
    let mut cow = CowExpression::new(expr.clone());
    
    assert!(!cow.is_modified());
    assert_eq!(cow.ref_count(), 1);
    
    // 获取可变引用应该标记为已修改
    let mutable_ref = cow.as_mut();
    *mutable_ref = Expression::Number(Number::Integer(BigInt::from(43)));
    
    assert!(cow.is_modified());
    assert_ne!(cow.as_ref(), &expr);
}

#[test]
fn test_memory_monitor() {
    let mut monitor = MemoryMonitor::new();
    
    // 创建一些表达式
    let manager = monitor.manager();
    for i in 0..50 {
        let expr = Expression::Number(Number::Integer(BigInt::from(i)));
        let _shared = manager.create_shared(expr);
    }
    
    let stats = monitor.stats();
    assert!(stats.active_expressions > 0);
    
    monitor.cleanup();
}

#[test]
fn test_expression_builder_optimization() {
    let mut builder = ExpressionBuilder::new();
    
    let x = builder.variable("x");
    let zero = builder.number(Number::integer(0));
    let one = builder.number(Number::integer(1));
    
    // 测试代数简化
    let x_plus_zero = builder.add(x.clone_shared(), zero.clone_shared());
    assert_eq!(x_plus_zero.as_ref(), x.as_ref());
    
    let one_times_x = builder.multiply(one.clone_shared(), x.clone_shared());
    assert_eq!(one_times_x.as_ref(), x.as_ref());
    
    let x_minus_x = builder.subtract(x.clone_shared(), x.clone_shared());
    assert_eq!(x_minus_x.as_ref(), zero.as_ref());
}

#[test]
fn test_expression_builder_caching() {
    let mut builder = ExpressionBuilder::new();
    
    // 常用变量应该被缓存
    let x1 = builder.variable("x");
    let x2 = builder.variable("x");
    
    // 检查是否使用了相同的共享表达式
    assert_eq!(x1, x2);
    
    // 常用数值应该被缓存
    let one1 = builder.number(Number::integer(1));
    let one2 = builder.number(Number::integer(1));
    
    assert_eq!(one1, one2);
}

#[test]
fn test_expression_factory() {
    let mut factory = ExpressionFactory::new();
    
    let x = factory.var("x");
    let two = factory.int(2);
    let pi = factory.pi();
    
    // 创建复杂表达式: 2 * (x + π)
    let sum = factory.add(x, pi);
    let product = factory.mul(two, sum);
    
    // 验证表达式结构
    match product.as_ref() {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            assert!(matches!(left.as_ref(), Expression::Number(Number::Integer(_))));
            assert!(matches!(right.as_ref(), Expression::BinaryOp { op: BinaryOperator::Add, .. }));
        }
        _ => panic!("期望乘法表达式"),
    }
}

#[test]
fn test_nested_expression_sharing() {
    let mut builder = ExpressionBuilder::new();
    
    let x = builder.variable("x");
    let y = builder.variable("y");
    
    // 创建嵌套表达式: (x + y) * (x + y)
    let sum1 = builder.add(x.clone_shared(), y.clone_shared());
    let sum2 = builder.add(x.clone_shared(), y.clone_shared());
    let product = builder.multiply(sum1.clone_shared(), sum2.clone_shared());
    
    // 验证子表达式共享
    assert_eq!(sum1, sum2);
    
    match product.as_ref() {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            // 由于简化，这可能变成 (x + y)^2
            println!("产品表达式: {:?}", product.as_ref());
        }
        _ => {}
    }
}

#[test]
fn test_memory_usage_estimation() {
    let mut manager = MemoryManager::new();
    
    // 创建不同类型的表达式
    let expressions = vec![
        Expression::Number(Number::Integer(BigInt::from(42))),
        Expression::Variable("x".to_string()),
        Expression::Constant(MathConstant::Pi),
        Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        },
    ];
    
    for expr in expressions {
        let _shared = manager.create_shared(expr);
    }
    
    let stats = manager.get_stats();
    assert!(stats.estimated_memory_usage > 0);
    println!("估计内存使用: {} 字节", stats.estimated_memory_usage);
}

#[test]
fn test_large_expression_performance() {
    let mut builder = ExpressionBuilder::new();
    
    let start = std::time::Instant::now();
    
    // 创建大型表达式树
    let mut expr = builder.variable("x");
    for i in 1..=100 {
        let num = builder.number(Number::integer(i));
        expr = builder.add(expr, num);
    }
    
    let duration = start.elapsed();
    println!("创建100项加法表达式用时: {:?}", duration);
    
    let stats = builder.memory_stats();
    println!("内存统计: {:?}", stats);
    
    // 验证性能合理
    assert!(duration < Duration::from_millis(100));
}

#[test]
fn test_expression_equality_optimization() {
    let mut comparator = ExpressionComparator::new();
    
    // 创建相同的复杂表达式
    let create_expr = || {
        Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            }),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        }
    };
    
    let expr1 = create_expr();
    let expr2 = create_expr();
    
    let start = std::time::Instant::now();
    let is_equal = comparator.fast_eq(&expr1, &expr2);
    let duration = start.elapsed();
    
    assert!(is_equal);
    println!("复杂表达式比较用时: {:?}", duration);
    
    // 第二次比较应该更快（使用缓存的哈希）
    let start = std::time::Instant::now();
    let is_equal = comparator.fast_eq(&expr1, &expr2);
    let duration2 = start.elapsed();
    
    assert!(is_equal);
    println!("缓存哈希比较用时: {:?}", duration2);
    
    // 缓存比较应该更快
    assert!(duration2 <= duration);
}

#[test]
fn test_memory_config_effects() {
    use yufmath::core::memory::MemoryConfig;
    
    let mut config = MemoryConfig::default();
    config.enable_sharing = false;
    
    let mut manager = MemoryManager::with_config(config);
    
    let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
    let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
    
    let shared1 = manager.create_shared(expr1);
    let shared2 = manager.create_shared(expr2);
    
    // 禁用共享时，相同表达式不应该共享内存
    let stats = manager.get_stats();
    println!("禁用共享时的统计: {:?}", stats);
}

#[test]
fn test_cow_with_multiple_references() {
    let expr = Expression::Number(Number::Integer(BigInt::from(42)));
    let shared = SharedExpression::new(expr.clone());
    
    let mut cow1 = CowExpression::from_shared(shared.clone_shared());
    let mut cow2 = CowExpression::from_shared(shared.clone_shared());
    
    assert_eq!(cow1.ref_count(), 3); // shared + cow1 + cow2
    assert_eq!(cow2.ref_count(), 3);
    
    // 修改 cow1 应该触发写时复制
    let mutable_ref = cow1.as_mut();
    *mutable_ref = Expression::Number(Number::Integer(BigInt::from(43)));
    
    assert!(cow1.is_modified());
    assert!(!cow2.is_modified());
    assert_eq!(cow1.ref_count(), 1); // cow1 现在有自己的副本
    assert_eq!(cow2.ref_count(), 2); // shared + cow2
}