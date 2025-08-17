//! # 数学常量定义
//!
//! 定义常用的数学常量，如 π、e、i 等，并提供精确处理和运算规则。

use super::Number;
use num_bigint::BigInt;
use num_rational::BigRational;
use bigdecimal::BigDecimal;
use std::collections::HashMap;

/// 数学常量类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MathConstant {
    /// 圆周率 π
    Pi,
    /// 自然常数 e
    E,
    /// 虚数单位 i
    I,
    /// 欧拉-马歇罗尼常数 γ
    EulerGamma,
    /// 黄金比例 φ
    GoldenRatio,
    /// 卡塔兰常数 G
    Catalan,
    /// 正无穷
    PositiveInfinity,
    /// 负无穷
    NegativeInfinity,
    /// 未定义（NaN）
    Undefined,
}

impl MathConstant {
    /// 获取常量的数值近似值
    pub fn approximate_value(&self) -> f64 {
        match self {
            MathConstant::Pi => std::f64::consts::PI,
            MathConstant::E => std::f64::consts::E,
            MathConstant::I => f64::NAN, // 复数单位需要特殊处理
            MathConstant::EulerGamma => 0.5772156649015329,
            MathConstant::GoldenRatio => 1.618033988749895,
            MathConstant::Catalan => 0.915965594177219,
            MathConstant::PositiveInfinity => f64::INFINITY,
            MathConstant::NegativeInfinity => f64::NEG_INFINITY,
            MathConstant::Undefined => f64::NAN,
        }
    }
    
    /// 获取常量的符号表示
    pub fn symbol(&self) -> &'static str {
        match self {
            MathConstant::Pi => "π",
            MathConstant::E => "e",
            MathConstant::I => "i",
            MathConstant::EulerGamma => "γ",
            MathConstant::GoldenRatio => "φ",
            MathConstant::Catalan => "G",
            MathConstant::PositiveInfinity => "∞",
            MathConstant::NegativeInfinity => "-∞",
            MathConstant::Undefined => "undefined",
        }
    }
    
    /// 从字符串解析常量
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pi" | "π" => Some(MathConstant::Pi),
            "e" => Some(MathConstant::E),
            "i" | "j" => Some(MathConstant::I),
            "gamma" | "γ" => Some(MathConstant::EulerGamma),
            "phi" | "φ" => Some(MathConstant::GoldenRatio),
            "catalan" | "g" => Some(MathConstant::Catalan),
            "inf" | "infinity" | "∞" => Some(MathConstant::PositiveInfinity),
            "-inf" | "-infinity" | "-∞" => Some(MathConstant::NegativeInfinity),
            "nan" | "undefined" => Some(MathConstant::Undefined),
            _ => None,
        }
    }
    
    /// 获取常量的完整名称
    pub fn name(&self) -> &'static str {
        match self {
            MathConstant::Pi => "圆周率",
            MathConstant::E => "自然常数",
            MathConstant::I => "虚数单位",
            MathConstant::EulerGamma => "欧拉-马歇罗尼常数",
            MathConstant::GoldenRatio => "黄金比例",
            MathConstant::Catalan => "卡塔兰常数",
            MathConstant::PositiveInfinity => "正无穷",
            MathConstant::NegativeInfinity => "负无穷",
            MathConstant::Undefined => "未定义",
        }
    }
    
    /// 检查常量是否为实数
    pub fn is_real(&self) -> bool {
        match self {
            MathConstant::I => false, // 虚数单位不是实数
            MathConstant::Undefined => false, // 未定义不是实数
            _ => true,
        }
    }
    
    /// 检查常量是否为复数
    pub fn is_complex(&self) -> bool {
        matches!(self, MathConstant::I)
    }
    
    /// 将常量转换为精确的数值表示（如果可能）
    pub fn to_exact_number(&self) -> Option<Number> {
        match self {
            MathConstant::I => {
                // 虚数单位 i = 0 + 1i
                Some(Number::Complex {
                    real: Box::new(Number::zero()),
                    imaginary: Box::new(Number::one()),
                })
            }
            MathConstant::PositiveInfinity => Some(Number::Float(f64::INFINITY)),
            MathConstant::NegativeInfinity => Some(Number::Float(f64::NEG_INFINITY)),
            MathConstant::Undefined => Some(Number::Float(f64::NAN)),
            _ => None, // 其他常量保持符号形式
        }
    }
    
    /// 获取常量的高精度数值近似（使用 BigDecimal）
    pub fn to_high_precision(&self, _precision: usize) -> Option<BigDecimal> {
        use num_traits::FromPrimitive;
        
        match self {
            MathConstant::Pi => {
                // 使用高精度 π 计算（这里使用简化版本，实际应该使用更精确的算法）
                BigDecimal::from_f64(std::f64::consts::PI)
            }
            MathConstant::E => {
                // 使用高精度 e 计算
                BigDecimal::from_f64(std::f64::consts::E)
            }
            MathConstant::EulerGamma => {
                BigDecimal::from_f64(0.5772156649015329)
            }
            MathConstant::GoldenRatio => {
                // φ = (1 + √5) / 2
                BigDecimal::from_f64(1.618033988749895)
            }
            MathConstant::Catalan => {
                BigDecimal::from_f64(0.915965594177219)
            }
            // 复数和无穷大不能用 BigDecimal 表示
            _ => None,
        }
    }
    

    
    /// 检查常量是否为有限值
    pub fn is_finite(&self) -> bool {
        match self {
            MathConstant::PositiveInfinity | MathConstant::NegativeInfinity | MathConstant::Undefined => false,
            _ => true,
        }
    }
    
    /// 检查常量是否为无穷大
    pub fn is_infinite(&self) -> bool {
        matches!(self, MathConstant::PositiveInfinity | MathConstant::NegativeInfinity)
    }
    
    /// 检查常量是否未定义
    pub fn is_undefined(&self) -> bool {
        matches!(self, MathConstant::Undefined)
    }
    
    /// 获取常量的数学性质描述
    pub fn properties(&self) -> Vec<&'static str> {
        match self {
            MathConstant::Pi => vec!["无理数", "超越数", "正数"],
            MathConstant::E => vec!["无理数", "超越数", "正数"],
            MathConstant::I => vec!["虚数单位", "复数"],
            MathConstant::EulerGamma => vec!["实数", "可能是无理数"],
            MathConstant::GoldenRatio => vec!["无理数", "代数数", "正数"],
            MathConstant::Catalan => vec!["实数", "可能是无理数", "正数"],
            MathConstant::PositiveInfinity => vec!["无穷大", "正数"],
            MathConstant::NegativeInfinity => vec!["无穷大", "负数"],
            MathConstant::Undefined => vec!["未定义"],
        }
    }
    
    /// 获取所有支持的常量列表
    pub fn all_constants() -> Vec<MathConstant> {
        vec![
            MathConstant::Pi,
            MathConstant::E,
            MathConstant::I,
            MathConstant::EulerGamma,
            MathConstant::GoldenRatio,
            MathConstant::Catalan,
            MathConstant::PositiveInfinity,
            MathConstant::NegativeInfinity,
            MathConstant::Undefined,
        ]
    }
    
    /// 获取常量的别名列表
    pub fn aliases(&self) -> Vec<&'static str> {
        match self {
            MathConstant::Pi => vec!["pi", "π", "PI"],
            MathConstant::E => vec!["e", "E", "euler"],
            MathConstant::I => vec!["i", "I", "j", "J"],
            MathConstant::EulerGamma => vec!["gamma", "γ", "euler_gamma"],
            MathConstant::GoldenRatio => vec!["phi", "φ", "golden", "golden_ratio"],
            MathConstant::Catalan => vec!["catalan", "G", "catalan_constant"],
            MathConstant::PositiveInfinity => vec!["inf", "infinity", "∞", "+inf"],
            MathConstant::NegativeInfinity => vec!["-inf", "-infinity", "-∞"],
            MathConstant::Undefined => vec!["nan", "undefined", "NaN"],
        }
    }
}

/// 数学常量的运算规则
impl MathConstant {
    /// 应用加法运算规则
    pub fn add_rule(&self, other: &MathConstant) -> Option<super::Expression> {
        use super::Expression;
        
        match (self, other) {
            // i + i = 2i
            (MathConstant::I, MathConstant::I) => {
                Some(Expression::BinaryOp {
                    op: super::BinaryOperator::Multiply,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(Expression::Constant(MathConstant::I)),
                })
            }
            // 无穷大运算规则
            (MathConstant::PositiveInfinity, MathConstant::NegativeInfinity) |
            (MathConstant::NegativeInfinity, MathConstant::PositiveInfinity) => {
                Some(Expression::Constant(MathConstant::Undefined))
            }
            (MathConstant::PositiveInfinity, _) if other.is_finite() => {
                Some(Expression::Constant(MathConstant::PositiveInfinity))
            }
            (MathConstant::NegativeInfinity, _) if other.is_finite() => {
                Some(Expression::Constant(MathConstant::NegativeInfinity))
            }
            (_, MathConstant::PositiveInfinity) if self.is_finite() => {
                Some(Expression::Constant(MathConstant::PositiveInfinity))
            }
            (_, MathConstant::NegativeInfinity) if self.is_finite() => {
                Some(Expression::Constant(MathConstant::NegativeInfinity))
            }
            // 未定义值的运算
            (MathConstant::Undefined, _) | (_, MathConstant::Undefined) => {
                Some(Expression::Constant(MathConstant::Undefined))
            }
            // 其他情况不应用特殊规则
            _ => None,
        }
    }
    
    /// 应用乘法运算规则
    pub fn multiply_rule(&self, other: &MathConstant) -> Option<super::Expression> {
        use super::Expression;
        
        match (self, other) {
            // i * i = -1
            (MathConstant::I, MathConstant::I) => {
                Some(Expression::Number(Number::Integer(BigInt::from(-1))))
            }
            // 无穷大乘法规则
            (MathConstant::PositiveInfinity, MathConstant::PositiveInfinity) |
            (MathConstant::NegativeInfinity, MathConstant::NegativeInfinity) => {
                Some(Expression::Constant(MathConstant::PositiveInfinity))
            }
            (MathConstant::PositiveInfinity, MathConstant::NegativeInfinity) |
            (MathConstant::NegativeInfinity, MathConstant::PositiveInfinity) => {
                Some(Expression::Constant(MathConstant::NegativeInfinity))
            }
            // 0 * ∞ = undefined（需要在调用处检查）
            // 其他有限数与无穷大的乘法
            (MathConstant::PositiveInfinity, _) | (_, MathConstant::PositiveInfinity) => {
                Some(Expression::Constant(MathConstant::PositiveInfinity))
            }
            (MathConstant::NegativeInfinity, _) | (_, MathConstant::NegativeInfinity) => {
                Some(Expression::Constant(MathConstant::NegativeInfinity))
            }
            // 未定义值的运算
            (MathConstant::Undefined, _) | (_, MathConstant::Undefined) => {
                Some(Expression::Constant(MathConstant::Undefined))
            }
            // 其他情况不应用特殊规则
            _ => None,
        }
    }
    
    /// 应用幂运算规则
    pub fn power_rule(&self, exponent: &super::Expression) -> Option<super::Expression> {
        use super::Expression;
        
        match (self, exponent) {
            // e^(i*π) = -1 (欧拉公式)
            (MathConstant::E, Expression::BinaryOp { 
                op: super::BinaryOperator::Multiply, 
                left, 
                right 
            }) => {
                // 检查是否为 i*π 或 π*i
                let is_i_pi = matches!(
                    (left.as_ref(), right.as_ref()),
                    (Expression::Constant(MathConstant::I), Expression::Constant(MathConstant::Pi)) |
                    (Expression::Constant(MathConstant::Pi), Expression::Constant(MathConstant::I))
                );
                
                if is_i_pi {
                    Some(Expression::Number(Number::Integer(BigInt::from(-1))))
                } else {
                    None
                }
            }
            // i^2 = -1
            (MathConstant::I, Expression::Number(Number::Integer(n))) if n == &BigInt::from(2) => {
                Some(Expression::Number(Number::Integer(BigInt::from(-1))))
            }
            // i^4 = 1
            (MathConstant::I, Expression::Number(Number::Integer(n))) if n == &BigInt::from(4) => {
                Some(Expression::Number(Number::Integer(BigInt::from(1))))
            }
            // 其他情况不应用特殊规则
            _ => None,
        }
    }
    
    /// 应用三角函数规则
    pub fn trigonometric_rule(&self, function: &str) -> Option<super::Expression> {
        use super::Expression;
        
        match (function, self) {
            // sin(0) = 0, cos(0) = 1
            ("sin", MathConstant::Pi) => {
                // sin(π) = 0
                Some(Expression::Number(Number::Integer(BigInt::from(0))))
            }
            ("cos", MathConstant::Pi) => {
                // cos(π) = -1
                Some(Expression::Number(Number::Integer(BigInt::from(-1))))
            }
            // 其他特殊值可以继续添加
            _ => None,
        }
    }
}

/// 常量缓存，用于提高性能
pub struct ConstantCache {
    /// 高精度数值缓存
    precision_cache: HashMap<(MathConstant, usize), BigDecimal>,
    /// 运算结果缓存
    operation_cache: HashMap<String, super::Expression>,
}

impl ConstantCache {
    /// 创建新的常量缓存
    pub fn new() -> Self {
        Self {
            precision_cache: HashMap::new(),
            operation_cache: HashMap::new(),
        }
    }
    
    /// 获取或计算高精度常量值
    pub fn get_high_precision(&mut self, constant: &MathConstant, precision: usize) -> Option<BigDecimal> {
        let key = (constant.clone(), precision);
        
        if let Some(cached) = self.precision_cache.get(&key) {
            Some(cached.clone())
        } else if let Some(value) = constant.to_high_precision(precision) {
            self.precision_cache.insert(key, value.clone());
            Some(value)
        } else {
            None
        }
    }
    
    /// 清理缓存
    pub fn clear(&mut self) {
        self.precision_cache.clear();
        self.operation_cache.clear();
    }
    
    /// 获取缓存统计信息
    pub fn stats(&self) -> (usize, usize) {
        (self.precision_cache.len(), self.operation_cache.len())
    }
}

impl Default for ConstantCache {
    fn default() -> Self {
        Self::new()
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "constants_tests.rs"]
mod constants_tests;