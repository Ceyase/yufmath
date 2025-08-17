//! # 数值类型定义
//!
//! 定义支持精确计算的数值类型系统，包括任意精度整数、
//! 有理数、实数和复数等类型。

use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use bigdecimal::{BigDecimal, Zero, ToPrimitive, FromPrimitive};
use num_traits::{Signed};
use std::fmt::{self, Display, Debug};

/// 支持多种数值类型的统一表示，优先使用精确表示
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// 数学常量
    Constant(crate::core::MathConstant),
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
            Number::Constant(_) => false,
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
            Number::Constant(_) => false,
            Number::Symbolic(_) => false,
        }
    }
    
    /// 检查是否为二
    pub fn is_two(&self) -> bool {
        match self {
            Number::Integer(i) => i == &BigInt::from(2),
            Number::Rational(r) => r == &BigRational::from(BigInt::from(2)),
            Number::Real(r) => r == &BigDecimal::from(2),
            Number::Float(f) => *f == 2.0,
            Number::Complex { real, imaginary } => real.is_two() && imaginary.is_zero(),
            Number::Constant(_) => false,
            Number::Symbolic(_) => false,
        }
    }
    
    /// 获取数值近似值（仅在需要时使用）
    pub fn approximate(&self) -> f64 {
        match self {
            Number::Integer(i) => ToPrimitive::to_f64(i).unwrap_or(f64::INFINITY),
            Number::Rational(r) => ToPrimitive::to_f64(r).unwrap_or(f64::NAN),
            Number::Real(r) => ToPrimitive::to_f64(r).unwrap_or(f64::NAN),
            Number::Complex { real, imaginary } => {
                // 对于复数，返回模长
                let r = real.approximate();
                let i = imaginary.approximate();
                (r * r + i * i).sqrt()
            }
            Number::Constant(c) => c.approximate_value(),
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
            Number::Constant(c) => write!(f, "{}", c.symbol()),
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
            Number::Constant(c) => c.approximate_value() < 0.0,
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
            Number::Constant(c) => c.approximate_value() > 0.0,
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
            Number::Constant(_) => false,
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
            Number::Constant(_) => false,
            _ => false,
        }
    }
    
    /// 检查是否为实数
    pub fn is_real(&self) -> bool {
        match self {
            Number::Integer(_) | Number::Rational(_) | Number::Real(_) | Number::Float(_) => true,
            Number::Complex { imaginary, .. } => imaginary.is_zero(),
            Number::Constant(c) => c.is_real(),
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
    pub fn abs(&self) -> Result<Self, crate::engine::ComputeError> {
        Ok(match self {
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
                    _ => return Ok(Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Abs,
                        operand: Box::new(crate::core::Expression::Number(self.clone())),
                    }))),
                };
                
                let imag_sq = match imaginary.as_ref() {
                    Number::Integer(i) => Number::Integer(i * i),
                    Number::Rational(i) => Number::Rational(i * i),
                    Number::Real(i) => Number::Real(i * i),
                    Number::Float(i) => Number::Float(i * i),
                    _ => return Ok(Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                        op: crate::core::UnaryOperator::Abs,
                        operand: Box::new(crate::core::Expression::Number(self.clone())),
                    }))),
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
            Number::Constant(_) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Abs,
                operand: Box::new(crate::core::Expression::Number(self.clone())),
            })),
            Number::Symbolic(_) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Abs,
                operand: Box::new(crate::core::Expression::Number(self.clone())),
            })),
        })
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
            Number::Constant(_) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Negate,
                operand: Box::new(crate::core::Expression::Number(self.clone())),
            })),
            Number::Symbolic(expr) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Negate,
                operand: expr.clone(),
            })),
        }
    }
    
    /// 检查是否为偶数
    pub fn is_even(&self) -> bool {
        match self {
            Number::Integer(i) => i % BigInt::from(2) == BigInt::from(0),
            Number::Rational(r) => {
                if r.denom() == &BigInt::from(1) {
                    r.numer() % BigInt::from(2) == BigInt::from(0)
                } else {
                    false
                }
            }
            Number::Float(f) => f.fract() == 0.0 && (*f as i64) % 2 == 0,
            Number::Complex { real, imaginary } => {
                imaginary.is_zero() && real.is_even()
            }
            _ => false,
        }
    }
    
    /// 取负值（别名）
    pub fn negate(&self) -> Result<Number, crate::engine::ComputeError> {
        Ok(self.neg())
    }
    
    /// 加法运算
    pub fn add(&self, other: &Number) -> Result<Number, crate::engine::ComputeError> {
        Ok(self.clone() + other.clone())
    }
    
    /// 减法运算
    pub fn subtract(&self, other: &Number) -> Result<Number, crate::engine::ComputeError> {
        Ok(self.clone() - other.clone())
    }
    
    /// 乘法运算
    pub fn multiply(&self, other: &Number) -> Result<Number, crate::engine::ComputeError> {
        Ok(self.clone() * other.clone())
    }
    
    /// 除法运算
    pub fn divide(&self, other: &Number) -> Result<Number, crate::engine::ComputeError> {
        if other.is_zero() {
            return Err(crate::engine::ComputeError::DivisionByZero);
        }
        Ok(self.clone() / other.clone())
    }
    
    /// 幂运算
    pub fn power(&self, other: &Number) -> Result<Number, crate::engine::ComputeError> {
        use num_traits::Pow;
        
        match (self, other) {
            (Number::Integer(base), Number::Integer(exp)) => {
                if exp >= &BigInt::from(0) {
                    if let Some(exp_u32) = exp.to_u32() {
                        Ok(Number::Integer(base.pow(exp_u32)))
                    } else {
                        // 指数太大，返回符号表示
                        Ok(Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                            op: crate::core::BinaryOperator::Power,
                            left: Box::new(crate::core::Expression::Number(self.clone())),
                            right: Box::new(crate::core::Expression::Number(other.clone())),
                        })))
                    }
                } else {
                    // 负指数，转换为有理数
                    let base_rational = BigRational::from(base.clone());
                    let exp_abs = exp.abs();
                    if let Some(exp_u32) = exp_abs.to_u32() {
                        let result = base_rational.pow(exp_u32);
                        Ok(Number::Rational(BigRational::from(BigInt::from(1)) / result))
                    } else {
                        Ok(Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                            op: crate::core::BinaryOperator::Power,
                            left: Box::new(crate::core::Expression::Number(self.clone())),
                            right: Box::new(crate::core::Expression::Number(other.clone())),
                        })))
                    }
                }
            }
            (Number::Rational(base), Number::Integer(exp)) => {
                if let Some(exp_i32) = exp.to_i32() {
                    Ok(Number::Rational(base.pow(exp_i32)))
                } else {
                    Ok(Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                        op: crate::core::BinaryOperator::Power,
                        left: Box::new(crate::core::Expression::Number(self.clone())),
                        right: Box::new(crate::core::Expression::Number(other.clone())),
                    })))
                }
            }
            (Number::Float(base), Number::Float(exp)) => {
                Ok(Number::Float(base.powf(*exp)))
            }
            _ => {
                // 对于其他情况，返回符号表示
                Ok(Number::Symbolic(Box::new(crate::core::Expression::BinaryOp {
                    op: crate::core::BinaryOperator::Power,
                    left: Box::new(crate::core::Expression::Number(self.clone())),
                    right: Box::new(crate::core::Expression::Number(other.clone())),
                })))
            }
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
    
    /// 尝试转换为 f64
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Number::Integer(i) => i.to_f64(),
            Number::Rational(r) => {
                let num = r.numer().to_f64()?;
                let den = r.denom().to_f64()?;
                Some(num / den)
            }
            Number::Real(r) => r.to_f64(),
            Number::Float(f) => Some(*f),
            Number::Complex { real, imaginary } if imaginary.is_zero() => {
                real.to_f64()
            }
            Number::Constant(c) => Some(c.approximate_value()),
            _ => None,
        }
    }
    
    /// 尝试转换为 i64
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => i.to_i64(),
            Number::Rational(r) if r.denom() == &BigInt::from(1) => {
                r.numer().to_i64()
            }
            Number::Real(r) => {
                if let Some(int_val) = r.to_bigint() {
                    int_val.to_i64()
                } else {
                    None
                }
            }
            Number::Float(f) if f.fract() == 0.0 => Some(*f as i64),
            Number::Complex { real, imaginary } if imaginary.is_zero() => {
                real.to_i64()
            }
            _ => None,
        }
    }
    
    /// 获取数值的类型
    pub fn get_numeric_type(&self) -> crate::core::NumericType {
        match self {
            Number::Integer(_) => crate::core::NumericType::Integer,
            Number::Rational(_) => crate::core::NumericType::Rational,
            Number::Real(_) => crate::core::NumericType::Real,
            Number::Float(_) => crate::core::NumericType::Float,
            Number::Complex { .. } => crate::core::NumericType::Complex,
            Number::Constant(c) => {
                if c.is_complex() {
                    crate::core::NumericType::Complex
                } else {
                    crate::core::NumericType::Real
                }
            }
            Number::Symbolic(_) => crate::core::NumericType::Real, // 符号表达式默认为实数类型
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
                // 直接将有理数转换为 BigDecimal，避免通过 f64 的精度损失
                let decimal = BigDecimal::new(r.numer().clone(), 0) / BigDecimal::new(r.denom().clone(), 0);
                (Number::Real(decimal), b.clone())
            }
            (Number::Real(_), Number::Rational(r)) => {
                // 直接将有理数转换为 BigDecimal，避免通过 f64 的精度损失
                let decimal = BigDecimal::new(r.numer().clone(), 0) / BigDecimal::new(r.denom().clone(), 0);
                (a.clone(), Number::Real(decimal))
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
                
                // 如果虚部为0，返回实数
                if imag_part.is_zero() {
                    real_part
                } else {
                    Number::Complex {
                        real: Box::new(real_part),
                        imaginary: Box::new(imag_part),
                    }
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
            Number::Constant(_) => Number::Symbolic(Box::new(crate::core::Expression::UnaryOp {
                op: crate::core::UnaryOperator::Negate,
                operand: Box::new(crate::core::Expression::Number(self)),
            })),
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

// 手动实现 Eq trait
impl Eq for Number {}

// 手动实现 Hash trait
impl std::hash::Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use std::hash::Hash;
        
        match self {
            Number::Integer(i) => {
                0u8.hash(state);
                i.hash(state);
            }
            Number::Rational(r) => {
                1u8.hash(state);
                r.hash(state);
            }
            Number::Real(r) => {
                2u8.hash(state);
                // BigDecimal 不实现 Hash，我们使用其字符串表示
                r.to_string().hash(state);
            }
            Number::Float(f) => {
                3u8.hash(state);
                // f64 不实现 Hash，我们使用其位表示
                f.to_bits().hash(state);
            }
            Number::Complex { real, imaginary } => {
                4u8.hash(state);
                real.hash(state);
                imaginary.hash(state);
            }
            Number::Constant(c) => {
                5u8.hash(state);
                c.hash(state);
            }
            Number::Symbolic(expr) => {
                6u8.hash(state);
                expr.hash(state);
            }
        }
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "number_tests.rs"]
mod number_tests;

#[cfg(test)]
#[path = "arithmetic_tests.rs"]
mod arithmetic_tests;