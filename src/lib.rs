//! # Yufmath - 计算机代数系统
//!
//! Yufmath 是一个基于 Rust 编写的高性能计算机代数系统（CAS）库，
//! 提供符号数学计算、精确算术运算和多种接口支持。
//!
//! ## 主要特性
//!
//! - **精确计算**：支持任意精度整数、有理数和复数运算
//! - **符号计算**：代数表达式的符号操作和简化
//! - **微积分**：符号求导和积分
//! - **多接口支持**：Rust API、C++ 绑定、命令行工具
//! - **高性能**：优化的算法和缓存机制
//! - **进度监控**：支持长时间计算的进度跟踪和取消
//! - **批量处理**：支持批量计算和并行处理
//!
//! ## 快速开始
//!
//! ### 基本使用
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! // 创建 Yufmath 实例
//! let yuf = Yufmath::new();
//!
//! // 基本计算
//! let result = yuf.compute("2 + 3 * 4").unwrap();
//! println!("计算结果: {}", result); // 输出: 14
//!
//! // 符号计算
//! let result = yuf.compute("x + x").unwrap();
//! println!("简化结果: {}", result); // 输出: 2*x
//!
//! // 求导
//! let expr = yuf.parse("x^2 + 2*x + 1").unwrap();
//! let derivative = yuf.diff(&expr, "x").unwrap();
//! println!("导数: {:?}", derivative);
//!
//! // 积分
//! let integral = yuf.integrate(&expr, "x").unwrap();
//! println!("积分: {:?}", integral);
//! ```
//!
//! ### 配置使用
//!
//! ```rust,ignore
//! use yufmath::{Yufmath, ComputeConfig, PrecisionConfig};
//! use std::time::Duration;
//!
//! // 创建自定义配置
//! let precision_config = PrecisionConfig::new()
//!     .with_force_exact(true)
//!     .with_max_precision(1000);
//!
//! let config = ComputeConfig::new()
//!     .with_progress(true)
//!     .with_max_compute_time(Duration::from_secs(60))
//!     .with_precision(precision_config);
//!
//! // 使用配置创建实例
//! let yuf = Yufmath::with_config(config);
//! ```
//!
//! ### 进度监控
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! let mut yuf = Yufmath::new();
//!
//! // 设置进度回调
//! yuf.set_progress_callback(Box::new(|progress| {
//!     println!("进度: {:.1}% - {}", 
//!             progress.progress * 100.0, 
//!             progress.current_step);
//!     true // 返回 true 继续计算，false 取消计算
//! }));
//!
//! // 带进度的计算
//! let result = yuf.compute_with_progress("integrate(sin(x^2), x)").unwrap();
//! ```
//!
//! ### 批量计算
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! let yuf = Yufmath::new();
//!
//! // 批量计算
//! let expressions = vec!["2 + 3", "x^2 + 1", "sin(pi/2)"];
//! let results = yuf.batch_compute(&expressions);
//!
//! for (expr, result) in expressions.iter().zip(results.iter()) {
//!     match result {
//!         Ok(value) => println!("{} = {}", expr, value),
//!         Err(e) => println!("{} -> 错误: {}", expr, e),
//!     }
//! }
//! ```
//!
//! ### 高级数学功能
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! let yuf = Yufmath::new();
//!
//! // 多项式运算
//! let poly = yuf.parse("(x + 1)^3").unwrap();
//! let expanded = yuf.expand(&poly).unwrap();
//! println!("展开: {:?}", expanded);
//!
//! // 方程求解
//! let equation = yuf.parse("x^2 - 4").unwrap();
//! let solutions = yuf.solve(&equation, "x").unwrap();
//! println!("解: {:?}", solutions);
//!
//! // 数论函数
//! let gcd_result = yuf.gcd(&yuf.parse("48").unwrap(), &yuf.parse("18").unwrap()).unwrap();
//! println!("最大公约数: {:?}", gcd_result);
//!
//! // 矩阵运算
//! let matrix_a = yuf.parse("[[1,2],[3,4]]").unwrap();
//! let matrix_b = yuf.parse("[[5,6],[7,8]]").unwrap();
//! let product = yuf.matrix_multiply(&matrix_a, &matrix_b).unwrap();
//! println!("矩阵乘积: {:?}", product);
//! ```
//!
//! ## 错误处理
//!
//! Yufmath 提供了完善的错误处理机制：
//!
//! ```rust,ignore
//! use yufmath::{Yufmath, YufmathError};
//!
//! let yuf = Yufmath::new();
//!
//! match yuf.compute("2 + + 3") {
//!     Ok(result) => println!("结果: {}", result),
//!     Err(e) => {
//!         println!("错误: {}", e.user_friendly_message());
//!         println!("建议: {:?}", e.suggestions());
//!         
//!         // 生成完整的错误报告
//!         println!("{}", e.format_error_report(Some("2 + + 3")));
//!     }
//! }
//! ```
//!
//! ## 性能监控
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! let mut yuf = Yufmath::new();
//!
//! // 执行一些计算
//! yuf.compute("x^2 + 2*x + 1").unwrap();
//! yuf.compute("integrate(sin(x), x)").unwrap();
//!
//! // 获取性能统计
//! if let Some(stats) = yuf.get_performance_stats() {
//!     println!("总计算次数: {}", stats.total_computations);
//!     println!("成功率: {:.2}%", stats.success_rate() * 100.0);
//!     println!("平均计算时间: {:?}", stats.avg_compute_time);
//!     println!("精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
//! }
//! ```

pub mod core;
pub mod parser;
pub mod engine;
pub mod formatter;
pub mod api;
pub mod cli;
pub mod ffi;

// 重新导出主要的公共接口
pub use api::{
    Yufmath, 
    YufmathError, 
    ComputeConfig, 
    PrecisionConfig,
    ParallelConfig,
    CacheConfig,
    ComputeProgress,
    ComputePhase,
    PerformanceStats, 
    PerformanceMonitor,
    ProgressCallback,
    AsyncComputation,
    BatchAsyncComputer,
    AsyncConfig,
    TaskStatus,
};
pub use core::{
    Expression, Number, MathConstant, BinaryOperator, UnaryOperator,
    SharedExpression, CowExpression, MemoryManager, MemoryMonitor,
    MemoryStats, MemoryConfig, ExpressionComparator, ExpressionBuilder, ExpressionFactory
};
pub use engine::{
    ComputeEngine, ComputeError,
    LazyExpression, DependencyGraph, LazyState, DependencyGraphStats,
    ParallelComputeEngine, TaskScheduler, ComputeTask, SchedulerStats, 
    ExpressionPreprocessor, ParallelizationAnalysis
};
pub use parser::{Parser, ParseError};
pub use formatter::{Formatter, FormatOptions, FormatType};

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 库的描述
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_info() {
        assert_eq!(VERSION, "0.1.0");
        assert_eq!(NAME, "yufmath");
        assert!(!DESCRIPTION.is_empty());
    }
    
    #[test]
    fn test_number_creation() {
        let num1 = Number::from(42);
        assert!(matches!(num1, Number::Integer(_)));
        
        let num2 = Number::from(3.14);
        assert!(matches!(num2, Number::Float(_)));
    }
    
    #[test]
    fn test_expression_creation() {
        let expr = Expression::variable("x");
        assert!(matches!(expr, Expression::Variable(_)));
        
        let num_expr = Expression::number(Number::from(5));
        assert!(matches!(num_expr, Expression::Number(_)));
    }
    
    #[test]
    fn test_math_constants() {
        let pi = MathConstant::Pi;
        assert_eq!(pi.symbol(), "π");
        assert_eq!(pi.name(), "圆周率");
        
        let e = MathConstant::E;
        assert_eq!(e.symbol(), "e");
        assert_eq!(e.name(), "自然常数");
    }
    
    #[test]
    fn test_operators() {
        let add_op = BinaryOperator::Add;
        assert_eq!(add_op.symbol(), "+");
        assert_eq!(add_op.name(), "加法");
        assert_eq!(add_op.precedence(), 6);
        
        let neg_op = UnaryOperator::Negate;
        assert_eq!(neg_op.symbol(), "-");
        assert_eq!(neg_op.name(), "负号");
    }
}