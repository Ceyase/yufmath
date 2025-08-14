//! # 数值类型定义
//!
//! 定义支持精确计算的数值类型系统，包括任意精度整数、
//! 有理数、实数和复数等类型。

use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use bigdecimal::{BigDecimal, Zero, ToPrimitive, FromPrimitive};
use num_traits::{Zero as NumZero, One as NumOne, ToPrimitive as NumToPrimitive, FromPrimitive as NumFromPrimitive, Signed};
use std::fmt::{self, Display, Debug};

/// 支持多种数值类型的统一表示，优先使用精确表示
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    /// 任意精度整数（无精度限制）
    Integer(BigInt),
    /// 任意精度有理数（无精度限制）
    Rational(BigRational),
    /// 任意精度实数（用于无理数的精确表示）
    Real(BigDecimal),
    /// 任意精度复数
    Complex {
        real: Box<Number>,
        imaginary: Box<Number>,
    },
    /// 符号表示（用于无法精确计算的数值）
    Symbolic(Box<crate::core::Expression>),
    /// 浮点数（仅在明确要求数值近似时使用）
    Float(f64),
}

impl Number {
    /// 创建整数
    pub fn integer(value: impl Into<BigInt>) -> Self {
        Number::Integer(value.into())
    }
    
    /// 创建有理数
    pub fn rational(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        Number::Rational(BigRational::new(numerator.into(), denominator.into()))
    }
    
    /// 创建实数
    pub fn real(value: impl Into<BigDecimal>) -> Self {
        Number::Real(value.into())
    }
    
    /// 创建复数
    pub fn complex(real: Number, imaginary: Number) -> Self {
        Number::Complex {
            real: Box::new(real),
            imaginary: Box::new(imaginary),
        }
    }
    
    /// 创建浮点数
    pub fn float(value: f64) -> Self {
        Number::Float(value)
    }
    
    /// 转换为最精确的表示形式
    pub fn to_exact(&self) -> Self {
        match self {
            Number::Float(f) => {
                // 尝试将浮点数转换为有理数
                if let Some(rational) = Self::float_to_rational(*f) {
                    Number::Rational(rational)
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }
    
    /// 检查是否为精确表示
    pub fn is_exact(&self) -> bool {
        !matches!(self, Number::Float(_))
    }
    
    /// 检查是否为零
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_zero(),
            Number::Rational(r) => r.is_zero(),
            Number::Real(r) => r.is_zero(),
            Number::Complex { real, imaginary } => real.is_zero() && imaginary.is_zero(),
            Number::Float(f) => *f == 0.0,
            Number::Symbolic(_) => false,
        }
    }
    
    /// 检查是否为一
    pub fn is_one(&self) -> bool {
        match self {
            Number::Integer(i) => i == &BigInt::from(1),
            Number::Rational(r) => r == &BigRational::from(BigInt::from(1)),
            Number::Real(r) => r == &BigDecimal::from(1),
            Number::Float(f) => *f == 1.0,
            Number::Complex { real, imaginary } => real.is_one() && imaginary.is_zero(),
            Number::Symbolic(_) => false,
        }
    }
    
    /// 获取数值近似值（仅在需要时使用）
    pub fn approximate(&self) -> f64 {
        match self {
            Number::Integer(i) => NumToPrimitive::to_f64(i).unwrap_or(f64::INFINITY),
            Number::Rational(r) => NumToPrimitive::to_f64(r).unwrap_or(f64::NAN),
            Number::Real(r) => ToPrimitive::to_f64(r).unwrap_or(f64::NAN),
            Number::Complex { real, imaginary } => {
                // 对于复数，返回模长
                let r = real.approximate();
                let i = imaginary.approximate();
                (r * r + i * i).sqrt()
            }
            Number::Symbolic(_) => f64::NAN,
            Number::Float(f) => *f,
        }
    }
    
    /// 尝试将浮点数转换为有理数
    fn float_to_rational(f: f64) -> Option<BigRational> {
        if f.is_finite() {
            // 使用简单的分数近似算法
            // 实际实现中应该使用更精确的连分数算法
            let precision = 1e-10;
            let mut num = f;
            let mut denom = 1.0;
            
            while (num - num.round()).abs() > precision && denom < 1e6 {
                num *= 10.0;
                denom *= 10.0;
            }
            
            let numerator = BigInt::from(num.round() as i64);
            let denominator = BigInt::from(denom as i64);
            
            Some(BigRational::new(numerator, denominator))
        } else {
            None
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Integer(BigInt::from(value))
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Integer(BigInt::from(value))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl From<BigInt> for Number {
    fn from(value: BigInt) -> Self {
        Number::Integer(value)
    }
}

impl From<BigRational> for Number {
    fn from(value: BigRational) -> Self {
        Number::Rational(value)
    }
}

impl From<BigDecimal> for Number {
    fn from(value: BigDecimal) -> Self {
        Number::Real(value)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Rational(r) => {
                if r.denom() == &BigInt::from(1) {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            }
            Number::Real(r) => write!(f, "{}", r),
            Number::Complex { real, imaginary } => {
                if imaginary.is_zero() {
                    write!(f, "{}", real)
                } else if real.is_zero() {
                    if imaginary.is_one() {
                        write!(f, "i")
                    } else if **imaginary == Number::integer(-1) {
                        write!(f, "-i")
                    } else {
                        write!(f, "{}i", imaginary)
                    }
                } else {
                    let imag_str = if imaginary.is_one() {
                        "i".to_string()
                    } else if **imaginary == Number::integer(-1) {
                        "-i".to_string()
                    } else {
                        format!("{}i", imaginary)
                    };
                    
                    if imag_str.starts_with('-') {
                        write!(f, "{}{}", real, imag_str)
                    } else {
                        write!(f, "{}+{}", real, imag_str)
                    }
                }
            }
            Number::Symbolic(expr) => write!(f, "{}", expr),
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl Number {
    /// 创建零值
    pub fn zero() -> Self {
        Number::Integer(BigInt::from(0))
    }
    
    /// 创建一值
    pub fn one() -> Self {
        Number::Integer(BigInt::from(1))
    }
    
    /// 创建负一值
    pub fn neg_one() -> Self {
        Number::Integer(BigInt::from(-1))
    }
    
    /// 创建虚数单位 i
    pub fn i() -> Self {
        Number::Complex {
            real: Box::new(Number::zero()),
            imaginary: Box::new(Number::one()),
        }
    }
    
    /// 检查是否为负数
    pub fn is_negative(&self) -> bool {
        match self {
            Number::Integer(i) => i < &BigInt::from(0),
            Number::Rational(r) => r < &BigRational::from(BigInt::from(0)),
            Number::Real(r) => r < &BigDecimal::from(0),
            Number::Float(f) => *f < 0.0,
            Number::Complex { .. } => false, // 复数没有正负概念
            Number::Symbolic(_) => false, // 符号表达式无法确定
        }
    }
    
    /// 检查是否为正数
    pub fn is_positive(&self) -> bool {
        match self {
            Number::Integer(i) => i > &BigInt::from(0),
            Number::Rational(r) => r > &BigRational::from(BigInt::from(0)),
            Number::Real(r) => r > &BigDecimal::from(0),
            Number::Float(f) => *f > 0.0,
            Number::Complex { .. } => false, // 复数没有正负概念
            Number::Symbolic(_) => false, // 符号表达式无法确定
        }
    }
    
    /// 检查是否为整数
    pub fn is_integer(&self) -> bool {
        match self {
            Number::Integer(_) => true,
            Number::Rational(r) => r.denom() == &BigInt::from(1),
            Number::Real(r) => {
                // BigDecimal 没有 fract 方法，我们需要检查是否为整数
                if let Some(int_val) = r.to_bigint() {
                    BigDecimal::from(int_val) == *r
                } else {
                    false
                }
            },
            Number::Float(f) => f.fract() == 0.0,
            Number::Complex { real, imaginary } => {
                imaginary.is_zero() && real.is_integer()
            }
            Number::Symbolic(_) => false,
        }
    }
    
    /// 检查是否为有理数
    pub fn is_rational(&self) -> bool {
        match self {
            Number::Integer(_) | Number::Rational(_) => true,
            Number::Complex { real, imaginary } => {
                imaginary.is_zero() && real.is_rational()
            }
            _ => false,
        }
    }
    
    /// 检查是否为实数
    pub fn is_real(&self) -> bool {
        match self {
            Number::Integer(_) | Number::Rational(_) | Number::Real(_) | Number::Float(_) => true,
            Number::Complex { imaginary, .. } => imaginary.is_zero(),
            Number::Symbolic(_) => false,
        }
    }
    
    /// 检查是否为复数
    pub fn is_complex(&self) -> bool {
        match self {
            Number::Complex { imaginary, .. } => !imaginary.is_zero(),
            _ => false,
        }
    }
    
    /// 获取绝对值
    pub fn abs(&self) -> Self {
        match self {
            Number::Integer(i) => Number::Integer(i.abs()),
            Number::Rational(r) => Number::Rational(r.abs()),
            Number::Real(r) => Number::Real(r.abs()),
            Number::Float(f) => Number::Float(f.abs()),
            Number::Complex { real, imaginary } => {
                // |a + bi| = sqrt(a^2 + b^2)
                let real_sq = match real.as_ref() {
                    Number::Integer(r) => Number::Integer(r * r),
                    Number::Rational(r) => Number::Rational(r * r),
                    Number::Real(r) => Number::Real(r * r),
                    Number::Float(r) => Number::Float(r * r),
                    _ => return Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Abs,
                        operand: Box::new(crate::core::Expression::Number(self.clone())),
                    })),
                };
                
                let imag_sq = match imaginary.as_ref() {
                    Number::Integer(i) => Number::Integer(i * i),
                    Number::Rational(i) => Number::Rational(i * i),
                    Number::Real(i) => Number::Real(i * i),
                    Number::Float(i) => Number::Float(i * i),
                    _ => return Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Abs,
                        operand: Box::new(crate::core::Expression::Number(self.clone())),
                    })),
                };
                
                // 返回符号表示，因为需要开方运算
                Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                    op: crate::core::UnaryOperator::Sqrt,
                    operand: Box::new(crate::core::Expression::BinaryOp {
                        op: crate::core::BinaryOperator::Add,
                        left: Box::new(crate::core::Expression::Number(real_sq)),
                        right: Box::new(crate::core::Expression::Number(imag_sq)),
                    }),
                }))
            }
            Number::Symbolic(_) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Abs,
                operand: Box::new(crate::core::Expression::Number(self.clone())),
            })),
        }
    }
    
    /// 取负值
    pub fn neg(&self) -> Self {
        match self {
            Number::Integer(i) => Number::Integer(-i),
            Number::Rational(r) => Number::Rational(-r),
            Number::Real(r) => Number::Real(-r),
            Number::Float(f) => Number::Float(-f),
            Number::Complex { real, imaginary } => Number::Complex {
                real: Box::new(real.clone().neg()),
                imaginary: Box::new(imaginary.clone().neg()),
            },
            Number::Symbolic(expr) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Negate,
                operand: expr.clone(),
            })),
        }
    }
    
    /// 尝试转换为整数
    pub fn to_integer(&self) -> Option<BigInt> {
        match self {
            Number::Integer(i) => Some(i.clone()),
            Number::Rational(r) if r.denom() == &BigInt::from(1) => Some(r.numer().clone()),
            Number::Real(r) => {
                // 检查是否为整数
                if let Some(int_val) = r.to_bigint() {
                    if BigDecimal::from(int_val.clone()) == *r {
                        Some(int_val)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Number::Float(f) if f.fract() == 0.0 => {
                BigInt::from_f64(*f)
            }
            Number::Complex { real, imaginary } if imaginary.is_zero() => {
                real.to_integer()
            }
            _ => None,
        }
    }
    
    /// 尝试转换为有理数
    pub fn to_rational(&self) -> Option<BigRational> {
        match self {
            Number::Integer(i) => Some(BigRational::from(i.clone())),
            Number::Rational(r) => Some(r.clone()),
            Number::Float(f) => Self::float_to_rational(*f),
            Number::Complex { real, imaginary } if imaginary.is_zero() => {
                real.to_rational()
            }
            _ => None,
        }
    }
    
    /// 类型提升：将两个数值提升到相同的类型
    pub fn promote_types(a: &Number, b: &Number) -> (Number, Number) {
        match (a, b) {
            // 如果类型相同，直接返回
            (Number::Integer(_), Number::Integer(_)) => (a.clone(), b.clone()),
            (Number::Rational(_), Number::Rational(_)) => (a.clone(), b.clone()),
            (Number::Real(_), Number::Real(_)) => (a.clone(), b.clone()),
            (Number::Float(_), Number::Float(_)) => (a.clone(), b.clone()),
            
            // 整数到有理数
            (Number::Integer(i), Number::Rational(_)) => {
                (Number::Rational(BigRational::from(i.clone())), b.clone())
            }
            (Number::Rational(_), Number::Integer(i)) => {
                (a.clone(), Number::Rational(BigRational::from(i.clone())))
            }
            
            // 整数/有理数到实数
            (Number::Integer(i), Number::Real(_)) => {
                (Number::Real(BigDecimal::from(i.clone())), b.clone())
            }
            (Number::Real(_), Number::Integer(i)) => {
                (a.clone(), Number::Real(BigDecimal::from(i.clone())))
            }
            (Number::Rational(r), Number::Real(_)) => {
                if let Some(decimal) = r.to_f64().and_then(BigDecimal::from_f64) {
                    (Number::Real(decimal), b.clone())
                } else {
                    (a.clone(), b.clone())
                }
            }
            (Number::Real(_), Number::Rational(r)) => {
                if let Some(decimal) = r.to_f64().and_then(BigDecimal::from_f64) {
                    (a.clone(), Number::Real(decimal))
                } else {
                    (a.clone(), b.clone())
                }
            }
            
            // 到浮点数的转换（精度损失）
            (Number::Float(_), _) => (a.clone(), Number::Float(b.approximate())),
            (_, Number::Float(_)) => (Number::Float(a.approximate()), b.clone()),
            
            // 复数处理
            (Number::Complex { .. }, _) if b.is_real() => {
                (a.clone(), Number::Complex {
                    real: Box::new(b.clone()),
                    imaginary: Box::new(Number::zero()),
                })
            }
            (_, Number::Complex { .. }) if a.is_real() => {
                (Number::Complex {
                    real: Box::new(a.clone()),
                    imaginary: Box::new(Number::zero()),
                }, b.clone())
            }
            
            // 其他情况保持原样
            _ => (a.clone(), b.clone()),
        }
    }
}

use std::ops::{Add, Sub, Mul, Div, Neg};

// 实现加法运算
impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        let (a, b) = Number::promote_types(&self, &other);
        
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Number::Integer(a + b)
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Number::Rational(a + b)
            }
            (Number::Real(a), Number::Real(b)) => {
                Number::Real(a + b)
            }
            (Number::Float(a), Number::Float(b)) => {
                Number::Float(a + b)
            }
            (Number::Complex { real: real_a, imaginary: imag_a }, 
             Number::Complex { real: real_b, imaginary: imag_b }) => {
                Number::Complex {
                    real: Box::new(*real_a + *real_b),
                    imaginary: Box::new(*imag_a + *imag_b),
                }
            }
            // 对于无法精确计算的情况，返回符号表示
            _ => Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                op: crate::core::BinaryOperator::Add,
                left: Box::new(crate::core::Expression::Number(self)),
                right: Box::new(crate::core::Expression::Number(other)),
            }))
        }
    }
}

// 实现减法运算
impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        let (a, b) = Number::promote_types(&self, &other);
        
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Number::Integer(a - b)
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Number::Rational(a - b)
            }
            (Number::Real(a), Number::Real(b)) => {
                Number::Real(a - b)
            }
            (Number::Float(a), Number::Float(b)) => {
                Number::Float(a - b)
            }
            (Number::Complex { real: real_a, imaginary: imag_a }, 
             Number::Complex { real: real_b, imaginary: imag_b }) => {
                Number::Complex {
                    real: Box::new(*real_a - *real_b),
                    imaginary: Box::new(*imag_a - *imag_b),
                }
            }
            // 对于无法精确计算的情况，返回符号表示
            _ => Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                op: crate::core::BinaryOperator::Subtract,
                left: Box::new(crate::core::Expression::Number(self)),
                right: Box::new(crate::core::Expression::Number(other)),
            }))
        }
    }
}

// 实现乘法运算
impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        // 特殊情况：零乘以任何数都是零
        if self.is_zero() || other.is_zero() {
            return Number::zero();
        }
        
        // 特殊情况：一乘以任何数都是该数本身
        if self.is_one() {
            return other;
        }
        if other.is_one() {
            return self;
        }
        
        let (a, b) = Number::promote_types(&self, &other);
        
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                Number::Integer(a * b)
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Number::Rational(a * b)
            }
            (Number::Real(a), Number::Real(b)) => {
                Number::Real(a * b)
            }
            (Number::Float(a), Number::Float(b)) => {
                Number::Float(a * b)
            }
            (Number::Complex { real: real_a, imaginary: imag_a }, 
             Number::Complex { real: real_b, imaginary: imag_b }) => {
                // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
                let real_part = (*real_a.clone() * *real_b.clone()) - (*imag_a.clone() * *imag_b.clone());
                let imag_part = (*real_a * *imag_b) + (*imag_a * *real_b);
                Number::Complex {
                    real: Box::new(real_part),
                    imaginary: Box::new(imag_part),
                }
            }
            // 对于无法精确计算的情况，返回符号表示
            _ => Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                op: crate::core::BinaryOperator::Multiply,
                left: Box::new(crate::core::Expression::Number(self)),
                right: Box::new(crate::core::Expression::Number(other)),
            }))
        }
    }
}

// 实现除法运算
impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        // 检查除零错误
        if other.is_zero() {
            // 返回符号表示，表示除零错误
            return Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                op: crate::core::BinaryOperator::Divide,
                left: Box::new(crate::core::Expression::Number(self)),
                right: Box::new(crate::core::Expression::Number(other)),
            }));
        }
        
        // 特殊情况：零除以任何非零数都是零
        if self.is_zero() {
            return Number::zero();
        }
        
        // 特殊情况：任何数除以一都是该数本身
        if other.is_one() {
            return self;
        }
        
        let (a, b) = Number::promote_types(&self, &other);
        
        match (a, b) {
            (Number::Integer(a), Number::Integer(b)) => {
                // 整数除法：如果能整除则返回整数，否则返回有理数
                if &a % &b == BigInt::from(0) {
                    Number::Integer(a / b)
                } else {
                    Number::Rational(BigRational::new(a, b))
                }
            }
            (Number::Rational(a), Number::Rational(b)) => {
                Number::Rational(a / b)
            }
            (Number::Real(a), Number::Real(b)) => {
                Number::Real(a / b)
            }
            (Number::Float(a), Number::Float(b)) => {
                Number::Float(a / b)
            }
            (Number::Complex { real: real_a, imaginary: imag_a }, 
             Number::Complex { real: real_b, imaginary: imag_b }) => {
                // (a + bi) / (c + di) = [(a + bi)(c - di)] / (c² + d²)
                let denominator = (*real_b.clone() * *real_b.clone()) + (*imag_b.clone() * *imag_b.clone());
                let numerator_real = (*real_a.clone() * *real_b.clone()) + (*imag_a.clone() * *imag_b.clone());
                let numerator_imag = (*imag_a * *real_b) - (*real_a * *imag_b);
                
                Number::Complex {
                    real: Box::new(numerator_real / denominator.clone()),
                    imaginary: Box::new(numerator_imag / denominator),
                }
            }
            // 对于无法精确计算的情况，返回符号表示
            _ => Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                op: crate::core::BinaryOperator::Divide,
                left: Box::new(crate::core::Expression::Number(self)),
                right: Box::new(crate::core::Expression::Number(other)),
            }))
        }
    }
}

// 实现负号运算
impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        match self {
            Number::Integer(i) => Number::Integer(-i),
            Number::Rational(r) => Number::Rational(-r),
            Number::Real(r) => Number::Real(-r),
            Number::Float(f) => Number::Float(-f),
            Number::Complex { real, imaginary } => Number::Complex {
                real: Box::new(-*real),
                imaginary: Box::new(-*imaginary),
            },
            Number::Symbolic(expr) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Negate,
                operand: expr,
            })),
        }
    }
}

// 为引用类型实现运算符
impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, other: &Number) -> Number {
        self.clone() + other.clone()
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;

    fn sub(self, other: &Number) -> Number {
        self.clone() - other.clone()
    }
}

impl Mul<&Number> for &Number {
    type Output = Number;

    fn mul(self, other: &Number) -> Number {
        self.clone() * other.clone()
    }
}

impl Div<&Number> for &Number {
    type Output = Number;

    fn div(self, other: &Number) -> Number {
        self.clone() / other.clone()
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "number_tests.rs"]
mod number_tests;

#[cfg(test)]
#[path = "arithmetic_tests.rs"]
mod arithmetic_tests;