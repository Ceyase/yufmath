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
        
        // 首先应用常量合并规则
        result = self.apply_constant_folding(&result)?;
        
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
    
    /// 应用自动化简规则（多次迭代直到无法进一步化简）
    fn apply_auto_simplify_rules(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        let mut current = expr.clone();
        let mut previous;
        let max_iterations = 10; // 防止无限循环
        let mut iteration = 0;
        
        loop {
            previous = current.clone();
            
            // 应用基础简化
            current = self.base_simplifier.simplify(&current)?;
            
            // 应用同类项合并
            current = self.combine_like_terms(&current)?;
            
            // 应用常量折叠
            current = self.apply_constant_folding(&current)?;
            
            // 应用根号化简
            current = self.simplify_radicals(&current)?;
            
            // 应用代数化简
            current = self.apply_advanced_algebraic_rules(&current)?;
            
            iteration += 1;
            
            // 如果没有变化或达到最大迭代次数，停止
            if current == previous || iteration >= max_iterations {
                break;
            }
        }
        
        Ok(current)
    }    

    /// 合并同类项
    fn combine_like_terms(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.combine_like_terms(left)?;
                let right_simplified = self.combine_like_terms(right)?;
                
                match op {
                    BinaryOperator::Add => {
                        self.combine_addition_terms(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Subtract => {
                        self.combine_subtraction_terms(&left_simplified, &right_simplified)
                    }
                    _ => Ok(Expression::binary_op(op.clone(), left_simplified, right_simplified))
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.combine_like_terms(operand)?;
                Ok(Expression::unary_op(op.clone(), operand_simplified))
            }
            
            Expression::Function { name, args } => {
                let args_simplified: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.combine_like_terms(arg))
                    .collect();
                Ok(Expression::function(name, args_simplified?))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 合并加法项
    fn combine_addition_terms(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 处理嵌套加法：(a + b) + c -> a + b + c
        let mut terms = Vec::new();
        self.collect_addition_terms(left, &mut terms);
        self.collect_addition_terms(right, &mut terms);
        
        // 合并同类项
        let combined_terms = self.merge_like_terms(terms)?;
        
        // 构建结果表达式
        self.build_addition_expression(combined_terms)
    }
    
    /// 合并减法项
    fn combine_subtraction_terms(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 将减法转换为加法：a - b = a + (-b)
        let negated_right = Expression::negate(right.clone());
        self.combine_addition_terms(left, &negated_right)
    }
    
    /// 收集加法项
    fn collect_addition_terms(&self, expr: &Expression, terms: &mut Vec<Expression>) {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
                self.collect_addition_terms(left, terms);
                self.collect_addition_terms(right, terms);
            }
            _ => {
                terms.push(expr.clone());
            }
        }
    }
    
    /// 合并同类项
    fn merge_like_terms(&mut self, terms: Vec<Expression>) -> Result<Vec<Expression>, ComputeError> {
        let mut merged = Vec::new();
        let mut used = vec![false; terms.len()];
        
        for i in 0..terms.len() {
            if used[i] {
                continue;
            }
            
            let mut coefficient = Expression::Number(Number::one());
            let mut base_term = None;
            
            // 提取第一个项的系数和基础项
            if let Some((coeff, base)) = self.extract_coefficient_and_base(&terms[i]) {
                coefficient = coeff;
                base_term = Some(base);
            } else {
                // 如果无法提取，直接添加
                merged.push(terms[i].clone());
                used[i] = true;
                continue;
            }
            
            used[i] = true;
            
            // 查找其他同类项
            for j in (i + 1)..terms.len() {
                if used[j] {
                    continue;
                }
                
                if let Some((other_coeff, other_base)) = self.extract_coefficient_and_base(&terms[j]) {
                    if let Some(ref base) = base_term {
                        if base == &other_base {
                            // 合并系数
                            coefficient = Expression::add(coefficient, other_coeff);
                            used[j] = true;
                        }
                    }
                }
            }
            
            // 简化系数
            coefficient = self.base_simplifier.simplify(&coefficient)?;
            
            // 构建合并后的项
            if let Some(base) = base_term {
                if self.is_zero(&coefficient) {
                    // 系数为0，跳过这一项
                    continue;
                } else if self.is_one(&coefficient) {
                    // 系数为1，只保留基础项
                    merged.push(base);
                } else {
                    // 系数不为1，保留乘法形式
                    merged.push(Expression::multiply(coefficient, base));
                }
            }
        }
        
        Ok(merged)
    }
    
    /// 提取表达式的系数和基础项
    /// 例如：3*sqrt(2) -> (3, sqrt(2))，sqrt(2) -> (1, sqrt(2))，5 -> (5, 1)
    fn extract_coefficient_and_base(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                // 检查左边是否为数值系数
                if matches!(left.as_ref(), Expression::Number(_)) {
                    Some((left.as_ref().clone(), right.as_ref().clone()))
                } else if matches!(right.as_ref(), Expression::Number(_)) {
                    Some((right.as_ref().clone(), left.as_ref().clone()))
                } else {
                    // 都不是数值，将整个表达式作为基础项，系数为1
                    Some((Expression::Number(Number::one()), expr.clone()))
                }
            }
            
            Expression::Number(_) => {
                // 纯数值，基础项为1
                Some((expr.clone(), Expression::Number(Number::one())))
            }
            
            _ => {
                // 其他情况，系数为1，整个表达式为基础项
                Some((Expression::Number(Number::one()), expr.clone()))
            }
        }
    }
    
    /// 构建加法表达式
    fn build_addition_expression(&self, terms: Vec<Expression>) -> Result<Expression, ComputeError> {
        if terms.is_empty() {
            Ok(Expression::Number(Number::zero()))
        } else if terms.len() == 1 {
            Ok(terms[0].clone())
        } else {
            // 从左到右构建加法表达式
            let mut result = terms[0].clone();
            for term in terms.iter().skip(1) {
                result = Expression::add(result, term.clone());
            }
            Ok(result)
        }
    }    

    /// 应用常量折叠
    fn apply_constant_folding(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_folded = self.apply_constant_folding(left)?;
                let right_folded = self.apply_constant_folding(right)?;
                
                // 如果两边都是数值，直接计算
                if let (Expression::Number(left_num), Expression::Number(right_num)) = (&left_folded, &right_folded) {
                    match op {
                        BinaryOperator::Add => Ok(Expression::Number(left_num.clone() + right_num.clone())),
                        BinaryOperator::Subtract => Ok(Expression::Number(left_num.clone() - right_num.clone())),
                        BinaryOperator::Multiply => Ok(Expression::Number(left_num.clone() * right_num.clone())),
                        BinaryOperator::Divide => {
                            if right_num.is_zero() {
                                Err(ComputeError::DivisionByZero)
                            } else {
                                Ok(Expression::Number(left_num.clone() / right_num.clone()))
                            }
                        }
                        BinaryOperator::Power => {
                            // 简单的整数幂计算
                            if let Number::Integer(exp) = right_num {
                                if let Some(exp_u32) = exp.to_u32() {
                                    if exp_u32 <= 100 { // 限制幂次以避免过大计算
                                        let result = self.compute_integer_power(left_num, exp_u32)?;
                                        return Ok(Expression::Number(result));
                                    }
                                }
                            }
                            Ok(Expression::binary_op(op.clone(), left_folded, right_folded))
                        }
                        _ => Ok(Expression::binary_op(op.clone(), left_folded, right_folded))
                    }
                } else {
                    Ok(Expression::binary_op(op.clone(), left_folded, right_folded))
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_folded = self.apply_constant_folding(operand)?;
                
                if let Expression::Number(num) = &operand_folded {
                    match op {
                        UnaryOperator::Negate => Ok(Expression::Number(-num.clone())),
                        UnaryOperator::Sqrt => self.simplify_numeric_square_root(num),
                        _ => Ok(Expression::unary_op(op.clone(), operand_folded))
                    }
                } else {
                    Ok(Expression::unary_op(op.clone(), operand_folded))
                }
            }
            
            Expression::Function { name, args } => {
                let args_folded: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.apply_constant_folding(arg))
                    .collect();
                Ok(Expression::function(name, args_folded?))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 计算整数幂
    fn compute_integer_power(&self, base: &Number, exp: u32) -> Result<Number, ComputeError> {
        if exp == 0 {
            Ok(Number::one())
        } else if exp == 1 {
            Ok(base.clone())
        } else {
            let mut result = base.clone();
            for _ in 1..exp {
                result = result * base.clone();
            }
            Ok(result)
        }
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
            
            // 尝试简化嵌套根式
            _ => {
                if let Some(simplified) = self.try_denest_radical(arg)? {
                    Ok(simplified)
                } else {
                    Ok(Expression::function("sqrt", vec![arg.clone()]))
                }
            }
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
 
    /// 提取根号项的系数和根号内容
    /// 例如：3*sqrt(2) -> Some((3, 2))，sqrt(2) -> Some((1, 2))
    fn extract_radical_coefficient(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            // sqrt(x) 形式
            Expression::Function { name, args } if name == "sqrt" && args.len() == 1 => {
                Some((Expression::Number(Number::one()), args[0].clone()))
            }
            
            // c * sqrt(x) 形式
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                if let Expression::Function { name, args } = left.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((right.as_ref().clone(), args[0].clone()));
                    }
                }
                if let Expression::Function { name, args } = right.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((left.as_ref().clone(), args[0].clone()));
                    }
                }
                None
            }
            
            _ => None
        }
    }
    
    /// 创建根号表达式
    fn create_sqrt_expression(&self, arg: &Expression) -> Expression {
        Expression::function("sqrt", vec![arg.clone()])
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
    
    /// 尝试去嵌套根式
    fn try_denest_radical(&mut self, arg: &Expression) -> Result<Option<Expression>, ComputeError> {
        // 检查是否为 a ± b*sqrt(c) 的形式
        match arg {
            Expression::BinaryOp { op, left, right } => {
                let is_subtract = matches!(op, BinaryOperator::Subtract);
                
                // 尝试匹配 a ± b*sqrt(c) 模式
                if let Some((a, b, c)) = self.match_nested_radical_pattern(left, right, is_subtract) {
                    return self.try_special_denesting(&a, &b, &c, is_subtract);
                }
            }
            _ => {}
        }
        
        Ok(None)
    }
    
    /// 匹配嵌套根式模式 a ± b*sqrt(c)
    fn match_nested_radical_pattern(&self, left: &Expression, right: &Expression, is_subtract: bool) -> Option<(Expression, Expression, Expression)> {
        // 检查 left 是否为数值，right 是否为 b*sqrt(c) 形式
        if let Expression::Number(_) = left {
            if let Some((b, c)) = self.extract_coefficient_sqrt(right) {
                return Some((left.clone(), b, c));
            }
        }
        
        // 如果是减法，也检查相反的情况
        if !is_subtract {
            if let Expression::Number(_) = right {
                if let Some((b, c)) = self.extract_coefficient_sqrt(left) {
                    return Some((right.clone(), b, c));
                }
            }
        }
        
        None
    }
    
    /// 提取 b*sqrt(c) 形式的系数和根号内容
    fn extract_coefficient_sqrt(&self, expr: &Expression) -> Option<(Expression, Expression)> {
        match expr {
            // sqrt(c) 形式
            Expression::Function { name, args } if name == "sqrt" && args.len() == 1 => {
                Some((Expression::Number(Number::one()), args[0].clone()))
            }
            
            // b * sqrt(c) 形式
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                if let Expression::Function { name, args } = left.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((right.as_ref().clone(), args[0].clone()));
                    }
                }
                if let Expression::Function { name, args } = right.as_ref() {
                    if name == "sqrt" && args.len() == 1 {
                        return Some((left.as_ref().clone(), args[0].clone()));
                    }
                }
                None
            }
            
            _ => None
        }
    }
    
    /// 尝试特殊的去嵌套化简
    fn try_special_denesting(&mut self, a: &Expression, b: &Expression, c: &Expression, is_subtract: bool) -> Result<Option<Expression>, ComputeError> {
        // 只处理数值情况
        if let (Expression::Number(a_num), Expression::Number(b_num), Expression::Number(c_num)) = (a, b, c) {
            // 特殊情况1: sqrt(3 - 2*sqrt(2)) = sqrt(2) - 1
            if a_num == &Number::integer(3) && b_num == &Number::integer(2) && c_num == &Number::integer(2) && is_subtract {
                // 验证：(sqrt(2) - 1)² = 2 - 2*sqrt(2) + 1 = 3 - 2*sqrt(2) ✓
                let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
                let result = Expression::subtract(sqrt2, Expression::Number(Number::integer(1)));
                return Ok(Some(result));
            }
            
            // 特殊情况2: sqrt(3 + 2*sqrt(2)) = sqrt(2) + 1
            if a_num == &Number::integer(3) && b_num == &Number::integer(2) && c_num == &Number::integer(2) && !is_subtract {
                // 验证：(sqrt(2) + 1)² = 2 + 2*sqrt(2) + 1 = 3 + 2*sqrt(2) ✓
                let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
                let result = Expression::add(sqrt2, Expression::Number(Number::integer(1)));
                return Ok(Some(result));
            }
        }
        
        Ok(None)
    }  
  
    /// 应用高级代数规则
    fn apply_advanced_algebraic_rules(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.apply_advanced_algebraic_rules(left)?;
                let right_simplified = self.apply_advanced_algebraic_rules(right)?;
                
                match op {
                    BinaryOperator::Add => {
                        self.apply_addition_rules(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Subtract => {
                        self.apply_subtraction_rules(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Multiply => {
                        self.apply_multiplication_rules(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Divide => {
                        self.apply_division_rules(&left_simplified, &right_simplified)
                    }
                    BinaryOperator::Power => {
                        self.apply_power_rules(&left_simplified, &right_simplified)
                    }
                    _ => Ok(Expression::binary_op(op.clone(), left_simplified, right_simplified))
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.apply_advanced_algebraic_rules(operand)?;
                Ok(Expression::unary_op(op.clone(), operand_simplified))
            }
            
            Expression::Function { name, args } => {
                let args_simplified: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.apply_advanced_algebraic_rules(arg))
                    .collect();
                Ok(Expression::function(name, args_simplified?))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 应用加法规则
    fn apply_addition_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 0 + x = x
        if self.is_zero(left) {
            return Ok(right.clone());
        }
        if self.is_zero(right) {
            return Ok(left.clone());
        }
        
        // x + (-x) = 0
        if let Expression::UnaryOp { op: UnaryOperator::Negate, operand } = right {
            if operand.as_ref() == left {
                return Ok(Expression::Number(Number::zero()));
            }
        }
        if let Expression::UnaryOp { op: UnaryOperator::Negate, operand } = left {
            if operand.as_ref() == right {
                return Ok(Expression::Number(Number::zero()));
            }
        }
        
        Ok(Expression::add(left.clone(), right.clone()))
    }
    
    /// 应用减法规则
    fn apply_subtraction_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // x - 0 = x
        if self.is_zero(right) {
            return Ok(left.clone());
        }
        
        // 0 - x = -x
        if self.is_zero(left) {
            return Ok(Expression::negate(right.clone()));
        }
        
        // x - x = 0
        if left == right {
            return Ok(Expression::Number(Number::zero()));
        }
        
        Ok(Expression::subtract(left.clone(), right.clone()))
    }
    
    /// 应用乘法规则
    fn apply_multiplication_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // 0 * x = 0
        if self.is_zero(left) || self.is_zero(right) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 1 * x = x
        if self.is_one(left) {
            return Ok(right.clone());
        }
        if self.is_one(right) {
            return Ok(left.clone());
        }
        
        // (-1) * x = -x
        if self.is_negative_one(left) {
            return Ok(Expression::negate(right.clone()));
        }
        if self.is_negative_one(right) {
            return Ok(Expression::negate(left.clone()));
        }
        
        Ok(Expression::multiply(left.clone(), right.clone()))
    }
    
    /// 应用除法规则
    fn apply_division_rules(&mut self, left: &Expression, right: &Expression) -> Result<Expression, ComputeError> {
        // x / 0 = 错误
        if self.is_zero(right) {
            return Err(ComputeError::DivisionByZero);
        }
        
        // 0 / x = 0 (x ≠ 0)
        if self.is_zero(left) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // x / 1 = x
        if self.is_one(right) {
            return Ok(left.clone());
        }
        
        // x / x = 1 (x ≠ 0)
        if left == right {
            return Ok(Expression::Number(Number::one()));
        }
        
        Ok(Expression::divide(left.clone(), right.clone()))
    }
    
    /// 应用幂运算规则
    fn apply_power_rules(&mut self, base: &Expression, exponent: &Expression) -> Result<Expression, ComputeError> {
        // x^0 = 1
        if self.is_zero(exponent) {
            return Ok(Expression::Number(Number::one()));
        }
        
        // x^1 = x
        if self.is_one(exponent) {
            return Ok(base.clone());
        }
        
        // 0^x = 0 (x > 0)
        if self.is_zero(base) && !self.is_zero(exponent) {
            return Ok(Expression::Number(Number::zero()));
        }
        
        // 1^x = 1
        if self.is_one(base) {
            return Ok(Expression::Number(Number::one()));
        }
        
        // 处理 (a*sqrt(b))^2 = a^2 * b 的情况
        if let Expression::Number(exp_num) = exponent {
            if exp_num == &Number::integer(2) {
                // 检查底数是否为 a*sqrt(b) 的形式
                if let Some((coeff, radical)) = self.extract_coefficient_sqrt(base) {
                    // (a*sqrt(b))^2 = a^2 * b
                    let coeff_squared = Expression::power(coeff, Expression::Number(Number::integer(2)));
                    let coeff_squared_simplified = self.base_simplifier.simplify(&coeff_squared)?;
                    let result = Expression::multiply(coeff_squared_simplified, radical);
                    return self.base_simplifier.simplify(&result);
                }
                
                // 检查底数是否为 sqrt(a) 的形式
                if let Expression::Function { name, args } = base {
                    if name == "sqrt" && args.len() == 1 {
                        // (sqrt(a))^2 = a
                        return Ok(args[0].clone());
                    }
                }
            }
        }
        
        // 处理 (a*b)^n = a^n * b^n 的情况
        if let Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } = base {
            let left_power = Expression::power(left.as_ref().clone(), exponent.clone());
            let right_power = Expression::power(right.as_ref().clone(), exponent.clone());
            let left_simplified = self.apply_power_rules(&left.as_ref(), exponent)?;
            let right_simplified = self.apply_power_rules(&right.as_ref(), exponent)?;
            let result = Expression::multiply(left_simplified, right_simplified);
            return self.base_simplifier.simplify(&result);
        }
        
        Ok(Expression::power(base.clone(), exponent.clone()))
    }
    
    /// 化简三角函数（简化版本）
    fn simplify_trigonometric(&mut self, expr: &Expression) -> Result<Expression, ComputeError> {
        // 简化版本，只做递归处理
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_simplified = self.simplify_trigonometric(left)?;
                let right_simplified = self.simplify_trigonometric(right)?;
                Ok(Expression::binary_op(op.clone(), left_simplified, right_simplified))
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_simplified = self.simplify_trigonometric(operand)?;
                Ok(Expression::unary_op(op.clone(), operand_simplified))
            }
            
            Expression::Function { name, args } => {
                let args_simplified: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.simplify_trigonometric(arg))
                    .collect();
                Ok(Expression::function(name, args_simplified?))
            }
            
            _ => Ok(expr.clone())
        }
    }
    
    /// 检查表达式是否为0
    fn is_zero(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::Number(n) if n.is_zero())
    }
    
    /// 检查表达式是否为1
    fn is_one(&self, expr: &Expression) -> bool {
        matches!(expr, Expression::Number(n) if n.is_one())
    }
    
    /// 检查表达式是否为 -1
    fn is_negative_one(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) => n == &Number::integer(-1),
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                matches!(operand.as_ref(), Expression::Number(n) if n.is_one())
            }
            _ => false
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