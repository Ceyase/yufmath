//! # 计算引擎
//!
//! 本模块实现数学表达式的计算、简化、求导、积分等核心功能。
//! 包含代数运算、微积分、多项式运算等各种数学操作。

pub mod compute;
pub mod simplify;
pub mod calculus;
pub mod algebra;
pub mod polynomial;
pub mod number_theory;
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
    
    /// 多项式展开
    fn expand(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    /// 因式分解
    fn factor(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    /// 同类项收集
    fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError>;
    
    /// 多项式除法
    fn polynomial_divide(&self, dividend: &Expression, divisor: &Expression) 
        -> Result<(Expression, Expression), ComputeError>;
    
    /// 多项式最大公约数
    fn polynomial_gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    // 数论和组合数学功能
    
    /// 计算最大公约数
    fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算最小公倍数
    fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 判断是否为素数
    fn is_prime(&self, n: &Expression) -> Result<bool, ComputeError>;
    
    /// 质因数分解
    fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, ComputeError>;
    
    /// 计算二项式系数
    fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算排列数
    fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算平均值
    fn mean(&self, values: &[Expression]) -> Result<Expression, ComputeError>;
    
    /// 计算方差
    fn variance(&self, values: &[Expression]) -> Result<Expression, ComputeError>;
    
    /// 计算标准差
    fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, ComputeError>;
}