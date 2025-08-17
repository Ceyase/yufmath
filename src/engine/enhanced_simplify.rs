//! # 增强化简器
//!
//! 实现运行时化简增强功能，包括：
//! - 每次运算后自动化简
//! - 根号表达式化简
//! - 三角函数化简
//! - 更多代数化简规则

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
use crate::engine::error::ComputeError;
use crate::engine::simplify::Simplifier;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, ToPrimitive};

/// 增强化简器
pub struct EnhancedSimplifier {
    /// 基础简化器
    base_simplifier: Simplifier,
    /// 是否启用自动化简
    auto_simplify: bool,
    /// 化简规则缓存
    rule_cache: HashMap<Expression, Expression>,
}

impl EnhancedSimplifier {
    /// 创建新的增强化简器
    pub fn new() -> Self {
        Self {
            base_simplifier: Simplifier::new(),
            auto_simplify: true,
            rule_cache: HashMap::new(),
        }
    }
    
    /// 设置是否启用自动化简
    pub fn set_auto_simplify(&mut self, enabled: bool) {
        self.auto_simplify = enabled;
    }
    
    /// 增强化简表达式
    pub fn enhanced_simplify(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 首先应用基础简化
        let mut simplified = self.base_simplifier.simplify(expr)?;
        
        // 应用增强化简规则
        simplified = self.apply_enhanced_rules(&simplified)?;
        
        // 如果启用自动化简，继续应用更多规则
        if self.auto_simplify {
            simplified = self.apply_auto_simplify_rules(&simplified)?;
        }
        
        Ok(simplified)
    }
    
    /// 应用增强化简规则
    fn apply_enhanced_rules(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 检查缓存
        if let Some(cached) = self.rule_cache.get(expr) {
            return Ok(cached.clone());
        }
        
        let mut result = expr.clone();
        
        // 应用根号化简规则
        result = self.simplify_radicals(&result)?;
        
        // 应用三角函数化简规则
        result = self.simplify_trigonometric(&result)?;
        
        // 应用更多代数化简规则
        result = self.apply_advanced_algebraic_rules(&result)?;
        
        // 缓存结果
        self.rule_cache.insert(expr.clone(), result.clone());
        
        Ok(result)
    }
    
    /// 化简根号表达式
    fn simplify_radicals(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            // 递归处理子表达式
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.simplify_radicals(left)?;
                let right_simplified = self.simplify_radicals(right)?;
                
                match op {
                    BinaryOperator::Add => {
                        self.simplify_radical_addition(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Subtract => {
                        self.simplify_radical_subtraction(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Multiply => {
                        self.simplify_radical_multiplication(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Divide => {
                        self.simplify_radical_division(&left_simplified, &right_simplified)
                    }
                    _ => Ok(Expression::binary_op(op.clone(), left_simplified, right_simplified))
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.simplify_radicals(operand)?;
                match op {
                    UnaryOperator::Sqrt => {
                        self.simplify_square_root(&operand_simplified)
                    }
                    _ => Ok(Expression::unary_op(op.clone(), operand_simplified))
                }
            }
            
            Expression::Function { name, args } => {
                if name == "sqrt" && args.len() == 1 {
                    let arg_simplified = self.simplify_radicals(&args[0])?;
                    self.simplify_square_root(&arg_simplified)
                } else {
                    let args_simplified: Result<Vec<_>, _> = args.iter()
                        .map(|arg| self.simplify_radicals(arg))
                        .collect();
                    Ok(Expression::function(name, args_simplified?))
                }
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 化简根号加法：sqrt(a) + sqrt(b) 或 c*sqrt(a) + d*sqrt(b)
    fn simplify_radical_addition(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 提取根号项的系数和根号内容
        if let (Some((coeff_a, radical_a)), Some((coeff_b, radical_b))) = (
            self.extract_radical_coefficient(left),
            self.extract_radical_coefficient(right)
        ) {
            // 如果根号内容相同，合并系数
            if radical_a == radical_b {
                let new_coeff = Expression::add(coeff_a, coeff_b);
                // 简化系数
                let simplified_coeff = self.base_simplifier.simplify(&new_coeff)?;
                
                // 如果系数为0，返回0
                if self.is_zero(&simplified_coeff) {
                    return Ok(Expression::Number(Number::zero()));
                }
                
                // 如果系数为1，直接返回根号
                if self.is_one(&simplified_coeff) {
                    return Ok(self.create_sqrt_expression(&radical_a));
                }
                
                // 否则返回 coeff * sqrt(radical)
                return Ok(Expression::multiply(
                    simplified_coeff,
                    self.create_sqrt_expression(&radical_a)
                ));
            }
        }
        
        // 无法合并，返回原表达式
        Ok(Expression::add(left.clone(), right.clone()))
    }
    
    /// 化简根号减法
    fn simplify_radical_subtraction(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 提取根号项的系数和根号内容
        if let (Some((coeff_a, radical_a)), Some((coeff_b, radical_b))) = (
            self.extract_radical_coefficient(left),
            self.extract_radical_coefficient(right)
        ) {
            // 如果根号内容相同，合并系数
            if radical_a == radical_b {
                let new_coeff = Expression::subtract(coeff_a, coeff_b);
                // 简化系数
                let simplified_coeff = self.base_simplifier.simplify(&new_coeff)?;
                
                // 如果系数为0，返回0
                if self.is_zero(&simplified_coeff) {
                    return Ok(Expression::Number(Number::zero()));
                }
                
                // 如果系数为1，直接返回根号
                if self.is_one(&simplified_coeff) {
                    return Ok(self.create_sqrt_expression(&radical_a));
                }
                
                // 否则返回 coeff * sqrt(radical)
                return Ok(Expression::multiply(
                    simplified_coeff,
                    self.create_sqrt_expression(&radical_a)
                ));
            }
        }
        
        // 无法合并，返回原表达式
        Ok(Expression::subtract(left.clone(), right.clone()))
    }
    
    /// 化简根号乘法：sqrt(a) * sqrt(b) = sqrt(a*b)
    fn simplify_radical_multiplication(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        match (left, right) {
            // sqrt(a) * sqrt(b) = sqrt(a*b)
            (Expression::Function { name: name1, args: args1 }, 
             Expression::Function { name: name2, args: args2 })
                if name1 == "sqrt" && name2 == "sqrt" && args1.len() == 1 && args2.len() == 1 => {
                let product = Expression::multiply(args1[0].clone(), args2[0].clone());
                let simplified_product = self.base_simplifier.simplify(&product)?;
                self.simplify_square_root(&simplified_product)
            }
            
            _ => Ok(Expression::multiply(left.clone(), right.clone()))
        }
    }
    
    /// 化简根号除法：sqrt(a) / sqrt(b) = sqrt(a/b)
    fn simplify_radical_division(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        match (left, right) {
            // sqrt(a) / sqrt(b) = sqrt(a/b)
            (Expression::Function { name: name1, args: args1 }, 
             Expression::Function { name: name2, args: args2 })
                if name1 == "sqrt" && name2 == "sqrt" && args1.len() == 1 && args2.len() == 1 => {
                let quotient = Expression::divide(args1[0].clone(), args2[0].clone());
                let simplified_quotient = self.base_simplifier.simplify(&quotient)?;
                Ok(Expression::function("sqrt", vec![simplified_quotient]))
            }
            
            _ => Ok(Expression::divide(left.clone(), right.clone()))
        }
    }
    
    /// 化简平方根
    fn simplify_square_root(&mut self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // sqrt(a^2) = |a|
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } 
                if matches!(right.as_ref(), Expression::Number(n) if n.is_two()) => {
                Ok(Expression::function("abs", vec![left.as_ref().clone()]))
            }
            
            // 对于数值，尝试简化
            Expression::Number(n) => {
                self.simplify_numeric_square_root(n)
            }
            
            _ => Ok(Expression::function("sqrt", vec![arg.clone()]))
        }
    }
    
    /// 化简数值平方根
    fn simplify_numeric_square_root(&self, n: &Number) -> Result<Expression, ComputeError> {
        match n {
            Number::Integer(i) => {
                if i < &BigInt::zero() {
                    // 负数的平方根涉及复数，保持符号形式
                    Ok(Expression::function("sqrt", vec![Expression::Number(n.clone())]))
                } else {
                    // 检查是否是完全平方数
                    if let Some(sqrt_int) = self.integer_sqrt(i) {
                        Ok(Expression::Number(Number::Integer(sqrt_int)))
                    } else {
                        // 尝试提取完全平方因子
                        self.extract_square_factors(i)
                    }
                }
            }
            
            _ => Ok(Expression::function("sqrt", vec![Expression::Number(n.clone())]))
        }
    }
    
    /// 计算整数的平方根（如果是完全平方数）
    fn integer_sqrt(&self, n: &BigInt) -> Option<BigInt> {
        if n < &BigInt::zero() {
            return None;
        }
        
        if n == &BigInt::zero() || n == &BigInt::one() {
            return Some(n.clone());
        }
        
        // 对于小数值，使用浮点数快速检查
        if let Some(n_f64) = n.to_f64() {
            if n_f64 <= (u64::MAX as f64) {
                let sqrt_f64 = n_f64.sqrt();
                if sqrt_f64.fract() == 0.0 {
                    let sqrt_int = sqrt_f64 as u64;
                    let sqrt_bigint = BigInt::from(sqrt_int);
                    if &(&sqrt_bigint * &sqrt_bigint) == n {
                        return Some(sqrt_bigint);
                    }
                }
                return None;
            }
        }
        
        None
    }
    
    /// 提取完全平方因子
    fn extract_square_factors(&self, n: &BigInt) -> Result<Expression, ComputeError> {
        if n <= &BigInt::zero() {
            return Ok(Expression::function("sqrt", vec![Expression::Number(Number::Integer(n.clone()))]));
        }
        
        let mut remaining = n.clone();
        let mut extracted = BigInt::one();
        
        // 检查小素数的平方因子
        let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        
        for &p in &small_primes {
            let p_big = BigInt::from(p);
            let p_squared = &p_big * &p_big;
            
            while &remaining % &p_squared == BigInt::zero() {
                remaining /= &p_squared;
                extracted *= &p_big;
            }
        }
        
        // 构造结果
        if extracted == BigInt::one() {
            // 没有完全平方因子
            if remaining == BigInt::one() {
                Ok(Expression::Number(Number::Integer(BigInt::one())))
            } else {
                Ok(Expression::function("sqrt", vec![Expression::Number(Number::Integer(remaining))]))
            }
        } else {
            // 有完全平方因子
            if remaining == BigInt::one() {
                Ok(Expression::Number(Number::Integer(extracted)))
            } else {
                Ok(Expression::multiply(
                    Expression::Number(Number::Integer(extracted)),
                    Expression::function("sqrt", vec![Expression::Number(Number::Integer(remaining))])
                ))
            }
        }
    }
    
    /// 化简三角函数
    fn simplify_trigonometric(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::Function { name, args } => {
                match name.as_str() {
                    "sin" => self.simplify_sine_function(args),
                    "cos" => self.simplify_cosine_function(args),
                    "tan" => self.simplify_tangent_function(args),
                    _ => {
                        // 递归处理参数
                        let args_simplified: Result<Vec<_>, _> = args.iter()
                            .map(|arg| self.simplify_trigonometric(arg))
                            .collect();
                        Ok(Expression::function(name, args_simplified?))
                    }
                }
            }
            
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.simplify_trigonometric(left)?;
                let right_simplified = self.simplify_trigonometric(right)?;
                
                // 应用三角恒等式
                self.apply_trigonometric_identities(op, &left_simplified, &right_simplified)
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.simplify_trigonometric(operand)?;
                Ok(Expression::unary_op(op.clone(), operand_simplified))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 化简正弦函数
    fn simplify_sine_function(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("sin", args.to_vec()));
        }
        
        let arg = &args[0];
        
        // 应用诱导公式
        match arg {
            // sin(-x) = -sin(x)
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                Ok(Expression::negate(Expression::function("sin", vec![operand.as_ref().clone()])))
            }
            
            // 特殊角度值
            _ => self.evaluate_sine_special_angles(arg)
        }
    }
    
    /// 化简余弦函数
    fn simplify_cosine_function(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("cos", args.to_vec()));
        }
        
        let arg = &args[0];
        
        // 应用诱导公式
        match arg {
            // cos(-x) = cos(x)
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                Ok(Expression::function("cos", vec![operand.as_ref().clone()]))
            }
            
            // 特殊角度值
            _ => self.evaluate_cosine_special_angles(arg)
        }
    }
    
    /// 化简正切函数
    fn simplify_tangent_function(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("tan", args.to_vec()));
        }
        
        let arg = &args[0];
        
        // 应用诱导公式
        match arg {
            // tan(-x) = -tan(x)
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                Ok(Expression::negate(Expression::function("tan", vec![operand.as_ref().clone()])))
            }
            
            // 特殊角度值
            _ => self.evaluate_tangent_special_angles(arg)
        }
    }
    
    /// 应用三角恒等式
    fn apply_trigonometric_identities(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        match op {
            BinaryOperator::Add => {
                // sin²x + cos²x = 1
                if self.is_sin_squared(left) && self.is_cos_squared_same_arg(right, left) {
                    return Ok(Expression::Number(Number::one()));
                }
                if self.is_cos_squared(left) && self.is_sin_squared_same_arg(right, left) {
                    return Ok(Expression::Number(Number::one()));
                }
            }
            
            _ => {}
        }
        
        Ok(Expression::binary_op(op.clone(), left.clone(), right.clone()))
    }
    
    /// 应用更多代数化简规则
    fn apply_advanced_algebraic_rules(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.apply_advanced_algebraic_rules(left)?;
                let right_simplified = self.apply_advanced_algebraic_rules(right)?;
                
                match op {
                    BinaryOperator::Add => {
                        self.apply_advanced_addition_rules(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Multiply => {
                        self.apply_advanced_multiplication_rules(&left_simplified, &right_simplified)
                    }
                    _ => Ok(Expression::binary_op(op.clone(), left_simplified, right_simplified))
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.apply_advanced_algebraic_rules(operand)?;
                Ok(Expression::unary_op(op.clone(), operand_simplified))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 应用高级加法规则
    fn apply_advanced_addition_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 分数加法：a/b + c/d = (ad + bc)/(bd)
        if let (Some((a, b)), Some((c, d))) = (self.extract_fraction(left), self.extract_fraction(right)) {
            let numerator = Expression::add(
                Expression::multiply(a, d.clone()),
                Expression::multiply(c, b.clone())
            );
            let denominator = Expression::multiply(b, d);
            
            let simplified_num = self.base_simplifier.simplify(&numerator)?;
            let simplified_den = self.base_simplifier.simplify(&denominator)?;
            
            return Ok(Expression::divide(simplified_num, simplified_den));
        }
        
        Ok(Expression::add(left.clone(), right.clone()))
    }
    
    /// 应用高级乘法规则
    fn apply_advanced_multiplication_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // (a + b)(c + d) = ac + ad + bc + bd
        if let (Some((a, b)), Some((c, d))) = (self.extract_sum(left), self.extract_sum(right)) {
            let ac = Expression::multiply(a.clone(), c.clone());
            let ad = Expression::multiply(a, d.clone());
            let bc = Expression::multiply(b.clone(), c);
            let bd = Expression::multiply(b, d);
            
            let result = Expression::add(
                Expression::add(ac, ad),
                Expression::add(bc, bd)
            );
            
            return self.base_simplifier.simplify(&result);
        }
        
        // (a - b)(a + b) = a² - b²
        if let (Some((a1, b1)), Some((a2, b2))) = (self.extract_difference(left), self.extract_sum(right)) {
            if a1 == a2 && b1 == b2 {
                let a_squared = Expression::power(a1, Expression::Number(Number::integer(2)));
                let b_squared = Expression::power(b1, Expression::Number(Number::integer(2)));
                return Ok(Expression::subtract(a_squared, b_squared));
            }
        }
        
        Ok(Expression::multiply(left.clone(), right.clone()))
    }
    
    /// 应用自动化简规则
    fn apply_auto_simplify_rules(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 递归应用化简规则直到不再变化
        let mut current = expr.clone();
        let mut previous;
        let max_iterations = 10; // 防止无限循环
        let mut iterations = 0;
        
        loop {
            previous = current.clone();
            current = self.apply_enhanced_rules(&current)?;
            
            iterations += 1;
            if current == previous || iterations >= max_iterations {
                break;
            }
        }
        
        Ok(current)
    }
    
    // 辅助方法
    
    /// 提取根号项的系数和根号内容
    fn extract_radical_coefficient(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            // sqrt(a) = 1 * sqrt(a)
            Expression::Function { name, args } if name == "sqrt" && args.len() == 1 => {
                Some((Expression::Number(Number::one()), args[0].clone()))
            }
            
            // c * sqrt(a)
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                if let Expression::Function { name, args } = right.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((left.as_ref().clone(), args[0].clone()));
                    }
                }
                if let Expression::Function { name, args } = left.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((right.as_ref().clone(), args[0].clone()));
                    }
                }
                None
            }
            
            _ => None
        }
    }
    
    /// 创建平方根表达式
    fn create_sqrt_expression(&self, arg: &Expression) -> Expression {
        Expression::function("sqrt", vec![arg.clone()])
    }
    
    /// 检查表达式是否为零
    fn is_zero(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::Number(n) if n.is_zero())
    }
    
    /// 检查表达式是否为一
    fn is_one(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::Number(n) if n.is_one())
    }
    
    /// 检查是否为 sin²(x) 形式
    fn is_sin_squared(&self, expr: &Expression) -> bool {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } => {
                matches!(left.as_ref(), Expression::Function { name, .. } if name == "sin") &&
                matches!(right.as_ref(), Expression::Number(n) if n.is_two())
            }
            _ => false,
        }
    }
    
    /// 检查是否为 cos²(x) 形式
    fn is_cos_squared(&self, expr: &Expression) -> bool {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } => {
                matches!(left.as_ref(), Expression::Function { name, .. } if name == "cos") &&
                matches!(right.as_ref(), Expression::Number(n) if n.is_two())
            }
            _ => false,
        }
    }
    
    /// 检查是否为 cos²(x) 形式，且与给定的 sin²(x) 有相同参数
    fn is_cos_squared_same_arg(&self, expr: &Expression, sin_squared: &Expression) -> bool {
        if let (
            Expression::BinaryOp { op: BinaryOperator::Power, left: cos_func, right: cos_exp },
            Expression::BinaryOp { op: BinaryOperator::Power, left: sin_func, right: _ }
        ) = (expr, sin_squared) {
            if matches!(cos_func.as_ref(), Expression::Function { name, .. } if name == "cos") &&
               matches!(cos_exp.as_ref(), Expression::Number(n) if n.is_two()) {
                if let (
                    Expression::Function { args: cos_args, .. },
                    Expression::Function { args: sin_args, .. }
                ) = (cos_func.as_ref(), sin_func.as_ref()) {
                    return cos_args == sin_args;
                }
            }
        }
        false
    }
    
    /// 检查是否为 sin²(x) 形式，且与给定的 cos²(x) 有相同参数
    fn is_sin_squared_same_arg(&self, expr: &Expression, cos_squared: &Expression) -> bool {
        self.is_cos_squared_same_arg(cos_squared, expr)
    }
    
    /// 计算特殊角度的正弦值
    fn evaluate_sine_special_angles(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // sin(0) = 0
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::zero()))
            }
            
            // sin(π/6) = 1/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(6)) => {
                Ok(Expression::Number(Number::rational(1, 2)))
            }
            
            _ => Ok(Expression::function("sin", vec![arg.clone()]))
        }
    }
    
    /// 计算特殊角度的余弦值
    fn evaluate_cosine_special_angles(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // cos(0) = 1
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::one()))
            }
            
            _ => Ok(Expression::function("cos", vec![arg.clone()]))
        }
    }
    
    /// 计算特殊角度的正切值
    fn evaluate_tangent_special_angles(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // tan(0) = 0
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::zero()))
            }
            
            _ => Ok(Expression::function("tan", vec![arg.clone()]))
        }
    }
    
    /// 提取分数形式 a/b
    fn extract_fraction(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                Some((left.as_ref().clone(), right.as_ref().clone()))
            }
            _ => None,
        }
    }
    
    /// 提取加法形式 a + b
    fn extract_sum(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
                Some((left.as_ref().clone(), right.as_ref().clone()))
            }
            _ => None,
        }
    }
    
    /// 提取减法形式 a - b
    fn extract_difference(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Subtract, left, right } => {
                Some((left.as_ref().clone(), right.as_ref().clone()))
            }
            _ => None,
        }
    }
}

impl Default for EnhancedSimplifier {
    fn default() -> Self {
        Self::new()
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "enhanced_simplify_tests.rs"]
mod enhanced_simplify_tests;