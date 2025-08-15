//! # 多项式运算系统
//!
//! 实现多项式的展开、因式分解、同类项收集等功能。

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator};
use crate::engine::error::ComputeError;
use std::collections::HashMap;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, Signed};
use num_integer::Integer;

/// 多项式项，表示为系数 * 变量^指数的形式
#[derive(Debug, Clone, PartialEq)]
pub struct PolynomialTerm {
    /// 系数
    pub coefficient: Number,
    /// 变量及其指数的映射
    pub variables: HashMap<String, i32>,
}

impl PolynomialTerm {
    /// 创建新的多项式项
    pub fn new(coefficient: Number, variables: HashMap<String, i32>) -> Self {
        Self { coefficient, variables }
    }
    
    /// 创建常数项
    pub fn constant(coefficient: Number) -> Self {
        Self {
            coefficient,
            variables: HashMap::new(),
        }
    }
    
    /// 创建单变量项
    pub fn variable(var: String, power: i32, coefficient: Number) -> Self {
        let mut variables = HashMap::new();
        if power != 0 {
            variables.insert(var, power);
        }
        Self { coefficient, variables }
    }
    
    /// 检查是否为常数项
    pub fn is_constant(&self) -> bool {
        self.variables.is_empty()
    }
    
    /// 检查是否为零项
    pub fn is_zero(&self) -> bool {
        self.coefficient.is_zero()
    }
    
    /// 获取项的次数（所有变量指数之和）
    pub fn degree(&self) -> i32 {
        self.variables.values().sum()
    }
    
    /// 获取指定变量的次数
    pub fn degree_of(&self, var: &str) -> i32 {
        self.variables.get(var).copied().unwrap_or(0)
    }
    
    /// 乘以另一个项
    pub fn multiply(&self, other: &PolynomialTerm) -> PolynomialTerm {
        let mut new_variables = self.variables.clone();
        
        // 合并变量指数
        for (var, power) in &other.variables {
            let current_power = new_variables.get(var).copied().unwrap_or(0);
            let new_power = current_power + power;
            if new_power == 0 {
                new_variables.remove(var);
            } else {
                new_variables.insert(var.clone(), new_power);
            }
        }
        
        PolynomialTerm {
            coefficient: self.coefficient.clone() * other.coefficient.clone(),
            variables: new_variables,
        }
    }
    
    /// 除以另一个项（如果可能）
    pub fn divide(&self, other: &PolynomialTerm) -> Result<PolynomialTerm, ComputeError> {
        if other.coefficient.is_zero() {
            return Err(ComputeError::DivisionByZero);
        }
        
        let mut new_variables = self.variables.clone();
        
        // 检查是否可以整除
        for (var, other_power) in &other.variables {
            let self_power = self.variables.get(var).copied().unwrap_or(0);
            if self_power < *other_power {
                return Err(ComputeError::UnsupportedOperation {
                    operation: "多项式除法：被除数的变量次数小于除数".to_string()
                });
            }
            
            let new_power = self_power - other_power;
            if new_power == 0 {
                new_variables.remove(var);
            } else {
                new_variables.insert(var.clone(), new_power);
            }
        }
        
        Ok(PolynomialTerm {
            coefficient: self.coefficient.clone() / other.coefficient.clone(),
            variables: new_variables,
        })
    }
    
    /// 转换为表达式
    pub fn to_expression(&self) -> Expression {
        if self.is_zero() {
            return Expression::Number(Number::zero());
        }
        
        let mut result = Expression::Number(self.coefficient.clone());
        
        // 添加变量部分
        for (var, power) in &self.variables {
            let var_expr = if *power == 1 {
                Expression::Variable(var.clone())
            } else {
                Expression::power(
                    Expression::Variable(var.clone()),
                    Expression::Number(Number::integer(*power as i64))
                )
            };
            
            result = if result == Expression::Number(Number::one()) {
                var_expr
            } else {
                Expression::multiply(result, var_expr)
            };
        }
        
        result
    }
    
    /// 检查两个项是否为同类项
    pub fn is_like_term(&self, other: &PolynomialTerm) -> bool {
        self.variables == other.variables
    }
}

/// 多项式表示
#[derive(Debug, Clone)]
pub struct Polynomial {
    /// 多项式的项
    pub terms: Vec<PolynomialTerm>,
}

impl Polynomial {
    /// 创建新的多项式
    pub fn new(terms: Vec<PolynomialTerm>) -> Self {
        let mut poly = Self { terms };
        poly.simplify();
        poly
    }
    
    /// 计算两个数的最大公约数（静态方法）
    fn gcd_numbers_static(a: &Number, b: &Number) -> Number {
        match (a, b) {
            (Number::Integer(a_int), Number::Integer(b_int)) => {
                use num_integer::Integer;
                Number::Integer(a_int.gcd(b_int))
            }
            (Number::Rational(a_rat), Number::Rational(b_rat)) => {
                use num_integer::Integer;
                let a_num = a_rat.numer();
                let a_den = a_rat.denom();
                let b_num = b_rat.numer();
                let b_den = b_rat.denom();
                
                let gcd_num = a_num.gcd(b_num);
                let lcm_den = a_den.lcm(b_den);
                
                Number::Rational(BigRational::new(gcd_num, lcm_den))
            }
            _ => Number::one(), // 对于其他类型，返回1
        }
    }
    
    /// 创建零多项式
    pub fn zero() -> Self {
        Self { terms: vec![] }
    }
    
    /// 创建常数多项式
    pub fn constant(value: Number) -> Self {
        if value.is_zero() {
            Self::zero()
        } else {
            Self {
                terms: vec![PolynomialTerm::constant(value)],
            }
        }
    }
    
    /// 创建单变量多项式
    pub fn variable(var: String) -> Self {
        Self {
            terms: vec![PolynomialTerm::variable(var, 1, Number::one())],
        }
    }
    
    /// 检查是否为零多项式
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty() || self.terms.iter().all(|term| term.is_zero())
    }
    
    /// 检查是否为常数多项式
    pub fn is_constant(&self) -> bool {
        self.terms.len() <= 1 && self.terms.iter().all(|term| term.is_constant())
    }
    
    /// 获取多项式的次数
    pub fn degree(&self) -> i32 {
        self.terms.iter().map(|term| term.degree()).max().unwrap_or(0)
    }
    
    /// 获取指定变量的次数
    pub fn degree_of(&self, var: &str) -> i32 {
        self.terms.iter().map(|term| term.degree_of(var)).max().unwrap_or(0)
    }
    
    /// 简化多项式（合并同类项，移除零项）
    pub fn simplify(&mut self) {
        // 移除零项
        self.terms.retain(|term| !term.is_zero());
        
        // 合并同类项
        let mut simplified_terms = Vec::new();
        let mut processed = vec![false; self.terms.len()];
        
        for i in 0..self.terms.len() {
            if processed[i] {
                continue;
            }
            
            let mut combined_coefficient = self.terms[i].coefficient.clone();
            processed[i] = true;
            
            // 查找同类项
            for j in (i + 1)..self.terms.len() {
                if !processed[j] && self.terms[i].is_like_term(&self.terms[j]) {
                    combined_coefficient = combined_coefficient + self.terms[j].coefficient.clone();
                    processed[j] = true;
                }
            }
            
            // 如果合并后的系数不为零，添加到结果中
            if !combined_coefficient.is_zero() {
                simplified_terms.push(PolynomialTerm {
                    coefficient: combined_coefficient,
                    variables: self.terms[i].variables.clone(),
                });
            }
        }
        
        self.terms = simplified_terms;
        
        // 按字典序排序项
        self.terms.sort_by(|a, b| {
            // 首先按总次数排序
            let degree_cmp = b.degree().cmp(&a.degree());
            if degree_cmp != std::cmp::Ordering::Equal {
                return degree_cmp;
            }
            
            // 然后按变量名排序
            let a_vars: Vec<_> = a.variables.keys().collect();
            let b_vars: Vec<_> = b.variables.keys().collect();
            a_vars.cmp(&b_vars)
        });
    }
    
    /// 多项式加法
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let mut terms = self.terms.clone();
        terms.extend(other.terms.clone());
        Polynomial::new(terms)
    }
    
    /// 多项式减法
    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        let mut terms = self.terms.clone();
        
        // 添加负的项
        for term in &other.terms {
            terms.push(PolynomialTerm {
                coefficient: -term.coefficient.clone(),
                variables: term.variables.clone(),
            });
        }
        
        Polynomial::new(terms)
    }
    
    /// 多项式乘法
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        if self.is_zero() || other.is_zero() {
            return Polynomial::zero();
        }
        
        let mut terms = Vec::new();
        
        for term1 in &self.terms {
            for term2 in &other.terms {
                terms.push(term1.multiply(term2));
            }
        }
        
        Polynomial::new(terms)
    }
    
    /// 多项式除法（返回商和余式）
    pub fn divide(&self, divisor: &Polynomial) -> Result<(Polynomial, Polynomial), ComputeError> {
        if divisor.is_zero() {
            return Err(ComputeError::DivisionByZero);
        }
        
        if self.is_zero() {
            return Ok((Polynomial::zero(), Polynomial::zero()));
        }
        
        // 如果除数是常数，直接除以常数
        if divisor.is_constant() {
            let divisor_const = &divisor.terms[0].coefficient;
            let mut quotient_terms = Vec::new();
            
            for term in &self.terms {
                quotient_terms.push(PolynomialTerm {
                    coefficient: term.coefficient.clone() / divisor_const.clone(),
                    variables: term.variables.clone(),
                });
            }
            
            return Ok((Polynomial::new(quotient_terms), Polynomial::zero()));
        }
        
        // 多项式长除法
        let mut dividend = self.clone();
        let mut quotient = Polynomial::zero();
        
        while !dividend.is_zero() && dividend.degree() >= divisor.degree() {
            // 获取被除数和除数的首项
            let dividend_leading = &dividend.terms[0];
            let divisor_leading = &divisor.terms[0];
            
            // 计算商的当前项
            let quotient_term = dividend_leading.divide(divisor_leading)?;
            
            // 更新商
            quotient.terms.push(quotient_term.clone());
            
            // 计算 quotient_term * divisor
            let subtrahend = Polynomial::new(vec![quotient_term]).multiply(divisor);
            
            // 更新被除数
            dividend = dividend.subtract(&subtrahend);
        }
        
        quotient.simplify();
        dividend.simplify(); // dividend 现在是余式
        
        Ok((quotient, dividend))
    }
    
    /// 计算多项式的最大公约数
    pub fn gcd(&self, other: &Polynomial) -> Result<Polynomial, ComputeError> {
        if self.is_zero() {
            return Ok(other.clone());
        }
        if other.is_zero() {
            return Ok(self.clone());
        }
        
        let mut a = self.clone();
        let mut b = other.clone();
        
        // 欧几里得算法
        while !b.is_zero() {
            let (_, remainder) = a.divide(&b)?;
            a = b;
            b = remainder;
        }
        
        // 使首项系数为正并提取公因子
        if let Some(leading_term) = a.terms.first_mut() {
            if leading_term.coefficient.is_negative() {
                for term in &mut a.terms {
                    term.coefficient = -term.coefficient.clone();
                }
            }
        }
        
        // 提取数值公因子，使 GCD 为最简形式
        if !a.terms.is_empty() {
            let mut gcd_coeff = a.terms[0].coefficient.clone();
            for term in &a.terms[1..] {
                gcd_coeff = Polynomial::gcd_numbers_static(&gcd_coeff, &term.coefficient);
            }
            
            // 如果公因子不为1，则提取它
            if gcd_coeff != Number::one() && !gcd_coeff.is_zero() {
                for term in &mut a.terms {
                    term.coefficient = term.coefficient.clone() / gcd_coeff.clone();
                }
            }
        }
        
        Ok(a)
    }
    
    /// 转换为表达式
    pub fn to_expression(&self) -> Expression {
        if self.is_zero() {
            return Expression::Number(Number::zero());
        }
        
        if self.terms.len() == 1 {
            return self.terms[0].to_expression();
        }
        
        let mut result = self.terms[0].to_expression();
        
        for term in &self.terms[1..] {
            let term_expr = term.to_expression();
            if term.coefficient.is_negative() {
                // 如果系数为负，使用减法
                let positive_term = PolynomialTerm {
                    coefficient: -term.coefficient.clone(),
                    variables: term.variables.clone(),
                };
                result = Expression::subtract(result, positive_term.to_expression());
            } else {
                result = Expression::add(result, term_expr);
            }
        }
        
        result
    }
}

/// 多项式运算引擎
pub struct PolynomialEngine;

impl PolynomialEngine {
    /// 创建新的多项式引擎
    pub fn new() -> Self {
        Self
    }
    
    /// 将表达式转换为多项式（如果可能）
    pub fn expression_to_polynomial(&self, expr: &Expression) -> Result<Polynomial, ComputeError> {
        match expr {
            Expression::Number(n) => Ok(Polynomial::constant(n.clone())),
            
            Expression::Variable(var) => Ok(Polynomial::variable(var.clone())),
            
            Expression::BinaryOp { op, left, right } => {
                let left_poly = self.expression_to_polynomial(left)?;
                let right_poly = self.expression_to_polynomial(right)?;
                
                match op {
                    BinaryOperator::Add => Ok(left_poly.add(&right_poly)),
                    BinaryOperator::Subtract => Ok(left_poly.subtract(&right_poly)),
                    BinaryOperator::Multiply => Ok(left_poly.multiply(&right_poly)),
                    BinaryOperator::Power => {
                        // 只支持正整数幂
                        if let Expression::Number(exp) = right.as_ref() {
                            if let Some(exp_int) = exp.to_integer() {
                                if exp_int >= BigInt::zero() {
                                    use num_traits::ToPrimitive;
                                    if let Some(exp_u32) = exp_int.to_u32() {
                                        return Ok(self.power(&left_poly, exp_u32));
                                    }
                                }
                            }
                        }
                        Err(ComputeError::UnsupportedOperation {
                            operation: "多项式只支持正整数幂".to_string()
                        })
                    }
                    _ => Err(ComputeError::UnsupportedOperation {
                        operation: format!("多项式不支持 {:?} 运算", op)
                    })
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                match op {
                    UnaryOperator::Negate => {
                        let poly = self.expression_to_polynomial(operand)?;
                        let mut negated_terms = Vec::new();
                        for term in poly.terms {
                            negated_terms.push(PolynomialTerm {
                                coefficient: -term.coefficient,
                                variables: term.variables,
                            });
                        }
                        Ok(Polynomial::new(negated_terms))
                    }
                    UnaryOperator::Plus => self.expression_to_polynomial(operand),
                    _ => Err(ComputeError::UnsupportedOperation {
                        operation: format!("多项式不支持 {:?} 运算", op)
                    })
                }
            }
            
            _ => Err(ComputeError::UnsupportedOperation {
                operation: "表达式无法转换为多项式".to_string()
            })
        }
    }
    
    /// 多项式幂运算
    fn power(&self, poly: &Polynomial, exponent: u32) -> Polynomial {
        if exponent == 0 {
            return Polynomial::constant(Number::one());
        }
        if exponent == 1 {
            return poly.clone();
        }
        
        // 快速幂算法
        let half_power = self.power(poly, exponent / 2);
        let result = half_power.multiply(&half_power);
        
        if exponent % 2 == 0 {
            result
        } else {
            result.multiply(poly)
        }
    }
    
    /// 展开表达式
    pub fn expand(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let poly = self.expression_to_polynomial(expr)?;
        Ok(poly.to_expression())
    }
    
    /// 收集同类项
    pub fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        let poly = self.expression_to_polynomial(expr)?;
        
        // 按指定变量的次数分组
        let mut terms_by_power: HashMap<i32, Vec<PolynomialTerm>> = HashMap::new();
        
        for term in poly.terms {
            let power = term.degree_of(var);
            terms_by_power.entry(power).or_insert_with(Vec::new).push(term);
        }
        
        // 重新构建多项式
        let mut new_terms = Vec::new();
        for (power, terms) in terms_by_power {
            let mut combined_poly = Polynomial::zero();
            for term in terms {
                // 移除指定变量，创建系数多项式
                let mut coeff_variables = term.variables.clone();
                coeff_variables.remove(var);
                
                let coeff_term = PolynomialTerm {
                    coefficient: term.coefficient,
                    variables: coeff_variables,
                };
                combined_poly.terms.push(coeff_term);
            }
            combined_poly.simplify();
            
            // 重新添加变量
            for mut coeff_term in combined_poly.terms {
                if power != 0 {
                    coeff_term.variables.insert(var.to_string(), power);
                }
                new_terms.push(coeff_term);
            }
        }
        
        let result_poly = Polynomial::new(new_terms);
        Ok(result_poly.to_expression())
    }
    
    /// 简单因式分解（提取公因子）
    pub fn factor(&self, expr: &Expression) -> Result<Expression, ComputeError> {
        let poly = self.expression_to_polynomial(expr)?;
        
        if poly.is_zero() || poly.terms.len() <= 1 {
            return Ok(expr.clone());
        }
        
        // 提取数值公因子
        let mut gcd_coeff = poly.terms[0].coefficient.clone();
        for term in &poly.terms[1..] {
            gcd_coeff = self.gcd_numbers(&gcd_coeff, &term.coefficient);
        }
        
        // 提取变量公因子
        let mut common_variables = poly.terms[0].variables.clone();
        for term in &poly.terms[1..] {
            let mut new_common = HashMap::new();
            for (var, power) in &common_variables {
                if let Some(term_power) = term.variables.get(var) {
                    let min_power = (*power).min(*term_power);
                    if min_power > 0 {
                        new_common.insert(var.clone(), min_power);
                    }
                }
            }
            common_variables = new_common;
        }
        
        // 如果没有公因子，返回原表达式
        if gcd_coeff == Number::one() && common_variables.is_empty() {
            return Ok(expr.clone());
        }
        
        // 构建公因子
        let common_factor = PolynomialTerm {
            coefficient: gcd_coeff.clone(),
            variables: common_variables.clone(),
        };
        
        // 提取公因子后的多项式
        let mut factored_terms = Vec::new();
        for term in &poly.terms {
            let mut new_variables = term.variables.clone();
            for (var, common_power) in &common_variables {
                let current_power = new_variables.get(var).copied().unwrap_or(0);
                let new_power = current_power - common_power;
                if new_power == 0 {
                    new_variables.remove(var);
                } else {
                    new_variables.insert(var.clone(), new_power);
                }
            }
            
            factored_terms.push(PolynomialTerm {
                coefficient: term.coefficient.clone() / gcd_coeff.clone(),
                variables: new_variables,
            });
        }
        
        let factored_poly = Polynomial::new(factored_terms);
        
        // 构建结果表达式
        let common_factor_expr = common_factor.to_expression();
        let factored_expr = factored_poly.to_expression();
        
        Ok(Expression::multiply(common_factor_expr, factored_expr))
    }
    
    /// 计算两个数的最大公约数
    fn gcd_numbers(&self, a: &Number, b: &Number) -> Number {
        match (a, b) {
            (Number::Integer(a_int), Number::Integer(b_int)) => {
                use num_integer::Integer;
                Number::Integer(a_int.gcd(b_int))
            }
            (Number::Rational(a_rat), Number::Rational(b_rat)) => {
                use num_integer::Integer;
                let a_num = a_rat.numer();
                let a_den = a_rat.denom();
                let b_num = b_rat.numer();
                let b_den = b_rat.denom();
                
                let gcd_num = a_num.gcd(b_num);
                let lcm_den = a_den.lcm(b_den);
                
                Number::Rational(BigRational::new(gcd_num, lcm_den))
            }
            _ => Number::one(), // 对于其他类型，返回1
        }
    }
    
    /// 多项式除法
    pub fn polynomial_divide(&self, dividend: &Expression, divisor: &Expression) -> Result<(Expression, Expression), ComputeError> {
        let dividend_poly = self.expression_to_polynomial(dividend)?;
        let divisor_poly = self.expression_to_polynomial(divisor)?;
        
        let (quotient, remainder) = dividend_poly.divide(&divisor_poly)?;
        
        Ok((quotient.to_expression(), remainder.to_expression()))
    }
    
    /// 多项式最大公约数
    pub fn polynomial_gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let poly_a = self.expression_to_polynomial(a)?;
        let poly_b = self.expression_to_polynomial(b)?;
        
        let gcd_poly = poly_a.gcd(&poly_b)?;
        Ok(gcd_poly.to_expression())
    }
}

impl Default for PolynomialEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "polynomial_tests.rs"]
mod polynomial_tests;