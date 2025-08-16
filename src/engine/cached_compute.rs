//! # 带缓存的计算引擎
//!
//! 实现集成多层缓存系统的高性能计算引擎。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use crate::api::CacheConfig;
use super::{ComputeEngine, ComputeError};
use super::compute::BasicComputeEngine;
use super::cache::{ComputeCache, CacheManager, FastCacheKey, ExactCacheKey, SymbolicCacheKey};
use num_bigint::BigInt;

/// 带缓存的计算引擎
pub struct CachedComputeEngine {
    /// 基础计算引擎
    base_engine: BasicComputeEngine,
    /// 缓存管理器
    cache_manager: Arc<Mutex<CacheManager>>,
}

impl CachedComputeEngine {
    /// 创建新的带缓存计算引擎
    pub fn new(cache_config: CacheConfig) -> Self {
        Self {
            base_engine: BasicComputeEngine::new(),
            cache_manager: Arc::new(Mutex::new(CacheManager::new(cache_config))),
        }
    }
    
    /// 获取缓存统计信息
    pub fn get_cache_stats(&self) -> Result<super::cache::CacheStats, ComputeError> {
        self.cache_manager.lock()
            .map_err(|_| ComputeError::internal("无法获取缓存管理器锁"))?
            .cache()
            .get_stats()
            .pipe(Ok)
    }
    
    /// 获取缓存使用情况
    pub fn get_cache_usage(&self) -> Result<super::cache::CacheUsageInfo, ComputeError> {
        self.cache_manager.lock()
            .map_err(|_| ComputeError::internal("无法获取缓存管理器锁"))?
            .cache()
            .get_usage_info()
            .pipe(Ok)
    }
    
    /// 清理缓存
    pub fn cleanup_cache(&self) -> Result<(), ComputeError> {
        self.cache_manager.lock()
            .map_err(|_| ComputeError::internal("无法获取缓存管理器锁"))?
            .force_cleanup();
        Ok(())
    }
    
    /// 清空所有缓存
    pub fn clear_cache(&self) -> Result<(), ComputeError> {
        self.cache_manager.lock()
            .map_err(|_| ComputeError::internal("无法获取缓存管理器锁"))?
            .cache()
            .clear_all();
        Ok(())
    }
    
    /// 尝试从快速缓存获取二元运算结果
    fn try_fast_binary_op(&self, left: &Number, right: &Number, op: &BinaryOperator) -> Option<Number> {
        // 只对小整数使用快速缓存
        if let (Number::Integer(l), Number::Integer(r)) = (left, right) {
            if let (Ok(l_i64), Ok(r_i64)) = (l.try_into(), r.try_into()) {
                let l_i64: i64 = l_i64;
                let r_i64: i64 = r_i64;
                
                // 检查范围，避免溢出
                if l_i64.abs() < 1_000_000 && r_i64.abs() < 1_000_000 {
                    let key = FastCacheKey::BinaryOp(l_i64, r_i64, op.clone());
                    
                    if let Ok(cache_manager) = self.cache_manager.lock() {
                        if let Some(result) = cache_manager.cache().get_fast(&key) {
                            return Some(Number::Integer(BigInt::from(result)));
                        }
                    }
                }
            }
        }
        None
    }
    
    /// 将二元运算结果存入快速缓存
    fn cache_fast_binary_op(&self, left: &Number, right: &Number, op: &BinaryOperator, result: &Number) {
        // 只对小整数使用快速缓存
        if let (Number::Integer(l), Number::Integer(r), Number::Integer(res)) = (left, right, result) {
            if let (Ok(l_i64), Ok(r_i64), Ok(res_i64)) = (l.try_into(), r.try_into(), res.try_into()) {
                let l_i64: i64 = l_i64;
                let r_i64: i64 = r_i64;
                let res_i64: i64 = res_i64;
                
                // 检查范围
                if l_i64.abs() < 1_000_000 && r_i64.abs() < 1_000_000 && res_i64.abs() < 10_000_000 {
                    let key = FastCacheKey::BinaryOp(l_i64, r_i64, op.clone());
                    
                    if let Ok(cache_manager) = self.cache_manager.lock() {
                        // 计算成本基于操作类型
                        let cost = match op {
                            BinaryOperator::Add | BinaryOperator::Subtract => 1,
                            BinaryOperator::Multiply => 2,
                            BinaryOperator::Divide => 5,
                            BinaryOperator::Power => 10,
                            _ => 3,
                        };
                        cache_manager.cache().put_fast(key, res_i64, cost);
                    }
                }
            }
        }
    }
    
    /// 尝试从精确缓存获取运算结果
    fn try_exact_cache(&self, operand1: &Number, operand2: Option<&Number>, operation: &str) -> Option<Number> {
        let key = ExactCacheKey {
            operand1: operand1.clone(),
            operand2: operand2.cloned(),
            operation: operation.to_string(),
        };
        
        if let Ok(cache_manager) = self.cache_manager.lock() {
            cache_manager.cache().get_exact(&key)
        } else {
            None
        }
    }
    
    /// 将运算结果存入精确缓存
    fn cache_exact_result(&self, operand1: &Number, operand2: Option<&Number>, operation: &str, result: &Number, cost: u32) {
        let key = ExactCacheKey {
            operand1: operand1.clone(),
            operand2: operand2.cloned(),
            operation: operation.to_string(),
        };
        
        if let Ok(cache_manager) = self.cache_manager.lock() {
            cache_manager.cache().put_exact(key, result.clone(), cost);
        }
    }
    
    /// 尝试从符号缓存获取结果
    fn try_symbolic_cache(&self, expr: &Expression, operation: &str, variable: Option<&str>) -> Option<Expression> {
        let key = SymbolicCacheKey {
            expression: expr.clone(),
            operation: operation.to_string(),
            variable: variable.map(|s| s.to_string()),
        };
        
        if let Ok(cache_manager) = self.cache_manager.lock() {
            cache_manager.cache().get_symbolic(&key)
        } else {
            None
        }
    }
    
    /// 将符号运算结果存入缓存
    fn cache_symbolic_result(&self, expr: &Expression, operation: &str, variable: Option<&str>, result: &Expression, cost: u32) {
        let key = SymbolicCacheKey {
            expression: expr.clone(),
            operation: operation.to_string(),
            variable: variable.map(|s| s.to_string()),
        };
        
        if let Ok(cache_manager) = self.cache_manager.lock() {
            cache_manager.cache().put_symbolic(key, result.clone(), cost);
        }
    }
    
    /// 执行定期缓存清理
    fn periodic_cleanup(&self) {
        if let Ok(mut cache_manager) = self.cache_manager.lock() {
            cache_manager.periodic_cleanup();
        }
    }
    
    /// 计算表达式复杂度（用于确定缓存成本）
    fn compute_complexity(&self, expr: &Expression) -> u32 {
        match expr {
            Expression::Number(_) | Expression::Variable(_) | Expression::Constant(_) => 1,
            Expression::UnaryOp { operand, .. } => 1 + self.compute_complexity(operand),
            Expression::BinaryOp { left, right, .. } => 1 + self.compute_complexity(left) + self.compute_complexity(right),
            Expression::Function { args, .. } => 5 + args.iter().map(|arg| self.compute_complexity(arg)).sum::<u32>(),
            Expression::Matrix(rows) => {
                10 + rows.iter().flat_map(|row| row.iter()).map(|elem| self.compute_complexity(elem)).sum::<u32>()
            }
            Expression::Vector(elements) => {
                5 + elements.iter().map(|elem| self.compute_complexity(elem)).sum::<u32>()
            }
            Expression::Set(elements) => {
                3 + elements.iter().map(|elem| self.compute_complexity(elem)).sum::<u32>()
            }
            Expression::Interval { start, end, .. } => {
                2 + self.compute_complexity(start) + self.compute_complexity(end)
            }
        }
    }
}

impl ComputeEngine for CachedComputeEngine {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn simplify(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "simplify", None) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.simplify(expr)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "simplify", None, &result, complexity);
        
        Ok(result)
    }
    
    fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) -> Result<Number, ComputeError> {
        // 对于简单的二元运算，尝试快速缓存
        if let Expression::BinaryOp { op, left, right } = expr {
            if let (Expression::Number(l), Expression::Number(r)) = (left.as_ref(), right.as_ref()) {
                if let Some(cached_result) = self.try_fast_binary_op(l, r, op) {
                    return Ok(cached_result);
                }
                
                // 尝试精确缓存
                let operation = format!("evaluate_{:?}", op);
                if let Some(cached_result) = self.try_exact_cache(l, Some(r), &operation) {
                    return Ok(cached_result);
                }
                
                // 执行实际计算
                let result = self.base_engine.evaluate(expr, vars)?;
                
                // 缓存结果
                self.cache_fast_binary_op(l, r, op, &result);
                self.cache_exact_result(l, Some(r), &operation, &result, 5);
                
                return Ok(result);
            }
        }
        
        // 对于其他表达式，直接计算
        self.base_engine.evaluate(expr, vars)
    }
    
    fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "differentiate", Some(var)) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.differentiate(expr, var)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "differentiate", Some(var), &result, complexity * 2);
        
        Ok(result)
    }
    
    fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "integrate", Some(var)) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.integrate(expr, var)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "integrate", Some(var), &result, complexity * 5);
        
        Ok(result)
    }
    
    fn expand(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "expand", None) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.expand(expr)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "expand", None, &result, complexity * 3);
        
        Ok(result)
    }
    
    fn factor(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "factor", None) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.factor(expr)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "factor", None, &result, complexity * 4);
        
        Ok(result)
    }
    
    fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 执行定期清理
        self.periodic_cleanup();
        
        // 尝试从符号缓存获取结果
        if let Some(cached_result) = self.try_symbolic_cache(expr, "collect", Some(var)) {
            return Ok(cached_result);
        }
        
        // 执行实际计算
        let result = self.base_engine.collect(expr, var)?;
        
        // 缓存结果
        let complexity = self.compute_complexity(expr);
        self.cache_symbolic_result(expr, "collect", Some(var), &result, complexity * 2);
        
        Ok(result)
    }
    
    // 对于其他方法，直接委托给基础引擎（可以根据需要添加缓存）
    
    fn limit(&self, expr: &Expression, var: &str, point: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.limit(expr, var, point)
    }
    
    fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) -> Result<Expression, ComputeError> {
        self.base_engine.series(expr, var, point, order)
    }
    
    fn numerical_evaluate(&self, expr: &Expression, vars: &HashMap<String, f64>) -> Result<f64, ComputeError> {
        self.base_engine.numerical_evaluate(expr, vars)
    }
    
    fn constant_to_number(&self, constant: &MathConstant) -> Result<Number, ComputeError> {
        self.base_engine.constant_to_number(constant)
    }
    
    fn simplify_constants(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.simplify_constants(expr)
    }
    
    fn polynomial_divide(&self, dividend: &Expression, divisor: &Expression) -> Result<(Expression, Expression), ComputeError> {
        self.base_engine.polynomial_divide(dividend, divisor)
    }
    
    fn polynomial_gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.polynomial_gcd(a, b)
    }
    
    fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.gcd(a, b)
    }
    
    fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.lcm(a, b)
    }
    
    fn is_prime(&self, n: &Expression) -> Result<bool, ComputeError> {
        self.base_engine.is_prime(n)
    }
    
    fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, ComputeError> {
        self.base_engine.prime_factors(n)
    }
    
    fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.binomial(n, k)
    }
    
    fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.permutation(n, k)
    }
    
    fn mean(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        self.base_engine.mean(values)
    }
    
    fn variance(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        self.base_engine.variance(values)
    }
    
    fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        self.base_engine.standard_deviation(values)
    }
    
    fn solve(&self, equation: &Expression, var: &str) -> Result<Vec<Expression>, ComputeError> {
        self.base_engine.solve(equation, var)
    }
    
    fn solve_system(&self, equations: &[Expression], vars: &[String]) -> Result<Vec<HashMap<String, Expression>>, ComputeError> {
        self.base_engine.solve_system(equations, vars)
    }
    
    fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.matrix_add(a, b)
    }
    
    fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.matrix_multiply(a, b)
    }
    
    fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.matrix_determinant(matrix)
    }
    
    fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.matrix_inverse(matrix)
    }
    
    fn complex_conjugate(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.complex_conjugate(expr)
    }
    
    fn complex_modulus(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.complex_modulus(expr)
    }
    
    fn complex_argument(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.complex_argument(expr)
    }
    
    fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.vector_dot(a, b)
    }
    
    fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.vector_cross(a, b)
    }
    
    fn vector_norm(&self, v: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.vector_norm(v)
    }
    
    fn set_union(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.set_union(a, b)
    }
    
    fn set_intersection(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.set_intersection(a, b)
    }
    
    fn set_difference(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        self.base_engine.set_difference(a, b)
    }
}

// 辅助 trait 用于管道操作
trait Pipe<T> {
    fn pipe<U, F>(self, f: F) -> U
    where
        F: FnOnce(T) -> U;
}

impl<T> Pipe<T> for T {
    fn pipe<U, F>(self, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Expression, Number, BinaryOperator};
    use crate::api::CacheConfig;
    use num_bigint::BigInt;
    
    #[test]
    fn test_cached_engine_creation() {
        let config = CacheConfig::default();
        let engine = CachedComputeEngine::new(config);
        
        // 测试缓存统计获取
        let stats = engine.get_cache_stats().unwrap();
        assert_eq!(stats.total_hit_rate(), 0.0);
        
        // 测试缓存使用情况获取
        let usage = engine.get_cache_usage().unwrap();
        assert_eq!(usage.total_usage_rate(), 0.0);
    }
    
    #[test]
    fn test_fast_cache_integration() {
        let config = CacheConfig::default();
        let engine = CachedComputeEngine::new(config);
        
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
        };
        
        // 第一次计算应该缓存结果
        let vars = HashMap::new();
        let result1 = engine.evaluate(&expr, &vars);
        
        // 第二次计算应该从缓存获取
        let result2 = engine.evaluate(&expr, &vars);
        
        // 结果应该相同
        assert_eq!(result1.is_ok(), result2.is_ok());
        
        // 检查缓存统计
        let stats = engine.get_cache_stats().unwrap();
        assert!(stats.fast_hits > 0 || stats.exact_hits > 0);
    }
    
    #[test]
    fn test_symbolic_cache_integration() {
        let config = CacheConfig::default();
        let engine = CachedComputeEngine::new(config);
        
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Variable("x".to_string())),
        };
        
        // 第一次简化应该缓存结果
        let result1 = engine.simplify(&expr);
        
        // 第二次简化应该从缓存获取
        let result2 = engine.simplify(&expr);
        
        // 结果应该相同
        assert_eq!(result1.is_ok(), result2.is_ok());
        
        // 检查缓存统计
        let stats = engine.get_cache_stats().unwrap();
        assert!(stats.symbolic_hits > 0);
    }
    
    #[test]
    fn test_cache_cleanup() {
        let config = CacheConfig::default();
        let engine = CachedComputeEngine::new(config);
        
        // 执行一些操作以填充缓存
        let expr = Expression::Variable("x".to_string());
        let _ = engine.simplify(&expr);
        
        // 测试缓存清理
        assert!(engine.cleanup_cache().is_ok());
        
        // 测试缓存清空
        assert!(engine.clear_cache().is_ok());
        
        let usage = engine.get_cache_usage().unwrap();
        assert_eq!(usage.total_usage_rate(), 0.0);
    }
    
    #[test]
    fn test_complexity_calculation() {
        let config = CacheConfig::default();
        let engine = CachedComputeEngine::new(config);
        
        // 简单表达式
        let simple = Expression::Variable("x".to_string());
        assert_eq!(engine.compute_complexity(&simple), 1);
        
        // 复杂表达式
        let complex = Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(Expression::Variable("y".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            }),
        };
        assert!(engine.compute_complexity(&complex) > 1);
    }
}