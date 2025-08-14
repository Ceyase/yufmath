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
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! use yufmath::Yufmath;
//!
//! let yuf = Yufmath::new();
//! let result = yuf.compute("x^2 + 2*x + 1").unwrap();
//! println!("结果: {}", result);
//! ```

pub mod core;
pub mod parser;
pub mod engine;
pub mod formatter;
pub mod api;
pub mod cli;
pub mod ffi;

// 重新导出主要的公共接口
pub use api::Yufmath;
pub use core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
pub use engine::{ComputeEngine, ComputeError};
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