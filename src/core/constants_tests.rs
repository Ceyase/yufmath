//! # 数学常量测试
//!
//! 测试数学常量的各种功能和运算规则。

use super::*;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_creation() {
        // 测试所有常量的创建
        let constants = MathConstant::all_constants();
        assert_eq!(constants.len(), 9);
        
        // 验证每个常量都能正确创建
        assert!(constants.contains(&MathConstant::Pi));
        assert!(constants.contains(&MathConstant::E));
        assert!(constants.contains(&MathConstant::I));
        assert!(constants.contains(&MathConstant::EulerGamma));
        assert!(constants.contains(&MathConstant::GoldenRatio));
        assert!(constants.contains(&MathConstant::Catalan));
        assert!(constants.contains(&MathConstant::PositiveInfinity));
        assert!(constants.contains(&MathConstant::NegativeInfinity));
        assert!(constants.contains(&MathConstant::Undefined));
    }

    #[test]
    fn test_constant_symbols() {
        // 测试符号表示
        assert_eq!(MathConstant::Pi.symbol(), "π");
        assert_eq!(MathConstant::E.symbol(), "e");
        assert_eq!(MathConstant::I.symbol(), "i");
        assert_eq!(MathConstant::EulerGamma.symbol(), "γ");
        assert_eq!(MathConstant::GoldenRatio.symbol(), "φ");
        assert_eq!(MathConstant::Catalan.symbol(), "G");
        assert_eq!(MathConstant::PositiveInfinity.symbol(), "∞");
        assert_eq!(MathConstant::NegativeInfinity.symbol(), "-∞");
        assert_eq!(MathConstant::Undefined.symbol(), "undefined");
    }

    #[test]
    fn test_constant_names() {
        // 测试完整名称
        assert_eq!(MathConstant::Pi.name(), "圆周率");
        assert_eq!(MathConstant::E.name(), "自然常数");
        assert_eq!(MathConstant::I.name(), "虚数单位");
        assert_eq!(MathConstant::EulerGamma.name(), "欧拉-马歇罗尼常数");
        assert_eq!(MathConstant::GoldenRatio.name(), "黄金比例");
        assert_eq!(MathConstant::Catalan.name(), "卡塔兰常数");
        assert_eq!(MathConstant::PositiveInfinity.name(), "正无穷");
        assert_eq!(MathConstant::NegativeInfinity.name(), "负无穷");
        assert_eq!(MathConstant::Undefined.name(), "未定义");
    }

    #[test]
    fn test_constant_from_string() {
        // 测试字符串解析
        assert_eq!(MathConstant::from_str("pi"), Some(MathConstant::Pi));
        assert_eq!(MathConstant::from_str("π"), Some(MathConstant::Pi));
        assert_eq!(MathConstant::from_str("PI"), Some(MathConstant::Pi));
        
        assert_eq!(MathConstant::from_str("e"), Some(MathConstant::E));
        assert_eq!(MathConstant::from_str("E"), Some(MathConstant::E));
        
        assert_eq!(MathConstant::from_str("i"), Some(MathConstant::I));
        assert_eq!(MathConstant::from_str("I"), Some(MathConstant::I));
        assert_eq!(MathConstant::from_str("j"), Some(MathConstant::I));
        
        assert_eq!(MathConstant::from_str("gamma"), Some(MathConstant::EulerGamma));
        assert_eq!(MathConstant::from_str("γ"), Some(MathConstant::EulerGamma));
        
        assert_eq!(MathConstant::from_str("phi"), Some(MathConstant::GoldenRatio));
        assert_eq!(MathConstant::from_str("φ"), Some(MathConstant::GoldenRatio));
        
        assert_eq!(MathConstant::from_str("catalan"), Some(MathConstant::Catalan));
        assert_eq!(MathConstant::from_str("G"), Some(MathConstant::Catalan));
        
        assert_eq!(MathConstant::from_str("inf"), Some(MathConstant::PositiveInfinity));
        assert_eq!(MathConstant::from_str("∞"), Some(MathConstant::PositiveInfinity));
        
        assert_eq!(MathConstant::from_str("-inf"), Some(MathConstant::NegativeInfinity));
        assert_eq!(MathConstant::from_str("-∞"), Some(MathConstant::NegativeInfinity));
        
        assert_eq!(MathConstant::from_str("nan"), Some(MathConstant::Undefined));
        assert_eq!(MathConstant::from_str("undefined"), Some(MathConstant::Undefined));
        
        // 测试无效输入
        assert_eq!(MathConstant::from_str("invalid"), None);
        assert_eq!(MathConstant::from_str(""), None);
    }

    #[test]
    fn test_constant_approximate_values() {
        // 测试数值近似
        assert!((MathConstant::Pi.approximate_value() - std::f64::consts::PI).abs() < 1e-15);
        assert!((MathConstant::E.approximate_value() - std::f64::consts::E).abs() < 1e-15);
        assert!(MathConstant::I.approximate_value().is_nan());
        assert!((MathConstant::EulerGamma.approximate_value() - 0.5772156649015329).abs() < 1e-15);
        assert!((MathConstant::GoldenRatio.approximate_value() - 1.618033988749895).abs() < 1e-15);
        assert!((MathConstant::Catalan.approximate_value() - 0.915965594177219).abs() < 1e-15);
        assert!(MathConstant::PositiveInfinity.approximate_value().is_infinite());
        assert!(MathConstant::PositiveInfinity.approximate_value() > 0.0);
        assert!(MathConstant::NegativeInfinity.approximate_value().is_infinite());
        assert!(MathConstant::NegativeInfinity.approximate_value() < 0.0);
        assert!(MathConstant::Undefined.approximate_value().is_nan());
    }

    #[test]
    fn test_constant_to_exact_number() {
        // 测试精确数值转换
        
        // 虚数单位 i
        if let Some(Number::Complex { real, imaginary }) = MathConstant::I.to_exact_number() {
            assert_eq!(real.as_ref(), &Number::Integer(BigInt::from(0)));
            assert_eq!(imaginary.as_ref(), &Number::Integer(BigInt::from(1)));
        } else {
            panic!("虚数单位 i 应该转换为复数");
        }
        
        // 正无穷
        if let Some(Number::Float(f)) = MathConstant::PositiveInfinity.to_exact_number() {
            assert!(f.is_infinite() && f > 0.0);
        } else {
            panic!("正无穷应该转换为浮点数");
        }
        
        // 负无穷
        if let Some(Number::Float(f)) = MathConstant::NegativeInfinity.to_exact_number() {
            assert!(f.is_infinite() && f < 0.0);
        } else {
            panic!("负无穷应该转换为浮点数");
        }
        
        // 未定义
        if let Some(Number::Float(f)) = MathConstant::Undefined.to_exact_number() {
            assert!(f.is_nan());
        } else {
            panic!("未定义应该转换为 NaN");
        }
        
        // 其他常量应该返回 None（保持符号形式）
        assert!(MathConstant::Pi.to_exact_number().is_none());
        assert!(MathConstant::E.to_exact_number().is_none());
        assert!(MathConstant::EulerGamma.to_exact_number().is_none());
        assert!(MathConstant::GoldenRatio.to_exact_number().is_none());
        assert!(MathConstant::Catalan.to_exact_number().is_none());
    }

    #[test]
    fn test_constant_properties() {
        // 测试常量性质
        
        // 实数性质
        assert!(MathConstant::Pi.is_real());
        assert!(MathConstant::E.is_real());
        assert!(!MathConstant::I.is_real());
        assert!(MathConstant::EulerGamma.is_real());
        assert!(MathConstant::GoldenRatio.is_real());
        assert!(MathConstant::Catalan.is_real());
        assert!(MathConstant::PositiveInfinity.is_real());
        assert!(MathConstant::NegativeInfinity.is_real());
        assert!(!MathConstant::Undefined.is_real());
        
        // 复数性质
        assert!(!MathConstant::Pi.is_complex());
        assert!(!MathConstant::E.is_complex());
        assert!(MathConstant::I.is_complex());
        assert!(!MathConstant::EulerGamma.is_complex());
        assert!(!MathConstant::GoldenRatio.is_complex());
        assert!(!MathConstant::Catalan.is_complex());
        assert!(!MathConstant::PositiveInfinity.is_complex());
        assert!(!MathConstant::NegativeInfinity.is_complex());
        assert!(!MathConstant::Undefined.is_complex());
        
        // 有限性质
        assert!(MathConstant::Pi.is_finite());
        assert!(MathConstant::E.is_finite());
        assert!(MathConstant::I.is_finite());
        assert!(MathConstant::EulerGamma.is_finite());
        assert!(MathConstant::GoldenRatio.is_finite());
        assert!(MathConstant::Catalan.is_finite());
        assert!(!MathConstant::PositiveInfinity.is_finite());
        assert!(!MathConstant::NegativeInfinity.is_finite());
        assert!(!MathConstant::Undefined.is_finite());
        
        // 无穷性质
        assert!(!MathConstant::Pi.is_infinite());
        assert!(!MathConstant::E.is_infinite());
        assert!(!MathConstant::I.is_infinite());
        assert!(!MathConstant::EulerGamma.is_infinite());
        assert!(!MathConstant::GoldenRatio.is_infinite());
        assert!(!MathConstant::Catalan.is_infinite());
        assert!(MathConstant::PositiveInfinity.is_infinite());
        assert!(MathConstant::NegativeInfinity.is_infinite());
        assert!(!MathConstant::Undefined.is_infinite());
        
        // 未定义性质
        assert!(!MathConstant::Pi.is_undefined());
        assert!(!MathConstant::E.is_undefined());
        assert!(!MathConstant::I.is_undefined());
        assert!(!MathConstant::EulerGamma.is_undefined());
        assert!(!MathConstant::GoldenRatio.is_undefined());
        assert!(!MathConstant::Catalan.is_undefined());
        assert!(!MathConstant::PositiveInfinity.is_undefined());
        assert!(!MathConstant::NegativeInfinity.is_undefined());
        assert!(MathConstant::Undefined.is_undefined());
    }

    #[test]
    fn test_constant_mathematical_properties() {
        // 测试数学性质描述
        let pi_props = MathConstant::Pi.properties();
        assert!(pi_props.contains(&"无理数"));
        assert!(pi_props.contains(&"超越数"));
        assert!(pi_props.contains(&"正数"));
        
        let e_props = MathConstant::E.properties();
        assert!(e_props.contains(&"无理数"));
        assert!(e_props.contains(&"超越数"));
        assert!(e_props.contains(&"正数"));
        
        let i_props = MathConstant::I.properties();
        assert!(i_props.contains(&"虚数单位"));
        assert!(i_props.contains(&"复数"));
        
        let gamma_props = MathConstant::EulerGamma.properties();
        assert!(gamma_props.contains(&"实数"));
        assert!(gamma_props.contains(&"可能是无理数"));
        
        let phi_props = MathConstant::GoldenRatio.properties();
        assert!(phi_props.contains(&"无理数"));
        assert!(phi_props.contains(&"代数数"));
        assert!(phi_props.contains(&"正数"));
        
        let catalan_props = MathConstant::Catalan.properties();
        assert!(catalan_props.contains(&"实数"));
        assert!(catalan_props.contains(&"可能是无理数"));
        assert!(catalan_props.contains(&"正数"));
        
        let pos_inf_props = MathConstant::PositiveInfinity.properties();
        assert!(pos_inf_props.contains(&"无穷大"));
        assert!(pos_inf_props.contains(&"正数"));
        
        let neg_inf_props = MathConstant::NegativeInfinity.properties();
        assert!(neg_inf_props.contains(&"无穷大"));
        assert!(neg_inf_props.contains(&"负数"));
        
        let undef_props = MathConstant::Undefined.properties();
        assert!(undef_props.contains(&"未定义"));
    }

    #[test]
    fn test_constant_aliases() {
        // 测试别名
        let pi_aliases = MathConstant::Pi.aliases();
        assert!(pi_aliases.contains(&"pi"));
        assert!(pi_aliases.contains(&"π"));
        assert!(pi_aliases.contains(&"PI"));
        
        let e_aliases = MathConstant::E.aliases();
        assert!(e_aliases.contains(&"e"));
        assert!(e_aliases.contains(&"E"));
        assert!(e_aliases.contains(&"euler"));
        
        let i_aliases = MathConstant::I.aliases();
        assert!(i_aliases.contains(&"i"));
        assert!(i_aliases.contains(&"I"));
        assert!(i_aliases.contains(&"j"));
        assert!(i_aliases.contains(&"J"));
        
        let gamma_aliases = MathConstant::EulerGamma.aliases();
        assert!(gamma_aliases.contains(&"gamma"));
        assert!(gamma_aliases.contains(&"γ"));
        assert!(gamma_aliases.contains(&"euler_gamma"));
        
        let phi_aliases = MathConstant::GoldenRatio.aliases();
        assert!(phi_aliases.contains(&"phi"));
        assert!(phi_aliases.contains(&"φ"));
        assert!(phi_aliases.contains(&"golden"));
        assert!(phi_aliases.contains(&"golden_ratio"));
        
        let catalan_aliases = MathConstant::Catalan.aliases();
        assert!(catalan_aliases.contains(&"catalan"));
        assert!(catalan_aliases.contains(&"G"));
        assert!(catalan_aliases.contains(&"catalan_constant"));
        
        let pos_inf_aliases = MathConstant::PositiveInfinity.aliases();
        assert!(pos_inf_aliases.contains(&"inf"));
        assert!(pos_inf_aliases.contains(&"infinity"));
        assert!(pos_inf_aliases.contains(&"∞"));
        assert!(pos_inf_aliases.contains(&"+inf"));
        
        let neg_inf_aliases = MathConstant::NegativeInfinity.aliases();
        assert!(neg_inf_aliases.contains(&"-inf"));
        assert!(neg_inf_aliases.contains(&"-infinity"));
        assert!(neg_inf_aliases.contains(&"-∞"));
        
        let undef_aliases = MathConstant::Undefined.aliases();
        assert!(undef_aliases.contains(&"nan"));
        assert!(undef_aliases.contains(&"undefined"));
        assert!(undef_aliases.contains(&"NaN"));
    }

    #[test]
    fn test_constant_high_precision() {
        // 测试高精度数值
        let precision = 50;
        
        // π 的高精度值
        if let Some(pi_high) = MathConstant::Pi.to_high_precision(precision) {
            let pi_f64 = pi_high.to_f64().unwrap();
            assert!((pi_f64 - std::f64::consts::PI).abs() < 1e-10);
        } else {
            panic!("π 应该有高精度表示");
        }
        
        // e 的高精度值
        if let Some(e_high) = MathConstant::E.to_high_precision(precision) {
            let e_f64 = e_high.to_f64().unwrap();
            assert!((e_f64 - std::f64::consts::E).abs() < 1e-10);
        } else {
            panic!("e 应该有高精度表示");
        }
        
        // γ 的高精度值
        if let Some(gamma_high) = MathConstant::EulerGamma.to_high_precision(precision) {
            let gamma_f64 = gamma_high.to_f64().unwrap();
            assert!((gamma_f64 - 0.5772156649015329).abs() < 1e-10);
        } else {
            panic!("γ 应该有高精度表示");
        }
        
        // φ 的高精度值
        if let Some(phi_high) = MathConstant::GoldenRatio.to_high_precision(precision) {
            let phi_f64 = phi_high.to_f64().unwrap();
            assert!((phi_f64 - 1.618033988749895).abs() < 1e-10);
        } else {
            panic!("φ 应该有高精度表示");
        }
        
        // G 的高精度值
        if let Some(catalan_high) = MathConstant::Catalan.to_high_precision(precision) {
            let catalan_f64 = catalan_high.to_f64().unwrap();
            assert!((catalan_f64 - 0.915965594177219).abs() < 1e-10);
        } else {
            panic!("G 应该有高精度表示");
        }
        
        // 复数和无穷大不应该有高精度表示
        assert!(MathConstant::I.to_high_precision(precision).is_none());
        assert!(MathConstant::PositiveInfinity.to_high_precision(precision).is_none());
        assert!(MathConstant::NegativeInfinity.to_high_precision(precision).is_none());
        assert!(MathConstant::Undefined.to_high_precision(precision).is_none());
    }

    #[test]
    fn test_constant_cache() {
        let mut cache = ConstantCache::new();
        
        // 测试缓存统计
        let (precision_count, operation_count) = cache.stats();
        assert_eq!(precision_count, 0);
        assert_eq!(operation_count, 0);
        
        // 测试高精度缓存
        let precision = 50;
        let pi_value1 = cache.get_high_precision(&MathConstant::Pi, precision);
        assert!(pi_value1.is_some());
        
        let (precision_count, _) = cache.stats();
        assert_eq!(precision_count, 1);
        
        // 再次获取应该从缓存返回
        let pi_value2 = cache.get_high_precision(&MathConstant::Pi, precision);
        assert!(pi_value2.is_some());
        assert_eq!(pi_value1, pi_value2);
        
        // 缓存计数不应该增加
        let (precision_count, _) = cache.stats();
        assert_eq!(precision_count, 1);
        
        // 不同精度应该创建新的缓存项
        let pi_value3 = cache.get_high_precision(&MathConstant::Pi, precision + 10);
        assert!(pi_value3.is_some());
        
        let (precision_count, _) = cache.stats();
        assert_eq!(precision_count, 2);
        
        // 测试缓存清理
        cache.clear();
        let (precision_count, operation_count) = cache.stats();
        assert_eq!(precision_count, 0);
        assert_eq!(operation_count, 0);
    }
}

#[cfg(test)]
mod operation_rules_tests {
    use super::*;
    use crate::core::{Expression, BinaryOperator};

    #[test]
    fn test_addition_rules() {
        // i + i = 2i
        if let Some(result) = MathConstant::I.add_rule(&MathConstant::I) {
            match result {
                Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                    assert_eq!(left.as_ref(), &Expression::Number(Number::Integer(BigInt::from(2))));
                    assert_eq!(right.as_ref(), &Expression::Constant(MathConstant::I));
                }
                _ => panic!("i + i 应该返回 2i"),
            }
        } else {
            panic!("i + i 应该有加法规则");
        }
        
        // +∞ + (-∞) = undefined
        if let Some(result) = MathConstant::PositiveInfinity.add_rule(&MathConstant::NegativeInfinity) {
            assert_eq!(result, Expression::Constant(MathConstant::Undefined));
        } else {
            panic!("+∞ + (-∞) 应该返回 undefined");
        }
        
        // +∞ + 有限数 = +∞
        if let Some(result) = MathConstant::PositiveInfinity.add_rule(&MathConstant::Pi) {
            assert_eq!(result, Expression::Constant(MathConstant::PositiveInfinity));
        } else {
            panic!("+∞ + π 应该返回 +∞");
        }
        
        // -∞ + 有限数 = -∞
        if let Some(result) = MathConstant::NegativeInfinity.add_rule(&MathConstant::E) {
            assert_eq!(result, Expression::Constant(MathConstant::NegativeInfinity));
        } else {
            panic!("-∞ + e 应该返回 -∞");
        }
        
        // undefined + 任何数 = undefined
        if let Some(result) = MathConstant::Undefined.add_rule(&MathConstant::Pi) {
            assert_eq!(result, Expression::Constant(MathConstant::Undefined));
        } else {
            panic!("undefined + π 应该返回 undefined");
        }
        
        // 没有特殊规则的情况应该返回 None
        assert!(MathConstant::Pi.add_rule(&MathConstant::E).is_none());
    }

    #[test]
    fn test_multiplication_rules() {
        // i * i = -1
        if let Some(result) = MathConstant::I.multiply_rule(&MathConstant::I) {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(-1))));
        } else {
            panic!("i * i 应该返回 -1");
        }
        
        // +∞ * +∞ = +∞
        if let Some(result) = MathConstant::PositiveInfinity.multiply_rule(&MathConstant::PositiveInfinity) {
            assert_eq!(result, Expression::Constant(MathConstant::PositiveInfinity));
        } else {
            panic!("+∞ * +∞ 应该返回 +∞");
        }
        
        // +∞ * (-∞) = -∞
        if let Some(result) = MathConstant::PositiveInfinity.multiply_rule(&MathConstant::NegativeInfinity) {
            assert_eq!(result, Expression::Constant(MathConstant::NegativeInfinity));
        } else {
            panic!("+∞ * (-∞) 应该返回 -∞");
        }
        
        // -∞ * -∞ = +∞
        if let Some(result) = MathConstant::NegativeInfinity.multiply_rule(&MathConstant::NegativeInfinity) {
            assert_eq!(result, Expression::Constant(MathConstant::PositiveInfinity));
        } else {
            panic!("-∞ * -∞ 应该返回 +∞");
        }
        
        // undefined * 任何数 = undefined
        if let Some(result) = MathConstant::Undefined.multiply_rule(&MathConstant::Pi) {
            assert_eq!(result, Expression::Constant(MathConstant::Undefined));
        } else {
            panic!("undefined * π 应该返回 undefined");
        }
        
        // 没有特殊规则的情况应该返回 None
        assert!(MathConstant::Pi.multiply_rule(&MathConstant::E).is_none());
    }

    #[test]
    fn test_power_rules() {
        // i^2 = -1
        let i_squared = Expression::Number(Number::Integer(BigInt::from(2)));
        if let Some(result) = MathConstant::I.power_rule(&i_squared) {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(-1))));
        } else {
            panic!("i^2 应该返回 -1");
        }
        
        // i^4 = 1
        let i_fourth = Expression::Number(Number::Integer(BigInt::from(4)));
        if let Some(result) = MathConstant::I.power_rule(&i_fourth) {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(1))));
        } else {
            panic!("i^4 应该返回 1");
        }
        
        // e^(i*π) = -1 (欧拉公式)
        let i_pi = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Constant(MathConstant::I)),
            right: Box::new(Expression::Constant(MathConstant::Pi)),
        };
        if let Some(result) = MathConstant::E.power_rule(&i_pi) {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(-1))));
        } else {
            panic!("e^(i*π) 应该返回 -1");
        }
        
        // e^(π*i) = -1 (欧拉公式，交换律)
        let pi_i = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Constant(MathConstant::I)),
        };
        if let Some(result) = MathConstant::E.power_rule(&pi_i) {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(-1))));
        } else {
            panic!("e^(π*i) 应该返回 -1");
        }
        
        // 没有特殊规则的情况应该返回 None
        let pi_squared = Expression::Number(Number::Integer(BigInt::from(2)));
        assert!(MathConstant::Pi.power_rule(&pi_squared).is_none());
    }

    #[test]
    fn test_trigonometric_rules() {
        // sin(π) = 0
        if let Some(result) = MathConstant::Pi.trigonometric_rule("sin") {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(0))));
        } else {
            panic!("sin(π) 应该返回 0");
        }
        
        // cos(π) = -1
        if let Some(result) = MathConstant::Pi.trigonometric_rule("cos") {
            assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(-1))));
        } else {
            panic!("cos(π) 应该返回 -1");
        }
        
        // 没有特殊规则的情况应该返回 None
        assert!(MathConstant::Pi.trigonometric_rule("tan").is_none());
        assert!(MathConstant::E.trigonometric_rule("sin").is_none());
    }
}