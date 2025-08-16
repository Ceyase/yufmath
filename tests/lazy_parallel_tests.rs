//! # 惰性求值和并行计算测试
//!
//! 测试惰性表达式系统和并行计算功能。

use yufmath::core::{Expression, Number};
use yufmath::engine::{
    LazyExpression, DependencyGraph, LazyState,
    ParallelComputeEngine, TaskScheduler, ExpressionPreprocessor,
};
use yufmath::engine::compute::BasicComputeEngine;
use yufmath::api::config::ParallelConfig;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_lazy_expression_basic() {
    // 测试基本的惰性表达式创建和状态管理
    let expr = Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    );
    
    let lazy_expr = LazyExpression::new(1, expr.clone());
    
    assert_eq!(lazy_expr.id(), 1);
    assert_eq!(lazy_expr.original(), &expr);
    assert!(matches!(lazy_expr.state(), LazyState::Pending));
    assert!(!lazy_expr.is_computed());
    assert!(!lazy_expr.is_failed());
    assert!(!lazy_expr.is_computing());
    assert!(lazy_expr.can_compute()); // 没有依赖，可以计算
}

#[test]
fn test_lazy_expression_computation() {
    // 测试惰性表达式的强制计算
    let engine = BasicComputeEngine::new();
    
    let expr = Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    );
    
    let lazy_expr = LazyExpression::new(1, expr);
    
    // 强制计算
    let result = lazy_expr.force_compute(&engine).unwrap();
    
    // 验证结果
    assert!(lazy_expr.is_computed());
    assert_eq!(lazy_expr.get_result(), Some(result.clone()));
    
    // 验证计算结果是否正确
    if let Expression::Number(n) = result {
        assert_eq!(n, Number::from(5));
    } else {
        panic!("期望得到数值结果");
    }
}

#[test]
fn test_lazy_expression_with_custom_compute_fn() {
    // 测试带自定义计算函数的惰性表达式
    let engine = BasicComputeEngine::new();
    
    let expr = Expression::variable("x");
    
    // 创建自定义计算函数：将变量 x 替换为数值 42
    let compute_fn = |_expr: &Expression, _engine: &dyn yufmath::engine::ComputeEngine| {
        Ok(Expression::number(42.into()))
    };
    
    let lazy_expr = LazyExpression::with_compute_fn(1, expr, compute_fn);
    
    // 强制计算
    let result = lazy_expr.force_compute(&engine).unwrap();
    
    // 验证结果
    assert!(lazy_expr.is_computed());
    if let Expression::Number(n) = result {
        assert_eq!(n, Number::from(42));
    } else {
        panic!("期望得到数值结果");
    }
}

#[test]
fn test_dependency_graph_basic() {
    // 测试基本的依赖图操作
    let mut graph = DependencyGraph::new();
    
    // 添加表达式
    let expr1 = graph.add_expression(Expression::variable("x"));
    let expr2 = graph.add_expression(Expression::variable("y"));
    let expr3 = graph.add_expression(Expression::add(
        Expression::variable("x"),
        Expression::variable("y")
    ));
    
    assert_eq!(expr1.id(), 1);
    assert_eq!(expr2.id(), 2);
    assert_eq!(expr3.id(), 3);
    
    // 添加依赖关系：expr3 依赖 expr1 和 expr2
    graph.add_dependency(expr3.id(), expr1.id()).unwrap();
    graph.add_dependency(expr3.id(), expr2.id()).unwrap();
    
    // 获取统计信息
    let stats = graph.get_stats();
    assert_eq!(stats.total_expressions, 3);
    assert_eq!(stats.pending_expressions, 3);
    assert_eq!(stats.total_dependencies, 2);
}

#[test]
fn test_dependency_graph_topological_sort() {
    // 测试拓扑排序
    let mut graph = DependencyGraph::new();
    
    // 创建表达式：x, y, x+y, z, (x+y)*z
    let expr_x = graph.add_expression(Expression::variable("x"));
    let expr_y = graph.add_expression(Expression::variable("y"));
    let expr_z = graph.add_expression(Expression::variable("z"));
    let expr_sum = graph.add_expression(Expression::add(
        Expression::variable("x"),
        Expression::variable("y")
    ));
    let expr_product = graph.add_expression(Expression::multiply(
        Expression::add(Expression::variable("x"), Expression::variable("y")),
        Expression::variable("z")
    ));
    
    // 添加依赖关系
    graph.add_dependency(expr_sum.id(), expr_x.id()).unwrap();
    graph.add_dependency(expr_sum.id(), expr_y.id()).unwrap();
    graph.add_dependency(expr_product.id(), expr_sum.id()).unwrap();
    graph.add_dependency(expr_product.id(), expr_z.id()).unwrap();
    
    // 获取拓扑排序
    let sorted = graph.topological_sort().unwrap();
    assert_eq!(sorted.len(), 5);
    
    // 验证依赖关系：x, y, z 应该在 x+y 之前
    let pos_x = sorted.iter().position(|&id| id == expr_x.id()).unwrap();
    let pos_y = sorted.iter().position(|&id| id == expr_y.id()).unwrap();
    let pos_z = sorted.iter().position(|&id| id == expr_z.id()).unwrap();
    let pos_sum = sorted.iter().position(|&id| id == expr_sum.id()).unwrap();
    let pos_product = sorted.iter().position(|&id| id == expr_product.id()).unwrap();
    
    assert!(pos_x < pos_sum);
    assert!(pos_y < pos_sum);
    assert!(pos_z < pos_product);
    assert!(pos_sum < pos_product);
}

#[test]
fn test_dependency_graph_parallel_groups() {
    // 测试并行组生成
    let mut graph = DependencyGraph::new();
    
    // 创建表达式：x, y, z, x+y, z*2, (x+y)+(z*2)
    let expr_x = graph.add_expression(Expression::variable("x"));
    let expr_y = graph.add_expression(Expression::variable("y"));
    let expr_z = graph.add_expression(Expression::variable("z"));
    let expr_sum = graph.add_expression(Expression::add(
        Expression::variable("x"),
        Expression::variable("y")
    ));
    let expr_double_z = graph.add_expression(Expression::multiply(
        Expression::variable("z"),
        Expression::number(2.into())
    ));
    let expr_final = graph.add_expression(Expression::add(
        Expression::add(Expression::variable("x"), Expression::variable("y")),
        Expression::multiply(Expression::variable("z"), Expression::number(2.into()))
    ));
    
    // 添加依赖关系
    graph.add_dependency(expr_sum.id(), expr_x.id()).unwrap();
    graph.add_dependency(expr_sum.id(), expr_y.id()).unwrap();
    graph.add_dependency(expr_double_z.id(), expr_z.id()).unwrap();
    graph.add_dependency(expr_final.id(), expr_sum.id()).unwrap();
    graph.add_dependency(expr_final.id(), expr_double_z.id()).unwrap();
    
    // 获取并行组
    let groups = graph.get_parallel_groups().unwrap();
    
    // 第一组应该包含 x, y, z（可以并行计算）
    // 第二组应该包含 x+y 和 z*2（可以并行计算）
    // 第三组应该包含最终表达式
    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].len(), 3); // x, y, z
    assert_eq!(groups[1].len(), 2); // x+y, z*2
    assert_eq!(groups[2].len(), 1); // final
}

#[test]
fn test_dependency_graph_cycle_detection() {
    // 测试循环依赖检测
    let mut graph = DependencyGraph::new();
    
    let expr1 = graph.add_expression(Expression::variable("x"));
    let expr2 = graph.add_expression(Expression::variable("y"));
    
    // 添加正常依赖
    graph.add_dependency(expr2.id(), expr1.id()).unwrap();
    
    // 尝试添加循环依赖
    let result = graph.add_dependency(expr1.id(), expr2.id());
    assert!(result.is_err());
}

#[test]
fn test_dependency_graph_computation() {
    // 测试依赖图的完整计算流程
    let engine = BasicComputeEngine::new();
    let mut graph = DependencyGraph::new();
    
    // 创建表达式：2, 3, 2+3
    let expr1 = graph.add_expression(Expression::number(2.into()));
    let expr2 = graph.add_expression(Expression::number(3.into()));
    let expr3 = graph.add_expression(Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    ));
    
    // 添加依赖关系
    graph.add_dependency(expr3.id(), expr1.id()).unwrap();
    graph.add_dependency(expr3.id(), expr2.id()).unwrap();
    
    // 计算所有表达式
    let sorted = graph.topological_sort().unwrap();
    for expr_id in sorted {
        if let Some(expr) = graph.get_expression(expr_id) {
            expr.force_compute(&engine).unwrap();
        }
    }
    
    // 验证结果
    assert!(expr1.is_computed());
    assert!(expr2.is_computed());
    assert!(expr3.is_computed());
    
    if let Some(Expression::Number(n)) = expr3.get_result() {
        assert_eq!(n, Number::from(5));
    } else {
        panic!("期望得到数值结果");
    }
}

#[test]
fn test_task_scheduler() {
    // 测试任务调度器
    let config = ParallelConfig::default();
    let scheduler = TaskScheduler::new(config);
    
    // 创建测试表达式
    let expr = Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    );
    let lazy_expr = Arc::new(LazyExpression::new(1, expr));
    
    // 添加任务
    let task_id = scheduler.add_task(lazy_expr.clone());
    assert_eq!(scheduler.pending_count(), 1);
    assert_eq!(scheduler.running_count(), 0);
    assert_eq!(scheduler.completed_count(), 0);
    
    // 获取任务
    let task = scheduler.get_next_task().unwrap();
    assert_eq!(task.id, task_id);
    assert_eq!(scheduler.pending_count(), 0);
    assert_eq!(scheduler.running_count(), 1);
    
    // 完成任务
    let result = Expression::number(5.into());
    scheduler.complete_task(task_id, Ok(result.clone()));
    assert_eq!(scheduler.running_count(), 0);
    assert_eq!(scheduler.completed_count(), 1);
    
    // 获取结果
    let task_result = scheduler.get_task_result(task_id).unwrap();
    assert!(task_result.is_ok());
    assert_eq!(task_result.unwrap(), result);
}

#[test]
fn test_parallel_compute_engine() {
    // 测试并行计算引擎
    let base_engine = Arc::new(BasicComputeEngine::new());
    let config = ParallelConfig::default();
    let parallel_engine = ParallelComputeEngine::new(base_engine, config).unwrap();
    
    // 测试并行计算多个表达式
    let expressions = vec![
        Expression::add(Expression::number(1.into()), Expression::number(2.into())),
        Expression::multiply(Expression::number(3.into()), Expression::number(4.into())),
        Expression::subtract(Expression::number(10.into()), Expression::number(5.into())),
        Expression::divide(Expression::number(20.into()), Expression::number(4.into())),
    ];
    
    let results = parallel_engine.compute_parallel(expressions);
    assert_eq!(results.len(), 4);
    
    // 验证所有结果都成功
    for result in &results {
        assert!(result.is_ok());
    }
    
    // 验证具体结果
    if let Ok(Expression::Number(n)) = &results[0] {
        assert_eq!(*n, Number::from(3));
    }
    if let Ok(Expression::Number(n)) = &results[1] {
        assert_eq!(*n, Number::from(12));
    }
    if let Ok(Expression::Number(n)) = &results[2] {
        assert_eq!(*n, Number::from(5));
    }
    if let Ok(Expression::Number(n)) = &results[3] {
        assert_eq!(*n, Number::from(5));
    }
}

#[test]
fn test_parallel_compute_with_dependencies() {
    // 测试带依赖关系的并行计算
    let base_engine = Arc::new(BasicComputeEngine::new());
    let config = ParallelConfig::default();
    let parallel_engine = ParallelComputeEngine::new(base_engine, config).unwrap();
    
    let mut graph = DependencyGraph::new();
    
    // 创建表达式：1, 2, 1+2, 3, (1+2)*3
    let expr1 = graph.add_expression(Expression::number(1.into()));
    let expr2 = graph.add_expression(Expression::number(2.into()));
    let expr3 = graph.add_expression(Expression::number(3.into()));
    let expr_sum = graph.add_expression(Expression::add(
        Expression::number(1.into()),
        Expression::number(2.into())
    ));
    let expr_product = graph.add_expression(Expression::multiply(
        Expression::add(Expression::number(1.into()), Expression::number(2.into())),
        Expression::number(3.into())
    ));
    
    // 添加依赖关系
    graph.add_dependency(expr_sum.id(), expr1.id()).unwrap();
    graph.add_dependency(expr_sum.id(), expr2.id()).unwrap();
    graph.add_dependency(expr_product.id(), expr_sum.id()).unwrap();
    graph.add_dependency(expr_product.id(), expr3.id()).unwrap();
    
    // 执行并行计算
    parallel_engine.compute_with_dependencies(&mut graph).unwrap();
    
    // 验证所有表达式都已计算完成
    assert!(expr1.is_computed());
    assert!(expr2.is_computed());
    assert!(expr3.is_computed());
    assert!(expr_sum.is_computed());
    assert!(expr_product.is_computed());
    
    // 验证最终结果
    if let Some(Expression::Number(n)) = expr_product.get_result() {
        assert_eq!(n, Number::from(9)); // (1+2)*3 = 9
    } else {
        panic!("期望得到数值结果");
    }
}

#[test]
fn test_expression_preprocessor() {
    // 测试表达式预处理器
    let config = ParallelConfig::default();
    let preprocessor = ExpressionPreprocessor::new(config);
    
    // 测试常量折叠
    let expr = Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    );
    
    let folded = preprocessor.preprocess(&expr);
    
    // 应该被折叠为常量 5
    if let Expression::Number(n) = folded {
        assert_eq!(n, Number::from(5));
    } else {
        panic!("常量折叠失败");
    }
}

#[test]
fn test_expression_preprocessor_algebraic_rules() {
    // 测试代数简化规则
    let config = ParallelConfig::default();
    let preprocessor = ExpressionPreprocessor::new(config);
    
    // 测试 x + 0 = x
    let expr = Expression::add(
        Expression::variable("x"),
        Expression::number(0.into())
    );
    
    let simplified = preprocessor.preprocess(&expr);
    assert_eq!(simplified, Expression::variable("x"));
    
    // 测试 x * 1 = x
    let expr = Expression::multiply(
        Expression::variable("x"),
        Expression::number(1.into())
    );
    
    let simplified = preprocessor.preprocess(&expr);
    assert_eq!(simplified, Expression::variable("x"));
    
    // 测试 x * 0 = 0
    let expr = Expression::multiply(
        Expression::variable("x"),
        Expression::number(0.into())
    );
    
    let simplified = preprocessor.preprocess(&expr);
    assert_eq!(simplified, Expression::number(0.into()));
}

#[test]
fn test_parallelization_analysis() {
    // 测试并行化分析
    let config = ParallelConfig::default();
    let preprocessor = ExpressionPreprocessor::new(config);
    
    // 创建简单表达式
    let simple_expr = Expression::add(
        Expression::variable("x"),
        Expression::number(1.into())
    );
    
    let analysis = preprocessor.analyze_parallelization_potential(&simple_expr);
    assert!(analysis.complexity > 0);
    assert!(!analysis.recommended_parallel); // 简单表达式不推荐并行
    
    // 创建复杂表达式
    let complex_expr = Expression::add(
        Expression::multiply(
            Expression::add(Expression::variable("x"), Expression::variable("y")),
            Expression::add(Expression::variable("z"), Expression::variable("w"))
        ),
        Expression::multiply(
            Expression::add(Expression::variable("a"), Expression::variable("b")),
            Expression::add(Expression::variable("c"), Expression::variable("d"))
        )
    );
    
    let analysis = preprocessor.analyze_parallelization_potential(&complex_expr);
    assert!(analysis.complexity > 10); // 复杂表达式
    assert!(analysis.independent_parts_count > 1);
    assert!(analysis.estimated_speedup > 1.0);
}

#[test]
fn test_scheduler_stats() {
    // 测试调度器统计信息
    let base_engine = Arc::new(BasicComputeEngine::new());
    let config = ParallelConfig::default();
    let parallel_engine = ParallelComputeEngine::new(base_engine, config).unwrap();
    
    let stats = parallel_engine.get_scheduler_stats();
    assert_eq!(stats.pending_tasks, 0);
    assert_eq!(stats.running_tasks, 0);
    assert_eq!(stats.completed_tasks, 0);
    assert!(!stats.is_running);
}

#[test]
fn test_lazy_expression_reset() {
    // 测试惰性表达式状态重置
    let engine = BasicComputeEngine::new();
    
    let expr = Expression::add(
        Expression::number(2.into()),
        Expression::number(3.into())
    );
    
    let lazy_expr = LazyExpression::new(1, expr);
    
    // 计算表达式
    lazy_expr.force_compute(&engine).unwrap();
    assert!(lazy_expr.is_computed());
    
    // 重置状态
    lazy_expr.reset();
    assert!(matches!(lazy_expr.state(), LazyState::Pending));
    assert!(!lazy_expr.is_computed());
}

#[test]
fn test_dependency_graph_cleanup() {
    // 测试依赖图清理功能
    let engine = BasicComputeEngine::new();
    let mut graph = DependencyGraph::new();
    
    // 添加一些表达式
    let expr1 = graph.add_expression(Expression::number(1.into()));
    let expr2 = graph.add_expression(Expression::number(2.into()));
    
    // 计算表达式
    expr1.force_compute(&engine).unwrap();
    expr2.force_compute(&engine).unwrap();
    
    // 验证表达式已完成
    let stats_before = graph.get_stats();
    assert_eq!(stats_before.total_expressions, 2);
    assert_eq!(stats_before.computed_expressions, 2);
    
    // 清理已完成的表达式
    graph.cleanup_completed();
    
    // 验证清理结果
    let stats_after = graph.get_stats();
    assert_eq!(stats_after.total_expressions, 0);
}

#[test]
fn test_parallel_config() {
    // 测试并行配置
    let config = ParallelConfig::new()
        .with_enabled(true)
        .with_thread_count(4)
        .with_complexity_threshold(50)
        .with_max_parallel_tasks(8);
    
    assert!(config.enabled);
    assert_eq!(config.thread_count, Some(4));
    assert_eq!(config.complexity_threshold, 50);
    assert_eq!(config.max_parallel_tasks, 8);
}

#[test]
fn test_compute_task_weight() {
    // 测试计算任务权重
    let expr = Expression::variable("x");
    let lazy_expr = Arc::new(LazyExpression::new(1, expr));
    let task = yufmath::engine::ComputeTask::new(1, lazy_expr);
    
    let weight = task.weight();
    assert!(weight >= 0.0); // 权重应该是非负数
}