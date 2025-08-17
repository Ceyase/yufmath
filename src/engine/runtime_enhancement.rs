//! # 运行时增强模块
//!
//! 提供运行时鲁棒性增强和变量管理功能

use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::core::{Expression, Number, BinaryOperator, UnaryOperator};
use super::{ComputeError, ComputeEngine};
use num_bigint::BigInt;
use num_traits::Signed;

/// 运行时增强配置
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// 最大计算复杂度阈值
    pub max_complexity_threshold: usize,
    /// 最大计算时间限制（毫秒）
    pub max_compute_time_ms: u64,
    /// 最大指数值（防止指数爆炸）
    pub max_exponent: i64,
    /// 最大整数位数
    pub max_integer_digits: usize,
    /// 是否启用复杂度检查
    pub enable_complexity_check: bool,
    /// 是否启用时间限制
    pub enable_time_limit: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_complexity_threshold: 10000,
            max_compute_time_ms: 5000, // 5秒
            max_exponent: 10000,
            max_integer_digits: 1000,
            enable_complexity_check: true,
            enable_time_limit: true,
        }
    }
}

/// 表达式复杂度分析器
pub struct ComplexityAnalyzer {
    config: RuntimeConfig,
}

impl ComplexityAnalyzer {
    /// 创建新的复杂度分析器
    pub fn new(config: RuntimeConfig) -> Self {
        Self { config }
    }
    
    /// 计算表达式的复杂度
    pub fn calculate_complexity(&self, expr: &Expression) -> usize {
        match expr {
            Expression::Number(num) => self.number_complexity(num),
            Expression::Variable(_) => 1,
            Expression::Constant(_) => 1,
            Expression::BinaryOp { op, left, right } => {
                let left_complexity = self.calculate_complexity(left);
                let right_complexity = self.calculate_complexity(right);
                
                match op {
                    BinaryOperator::Power => {
                        // 指数运算的复杂度特别高
                        let base_complexity = left_complexity;
                        let exp_complexity = right_complexity;
                        
                        // 检查指数是否过大
                        if let Expression::Number(Number::Integer(exp_int)) = right.as_ref() {
                            if exp_int.bits() > 64 || exp_int > &BigInt::from(self.config.max_exponent) {
                                return usize::MAX; // 标记为无限复杂度
                            }
                        }
                        
                        base_complexity * exp_complexity * 10 + 100
                    }
                    BinaryOperator::Multiply | BinaryOperator::Divide => {
                        left_complexity + right_complexity + 5
                    }
                    _ => left_complexity + right_complexity + 1
                }
            }
            Expression::UnaryOp { op, operand } => {
                let operand_complexity = self.calculate_complexity(operand);
                match op {
                    UnaryOperator::Factorial => operand_complexity * 20 + 50,
                    UnaryOperator::Exp => operand_complexity * 10 + 20,
                    _ => operand_complexity + 2
                }
            }
            Expression::Function { args, .. } => {
                args.iter().map(|arg| self.calculate_complexity(arg)).sum::<usize>() + 10
            }
            Expression::Matrix(rows) => {
                rows.iter()
                    .flat_map(|row| row.iter())
                    .map(|elem| self.calculate_complexity(elem))
                    .sum::<usize>() + rows.len() * rows.get(0).map_or(0, |r| r.len()) * 2
            }
            Expression::Vector(elements) => {
                elements.iter().map(|elem| self.calculate_complexity(elem)).sum::<usize>() + elements.len()
            }
            Expression::Set(elements) => {
                elements.iter().map(|elem| self.calculate_complexity(elem)).sum::<usize>() + elements.len()
            }
            Expression::Interval { start, end, .. } => {
                self.calculate_complexity(start) + self.calculate_complexity(end) + 2
            }
        }
    }
    
    /// 计算数值的复杂度
    fn number_complexity(&self, num: &Number) -> usize {
        match num {
            Number::Integer(int) => {
                let digits = int.to_string().len();
                if digits > self.config.max_integer_digits {
                    usize::MAX // 标记为无限复杂度
                } else {
                    (digits / 10).max(1)
                }
            }
            Number::Rational(rat) => {
                let numer_digits = rat.numer().to_string().len();
                let denom_digits = rat.denom().to_string().len();
                let total_digits = numer_digits + denom_digits;
                if total_digits > self.config.max_integer_digits {
                    usize::MAX
                } else {
                    (total_digits / 10).max(1)
                }
            }
            Number::Real(_) => 5,
            Number::Complex { real, imaginary } => {
                self.number_complexity(real) + self.number_complexity(imaginary)
            }
            Number::Symbolic(_) => 10,
            Number::Float(_) => 1,
            Number::Constant(_) => 2, // 数学常量的复杂度较低
        }
    }
    
    /// 检查表达式是否过于复杂
    pub fn is_too_complex(&self, expr: &Expression) -> bool {
        if !self.config.enable_complexity_check {
            return false;
        }
        
        let complexity = self.calculate_complexity(expr);
        complexity == usize::MAX || complexity > self.config.max_complexity_threshold
    }
    
    /// 检查指数运算是否安全
    pub fn is_safe_power(&self, base: &Expression, exponent: &Expression) -> bool {
        // 检查指数是否为合理的整数
        if let Expression::Number(Number::Integer(exp_int)) = exponent {
            if exp_int > &BigInt::from(self.config.max_exponent) {
                return false;
            }
            
            // 检查底数和指数的组合是否会导致结果过大
            if let Expression::Number(Number::Integer(base_int)) = base {
                // 更严格的检查：
                // 1. 如果底数绝对值大于等于10且指数大于1000，则不安全
                // 2. 如果底数绝对值大于等于2且指数大于10000，则不安全
                // 3. 如果指数大于100000，则直接不安全
                let base_abs = base_int.abs();
                if exp_int > &BigInt::from(100000) {
                    return false;
                }
                if base_abs >= BigInt::from(10) && exp_int > &BigInt::from(1000) {
                    return false;
                }
                if base_abs >= BigInt::from(2) && exp_int > &BigInt::from(10000) {
                    return false;
                }
            }
        }
        
        true
    }
}

/// 变量管理器
#[derive(Debug, Clone)]
pub struct VariableManager {
    /// 存储变量值
    variables: HashMap<String, Expression>,
    /// 存储数值变量（用于快速数值计算）
    numeric_variables: HashMap<String, Number>,
}

impl VariableManager {
    /// 创建新的变量管理器
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            numeric_variables: HashMap::new(),
        }
    }
    
    /// 设置变量值
    pub fn set_variable(&mut self, name: String, value: Expression) -> Result<(), ComputeError> {
        // 验证变量名
        if !self.is_valid_variable_name(&name) {
            return Err(ComputeError::domain_error(
                format!("无效的变量名: {}", name)
            ));
        }
        
        // 尝试将表达式转换为数值
        if let Expression::Number(num) = &value {
            self.numeric_variables.insert(name.clone(), num.clone());
        } else {
            // 移除数值变量中的对应项
            self.numeric_variables.remove(&name);
        }
        
        self.variables.insert(name, value);
        Ok(())
    }
    
    /// 获取变量值
    pub fn get_variable(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }
    
    /// 获取数值变量
    pub fn get_numeric_variable(&self, name: &str) -> Option<&Number> {
        self.numeric_variables.get(name)
    }
    
    /// 获取所有变量
    pub fn get_all_variables(&self) -> &HashMap<String, Expression> {
        &self.variables
    }
    
    /// 获取所有数值变量
    pub fn get_all_numeric_variables(&self) -> &HashMap<String, Number> {
        &self.numeric_variables
    }
    
    /// 清空所有变量
    pub fn clear(&mut self) {
        self.variables.clear();
        self.numeric_variables.clear();
    }
    
    /// 删除指定变量
    pub fn remove_variable(&mut self, name: &str) -> bool {
        let removed_expr = self.variables.remove(name).is_some();
        let removed_num = self.numeric_variables.remove(name).is_some();
        removed_expr || removed_num
    }
    
    /// 替换表达式中的变量
    pub fn substitute_variables(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Variable(name) => {
                if let Some(value) = self.get_variable(name) {
                    // 递归替换，防止变量引用链
                    self.substitute_variables(value)
                } else {
                    expr.clone()
                }
            }
            Expression::BinaryOp { op, left, right } => {
                Expression::BinaryOp {
                    op: op.clone(),
                    left: Box::new(self.substitute_variables(left)),
                    right: Box::new(self.substitute_variables(right)),
                }
            }
            Expression::UnaryOp { op, operand } => {
                Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(self.substitute_variables(operand)),
                }
            }
            Expression::Function { name, args } => {
                Expression::Function {
                    name: name.clone(),
                    args: args.iter().map(|arg| self.substitute_variables(arg)).collect(),
                }
            }
            Expression::Matrix(rows) => {
                Expression::Matrix(
                    rows.iter()
                        .map(|row| row.iter().map(|elem| self.substitute_variables(elem)).collect())
                        .collect()
                )
            }
            Expression::Vector(elements) => {
                Expression::Vector(
                    elements.iter().map(|elem| self.substitute_variables(elem)).collect()
                )
            }
            Expression::Set(elements) => {
                Expression::Set(
                    elements.iter().map(|elem| self.substitute_variables(elem)).collect()
                )
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                Expression::Interval {
                    start: Box::new(self.substitute_variables(start)),
                    end: Box::new(self.substitute_variables(end)),
                    start_inclusive: *start_inclusive,
                    end_inclusive: *end_inclusive,
                }
            }
            _ => expr.clone(),
        }
    }
    
    /// 检查变量名是否有效
    fn is_valid_variable_name(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        // 变量名必须以字母或下划线开头
        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }
        
        // 其余字符必须是字母、数字或下划线
        name.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
    
    /// 检查是否存在循环引用
    pub fn has_circular_reference(&self, name: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.check_circular_reference_recursive(name, &mut visited)
    }
    
    /// 递归检查循环引用
    fn check_circular_reference_recursive(&self, name: &str, visited: &mut std::collections::HashSet<String>) -> bool {
        if visited.contains(name) {
            return true; // 发现循环引用
        }
        
        if let Some(expr) = self.get_variable(name) {
            visited.insert(name.to_string());
            let has_cycle = self.check_expression_for_circular_reference(expr, visited);
            visited.remove(name);
            has_cycle
        } else {
            false
        }
    }
    
    /// 检查表达式中是否存在循环引用
    fn check_expression_for_circular_reference(&self, expr: &Expression, visited: &mut std::collections::HashSet<String>) -> bool {
        match expr {
            Expression::Variable(var_name) => {
                self.check_circular_reference_recursive(var_name, visited)
            }
            Expression::BinaryOp { left, right, .. } => {
                self.check_expression_for_circular_reference(left, visited) ||
                self.check_expression_for_circular_reference(right, visited)
            }
            Expression::UnaryOp { operand, .. } => {
                self.check_expression_for_circular_reference(operand, visited)
            }
            Expression::Function { args, .. } => {
                args.iter().any(|arg| self.check_expression_for_circular_reference(arg, visited))
            }
            Expression::Matrix(rows) => {
                rows.iter().any(|row| 
                    row.iter().any(|elem| self.check_expression_for_circular_reference(elem, visited))
                )
            }
            Expression::Vector(elements) => {
                elements.iter().any(|elem| self.check_expression_for_circular_reference(elem, visited))
            }
            Expression::Set(elements) => {
                elements.iter().any(|elem| self.check_expression_for_circular_reference(elem, visited))
            }
            Expression::Interval { start, end, .. } => {
                self.check_expression_for_circular_reference(start, visited) ||
                self.check_expression_for_circular_reference(end, visited)
            }
            _ => false,
        }
    }
}

/// 运行时增强器
pub struct RuntimeEnhancer {
    /// 复杂度分析器
    complexity_analyzer: ComplexityAnalyzer,
    /// 变量管理器
    variable_manager: VariableManager,
    /// 配置
    config: RuntimeConfig,
}

impl RuntimeEnhancer {
    /// 创建新的运行时增强器
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            complexity_analyzer: ComplexityAnalyzer::new(config.clone()),
            variable_manager: VariableManager::new(),
            config,
        }
    }
    
    /// 获取变量管理器的可变引用
    pub fn variable_manager_mut(&mut self) -> &mut VariableManager {
        &mut self.variable_manager
    }
    
    /// 获取变量管理器的引用
    pub fn variable_manager(&self) -> &VariableManager {
        &self.variable_manager
    }
    
    /// 安全计算表达式
    pub fn safe_compute<E: ComputeEngine>(&self, expr: &Expression, engine: &E) -> Result<Expression, ComputeError> {
        // 1. 替换变量
        let substituted = self.variable_manager.substitute_variables(expr);
        
        // 2. 检查复杂度
        if self.complexity_analyzer.is_too_complex(&substituted) {
            return Ok(substituted); // 返回原表达式，不进行计算
        }
        
        // 3. 检查特殊情况（如大指数）
        if let Some(safe_expr) = self.check_and_handle_special_cases(&substituted)? {
            return Ok(safe_expr);
        }
        
        // 4. 带时间限制的计算
        if self.config.enable_time_limit {
            self.compute_with_timeout(&substituted, engine)
        } else {
            engine.simplify(&substituted)
        }
    }
    
    /// 检查并处理特殊情况
    fn check_and_handle_special_cases(&self, expr: &Expression) -> Result<Option<Expression>, ComputeError> {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } => {
                if !self.complexity_analyzer.is_safe_power(left, right) {
                    // 对于不安全的指数运算，保持符号形式
                    return Ok(Some(expr.clone()));
                }
            }
            Expression::UnaryOp { op: UnaryOperator::Factorial, operand } => {
                // 检查阶乘的参数是否过大
                if let Expression::Number(Number::Integer(n)) = operand.as_ref() {
                    if n > &BigInt::from(1000) {
                        // 对于大数阶乘，保持符号形式
                        return Ok(Some(expr.clone()));
                    }
                }
            }
            _ => {}
        }
        
        Ok(None)
    }
    
    /// 带超时的计算
    fn compute_with_timeout<E: ComputeEngine>(&self, expr: &Expression, engine: &E) -> Result<Expression, ComputeError> {
        let start_time = Instant::now();
        let timeout = Duration::from_millis(self.config.max_compute_time_ms);
        
        // 这里简化实现，实际应该在单独的线程中执行计算
        // 并定期检查超时
        let result = engine.simplify(expr)?;
        
        if start_time.elapsed() > timeout {
            // 如果计算超时，返回原表达式
            Ok(expr.clone())
        } else {
            Ok(result)
        }
    }
    
    /// 更新配置
    pub fn update_config(&mut self, config: RuntimeConfig) {
        self.config = config.clone();
        self.complexity_analyzer = ComplexityAnalyzer::new(config);
    }
    
    /// 获取配置
    pub fn get_config(&self) -> &RuntimeConfig {
        &self.config
    }
}

impl Default for VariableManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RuntimeEnhancer {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    
    #[test]
    fn test_complexity_analyzer() {
        let config = RuntimeConfig::default();
        let analyzer = ComplexityAnalyzer::new(config);
        
        // 简单表达式
        let simple_expr = Expression::variable("x");
        assert_eq!(analyzer.calculate_complexity(&simple_expr), 1);
        
        // 复杂表达式
        let complex_expr = Expression::binary_op(
            BinaryOperator::Power,
            Expression::number(Number::from(10)),
            Expression::number(Number::from(10000))
        );
        assert!(analyzer.is_too_complex(&complex_expr));
    }
    
    #[test]
    fn test_variable_manager() {
        let mut manager = VariableManager::new();
        
        // 设置变量
        let x_value = Expression::number(Number::from(10));
        manager.set_variable("x".to_string(), x_value.clone()).unwrap();
        
        // 获取变量
        assert_eq!(manager.get_variable("x"), Some(&x_value));
        
        // 变量替换
        let expr = Expression::binary_op(
            BinaryOperator::Add,
            Expression::variable("x"),
            Expression::number(Number::from(5))
        );
        let substituted = manager.substitute_variables(&expr);
        
        // 检查替换是否成功（左操作数应该是数值而不是变量）
        if let Expression::BinaryOp { left, .. } = substituted {
            match left.as_ref() {
                Expression::Number(_) => {
                    // 替换成功
                    println!("变量替换成功");
                }
                Expression::Variable(_) => {
                    panic!("变量替换失败");
                }
                _ => {
                    panic!("意外的表达式类型");
                }
            }
        } else {
            panic!("Expected binary operation");
        }
    }
    
    #[test]
    fn test_circular_reference_detection() {
        let mut manager = VariableManager::new();
        
        // 设置循环引用: x = y, y = x
        manager.set_variable("x".to_string(), Expression::variable("y")).unwrap();
        manager.set_variable("y".to_string(), Expression::variable("x")).unwrap();
        
        assert!(manager.has_circular_reference("x"));
        assert!(manager.has_circular_reference("y"));
    }
    
    #[test]
    fn test_safe_power_check() {
        let config = RuntimeConfig::default();
        let analyzer = ComplexityAnalyzer::new(config);
        
        // 安全的指数
        let safe_base = Expression::number(Number::from(2));
        let safe_exp = Expression::number(Number::from(10));
        assert!(analyzer.is_safe_power(&safe_base, &safe_exp));
        
        // 不安全的指数
        let unsafe_base = Expression::number(Number::from(10));
        let unsafe_exp = Expression::number(Number::from(10000));
        assert!(!analyzer.is_safe_power(&unsafe_base, &unsafe_exp));
    }
}