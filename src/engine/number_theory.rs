//! # 数论和组合数学模块
//!
//! 实现数论相关的算法，包括最大公约数、最小公倍数、素数判断、
//! 质因数分解、二项式系数、排列组合等功能。

use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use num_traits::{Zero, One, Signed};
use crate::core::{Expression, Number};
use super::ComputeError;

/// 数论和组合数学引擎
pub struct NumberTheoryEngine;

impl NumberTheoryEngine {
    /// 创建新的数论引擎
    pub fn new() -> Self {
        Self
    }
    
    /// 计算两个整数的最大公约数（欧几里得算法）
    pub fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        match (a, b) {
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
                let result = self.gcd_bigint(a, b);
                Ok(Expression::Number(Number::Integer(result)))
            }
            (Expression::Number(Number::Rational(a)), Expression::Number(Number::Rational(b))) => {
                // 对于有理数，gcd(a/b, c/d) = gcd(a*d, c*b) / (b*d)
                let a_num = a.numer();
                let a_den = a.denom();
                let b_num = b.numer();
                let b_den = b.denom();
                
                let numerator_gcd = self.gcd_bigint(&(a_num * b_den), &(b_num * a_den));
                let denominator = a_den * b_den;
                
                let result = BigRational::new(numerator_gcd, denominator.clone());
                Ok(Expression::Number(Number::Rational(result)))
            }
            _ => Err(ComputeError::unsupported_operation(
                "gcd 函数只支持整数和有理数，请确保参数是整数或有理数"
            ))
        }
    }
    
    /// 计算两个整数的最小公倍数
    pub fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        match (a, b) {
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
                if a.is_zero() || b.is_zero() {
                    return Ok(Expression::Number(Number::Integer(BigInt::zero())));
                }
                
                let gcd_val = self.gcd_bigint(a, b);
                let result = (a * b).abs() / gcd_val;
                Ok(Expression::Number(Number::Integer(result)))
            }
            _ => Err(ComputeError::unsupported_operation(
                "lcm 函数只支持整数，请确保参数是整数"
            ))
        }
    }
    
    /// 判断一个数是否为素数
    pub fn is_prime(&self, n: &Expression) -> Result<bool, ComputeError> {
        match n {
            Expression::Number(Number::Integer(n)) => {
                Ok(self.is_prime_bigint(n))
            }
            _ => Err(ComputeError::unsupported_operation(
                "is_prime 函数只支持整数，请确保参数是正整数"
            ))
        }
    }
    
    /// 质因数分解
    pub fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, ComputeError> {
        match n {
            Expression::Number(Number::Integer(n)) => {
                if n <= &BigInt::one() {
                    return Err(ComputeError::domain_error(
                        "质因数分解要求输入大于1的正整数"
                    ));
                }
                
                let factors = self.prime_factors_bigint(n);
                let result = factors.into_iter()
                    .map(|f| Expression::Number(Number::Integer(f)))
                    .collect();
                Ok(result)
            }
            _ => Err(ComputeError::unsupported_operation(
                "prime_factors 函数只支持整数，请确保参数是大于1的正整数"
            ))
        }
    }
    
    /// 计算二项式系数 C(n, k) = n! / (k! * (n-k)!)
    pub fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        match (n, k) {
            (Expression::Number(Number::Integer(n)), Expression::Number(Number::Integer(k))) => {
                if n < &BigInt::zero() || k < &BigInt::zero() || k > n {
                    return Ok(Expression::Number(Number::Integer(BigInt::zero())));
                }
                
                let result = self.binomial_coefficient(n, k)?;
                Ok(Expression::Number(Number::Integer(result)))
            }
            _ => Err(ComputeError::unsupported_operation(
                "binomial 函数只支持非负整数，请确保参数是非负整数且 k <= n"
            ))
        }
    }
    
    /// 计算排列数 P(n, k) = n! / (n-k)!
    pub fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, ComputeError> {
        match (n, k) {
            (Expression::Number(Number::Integer(n)), Expression::Number(Number::Integer(k))) => {
                if n < &BigInt::zero() || k < &BigInt::zero() || k > n {
                    return Ok(Expression::Number(Number::Integer(BigInt::zero())));
                }
                
                let result = self.permutation_count(n, k)?;
                Ok(Expression::Number(Number::Integer(result)))
            }
            _ => Err(ComputeError::unsupported_operation(
                "permutation 函数只支持非负整数，请确保参数是非负整数且 k <= n"
            ))
        }
    }
    
    /// 计算平均值
    pub fn mean(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        if values.is_empty() {
            return Err(ComputeError::domain_error(
                "无法计算空列表的平均值，请提供至少一个数值"
            ));
        }
        
        let mut sum = Number::Integer(BigInt::zero());
        let count = values.len();
        
        for value in values {
            match value {
                Expression::Number(num) => {
                    sum = self.add_numbers(&sum, num)?;
                }
                _ => return Err(ComputeError::unsupported_operation(
                    "mean 函数只支持数值，请确保所有参数都是数值"
                ))
            }
        }
        
        // 计算平均值：sum / count
        let count_num = Number::Integer(count.to_bigint().unwrap());
        let result = self.divide_numbers(&sum, &count_num)?;
        Ok(Expression::Number(result))
    }
    
    /// 计算方差
    pub fn variance(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        if values.len() < 2 {
            return Err(ComputeError::domain_error(
                "计算方差需要至少两个数值"
            ));
        }
        
        // 计算平均值
        let mean_expr = self.mean(values)?;
        let mean_num = match mean_expr {
            Expression::Number(num) => num,
            _ => unreachable!()
        };
        
        // 计算方差：Σ(xi - mean)² / n
        let mut sum_squares = Number::Integer(BigInt::zero());
        
        for value in values {
            match value {
                Expression::Number(num) => {
                    let diff = self.subtract_numbers(num, &mean_num)?;
                    let square = self.multiply_numbers(&diff, &diff)?;
                    sum_squares = self.add_numbers(&sum_squares, &square)?;
                }
                _ => return Err(ComputeError::unsupported_operation(
                    "variance 函数只支持数值，请确保所有参数都是数值"
                ))
            }
        }
        
        let count_num = Number::Integer(values.len().to_bigint().unwrap());
        let result = self.divide_numbers(&sum_squares, &count_num)?;
        Ok(Expression::Number(result))
    }
    
    /// 计算标准差
    pub fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, ComputeError> {
        let variance_expr = self.variance(values)?;
        
        match variance_expr {
            Expression::Number(variance_num) => {
                // 计算平方根
                let sqrt_result = self.sqrt_number(&variance_num)?;
                Ok(Expression::Number(sqrt_result))
            }
            _ => unreachable!()
        }
    }
    
    // 私有辅助方法
    
    /// 计算两个 BigInt 的最大公约数
    fn gcd_bigint(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut a = a.abs();
        let mut b = b.abs();
        
        while !b.is_zero() {
            let temp = b.clone();
            b = &a % &b;
            a = temp;
        }
        
        a
    }
    
    /// 判断 BigInt 是否为素数（试除法）
    fn is_prime_bigint(&self, n: &BigInt) -> bool {
        if n <= &BigInt::one() {
            return false;
        }
        if n == &(BigInt::from(2)) {
            return true;
        }
        if n % 2 == BigInt::zero() {
            return false;
        }
        
        // 只检查到 sqrt(n)
        let mut i = BigInt::from(3);
        let sqrt_n = self.integer_sqrt(n);
        
        while i <= sqrt_n {
            if n % &i == BigInt::zero() {
                return false;
            }
            i += 2; // 只检查奇数
        }
        
        true
    }
    
    /// 质因数分解（试除法）
    fn prime_factors_bigint(&self, n: &BigInt) -> Vec<BigInt> {
        let mut factors = Vec::new();
        let mut n = n.clone();
        
        // 处理因子2
        while &n % 2 == BigInt::zero() {
            factors.push(BigInt::from(2));
            n /= 2;
        }
        
        // 处理奇数因子
        let mut i = BigInt::from(3);
        let sqrt_n = self.integer_sqrt(&n);
        
        while i <= sqrt_n && n > BigInt::one() {
            while &n % &i == BigInt::zero() {
                factors.push(i.clone());
                n /= &i;
            }
            i += 2;
        }
        
        // 如果 n 仍然大于1，那么它本身就是一个素因子
        if n > BigInt::one() {
            factors.push(n);
        }
        
        factors
    }
    
    /// 计算二项式系数
    fn binomial_coefficient(&self, n: &BigInt, k: &BigInt) -> Result<BigInt, ComputeError> {
        let k = if k > &(n - k) { n - k } else { k.clone() };
        
        if k.is_zero() {
            return Ok(BigInt::one());
        }
        
        let mut result = BigInt::one();
        let mut i = BigInt::zero();
        
        while i < k {
            result = result * (n - &i) / (&i + 1);
            i += 1;
        }
        
        Ok(result)
    }
    
    /// 计算排列数
    fn permutation_count(&self, n: &BigInt, k: &BigInt) -> Result<BigInt, ComputeError> {
        if k.is_zero() {
            return Ok(BigInt::one());
        }
        
        let mut result = BigInt::one();
        let mut i = BigInt::zero();
        
        while i < *k {
            result *= n - &i;
            i += 1;
        }
        
        Ok(result)
    }
    
    /// 计算整数平方根（牛顿法）
    fn integer_sqrt(&self, n: &BigInt) -> BigInt {
        if n.is_zero() {
            return BigInt::zero();
        }
        
        let mut x = n.clone();
        let mut y: BigInt = (n + 1) / 2;
        
        while y < x {
            x = y.clone();
            y = (&x + n / &x) / 2;
        }
        
        x
    }
    
    /// 数值加法
    fn add_numbers(&self, a: &Number, b: &Number) -> Result<Number, ComputeError> {
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Ok(Number::Integer(a + b))
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(a + b))
            }
            (Number::Integer(a), Number::Rational(b)) => {
                let a_rational = BigRational::from(a.clone());
                Ok(Number::Rational(a_rational + b))
            }
            (Number::Rational(a), Number::Integer(b)) => {
                let b_rational = BigRational::from(b.clone());
                Ok(Number::Rational(a + b_rational))
            }
            _ => Err(ComputeError::unsupported_operation(
                "不支持的数值类型组合"
            ))
        }
    }
    
    /// 数值减法
    fn subtract_numbers(&self, a: &Number, b: &Number) -> Result<Number, ComputeError> {
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Ok(Number::Integer(a - b))
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(a - b))
            }
            (Number::Integer(a), Number::Rational(b)) => {
                let a_rational = BigRational::from(a.clone());
                Ok(Number::Rational(a_rational - b))
            }
            (Number::Rational(a), Number::Integer(b)) => {
                let b_rational = BigRational::from(b.clone());
                Ok(Number::Rational(a - b_rational))
            }
            _ => Err(ComputeError::unsupported_operation(
                "不支持的数值类型组合"
            ))
        }
    }
    
    /// 数值乘法
    fn multiply_numbers(&self, a: &Number, b: &Number) -> Result<Number, ComputeError> {
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Ok(Number::Integer(a * b))
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(a * b))
            }
            (Number::Integer(a), Number::Rational(b)) => {
                let a_rational = BigRational::from(a.clone());
                Ok(Number::Rational(a_rational * b))
            }
            (Number::Rational(a), Number::Integer(b)) => {
                let b_rational = BigRational::from(b.clone());
                Ok(Number::Rational(a * b_rational))
            }
            _ => Err(ComputeError::unsupported_operation(
                "不支持的数值类型组合"
            ))
        }
    }
    
    /// 数值除法
    fn divide_numbers(&self, a: &Number, b: &Number) -> Result<Number, ComputeError> {
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                if b.is_zero() {
                    return Err(ComputeError::DivisionByZero);
                }
                Ok(Number::Rational(BigRational::new(a.clone(), b.clone())))
            }
            (Number::Rational(a), Number::Rational(b)) => {
                if b.is_zero() {
                    return Err(ComputeError::DivisionByZero);
                }
                Ok(Number::Rational(a / b))
            }
            (Number::Integer(a), Number::Rational(b)) => {
                if b.is_zero() {
                    return Err(ComputeError::DivisionByZero);
                }
                let a_rational = BigRational::from(a.clone());
                Ok(Number::Rational(a_rational / b))
            }
            (Number::Rational(a), Number::Integer(b)) => {
                if b.is_zero() {
                    return Err(ComputeError::DivisionByZero);
                }
                let b_rational = BigRational::from(b.clone());
                Ok(Number::Rational(a / b_rational))
            }
            _ => Err(ComputeError::unsupported_operation(
                "不支持的数值类型组合"
            ))
        }
    }
    
    /// 数值平方根（简化实现，返回符号形式）
    fn sqrt_number(&self, n: &Number) -> Result<Number, ComputeError> {
        match n {
            Number::Integer(n) => {
                if n < &BigInt::zero() {
                    return Err(ComputeError::domain_error(
                        "负数没有实数平方根，请使用复数运算"
                    ));
                }
                
                // 检查是否为完全平方数
                let sqrt_int = self.integer_sqrt(n);
                if &sqrt_int * &sqrt_int == *n {
                    Ok(Number::Integer(sqrt_int))
                } else {
                    // 返回符号形式
                    Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Sqrt,
                        operand: Box::new(Expression::Number(Number::Integer(n.clone()))),
                    })))
                }
            }
            Number::Rational(r) => {
                let num_sqrt = self.integer_sqrt(r.numer());
                let den_sqrt = self.integer_sqrt(r.denom());
                
                if &num_sqrt * &num_sqrt == *r.numer() && &den_sqrt * &den_sqrt == *r.denom() {
                    Ok(Number::Rational(BigRational::new(num_sqrt, den_sqrt)))
                } else {
                    // 返回符号形式
                    Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Sqrt,
                        operand: Box::new(Expression::Number(Number::Rational(r.clone()))),
                    })))
                }
            }
            _ => Err(ComputeError::unsupported_operation(
                "不支持的数值类型"
            ))
        }
    }
}

impl Default for NumberTheoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "number_theory_tests.rs"]
mod number_theory_tests;