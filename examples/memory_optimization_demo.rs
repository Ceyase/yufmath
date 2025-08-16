//! # 内存管理优化演示
//!
//! 演示 Yufmath 的内存管理优化功能，包括表达式共享、写时复制、哈希优化等。

use yufmath::{
    Expression, Number, BinaryOperator,
    SharedExpression, CowExpression, MemoryManager, MemoryMonitor,
    ExpressionBuilder, ExpressionComparator
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Yufmath 内存管理优化演示 ===\n");
    
    // 1. 基本共享表达式演示
    demo_shared_expressions()?;
    
    // 2. 写时复制演示
    demo_copy_on_write()?;
    
    // 3. 内存管理器演示
    demo_memory_manager()?;
    
    // 4. 表达式构建器演示
    demo_expression_builder()?;
    
    // 5. 性能比较演示
    demo_performance_comparison()?;
    
    // 6. 内存监控演示
    demo_memory_monitoring()?;
    
    Ok(())
}

fn demo_shared_expressions() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 共享表达式演示");
    println!("================");
    
    // 创建相同的表达式
    let expr = Expression::Number(Number::integer(42));
    let shared1 = SharedExpression::new(expr.clone());
    let shared2 = shared1.clone_shared();
    
    println!("创建共享表达式:");
    println!("  表达式内容: {:?}", shared1.as_ref());
    println!("  shared1 引用计数: {}", shared1.ref_count());
    println!("  shared2 引用计数: {}", shared2.ref_count());
    println!("  是否相等: {}", shared1 == shared2);
    println!("  shared1 是否唯一: {}", shared1.is_unique());
    
    // 释放一个引用
    drop(shared2);
    println!("\n释放 shared2 后:");
    println!("  shared1 引用计数: {}", shared1.ref_count());
    println!("  shared1 是否唯一: {}", shared1.is_unique());
    
    println!();
    Ok(())
}

fn demo_copy_on_write() -> Result<(), Box<dyn std::error::Error>> {
    println!("2. 写时复制演示");
    println!("==============");
    
    let expr = Expression::Number(Number::integer(42));
    let shared = SharedExpression::new(expr);
    
    let mut cow1 = CowExpression::from_shared(shared.clone_shared());
    let cow2 = CowExpression::from_shared(shared.clone_shared());
    
    println!("创建写时复制表达式:");
    println!("  cow1 引用计数: {}", cow1.ref_count());
    println!("  cow2 引用计数: {}", cow2.ref_count());
    println!("  cow1 是否已修改: {}", cow1.is_modified());
    
    // 修改 cow1，触发写时复制
    println!("\n修改 cow1 (触发写时复制):");
    let mutable_ref = cow1.as_mut();
    *mutable_ref = Expression::Number(Number::integer(43));
    
    println!("  cow1 引用计数: {}", cow1.ref_count());
    println!("  cow2 引用计数: {}", cow2.ref_count());
    println!("  cow1 是否已修改: {}", cow1.is_modified());
    println!("  cow1 内容: {:?}", cow1.as_ref());
    println!("  cow2 内容: {:?}", cow2.as_ref());
    
    println!();
    Ok(())
}

fn demo_memory_manager() -> Result<(), Box<dyn std::error::Error>> {
    println!("3. 内存管理器演示");
    println!("================");
    
    let mut manager = MemoryManager::new();
    
    // 创建多个相同的表达式
    println!("创建多个相同的表达式:");
    let expressions = vec![
        Expression::Number(Number::integer(42)),
        Expression::Number(Number::integer(42)),
        Expression::Variable("x".to_string()),
        Expression::Variable("x".to_string()),
    ];
    
    let mut shared_expressions = Vec::new();
    for (i, expr) in expressions.into_iter().enumerate() {
        let shared = manager.create_shared(expr);
        shared_expressions.push(shared);
        println!("  表达式 {}: 引用计数 {}", i + 1, shared_expressions[i].ref_count());
    }
    
    let stats = manager.get_stats();
    println!("\n内存统计:");
    println!("  活跃表达式: {}", stats.active_expressions);
    println!("  共享表达式: {}", stats.shared_expressions);
    println!("  缓存命中: {}", stats.cache_hits);
    println!("  缓存未命中: {}", stats.cache_misses);
    println!("  估计内存使用: {} 字节", stats.estimated_memory_usage);
    
    // 执行清理
    println!("\n执行内存清理...");
    manager.cleanup();
    let stats_after = manager.get_stats();
    println!("  清理后共享表达式: {}", stats_after.shared_expressions);
    
    println!();
    Ok(())
}

fn demo_expression_builder() -> Result<(), Box<dyn std::error::Error>> {
    println!("4. 表达式构建器演示");
    println!("==================");
    
    let mut builder = ExpressionBuilder::new();
    
    // 创建常用表达式
    println!("创建常用表达式 (应该使用缓存):");
    let x1 = builder.variable("x");
    let x2 = builder.variable("x");
    let one1 = builder.number(Number::integer(1));
    let one2 = builder.number(Number::integer(1));
    
    println!("  x1 == x2: {}", x1 == x2);
    println!("  one1 == one2: {}", one1 == one2);
    
    // 创建复杂表达式并应用简化
    println!("\n创建复杂表达式 (自动简化):");
    let x = builder.variable("x");
    let zero = builder.number(Number::integer(0));
    let one = builder.number(Number::integer(1));
    
    // x + 0 应该简化为 x
    let x_plus_zero = builder.add(x.clone_shared(), zero.clone_shared());
    println!("  x + 0 = {:?}", x_plus_zero.as_ref());
    println!("  简化结果与 x 相等: {}", x_plus_zero == x);
    
    // x * 1 应该简化为 x
    let x_times_one = builder.multiply(x.clone_shared(), one.clone_shared());
    println!("  x * 1 = {:?}", x_times_one.as_ref());
    println!("  简化结果与 x 相等: {}", x_times_one == x);
    
    // x - x 应该简化为 0
    let x_minus_x = builder.subtract(x.clone_shared(), x.clone_shared());
    println!("  x - x = {:?}", x_minus_x.as_ref());
    println!("  简化结果与 0 相等: {}", x_minus_x == zero);
    
    let stats = builder.memory_stats();
    println!("\n构建器内存统计:");
    println!("  缓存命中: {}", stats.cache_hits);
    println!("  缓存未命中: {}", stats.cache_misses);
    
    println!();
    Ok(())
}

fn demo_performance_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("5. 性能比较演示");
    println!("==============");
    
    const NUM_EXPRESSIONS: usize = 1000;
    
    // 测试普通表达式创建
    println!("创建 {} 个普通表达式:", NUM_EXPRESSIONS);
    let start = Instant::now();
    let mut normal_expressions = Vec::new();
    for i in 0..NUM_EXPRESSIONS {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::integer(i as i64))),
        };
        normal_expressions.push(expr);
    }
    let normal_duration = start.elapsed();
    println!("  用时: {:?}", normal_duration);
    
    // 测试共享表达式创建
    println!("\n创建 {} 个共享表达式:", NUM_EXPRESSIONS);
    let start = Instant::now();
    let mut manager = MemoryManager::new();
    let mut shared_expressions = Vec::new();
    for i in 0..NUM_EXPRESSIONS {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::integer(i as i64))),
        };
        let shared = manager.create_shared(expr);
        shared_expressions.push(shared);
    }
    let shared_duration = start.elapsed();
    println!("  用时: {:?}", shared_duration);
    
    let stats = manager.get_stats();
    println!("  缓存命中: {}", stats.cache_hits);
    println!("  缓存未命中: {}", stats.cache_misses);
    println!("  命中率: {:.2}%", 
             stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64 * 100.0);
    
    // 测试表达式比较性能
    println!("\n表达式比较性能测试:");
    let mut comparator = ExpressionComparator::new();
    
    let expr1 = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::integer(2))),
    };
    let expr2 = expr1.clone();
    
    // 普通比较
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = expr1 == expr2;
    }
    let normal_compare_duration = start.elapsed();
    
    // 优化比较
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = comparator.fast_eq(&expr1, &expr2);
    }
    let fast_compare_duration = start.elapsed();
    
    println!("  普通比较 1000 次: {:?}", normal_compare_duration);
    println!("  优化比较 1000 次: {:?}", fast_compare_duration);
    println!("  性能提升: {:.2}x", 
             normal_compare_duration.as_nanos() as f64 / fast_compare_duration.as_nanos() as f64);
    
    println!();
    Ok(())
}

fn demo_memory_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("6. 内存监控演示");
    println!("==============");
    
    let mut monitor = MemoryMonitor::new();
    monitor.set_interval(std::time::Duration::from_millis(100));
    
    println!("创建大量表达式并监控内存使用:");
    
    // 创建大量表达式
    for i in 0..500 {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(Expression::Variable(format!("x{}", i % 10))),
            right: Box::new(Expression::Number(Number::integer(i as i64))),
        };
        
        // 分别获取管理器引用来避免借用冲突
        {
            let manager = monitor.manager();
            let _shared = manager.create_shared(expr);
        }
        
        // 每100个表达式检查一次内存状态
        if i % 100 == 0 {
            if let Some(stats) = monitor.check() {
                println!("  第 {} 个表达式 - 内存使用: {} 字节, 共享表达式: {}", 
                        i, stats.estimated_memory_usage, stats.shared_expressions);
            }
        }
    }
    
    let final_stats = monitor.stats();
    println!("\n最终内存统计:");
    println!("  活跃表达式: {}", final_stats.active_expressions);
    println!("  共享表达式: {}", final_stats.shared_expressions);
    println!("  总缓存命中: {}", final_stats.cache_hits);
    println!("  总缓存未命中: {}", final_stats.cache_misses);
    println!("  估计内存使用: {} 字节", final_stats.estimated_memory_usage);
    
    // 执行清理
    println!("\n执行内存清理...");
    monitor.cleanup();
    let cleaned_stats = monitor.stats();
    println!("  清理后内存使用: {} 字节", cleaned_stats.estimated_memory_usage);
    println!("  清理后共享表达式: {}", cleaned_stats.shared_expressions);
    
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo_functions() {
        assert!(demo_shared_expressions().is_ok());
        assert!(demo_copy_on_write().is_ok());
        assert!(demo_memory_manager().is_ok());
        assert!(demo_expression_builder().is_ok());
        assert!(demo_performance_comparison().is_ok());
        assert!(demo_memory_monitoring().is_ok());
    }
}