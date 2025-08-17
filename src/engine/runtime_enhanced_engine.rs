//! # 运行时增强计算引擎
//!
//! 集成运行时增强功能的计算引擎，支持变量管理和复杂度控制

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::{Expression, Number, MathConstant};
use super::{ComputeEngine, ComputeError, EnhancedComputeEngine};
use super::runtime_enhancement::{RuntimeEnhancer, RuntimeConfig};

/// 运行时增强计算引擎
pub struct RuntimeEnhancedEngine {
    /// 基础增强计算引擎
    base_engine: EnhancedComputeEngine,
    /// 运行时增强器
    runtime_enhancer: Arc<Mutex<RuntimeEnhancer>>,
}

impl RuntimeEnhancedEngine {
    /// 创建新的运行时增强计算引擎
    pub fn new() -> Self {
        Self {
            base_engine: EnhancedComputeEngine::new(),
            runtime_enhancer: Arc::new(Mutex::new(RuntimeEnhancer::new(RuntimeConfig::default()))),
        }
    }
    
    /// 创建带配置的运行时增强计算引擎
    pub fn with_config(config: RuntimeConfig) -> Self {
        Self {
            base_engine: EnhancedComputeEngine::new(),
            runtime_enhancer: Arc::new(Mutex::new(RuntimeEnhancer::new(config))),
        }
    }
    
    /// 设置变量值
    pub fn set_variable(&self, name: String, value: Expression) -> Result<(), ComputeError> {
        let mut enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        // 检查循环引用
        if enhancer.variable_manager().has_circular_reference(&name) {
            return Err(ComputeError::domain_error(
                format!("变量 '{}' 存在循环引用", name)
            ));
        }
        
        enhancer.variable_manager_mut().set_variable(name, value)
    }
    
    /// 获取变量值
    pub fn get_variable(&self, name: &str) -> Result<Option<Expression>, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        Ok(enhancer.variable_manager().get_variable(name).cloned())
    }
    
    /// 获取所有变量
    pub fn get_all_variables(&self) -> Result<HashMap<String, Expression>, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        Ok(enhancer.variable_manager().get_all_variables().clone())
    }
    
    /// 清空所有变量
    pub fn clear_variables(&self) -> Result<(), ComputeError> {
        let mut enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        enhancer.variable_manager_mut().clear();
        Ok(())
    }
    
    /// 删除指定变量
    pub fn remove_variable(&self, name: &str) -> Result<bool, ComputeError> {
        let mut enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        Ok(enhancer.variable_manager_mut().remove_variable(name))
    }
    
    /// 更新运行时配置
    pub fn update_runtime_config(&self, config: RuntimeConfig) -> Result<(), ComputeError> {
        let mut enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        enhancer.update_config(config);
        Ok(())
    }
    
    /// 获取运行时配置
    pub fn get_runtime_config(&self) -> Result<RuntimeConfig, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        Ok(enhancer.get_config().clone())
    }
    
    /// 安全计算表达式（带运行时增强）
    pub fn safe_compute(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        enhancer.safe_compute(expr, &self.base_engine)
    }
    
    /// 计算表达式并自动替换变量
    pub fn compute_with_variables(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        // 替换变量
        let substituted = enhancer.variable_manager().substitute_variables(expr);
        
        // 安全计算
        drop(enhancer); // 释放锁
        self.safe_compute(&substituted)
    }
    
    /// 获取数值变量（用于快速数值计算）
    pub fn get_numeric_variables(&self) -> Result<HashMap<String, Number>, ComputeError> {
        let enhancer = self.runtime_enhancer.lock()
            .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
        
        Ok(enhancer.variable_manager().get_all_numeric_variables().clone())
    }
}

impl ComputeEngine for RuntimeEnhancedEngine {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn simplify(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 使用安全计算
        self.safe_compute(expr)
    }
    
    fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) -> Result<Number, ComputeError> {
        // 首先替换内部变量，然后使用传入的变量进行求值
        let substituted = {
            let enhancer = self.runtime_enhancer.lock()
                .map_err(|_| ComputeError::internal("无法获取运行时增强器锁"))?;
            enhancer.variable_manager().substitute_variables(expr)
        };
        
        self.base_engine.evaluate(&substituted, vars)
    }
    
    fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.differentiate(&substituted, var)
    }
    
    fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.integrate(&substituted, var)
    }
    
    fn limit(&self, expr: &Expression, var: &str, point: &Expression) -> Result<Expression, ComputeError> {
        let substituted_expr = self.compute_with_variables(expr)?;
        let substituted_point = self.compute_with_variables(point)?;
        self.base_engine.limit(&substituted_expr, var, &substituted_point)
    }
    
    fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) -> Result<Expression, ComputeError> {
        let substituted_expr = self.compute_with_variables(expr)?;
        let substituted_point = self.compute_with_variables(point)?;
        self.base_engine.series(&substituted_expr, var, &substituted_point, order)
    }
    
    fn numerical_evaluate(&self, expr: &Expression, vars: &HashMap<String, f64>) -> Result<f64, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.numerical_evaluate(&substituted, vars)
    }
    
    fn constant_to_number(&self, constant: &MathConstant) -> Result<Number, ComputeError> {
        self.base_engine.constant_to_number(constant)
    }
    
    fn simplify_constants(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.simplify_constants(&substituted)
    }
    
    fn expand(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.expand(&substituted)
    }
    
    fn factor(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.factor(&substituted)
    }
    
    fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.collect(&substituted, var)
    }
    
    fn polynomial_divide(&self, dividend: &Expression, divisor: &Expression) -> Result<(Expression, Expression), ComputeError> {
        let substituted_dividend = self.compute_with_variables(dividend)?;
        let substituted_divisor = self.compute_with_variables(divisor)?;
        self.base_engine.polynomial_divide(&substituted_dividend, &substituted_divisor)
    }
    
    fn polynomial_gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.polynomial_gcd(&substituted_a, &substituted_b)
    }
    
    fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.gcd(&substituted_a, &substituted_b)
    }
    
    fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.lcm(&substituted_a, &substituted_b)
    }
    
    fn is_prime(&self, n: &Expression) -> Result<bool, ComputeError> {
        let substituted = self.compute_with_variables(n)?;
        self.base_engine.is_prime(&substituted)
    }
    
    fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, ComputeError> {
        let substituted = self.compute_with_variables(n)?;
        self.base_engine.prime_factors(&substituted)
    }
    
    fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        let substituted_n = self.compute_with_variables(n)?;
        let substituted_k = self.compute_with_variables(k)?;
        self.base_engine.binomial(&substituted_n, &substituted_k)
    }
    
    fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        let substituted_n = self.compute_with_variables(n)?;
        let substituted_k = self.compute_with_variables(k)?;
        self.base_engine.permutation(&substituted_n, &substituted_k)
    }
    
    fn mean(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let substituted_values: Result<Vec<_>, _> = values.iter()
            .map(|v| self.compute_with_variables(v))
            .collect();
        self.base_engine.mean(&substituted_values?)
    }
    
    fn variance(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let substituted_values: Result<Vec<_>, _> = values.iter()
            .map(|v| self.compute_with_variables(v))
            .collect();
        self.base_engine.variance(&substituted_values?)
    }
    
    fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let substituted_values: Result<Vec<_>, _> = values.iter()
            .map(|v| self.compute_with_variables(v))
            .collect();
        self.base_engine.standard_deviation(&substituted_values?)
    }
    
    fn solve(&self, equation: &Expression, var: &str) -> Result<Vec<Expression>, ComputeError> {
        let substituted = self.compute_with_variables(equation)?;
        self.base_engine.solve(&substituted, var)
    }
    
    fn solve_system(&self, equations: &[Expression], vars: &[String]) -> Result<Vec<HashMap<String, Expression>>, ComputeError> {
        let substituted_equations: Result<Vec<_>, _> = equations.iter()
            .map(|eq| self.compute_with_variables(eq))
            .collect();
        self.base_engine.solve_system(&substituted_equations?, vars)
    }
    
    fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.matrix_add(&substituted_a, &substituted_b)
    }
    
    fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.matrix_multiply(&substituted_a, &substituted_b)
    }
    
    fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(matrix)?;
        self.base_engine.matrix_determinant(&substituted)
    }
    
    fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(matrix)?;
        self.base_engine.matrix_inverse(&substituted)
    }
    
    fn complex_conjugate(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.complex_conjugate(&substituted)
    }
    
    fn complex_modulus(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.complex_modulus(&substituted)
    }
    
    fn complex_argument(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(expr)?;
        self.base_engine.complex_argument(&substituted)
    }
    
    fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.vector_dot(&substituted_a, &substituted_b)
    }
    
    fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.vector_cross(&substituted_a, &substituted_b)
    }
    
    fn vector_norm(&self, v: &Expression) -> Result<Expression, ComputeError> {
        let substituted = self.compute_with_variables(v)?;
        self.base_engine.vector_norm(&substituted)
    }
    
    fn set_union(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.set_union(&substituted_a, &substituted_b)
    }
    
    fn set_intersection(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.set_intersection(&substituted_a, &substituted_b)
    }
    
    fn set_difference(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let substituted_a = self.compute_with_variables(a)?;
        let substituted_b = self.compute_with_variables(b)?;
        self.base_engine.set_difference(&substituted_a, &substituted_b)
    }
}

impl Default for RuntimeEnhancedEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Expression, Number, BinaryOperator};
    
    #[test]
    fn test_variable_management() {
        let engine = RuntimeEnhancedEngine::new();
        
        // 设置变量
        let x_value = Expression::number(Number::from(10));
        engine.set_variable("x".to_string(), x_value.clone()).unwrap();
        
        // 获取变量
        let retrieved = engine.get_variable("x").unwrap();
        assert_eq!(retrieved, Some(x_value));
        
        // 使用变量计算
        let expr = Expression::binary_op(
            BinaryOperator::Add,
            Expression::variable("x"),
            Expression::number(Number::from(5))
        );
        
        let result = engine.compute_with_variables(&expr).unwrap();
        // 结果应该是 15（如果能够计算的话）或者是替换了变量的表达式
        println!("计算结果: {:?}", result);
    }
    
    #[test]
    fn test_safe_power_computation() {
        let engine = RuntimeEnhancedEngine::new();
        
        // 测试安全的指数运算
        let safe_expr = Expression::binary_op(
            BinaryOperator::Power,
            Expression::number(Number::from(2)),
            Expression::number(Number::from(10))
        );
        
        let result = engine.safe_compute(&safe_expr).unwrap();
        println!("安全指数运算结果: {:?}", result);
        
        // 测试不安全的指数运算（应该保持符号形式）
        let unsafe_expr = Expression::binary_op(
            BinaryOperator::Power,
            Expression::number(Number::from(10)),
            Expression::number(Number::from(10000))
        );
        
        let result = engine.safe_compute(&unsafe_expr).unwrap();
        println!("不安全指数运算结果: {:?}", result);
        
        // 结果应该保持原始的符号形式，而不是计算出具体数值
        assert!(matches!(result, Expression::BinaryOp { .. }));
    }
    
    #[test]
    fn test_circular_reference_prevention() {
        let engine = RuntimeEnhancedEngine::new();
        
        // 设置第一个变量
        engine.set_variable("x".to_string(), Expression::variable("y")).unwrap();
        
        // 尝试设置循环引用（应该失败）
        let result = engine.set_variable("y".to_string(), Expression::variable("x"));
        assert!(result.is_err());
    }
}