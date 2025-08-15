//! # 表达式简化
//!
//! 实现代数表达式的简化规则和算法。

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
use crate::engine::error::ComputeError;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_rational::BigRational;

/// 表达式简化器
pub struct Simplifier {
    /// 简化规则缓存
    cache: HashMap<Expression, Expression>,
}

impl Simplifier {
    /// 创建新的简化器
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    /// 简化表达式
    pub fn simplify(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 检查缓存
        if let Some(cached) = self.cache.get(expr) {
            return Ok(cached.clone());
        }
        
        let simplified = self.simplify_recursive(expr)?;
        
        // 应用常量折叠
        let folded = self.constant_folding(&simplified)?;
        
        // 缓存结果
        self.cache.insert(expr.clone(), folded.clone());
        
        Ok(folded)
    }
    
    /// 常量折叠优化
    fn constant_folding(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            // 对于单个常量，不进行折叠，保持原形式
            Expression::Constant(_) => Ok(expr.clone()),
            
            // 如果表达式是常量，尝试计算其值
            _ if expr.is_constant() => {
                match self.evaluate_constant_expression(expr) {
                    Ok(value) => Ok(Expression::Number(value)),
                    Err(_) => Ok(expr.clone()), // 如果计算失败，保持原表达式
                }
            }
            _ => Ok(expr.clone()),
        }
    }
    
    /// 计算常量表达式的值
    fn evaluate_constant_expression(&self, expr: &Expression) -> Result<Number, ComputeError> {
        match expr {
            Expression::Number(n) => Ok(n.clone()),
            
            Expression::Constant(c) => {
                // 将数学常量转换为数值（如果可能的话）
                match c {
                    MathConstant::I => Ok(Number::i()),
                    MathConstant::PositiveInfinity => Ok(Number::Float(f64::INFINITY)),
                    MathConstant::NegativeInfinity => Ok(Number::Float(f64::NEG_INFINITY)),
                    MathConstant::Undefined => Ok(Number::Float(f64::NAN)),
                    _ => Ok(Number::Constant(c.clone())), // 其他常量保持符号形式
                }
            }
            
            Expression::BinaryOp { op, left, right } => {
                let left_val = self.evaluate_constant_expression(left)?;
                let right_val = self.evaluate_constant_expression(right)?;
                
                match op {
                    BinaryOperator::Add => Ok(left_val + right_val),
                    BinaryOperator::Subtract => Ok(left_val - right_val),
                    BinaryOperator::Multiply => Ok(left_val * right_val),
                    BinaryOperator::Divide => {
                        if right_val.is_zero() {
                            Err(ComputeError::DivisionByZero)
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    BinaryOperator::Power => {
                        // 只计算简单的整数幂
                        if let Some(result) = self.compute_integer_power(&left_val, &right_val) {
                            Ok(result)
                        } else {
                            Err(ComputeError::UnsupportedOperation { 
                                operation: "复杂幂运算".to_string() 
                            })
                        }
                    }
                    _ => Err(ComputeError::UnsupportedOperation { 
                        operation: format!("常量折叠中的 {:?} 运算", op) 
                    }),
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_val = self.evaluate_constant_expression(operand)?;
                
                match op {
                    UnaryOperator::Negate => Ok(-operand_val),
                    UnaryOperator::Plus => Ok(operand_val),
                    UnaryOperator::Abs => Ok(operand_val.abs()),
                    _ => Err(ComputeError::UnsupportedOperation { 
                        operation: format!("常量折叠中的 {:?} 运算", op) 
                    }),
                }
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "非常量表达式".to_string() 
            }),
        }
    }
    
    /// 递归简化表达式
    fn simplify_recursive(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            // 基本表达式不需要简化
            Expression::Number(_) | Expression::Variable(_) | Expression::Constant(_) => {
                Ok(expr.clone())
            }
            
            // 简化二元运算
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.simplify_recursive(left)?;
                let right_simplified = self.simplify_recursive(right)?;
                self.simplify_binary_op(op, &left_simplified, &right_simplified)
            }
            
            // 简化一元运算
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.simplify_recursive(operand)?;
                self.simplify_unary_op(op, &operand_simplified)
            }
            
            // 简化函数调用
            Expression::Function { name, args } => {
                let args_simplified: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.simplify_recursive(arg))
                    .collect();
                let args_simplified = args_simplified?;
                self.simplify_function(name, &args_simplified)
            }
            
            // 其他类型暂时不简化
            _ => Ok(expr.clone()),
        }
    }
    
    /// 简化二元运算
    fn simplify_binary_op(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        match op {
            BinaryOperator::Add => self.simplify_addition(left, right),
            BinaryOperator::Subtract => self.simplify_subtraction(left, right),
            BinaryOperator::Multiply => self.simplify_multiplication(left, right),
            BinaryOperator::Divide => self.simplify_division(left, right),
            BinaryOperator::Power => self.simplify_power(left, right),
            _ => Ok(Expression::binary_op(op.clone(), left.clone(), right.clone())),
        }
    }
    
    /// 简化加法运算
    fn simplify_addition(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 规则：0 + x = x
        if self.is_zero(left) {
            return Ok(right.clone());
        }
        if self.is_zero(right) {
            return Ok(left.clone());
        }
        
        // 规则：常量折叠
        if let (Expression::Number(a), Expression::Number(b)) = (left, right) {
            return Ok(Expression::Number(a.clone() + b.clone()));
        }
        
        // 规则：x + x = 2x
        if left == right {
            return Ok(Expression::multiply(
                Expression::Number(Number::integer(2)),
                left.clone()
            ));
        }
        
        // 规则：合并同类项 (ax + bx = (a+b)x)
        if let Some(simplified) = self.combine_like_terms_add(left, right) {
            return Ok(simplified);
        }
        
        // 规则：交换律排序（将常数项放在前面）
        if self.should_swap_for_canonical_form(left, right) {
            return Ok(Expression::add(right.clone(), left.clone()));
        }
        
        Ok(Expression::add(left.clone(), right.clone()))
    }
    
    /// 简化减法运算
    fn simplify_subtraction(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 规则：x - 0 = x
        if self.is_zero(right) {
            return Ok(left.clone());
        }
        
        // 规则：0 - x = -x
        if self.is_zero(left) {
            return Ok(Expression::negate(right.clone()));
        }
        
        // 规则：x - x = 0
        if left == right {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 规则：常量折叠
        if let (Expression::Number(a), Expression::Number(b)) = (left, right) {
            return Ok(Expression::Number(a.clone() - b.clone()));
        }
        
        // 规则：合并同类项 (ax - bx = (a-b)x)
        if let Some(simplified) = self.combine_like_terms_sub(left, right) {
            return Ok(simplified);
        }
        
        Ok(Expression::subtract(left.clone(), right.clone()))
    }
    
    /// 简化乘法运算
    fn simplify_multiplication(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 规则：0 * x = 0
        if self.is_zero(left) || self.is_zero(right) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 规则：1 * x = x
        if self.is_one(left) {
            return Ok(right.clone());
        }
        if self.is_one(right) {
            return Ok(left.clone());
        }
        
        // 规则：-1 * x = -x
        if self.is_neg_one(left) {
            return Ok(Expression::negate(right.clone()));
        }
        if self.is_neg_one(right) {
            return Ok(Expression::negate(left.clone()));
        }
        
        // 规则：常量折叠
        if let (Expression::Number(a), Expression::Number(b)) = (left, right) {
            return Ok(Expression::Number(a.clone() * b.clone()));
        }
        
        // 规则：x * x = x^2
        if left == right {
            return Ok(Expression::power(
                left.clone(),
                Expression::Number(Number::integer(2))
            ));
        }
        
        // 规则：合并同底数 (x^a * x^b = x^(a+b))
        if let Some(simplified) = self.combine_powers_multiply(left, right) {
            return Ok(simplified);
        }
        
        // 规则：交换律排序（将常数项放在前面）
        if self.should_swap_for_canonical_form(left, right) {
            return Ok(Expression::multiply(right.clone(), left.clone()));
        }
        
        Ok(Expression::multiply(left.clone(), right.clone()))
    }
    
    /// 简化除法运算
    fn simplify_division(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 规则：0 / x = 0 (x ≠ 0)
        if self.is_zero(left) && !self.is_zero(right) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 规则：x / 1 = x
        if self.is_one(right) {
            return Ok(left.clone());
        }
        
        // 规则：x / x = 1 (x ≠ 0)
        if left == right && !self.is_zero(left) {
            return Ok(Expression::Number(Number::one()));
        }
        
        // 规则：x / -1 = -x
        if self.is_neg_one(right) {
            return Ok(Expression::negate(left.clone()));
        }
        
        // 规则：常量折叠
        if let (Expression::Number(a), Expression::Number(b)) = (left, right) {
            if !b.is_zero() {
                return Ok(Expression::Number(a.clone() / b.clone()));
            }
        }
        
        // 规则：合并同底数 (x^a / x^b = x^(a-b))
        if let Some(simplified) = self.combine_powers_divide(left, right) {
            return Ok(simplified);
        }
        
        Ok(Expression::divide(left.clone(), right.clone()))
    }
    
    /// 简化幂运算
    fn simplify_power(&self, base: &Expression, exponent: &Expression) -> Result<Expression, ComputeError> {
        // 规则：x^0 = 1 (x ≠ 0)
        if self.is_zero(exponent) && !self.is_zero(base) {
            return Ok(Expression::Number(Number::one()));
        }
        
        // 规则：x^1 = x
        if self.is_one(exponent) {
            return Ok(base.clone());
        }
        
        // 规则：0^x = 0 (x > 0)
        if self.is_zero(base) && self.is_positive(exponent) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 规则：1^x = 1
        if self.is_one(base) {
            return Ok(Expression::Number(Number::one()));
        }
        
        // 规则：常量折叠（仅对小整数指数）
        if let (Expression::Number(a), Expression::Number(b)) = (base, exponent) {
            if let Some(result) = self.compute_integer_power(a, b) {
                return Ok(Expression::Number(result));
            }
        }
        
        // 规则：(x^a)^b = x^(a*b)
        if let Expression::BinaryOp { op: BinaryOperator::Power, left: inner_base, right: inner_exp } = base {
            let new_exp = Expression::multiply(inner_exp.as_ref().clone(), exponent.clone());
            // 尝试简化指数（仅对常量表达式）
            let simplified_exp = if new_exp.is_constant() {
                if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                    Expression::Number(value)
                } else {
                    new_exp
                }
            } else {
                new_exp
            };
            return Ok(Expression::power(inner_base.as_ref().clone(), simplified_exp));
        }
        
        Ok(Expression::power(base.clone(), exponent.clone()))
    }
    
    /// 简化一元运算
    fn simplify_unary_op(&self, op: &UnaryOperator, operand: &Expression) -> Result<Expression, ComputeError> {
        match op {
            UnaryOperator::Negate => self.simplify_negation(operand),
            UnaryOperator::Plus => Ok(operand.clone()), // +x = x
            UnaryOperator::Abs => self.simplify_absolute_value(operand),
            _ => Ok(Expression::unary_op(op.clone(), operand.clone())),
        }
    }
    
    /// 简化负号运算
    fn simplify_negation(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 规则：-(-x) = x
            Expression::UnaryOp { op: UnaryOperator::Negate, operand: inner } => {
                Ok(inner.as_ref().clone())
            }
            
            // 规则：-(a + b) = -a - b
            Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
                Ok(Expression::subtract(
                    Expression::negate(left.as_ref().clone()),
                    right.as_ref().clone()
                ))
            }
            
            // 规则：-(a - b) = b - a
            Expression::BinaryOp { op: BinaryOperator::Subtract, left, right } => {
                Ok(Expression::subtract(
                    right.as_ref().clone(),
                    left.as_ref().clone()
                ))
            }
            
            // 规则：-(a * b) = (-a) * b
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                Ok(Expression::multiply(
                    Expression::negate(left.as_ref().clone()),
                    right.as_ref().clone()
                ))
            }
            
            // 规则：常量折叠
            Expression::Number(n) => {
                Ok(Expression::Number(n.neg()))
            }
            
            _ => Ok(Expression::negate(operand.clone())),
        }
    }
    
    /// 简化绝对值运算
    fn simplify_absolute_value(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 规则：|x| = x 如果 x >= 0
            _ if self.is_non_negative(operand) => Ok(operand.clone()),
            
            // 规则：|x| = -x 如果 x < 0
            _ if self.is_negative(operand) => Ok(Expression::negate(operand.clone())),
            
            // 规则：|-x| = |x|
            Expression::UnaryOp { op: UnaryOperator::Negate, operand: inner } => {
                Ok(Expression::abs(inner.as_ref().clone()))
            }
            
            // 规则：|a * b| = |a| * |b|
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                Ok(Expression::multiply(
                    Expression::abs(left.as_ref().clone()),
                    Expression::abs(right.as_ref().clone())
                ))
            }
            
            // 规则：常量折叠
            Expression::Number(n) => {
                Ok(Expression::Number(n.abs()))
            }
            
            _ => Ok(Expression::abs(operand.clone())),
        }
    }
    
    /// 简化函数调用
    fn simplify_function(&self, name: &str, args: &[Expression]) -> Result<Expression, ComputeError> {
        // 暂时不实现函数简化
        Ok(Expression::function(name, args.to_vec()))
    }
    
    /// 检查表达式是否为零
    fn is_zero(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => n.is_zero(),
            _ => false,
        }
    }
    
    /// 检查表达式是否为一
    fn is_one(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => n.is_one(),
            _ => false,
        }
    }
    
    /// 检查表达式是否为负一
    fn is_neg_one(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => *n == Number::integer(-1),
            _ => false,
        }
    }
    
    /// 检查表达式是否为正数
    fn is_positive(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => n.is_positive(),
            _ => false,
        }
    }
    
    /// 检查表达式是否为负数
    fn is_negative(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => n.is_negative(),
            _ => false,
        }
    }
    
    /// 检查表达式是否为非负数
    fn is_non_negative(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => !n.is_negative(),
            Expression::UnaryOp { op: UnaryOperator::Abs, .. } => true, // |x| >= 0
            Expression::BinaryOp { op: BinaryOperator::Power, right, .. } => {
                // x^(偶数) >= 0
                if let Expression::Number(exp) = right.as_ref() {
                    if let Some(int_exp) = exp.to_integer() {
                        return &int_exp % 2 == BigInt::from(0);
                    }
                }
                false
            }
            _ => false,
        }
    }
    
    /// 合并加法中的同类项
    fn combine_like_terms_add(&self, left: &Expression, right: &Expression) -> Option<Expression> {
        // ax + bx = (a+b)x
        if let (Some((coeff_a, var_a)), Some((coeff_b, var_b))) = (
            self.extract_coefficient_and_variable(left),
            self.extract_coefficient_and_variable(right)
        ) {
            if var_a == var_b {
                let new_coeff = Expression::add(coeff_a, coeff_b);
                // 尝试简化系数（仅对常量表达式）
                let simplified_coeff = if new_coeff.is_constant() {
                    if let Ok(value) = self.evaluate_constant_expression(&new_coeff) {
                        Expression::Number(value)
                    } else {
                        new_coeff
                    }
                } else {
                    new_coeff
                };
                return Some(Expression::multiply(simplified_coeff, var_a));
            }
        }
        None
    }
    
    /// 合并减法中的同类项
    fn combine_like_terms_sub(&self, left: &Expression, right: &Expression) -> Option<Expression> {
        // ax - bx = (a-b)x
        if let (Some((coeff_a, var_a)), Some((coeff_b, var_b))) = (
            self.extract_coefficient_and_variable(left),
            self.extract_coefficient_and_variable(right)
        ) {
            if var_a == var_b {
                let new_coeff = Expression::subtract(coeff_a, coeff_b);
                // 尝试简化系数（仅对常量表达式）
                let simplified_coeff = if new_coeff.is_constant() {
                    if let Ok(value) = self.evaluate_constant_expression(&new_coeff) {
                        Expression::Number(value)
                    } else {
                        new_coeff
                    }
                } else {
                    new_coeff
                };
                return Some(Expression::multiply(simplified_coeff, var_a));
            }
        }
        None
    }
    
    /// 提取表达式的系数和变量部分
    fn extract_coefficient_and_variable(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            // 纯变量：x = 1*x
            Expression::Variable(_) => {
                Some((Expression::Number(Number::one()), expr.clone()))
            }
            
            // 乘法：a*x
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                if let Expression::Number(_) = left.as_ref() {
                    Some((left.as_ref().clone(), right.as_ref().clone()))
                } else if let Expression::Number(_) = right.as_ref() {
                    Some((right.as_ref().clone(), left.as_ref().clone()))
                } else {
                    None
                }
            }
            
            // 纯数字：a = a*1
            Expression::Number(_) => {
                Some((expr.clone(), Expression::Number(Number::one())))
            }
            
            _ => None,
        }
    }
    
    /// 合并乘法中的同底数幂
    fn combine_powers_multiply(&self, left: &Expression, right: &Expression) -> Option<Expression> {
        // x^a * x^b = x^(a+b)
        match (left, right) {
            (Expression::BinaryOp { op: BinaryOperator::Power, left: base1, right: exp1 },
             Expression::BinaryOp { op: BinaryOperator::Power, left: base2, right: exp2 }) => {
                if base1 == base2 {
                    let new_exp = Expression::add(exp1.as_ref().clone(), exp2.as_ref().clone());
                    // 尝试简化指数（仅对常量表达式）
                    let simplified_exp = if new_exp.is_constant() {
                        if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                            Expression::Number(value)
                        } else {
                            new_exp
                        }
                    } else {
                        new_exp
                    };
                    return Some(Expression::power(base1.as_ref().clone(), simplified_exp));
                }
            }
            
            // x * x^a = x^(1+a)
            (var, Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }) |
            (Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }, var) => {
                if var == base.as_ref() {
                    let new_exp = Expression::add(
                        Expression::Number(Number::one()),
                        exp.as_ref().clone()
                    );
                    // 尝试简化指数（仅对常量表达式）
                    let simplified_exp = if new_exp.is_constant() {
                        if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                            Expression::Number(value)
                        } else {
                            new_exp
                        }
                    } else {
                        new_exp
                    };
                    return Some(Expression::power(var.clone(), simplified_exp));
                }
            }
            
            _ => {}
        }
        None
    }
    
    /// 合并除法中的同底数幂
    fn combine_powers_divide(&self, left: &Expression, right: &Expression) -> Option<Expression> {
        // x^a / x^b = x^(a-b)
        match (left, right) {
            (Expression::BinaryOp { op: BinaryOperator::Power, left: base1, right: exp1 },
             Expression::BinaryOp { op: BinaryOperator::Power, left: base2, right: exp2 }) => {
                if base1 == base2 {
                    let new_exp = Expression::subtract(exp1.as_ref().clone(), exp2.as_ref().clone());
                    // 尝试简化指数（仅对常量表达式）
                    let simplified_exp = if new_exp.is_constant() {
                        if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                            Expression::Number(value)
                        } else {
                            new_exp
                        }
                    } else {
                        new_exp
                    };
                    return Some(Expression::power(base1.as_ref().clone(), simplified_exp));
                }
            }
            
            // x / x^a = x^(1-a)
            (var, Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }) => {
                if var == base.as_ref() {
                    let new_exp = Expression::subtract(
                        Expression::Number(Number::one()),
                        exp.as_ref().clone()
                    );
                    // 尝试简化指数（仅对常量表达式）
                    let simplified_exp = if new_exp.is_constant() {
                        if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                            Expression::Number(value)
                        } else {
                            new_exp
                        }
                    } else {
                        new_exp
                    };
                    return Some(Expression::power(var.clone(), simplified_exp));
                }
            }
            
            // x^a / x = x^(a-1)
            (Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }, var) => {
                if base.as_ref() == var {
                    let new_exp = Expression::subtract(
                        exp.as_ref().clone(),
                        Expression::Number(Number::one())
                    );
                    // 尝试简化指数（仅对常量表达式）
                    let simplified_exp = if new_exp.is_constant() {
                        if let Ok(value) = self.evaluate_constant_expression(&new_exp) {
                            Expression::Number(value)
                        } else {
                            new_exp
                        }
                    } else {
                        new_exp
                    };
                    return Some(Expression::power(var.clone(), simplified_exp));
                }
            }
            
            _ => {}
        }
        None
    }
    
    /// 计算整数幂（仅对小指数）
    fn compute_integer_power(&self, base: &Number, exponent: &Number) -> Option<Number> {
        if let Some(exp_int) = exponent.to_integer() {
            // 只计算小指数的幂，避免计算爆炸
            if exp_int >= BigInt::from(0) && exp_int <= BigInt::from(10) {
                use num_traits::ToPrimitive;
                let exp_u32 = exp_int.to_u32()?;
                return Some(self.power_by_squaring(base, exp_u32));
            }
        }
        None
    }
    
    /// 快速幂算法
    fn power_by_squaring(&self, base: &Number, exponent: u32) -> Number {
        if exponent == 0 {
            return Number::one();
        }
        if exponent == 1 {
            return base.clone();
        }
        
        let half_power = self.power_by_squaring(base, exponent / 2);
        let result = half_power.clone() * half_power;
        
        if exponent % 2 == 0 {
            result
        } else {
            result * base.clone()
        }
    }
    
    /// 检查是否应该交换操作数以获得规范形式
    fn should_swap_for_canonical_form(&self, left: &Expression, right: &Expression) -> bool {
        match (left, right) {
            // 常数应该在变量前面
            (Expression::Variable(_), Expression::Number(_)) => true,
            (Expression::Variable(_), Expression::Constant(_)) => true,
            
            // 按字典序排列变量
            (Expression::Variable(a), Expression::Variable(b)) => a > b,
            
            _ => false,
        }
    }
}

impl Default for Simplifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "simplify_tests.rs"]
mod simplify_tests;