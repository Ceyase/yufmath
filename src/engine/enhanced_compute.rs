//! # 增强计算引擎
//!
//! 集成运行时化简增强功能的计算引擎

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{ComputeEngine, ComputeError};
use super::enhanced_simplify::EnhancedSimplifier;
use super::compute::BasicComputeEngine;

/// 增强计算引擎，支持运行时自动化简
pub struct EnhancedComputeEngine {
    /// 基础计算引擎
    base_engine: BasicComputeEngine,
    /// 增强化简器
    enhanced_simplifier: Arc<Mutex<EnhancedSimplifier>>,
    /// 是否启用运行时自动化简（使用 Mutex 支持运行时修改）
    auto_simplify_enabled: Arc<Mutex<bool>>,
}

impl EnhancedComputeEngine {
    /// 创建新的增强计算引擎
    pub fn new() -> Self {
        Self {
            base_engine: BasicComputeEngine::new(),
            enhanced_simplifier: Arc::new(Mutex::new(EnhancedSimplifier::new())),
            auto_simplify_enabled: Arc::new(Mutex::new(true)),
        }
    }
    
    /// 设置是否启用运行时自动化简
    pub fn set_auto_simplify(&self, enabled: bool) {
        if let Ok(mut auto_simplify) = self.auto_simplify_enabled.lock() {
            *auto_simplify = enabled;
        }
        if let Ok(mut simplifier) = self.enhanced_simplifier.lock() {
            simplifier.set_auto_simplify(enabled);
        }
    }
    
    /// 是否启用了自动化简
    pub fn is_auto_simplify_enabled(&self) -> bool {
        self.auto_simplify_enabled.lock().map(|enabled| *enabled).unwrap_or(false)
    }
    
    /// 执行运算后自动化简
    fn auto_simplify_if_enabled(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let enabled = self.auto_simplify_enabled.lock()
            .map_err(|_| ComputeError::internal("无法获取自动化简开关锁"))?;
        
        if *enabled {
            self.enhanced_simplifier.lock()
                .map_err(|_| ComputeError::internal("无法获取增强化简器锁"))?
                .enhanced_simplify(expr)
        } else {
            Ok(expr.clone())
        }
    }
    
    /// 增强的二元运算，运算后自动化简
    pub fn enhanced_binary_op(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 首先创建二元运算表达式
        let result = Expression::binary_op(op.clone(), left.clone(), right.clone());
        
        // 应用自动化简
        self.auto_simplify_if_enabled(&result)
    }
    
    /// 增强的一元运算，运算后自动化简
    pub fn enhanced_unary_op(&self, op: &UnaryOperator, operand: &Expression) -> Result<Expression, ComputeError> {
        // 首先创建一元运算表达式
        let result = Expression::unary_op(op.clone(), operand.clone());
        
        // 应用自动化简
        self.auto_simplify_if_enabled(&result)
    }
    
    /// 增强的函数调用，运算后自动化简
    pub fn enhanced_function_call(&self, name: &str, args: &[Expression]) -> Result<Expression, ComputeError> {
        // 首先创建函数调用表达式
        let result = Expression::function(name, args.to_vec());
        
        // 应用自动化简
        self.auto_simplify_if_enabled(&result)
    }
}

impl ComputeEngine for EnhancedComputeEngine {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn simplify(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 使用增强化简器
        self.enhanced_simplifier.lock()
            .map_err(|_| ComputeError::internal("无法获取增强化简器锁"))?
            .enhanced_simplify(expr)
    }
    
    fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) -> Result<Number, ComputeError> {
        // 先化简表达式，再求值
        let enabled = self.auto_simplify_enabled.lock()
            .map_err(|_| ComputeError::internal("无法获取自动化简开关锁"))?;
        
        let simplified = if *enabled {
            self.simplify(expr)?
        } else {
            expr.clone()
        };
        
        self.base_engine.evaluate(&simplified, vars)
    }
    
    fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 求导后自动化简
        let derivative = self.base_engine.differentiate(expr, var)?;
        self.auto_simplify_if_enabled(&derivative)
    }
    
    fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 积分后自动化简
        let integral = self.base_engine.integrate(expr, var)?;
        self.auto_simplify_if_enabled(&integral)
    }
    
    fn limit(&self, expr: &Expression, var: &str, point: &Expression) -> Result<Expression, ComputeError> {
        // 极限计算后自动化简
        let limit_result = self.base_engine.limit(expr, var, point)?;
        self.auto_simplify_if_enabled(&limit_result)
    }
    
    fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) -> Result<Expression, ComputeError> {
        // 级数展开后自动化简
        let series_result = self.base_engine.series(expr, var, point, order)?;
        self.auto_simplify_if_enabled(&series_result)
    }
    
    fn numerical_evaluate(&self, expr: &Expression, vars: &HashMap<String, f64>) -> Result<f64, ComputeError> {
        // 数值计算前先化简
        let enabled = self.auto_simplify_enabled.lock()
            .map_err(|_| ComputeError::internal("无法获取自动化简开关锁"))?;
        
        let simplified = if *enabled {
            self.simplify(expr)?
        } else {
            expr.clone()
        };
        
        self.base_engine.numerical_evaluate(&simplified, vars)
    }
    
    fn constant_to_number(&self, constant: &MathConstant) -> Result<Number, ComputeError> {
        self.base_engine.constant_to_number(constant)
    }
    
    fn simplify_constants(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 常量简化后自动化简
        let simplified_constants = self.base_engine.simplify_constants(expr)?;
        self.auto_simplify_if_enabled(&simplified_constants)
    }
    
    fn expand(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 展开后自动化简
        let expanded = self.base_engine.expand(expr)?;
        self.auto_simplify_if_enabled(&expanded)
    }
    
    fn factor(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 因式分解后自动化简
        let factored = self.base_engine.factor(expr)?;
        self.auto_simplify_if_enabled(&factored)
    }
    
    fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        // 同类项收集后自动化简
        let collected = self.base_engine.collect(expr, var)?;
        self.auto_simplify_if_enabled(&collected)
    }
    
    fn polynomial_divide(&self, dividend: &Expression, divisor: &Expression) -> Result<(Expression, Expression), ComputeError> {
        // 多项式除法后自动化简
        let (quotient, remainder) = self.base_engine.polynomial_divide(dividend, divisor)?;
        let simplified_quotient = self.auto_simplify_if_enabled(&quotient)?;
        let simplified_remainder = self.auto_simplify_if_enabled(&remainder)?;
        Ok((simplified_quotient, simplified_remainder))
    }
    
    fn polynomial_gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        // 多项式最大公约数后自动化简
        let gcd_result = self.base_engine.polynomial_gcd(a, b)?;
        self.auto_simplify_if_enabled(&gcd_result)
    }
    
    fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let gcd_result = self.base_engine.gcd(a, b)?;
        self.auto_simplify_if_enabled(&gcd_result)
    }
    
    fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let lcm_result = self.base_engine.lcm(a, b)?;
        self.auto_simplify_if_enabled(&lcm_result)
    }
    
    fn is_prime(&self, n: &Expression) -> Result<bool, ComputeError> {
        self.base_engine.is_prime(n)
    }
    
    fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, ComputeError> {
        let factors = self.base_engine.prime_factors(n)?;
        // 对每个因子进行化简
        let simplified_factors: Result<Vec<_>, _> = factors.iter()
            .map(|factor| self.auto_simplify_if_enabled(factor))
            .collect();
        simplified_factors
    }
    
    fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        let binomial_result = self.base_engine.binomial(n, k)?;
        self.auto_simplify_if_enabled(&binomial_result)
    }
    
    fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        let permutation_result = self.base_engine.permutation(n, k)?;
        self.auto_simplify_if_enabled(&permutation_result)
    }
    
    fn mean(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let mean_result = self.base_engine.mean(values)?;
        self.auto_simplify_if_enabled(&mean_result)
    }
    
    fn variance(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let variance_result = self.base_engine.variance(values)?;
        self.auto_simplify_if_enabled(&variance_result)
    }
    
    fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let std_dev_result = self.base_engine.standard_deviation(values)?;
        self.auto_simplify_if_enabled(&std_dev_result)
    }
    
    fn solve(&self, equation: &Expression, var: &str) -> Result<Vec<Expression>, ComputeError> {
        let solutions = self.base_engine.solve(equation, var)?;
        // 对每个解进行化简
        let simplified_solutions: Result<Vec<_>, _> = solutions.iter()
            .map(|solution| self.auto_simplify_if_enabled(solution))
            .collect();
        simplified_solutions
    }
    
    fn solve_system(&self, equations: &[Expression], vars: &[String]) -> Result<Vec<HashMap<String, Expression>>, ComputeError> {
        let solutions = self.base_engine.solve_system(equations, vars)?;
        // 对每个解的每个变量值进行化简
        let mut simplified_solutions = Vec::new();
        for solution in solutions {
            let mut simplified_solution = HashMap::new();
            for (var, expr) in solution {
                let simplified_expr = self.auto_simplify_if_enabled(&expr)?;
                simplified_solution.insert(var, simplified_expr);
            }
            simplified_solutions.push(simplified_solution);
        }
        Ok(simplified_solutions)
    }
    
    fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let matrix_sum = self.base_engine.matrix_add(a, b)?;
        self.auto_simplify_if_enabled(&matrix_sum)
    }
    
    fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let matrix_product = self.base_engine.matrix_multiply(a, b)?;
        self.auto_simplify_if_enabled(&matrix_product)
    }
    
    fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let determinant = self.base_engine.matrix_determinant(matrix)?;
        self.auto_simplify_if_enabled(&determinant)
    }
    
    fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let inverse = self.base_engine.matrix_inverse(matrix)?;
        self.auto_simplify_if_enabled(&inverse)
    }
    
    fn complex_conjugate(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let conjugate = self.base_engine.complex_conjugate(expr)?;
        self.auto_simplify_if_enabled(&conjugate)
    }
    
    fn complex_modulus(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let modulus = self.base_engine.complex_modulus(expr)?;
        self.auto_simplify_if_enabled(&modulus)
    }
    
    fn complex_argument(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let argument = self.base_engine.complex_argument(expr)?;
        self.auto_simplify_if_enabled(&argument)
    }
    
    fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let dot_product = self.base_engine.vector_dot(a, b)?;
        self.auto_simplify_if_enabled(&dot_product)
    }
    
    fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let cross_product = self.base_engine.vector_cross(a, b)?;
        self.auto_simplify_if_enabled(&cross_product)
    }
    
    fn vector_norm(&self, v: &Expression) -> Result<Expression, ComputeError> {
        let norm = self.base_engine.vector_norm(v)?;
        self.auto_simplify_if_enabled(&norm)
    }
    
    fn set_union(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let union = self.base_engine.set_union(a, b)?;
        self.auto_simplify_if_enabled(&union)
    }
    
    fn set_intersection(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let intersection = self.base_engine.set_intersection(a, b)?;
        self.auto_simplify_if_enabled(&intersection)
    }
    
    fn set_difference(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let difference = self.base_engine.set_difference(a, b)?;
        self.auto_simplify_if_enabled(&difference)
    }
}

impl Default for EnhancedComputeEngine {
    fn default() -> Self {
        Self::new()
    }
}