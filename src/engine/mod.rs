//! # 计算引擎
//!
//! 本模块实现数学表达式的计算、简化、求导、积分等核心功能。
//! 包含代数运算、微积分、多项式运算等各种数学操作。

pub mod compute;
pub mod simplify;
pub mod calculus;
pub mod algebra;
pub mod error;

use std::collections::HashMap;
use crate::core::{Expression, Number, MathConstant};
pub use error::{ComputeError, ErrorSeverity};

/// 计算引擎 trait
pub trait ComputeEngine {
    /// 简化表达式
    fn simplify(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算表达式的值
    fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) 
        -> Result<Number, ComputeError>;
    
    /// 对表达式求导
    fn differentiate(&self, expr: &Expression, var: &str) 
        -> Result<Expression, ComputeError>;
    
    /// 对表达式积分
    fn integrate(&self, expr: &Expression, var: &str) 
        -> Result<Expression, ComputeError>;
    
    /// 将数学常量转换为数值
    fn constant_to_number(&self, constant: &MathConstant) -> Result<Number, ComputeError>;
    
    /// 简化包含数学常量的表达式
    fn simplify_constants(&self, expr: &Expression) -> Result<Expression, ComputeError>;
}