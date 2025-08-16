//! # 计算引擎
//!
//! 本模块实现数学表达式的计算、简化、求导、积分等核心功能。
//! 包含代数运算、微积分、多项式运算等各种数学操作。

pub mod compute;
pub mod cached_compute;
pub mod simplify;
pub mod calculus;
pub mod algebra;
pub mod polynomial;
pub mod number_theory;
pub mod cache;
pub mod lazy;
pub mod parallel;
pub mod error;

use std::collections::HashMap;
use crate::core::{Expression, Number, MathConstant};
pub use error::{ComputeError, ErrorSeverity};
pub use cache::{ComputeCache, CacheManager, CacheStats, CacheUsageInfo};
pub use cached_compute::CachedComputeEngine;
pub use lazy::{LazyExpression, DependencyGraph, LazyState, DependencyGraphStats};
pub use parallel::{ParallelComputeEngine, TaskScheduler, ComputeTask, SchedulerStats, ExpressionPreprocessor, ParallelizationAnalysis};

/// 计算引擎 trait
pub trait ComputeEngine: Send + Sync {
    /// 获取 Any trait 引用，用于动态类型转换
    fn as_any(&self) -> &dyn std::any::Any;
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
    
    /// 计算极限
    fn limit(&self, expr: &Expression, var: &str, point: &Expression) 
        -> Result<Expression, ComputeError>;
    
    /// 级数展开
    fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) 
        -> Result<Expression, ComputeError>;
    
    /// 数值计算
    fn numerical_evaluate(&self, expr: &Expression, vars: &HashMap<String, f64>) 
        -> Result<f64, ComputeError>;
    
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
    
    // 方程求解功能
    
    /// 求解单变量方程
    fn solve(&self, equation: &Expression, var: &str) -> Result<Vec<Expression>, ComputeError>;
    
    /// 求解多变量方程组
    fn solve_system(&self, equations: &[Expression], vars: &[String]) 
        -> Result<Vec<HashMap<String, Expression>>, ComputeError>;
    
    // 矩阵运算功能
    
    /// 矩阵加法
    fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 矩阵乘法
    fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算矩阵行列式
    fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, ComputeError>;
    
    /// 计算矩阵逆
    fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, ComputeError>;
    
    // 复数运算功能
    
    /// 复数共轭
    fn complex_conjugate(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    /// 复数模长
    fn complex_modulus(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    /// 复数幅角
    fn complex_argument(&self, expr: &Expression) -> Result<Expression, ComputeError>;
    
    // 向量运算功能
    
    /// 向量点积
    fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 向量叉积
    fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 向量范数
    fn vector_norm(&self, v: &Expression) -> Result<Expression, ComputeError>;
    
    // 集合运算功能
    
    /// 集合并集
    fn set_union(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 集合交集
    fn set_intersection(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
    
    /// 集合差集
    fn set_difference(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError>;
}