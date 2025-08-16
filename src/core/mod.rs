//! # 核心数据结构和类型定义
//!
//! 本模块包含 Yufmath 库的核心数据结构，包括表达式、数值类型、
//! 运算符定义等基础组件。

pub mod expression;
pub mod number;
pub mod constants;
pub mod operators;
pub mod types;
pub mod high_precision;
pub mod memory;
pub mod expression_builder;

#[cfg(test)]
pub mod precision_test;

// 重新导出主要类型
pub use expression::Expression;
pub use number::Number;
pub use constants::MathConstant;
pub use operators::{BinaryOperator, UnaryOperator};
pub use types::{ExprType, NumericType};
pub use memory::{
    SharedExpression, CowExpression, MemoryManager, MemoryMonitor,
    MemoryStats, MemoryConfig, ExpressionComparator
};
pub use expression_builder::{ExpressionBuilder, ExpressionFactory};