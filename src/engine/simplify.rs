//! # 表达式简化
//!
//! 实现代数表达式的简化规则和算法。

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
use crate::engine::error::ComputeError;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

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
                    UnaryOperator::Abs => Ok(operand_val.abs()?),
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
            
            // 简化矩阵表达式
            Expression::Matrix(rows) => {
                let mut simplified_rows = Vec::with_capacity(rows.len());
                for row in rows {
                    let mut simplified_row = Vec::with_capacity(row.len());
                    for elem in row {
                        let simplified_elem = self.simplify_recursive(elem)?;
                        simplified_row.push(simplified_elem);
                    }
                    simplified_rows.push(simplified_row);
                }
                Ok(Expression::Matrix(simplified_rows))
            }
            
            // 简化向量表达式
            Expression::Vector(elements) => {
                let mut simplified_elements = Vec::with_capacity(elements.len());
                for elem in elements {
                    let simplified_elem = self.simplify_recursive(elem)?;
                    simplified_elements.push(simplified_elem);
                }
                Ok(Expression::Vector(simplified_elements))
            }
            
            // 简化集合表达式
            Expression::Set(elements) => {
                let mut simplified_elements = Vec::with_capacity(elements.len());
                for elem in elements {
                    let simplified_elem = self.simplify_recursive(elem)?;
                    simplified_elements.push(simplified_elem);
                }
                Ok(Expression::Set(simplified_elements))
            }
            
            // 简化区间表达式
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                let simplified_start = self.simplify_recursive(start)?;
                let simplified_end = self.simplify_recursive(end)?;
                Ok(Expression::Interval {
                    start: Box::new(simplified_start),
                    end: Box::new(simplified_end),
                    start_inclusive: *start_inclusive,
                    end_inclusive: *end_inclusive,
                })
            }
        }
    }
    
    /// 简化二元运算
    fn simplify_binary_op(&mut self, op: &BinaryOperator, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        match op {
            BinaryOperator::Add => self.simplify_addition(left, right),
            BinaryOperator::Subtract => self.simplify_subtraction(left, right),
            BinaryOperator::Multiply => self.simplify_multiplication(left, right),
            BinaryOperator::Divide => self.simplify_division(left, right),
            BinaryOperator::Power => self.simplify_power(left, right),
            
            // 矩阵和向量运算的简化
            BinaryOperator::MatrixMultiply => self.simplify_matrix_multiply(left, right),
            BinaryOperator::DotProduct => self.simplify_dot_product(left, right),
            BinaryOperator::CrossProduct => self.simplify_cross_product(left, right),
            
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
    fn simplify_power(&mut self, base: &Expression, exponent: &Expression) -> Result<Expression, ComputeError> {
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
        
        // 规则：二项式展开 (a+b)^n 或 (a-b)^n，当 n 是小正整数时
        if let Some(expanded) = self.try_binomial_expansion(base, exponent)? {
            return Ok(expanded);
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
            
            // 矩阵专用运算符的简化
            UnaryOperator::Transpose => self.simplify_transpose(operand),
            UnaryOperator::Determinant => self.simplify_determinant(operand),
            UnaryOperator::Inverse => self.simplify_inverse(operand),
            UnaryOperator::Trace => self.simplify_trace(operand),
            
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
                Ok(Expression::Number(n.abs()?))
            }
            
            _ => Ok(Expression::abs(operand.clone())),
        }
    }
    
    /// 简化函数调用
    fn simplify_function(&self, name: &str, args: &[Expression]) -> Result<Expression, ComputeError> {
        // 如果参数都是常量，尝试计算函数值
        if args.iter().all(|arg| arg.is_constant()) {
            if let Ok(result) = self.evaluate_function(name, args) {
                return Ok(result);
            }
        }
        
        // 应用特定的函数简化规则
        match name {
            "ln" | "log" => self.simplify_logarithm(args),
            "sin" => self.simplify_sine(args),
            "cos" => self.simplify_cosine(args),
            "tan" => self.simplify_tangent(args),
            "exp" => self.simplify_exponential(args),
            "sqrt" => self.simplify_square_root(args),
            "abs" => self.simplify_absolute_value_function(args),
            _ => Ok(Expression::function(name, args.to_vec())),
        }
    }
    
    /// 计算函数值（当参数都是常量时）
    fn evaluate_function(&self, name: &str, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.is_empty() {
            return Err(ComputeError::domain_error("函数需要至少一个参数"));
        }
        
        match name {
            "ln" | "log" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("ln 函数需要恰好一个参数"));
                }
                self.evaluate_logarithm(&args[0])
            }
            "sin" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("sin 函数需要恰好一个参数"));
                }
                self.evaluate_sine(&args[0])
            }
            "cos" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("cos 函数需要恰好一个参数"));
                }
                self.evaluate_cosine(&args[0])
            }
            "tan" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("tan 函数需要恰好一个参数"));
                }
                self.evaluate_tangent(&args[0])
            }
            "exp" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("exp 函数需要恰好一个参数"));
                }
                self.evaluate_exponential(&args[0])
            }
            "sqrt" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("sqrt 函数需要恰好一个参数"));
                }
                self.evaluate_square_root(&args[0])
            }
            "abs" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("abs 函数需要恰好一个参数"));
                }
                self.evaluate_absolute_value(&args[0])
            }
            "factorial" => {
                if args.len() != 1 {
                    return Err(ComputeError::domain_error("factorial 函数需要恰好一个参数"));
                }
                self.evaluate_factorial(&args[0])
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("未知函数: {}", name) 
            }),
        }
    }
    
    /// 计算对数函数
    fn evaluate_logarithm(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // ln(e) = 1
            Expression::Constant(MathConstant::E) => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // ln(1) = 0
            Expression::Number(n) if n.is_one() => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // ln(e^x) = x
            Expression::Function { name, args } if name == "exp" && args.len() == 1 => {
                Ok(args[0].clone())
            }
            // 数值计算
            Expression::Number(n) => {
                if let Some(f) = n.to_f64() {
                    if f > 0.0 {
                        Ok(Expression::Number(Number::Float(f.ln())))
                    } else {
                        Err(ComputeError::domain_error("对数的参数必须为正数"))
                    }
                } else {
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "复数对数计算".to_string() 
                    })
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此对数表达式".to_string() 
            }),
        }
    }
    
    /// 计算正弦函数
    fn evaluate_sine(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // sin(0) = 0
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // sin(π/2) = 1
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n.is_two()) => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // sin(π/4) = √2/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(4)) => {
                Ok(Expression::divide(
                    Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]),
                    Expression::Number(Number::integer(2))
                ))
            }
            // sin(π/6) = 1/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(6)) => {
                Ok(Expression::Number(Number::rational(1, 2)))
            }
            // sin(π/3) = √3/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(3)) => {
                Ok(Expression::divide(
                    Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]),
                    Expression::Number(Number::integer(2))
                ))
            }
            // sin(π) = 0
            Expression::Constant(MathConstant::Pi) => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // 数值计算
            Expression::Number(n) => {
                if let Some(f) = n.to_f64() {
                    Ok(Expression::Number(Number::Float(f.sin())))
                } else {
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "复数正弦计算".to_string() 
                    })
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此正弦表达式".to_string() 
            }),
        }
    }
    
    /// 计算余弦函数
    fn evaluate_cosine(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // cos(0) = 1
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // cos(π/2) = 0
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n.is_two()) => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // cos(π/4) = √2/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(4)) => {
                Ok(Expression::divide(
                    Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]),
                    Expression::Number(Number::integer(2))
                ))
            }
            // cos(π/6) = √3/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(6)) => {
                Ok(Expression::divide(
                    Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]),
                    Expression::Number(Number::integer(2))
                ))
            }
            // cos(π/3) = 1/2
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(3)) => {
                Ok(Expression::Number(Number::rational(1, 2)))
            }
            // cos(π) = -1
            Expression::Constant(MathConstant::Pi) => {
                Ok(Expression::Number(Number::integer(-1)))
            }
            // 数值计算
            Expression::Number(n) => {
                if let Some(f) = n.to_f64() {
                    Ok(Expression::Number(Number::Float(f.cos())))
                } else {
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "复数余弦计算".to_string() 
                    })
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此余弦表达式".to_string() 
            }),
        }
    }
    
    /// 计算正切函数
    fn evaluate_tangent(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // tan(0) = 0
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // tan(π/4) = 1
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(4)) => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // tan(π/6) = √3/3
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(6)) => {
                Ok(Expression::divide(
                    Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]),
                    Expression::Number(Number::integer(3))
                ))
            }
            // tan(π/3) = √3
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } 
                if matches!(left.as_ref(), Expression::Constant(MathConstant::Pi)) 
                && matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(3)) => {
                Ok(Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]))
            }
            // 数值计算
            Expression::Number(n) => {
                if let Some(f) = n.to_f64() {
                    Ok(Expression::Number(Number::Float(f.tan())))
                } else {
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "复数正切计算".to_string() 
                    })
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此正切表达式".to_string() 
            }),
        }
    }
    
    /// 计算指数函数
    fn evaluate_exponential(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // exp(0) = 1
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // exp(1) = e
            Expression::Number(n) if n.is_one() => {
                Ok(Expression::Constant(MathConstant::E))
            }
            // exp(ln(x)) = x
            Expression::Function { name, args } if name == "ln" && args.len() == 1 => {
                Ok(args[0].clone())
            }
            // 数值计算
            Expression::Number(n) => {
                if let Some(f) = n.to_f64() {
                    Ok(Expression::Number(Number::Float(f.exp())))
                } else {
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "复数指数计算".to_string() 
                    })
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此指数表达式".to_string() 
            }),
        }
    }
    
    /// 计算平方根函数
    fn evaluate_square_root(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            // sqrt(0) = 0
            Expression::Number(n) if n.is_zero() => {
                Ok(Expression::Number(Number::integer(0)))
            }
            // sqrt(1) = 1
            Expression::Number(n) if n.is_one() => {
                Ok(Expression::Number(Number::integer(1)))
            }
            // 对于整数，检查是否是完全平方数
            Expression::Number(Number::Integer(i)) => {
                if i < &BigInt::from(0) {
                    return Err(ComputeError::domain_error("平方根的参数不能为负数"));
                }
                
                // 检查是否是完全平方数
                if let Some(sqrt_int) = self.integer_sqrt(i) {
                    Ok(Expression::Number(Number::Integer(sqrt_int)))
                } else {
                    // 不是完全平方数，保持符号形式
                    Ok(Expression::function("sqrt", vec![arg.clone()]))
                }
            }
            // 对于有理数，尝试简化
            Expression::Number(Number::Rational(r)) => {
                if r < &BigRational::from(BigInt::from(0)) {
                    return Err(ComputeError::domain_error("平方根的参数不能为负数"));
                }
                
                // 分别计算分子和分母的平方根
                let numer_sqrt = self.integer_sqrt(r.numer());
                let denom_sqrt = self.integer_sqrt(r.denom());
                
                match (numer_sqrt, denom_sqrt) {
                    (Some(n), Some(d)) => {
                        // 分子和分母都是完全平方数
                        Ok(Expression::Number(Number::Rational(BigRational::new(n, d))))
                    }
                    _ => {
                        // 不是完全平方数，保持符号形式
                        Ok(Expression::function("sqrt", vec![arg.clone()]))
                    }
                }
            }
            // 对于其他数值类型，只有在明确要求数值近似时才转换为浮点数
            Expression::Number(n) => {
                // 保持符号形式，避免精度损失
                Ok(Expression::function("sqrt", vec![arg.clone()]))
            }
            // 对于非数值表达式，保持符号形式
            _ => {
                Ok(Expression::function("sqrt", vec![arg.clone()]))
            }
        }
    }
    
    /// 计算绝对值函数
    fn evaluate_absolute_value(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            Expression::Number(n) => {
                Ok(Expression::Number(n.abs()?))
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此绝对值表达式".to_string() 
            }),
        }
    }
    
    /// 计算阶乘函数
    fn evaluate_factorial(&self, arg: &Expression) -> Result<Expression, ComputeError> {
        match arg {
            Expression::Number(n) => {
                if let Some(i) = n.to_i64() {
                    if i >= 0 && i <= 20 { // 限制在合理范围内
                        let mut result = 1i64;
                        for j in 1..=i {
                            result *= j;
                        }
                        Ok(Expression::Number(Number::integer(result)))
                    } else if i < 0 {
                        Err(ComputeError::domain_error("阶乘的参数不能为负数"))
                    } else {
                        Err(ComputeError::domain_error("阶乘的参数过大"))
                    }
                } else {
                    Err(ComputeError::domain_error("阶乘的参数必须为整数"))
                }
            }
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: "无法计算此阶乘表达式".to_string() 
            }),
        }
    }
    
    /// 简化对数函数（非常量参数）
    fn simplify_logarithm(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("ln", args.to_vec()));
        }
        
        match &args[0] {
            // ln(e^x) = x
            Expression::Function { name, args: inner_args } if name == "exp" && inner_args.len() == 1 => {
                Ok(inner_args[0].clone())
            }
            _ => Ok(Expression::function("ln", args.to_vec())),
        }
    }
    
    /// 简化正弦函数（非常量参数）
    fn simplify_sine(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("sin", args.to_vec()));
        }
        
        // 这里可以添加更多的三角恒等式简化
        Ok(Expression::function("sin", args.to_vec()))
    }
    
    /// 简化余弦函数（非常量参数）
    fn simplify_cosine(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("cos", args.to_vec()));
        }
        
        // 这里可以添加更多的三角恒等式简化
        Ok(Expression::function("cos", args.to_vec()))
    }
    
    /// 简化正切函数（非常量参数）
    fn simplify_tangent(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("tan", args.to_vec()));
        }
        
        // 这里可以添加更多的三角恒等式简化
        Ok(Expression::function("tan", args.to_vec()))
    }
    
    /// 简化指数函数（非常量参数）
    fn simplify_exponential(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("exp", args.to_vec()));
        }
        
        match &args[0] {
            // exp(ln(x)) = x
            Expression::Function { name, args: inner_args } if name == "ln" && inner_args.len() == 1 => {
                Ok(inner_args[0].clone())
            }
            _ => Ok(Expression::function("exp", args.to_vec())),
        }
    }
    
    /// 简化平方根函数（非常量参数）
    fn simplify_square_root(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("sqrt", args.to_vec()));
        }
        
        match &args[0] {
            // sqrt(x^2) = |x|
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } 
                if matches!(right.as_ref(), Expression::Number(n) if n.is_two()) => {
                Ok(Expression::function("abs", vec![left.as_ref().clone()]))
            }
            _ => Ok(Expression::function("sqrt", args.to_vec())),
        }
    }
    
    /// 简化绝对值函数（非常量参数）
    fn simplify_absolute_value_function(&self, args: &[Expression]) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Ok(Expression::function("abs", args.to_vec()));
        }
        
        match &args[0] {
            // abs(abs(x)) = abs(x)
            Expression::Function { name, args: inner_args } if name == "abs" && inner_args.len() == 1 => {
                Ok(Expression::function("abs", inner_args.clone()))
            }
            // abs(-x) = abs(x)
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                Ok(Expression::function("abs", vec![operand.as_ref().clone()]))
            }
            _ => Ok(Expression::function("abs", args.to_vec())),
        }
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
    
    /// 尝试二项式展开
    fn try_binomial_expansion(&mut self, base: &Expression, exponent: &Expression) -> Result<Option<Expression>, ComputeError> {
        // 只处理小正整数指数的情况
        if let Expression::Number(Number::Integer(n)) = exponent {
            if let Some(exp_val) = n.to_u32() {
                if exp_val <= 6 && exp_val >= 2 { // 限制在 2 到 6 次幂
                    return self.expand_binomial_power(base, exp_val);
                }
            }
        }
        Ok(None)
    }
    
    /// 展开二项式幂
    fn expand_binomial_power(&mut self, base: &Expression, n: u32) -> Result<Option<Expression>, ComputeError> {
        match base {
            // (a + b)^n 的展开
            Expression::BinaryOp { op: BinaryOperator::Add, left: a, right: b } => {
                Ok(Some(self.binomial_expansion(a, b, n, true)?))
            }
            // (a - b)^n 的展开
            Expression::BinaryOp { op: BinaryOperator::Subtract, left: a, right: b } => {
                Ok(Some(self.binomial_expansion(a, b, n, false)?))
            }
            _ => Ok(None),
        }
    }
    
    /// 二项式定理展开：(a ± b)^n
    fn binomial_expansion(&mut self, a: &Expression, b: &Expression, n: u32, is_add: bool) -> Result<Expression, ComputeError> {
        let mut terms = Vec::new();
        
        for k in 0..=n {
            // 计算二项式系数 C(n, k)
            let coeff = self.binomial_coefficient(n, k);
            
            // 计算 a^(n-k)
            let a_power = if n - k == 0 {
                Expression::Number(Number::one())
            } else if n - k == 1 {
                a.clone()
            } else {
                Expression::power(a.clone(), Expression::Number(Number::integer((n - k) as i64)))
            };
            
            // 计算 b^k
            let b_power = if k == 0 {
                Expression::Number(Number::one())
            } else if k == 1 {
                b.clone()
            } else {
                Expression::power(b.clone(), Expression::Number(Number::integer(k as i64)))
            };
            
            // 计算符号：对于 (a - b)^n，奇数项 k 为负
            let sign = if is_add || k % 2 == 0 { 1 } else { -1 };
            
            // 构造项：coeff * sign * a^(n-k) * b^k
            let mut term = Expression::Number(Number::integer(coeff as i64 * sign));
            
            // 乘以 a^(n-k)
            if !matches!(a_power, Expression::Number(Number::Integer(ref i)) if i == &BigInt::from(1)) {
                term = Expression::multiply(term, a_power);
            }
            
            // 乘以 b^k
            if !matches!(b_power, Expression::Number(Number::Integer(ref i)) if i == &BigInt::from(1)) {
                term = Expression::multiply(term, b_power);
            }
            
            terms.push(term);
        }
        
        // 将所有项相加
        let mut result = terms[0].clone();
        for term in terms.into_iter().skip(1) {
            result = Expression::add(result, term);
        }
        
        // 简化结果
        self.simplify_recursive(&result)
    }
    
    /// 计算二项式系数 C(n, k)
    fn binomial_coefficient(&self, n: u32, k: u32) -> u64 {
        if k > n {
            return 0;
        }
        if k == 0 || k == n {
            return 1;
        }
        
        let k = k.min(n - k); // 利用对称性
        let mut result = 1u64;
        
        for i in 0..k {
            result = result * (n - i) as u64 / (i + 1) as u64;
        }
        
        result
    }
    
    /// 计算整数的平方根（如果是完全平方数）
    fn integer_sqrt(&self, n: &BigInt) -> Option<BigInt> {
        use num_traits::ToPrimitive;
        
        if n < &BigInt::from(0) {
            return None;
        }
        
        if n == &BigInt::from(0) || n == &BigInt::from(1) {
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
        
        // 对于大数值，使用牛顿法
        let mut x = n.clone();
        let mut prev_x = BigInt::from(0);
        
        // 牛顿法迭代：x_{n+1} = (x_n + n/x_n) / 2
        while &x != &prev_x {
            prev_x = x.clone();
            x = (&x + n / &x) / BigInt::from(2);
        }
        
        // 检查结果是否精确
        if &(&x * &x) == n {
            Some(x)
        } else {
            None
        }
    }
    
    // 矩阵和向量运算的简化方法
    
    /// 简化矩阵乘法
    fn simplify_matrix_multiply(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 基本简化规则
        match (left, right) {
            // 零矩阵乘法：0 * A = 0
            (Expression::Matrix(left_rows), _) if self.is_zero_matrix(left_rows) => {
                Ok(left.clone())
            }
            (_, Expression::Matrix(right_rows)) if self.is_zero_matrix(right_rows) => {
                Ok(right.clone())
            }
            
            // 单位矩阵乘法：I * A = A, A * I = A
            (Expression::Matrix(left_rows), _) if self.is_identity_matrix(left_rows) => {
                Ok(right.clone())
            }
            (_, Expression::Matrix(right_rows)) if self.is_identity_matrix(right_rows) => {
                Ok(left.clone())
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::binary_op(BinaryOperator::MatrixMultiply, left.clone(), right.clone()))
        }
    }
    
    /// 简化向量点积
    fn simplify_dot_product(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 基本简化规则
        match (left, right) {
            // 零向量点积：0 · v = 0
            (Expression::Vector(left_elems), _) if self.is_zero_vector(left_elems) => {
                Ok(Expression::number(Number::from(0)))
            }
            (_, Expression::Vector(right_elems)) if self.is_zero_vector(right_elems) => {
                Ok(Expression::number(Number::from(0)))
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::binary_op(BinaryOperator::DotProduct, left.clone(), right.clone()))
        }
    }
    
    /// 简化向量叉积
    fn simplify_cross_product(&self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 基本简化规则
        match (left, right) {
            // 零向量叉积：0 × v = 0
            (Expression::Vector(left_elems), _) if self.is_zero_vector(left_elems) => {
                Ok(left.clone())
            }
            (_, Expression::Vector(right_elems)) if self.is_zero_vector(right_elems) => {
                Ok(right.clone())
            }
            
            // 向量与自身叉积：v × v = 0
            _ if left == right => {
                if let Expression::Vector(elems) = left {
                    let zero_vector = vec![Expression::number(Number::from(0)); elems.len()];
                    Ok(Expression::Vector(zero_vector))
                } else {
                    Ok(Expression::binary_op(BinaryOperator::CrossProduct, left.clone(), right.clone()))
                }
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::binary_op(BinaryOperator::CrossProduct, left.clone(), right.clone()))
        }
    }
    
    /// 简化矩阵转置
    fn simplify_transpose(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 转置的转置：(A^T)^T = A
            Expression::UnaryOp { op: UnaryOperator::Transpose, operand: inner } => {
                Ok(inner.as_ref().clone())
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::unary_op(UnaryOperator::Transpose, operand.clone()))
        }
    }
    
    /// 简化矩阵行列式
    fn simplify_determinant(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 单位矩阵的行列式：det(I) = 1
            Expression::Matrix(rows) if self.is_identity_matrix(rows) => {
                Ok(Expression::number(Number::from(1)))
            }
            
            // 零矩阵的行列式：det(0) = 0
            Expression::Matrix(rows) if self.is_zero_matrix(rows) => {
                Ok(Expression::number(Number::from(0)))
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::unary_op(UnaryOperator::Determinant, operand.clone()))
        }
    }
    
    /// 简化矩阵逆
    fn simplify_inverse(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 单位矩阵的逆：I^(-1) = I
            Expression::Matrix(rows) if self.is_identity_matrix(rows) => {
                Ok(operand.clone())
            }
            
            // 逆的逆：(A^(-1))^(-1) = A
            Expression::UnaryOp { op: UnaryOperator::Inverse, operand: inner } => {
                Ok(inner.as_ref().clone())
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::unary_op(UnaryOperator::Inverse, operand.clone()))
        }
    }
    
    /// 简化矩阵的迹
    fn simplify_trace(&self, operand: &Expression) -> Result<Expression, ComputeError> {
        match operand {
            // 零矩阵的迹：tr(0) = 0
            Expression::Matrix(rows) if self.is_zero_matrix(rows) => {
                Ok(Expression::number(Number::from(0)))
            }
            
            // n×n单位矩阵的迹：tr(I_n) = n
            Expression::Matrix(rows) if self.is_identity_matrix(rows) => {
                Ok(Expression::number(Number::from(rows.len() as i64)))
            }
            
            // 其他情况保持原样
            _ => Ok(Expression::unary_op(UnaryOperator::Trace, operand.clone()))
        }
    }
    
    /// 检查是否为零矩阵
    fn is_zero_matrix(&self, rows: &[Vec<Expression>]) -> bool {
        rows.iter().all(|row| {
            row.iter().all(|elem| self.is_zero(elem))
        })
    }
    
    /// 检查是否为单位矩阵
    fn is_identity_matrix(&self, rows: &[Vec<Expression>]) -> bool {
        if rows.is_empty() {
            return false;
        }
        
        let n = rows.len();
        if rows.iter().any(|row| row.len() != n) {
            return false; // 不是方阵
        }
        
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    // 对角线元素应该是1
                    if !self.is_one(&rows[i][j]) {
                        return false;
                    }
                } else {
                    // 非对角线元素应该是0
                    if !self.is_zero(&rows[i][j]) {
                        return false;
                    }
                }
            }
        }
        
        true
    }
    
    /// 检查是否为零向量
    fn is_zero_vector(&self, elements: &[Expression]) -> bool {
        elements.iter().all(|elem| self.is_zero(elem))
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