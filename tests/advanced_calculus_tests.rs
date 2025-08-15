//! # 高级微积分功能集成测试
//!
//! 测试极限、级数展开和数值计算功能在整个系统中的集成情况。

use yufmath::api::Yufmath;
use yufmath::core::{Expression, Number, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;
use std::collections::HashMap;

#[test]
fn test_limit_calculations() {
    let yuf = Yufmath::new();
    
    // 测试 lim(x->0) sin(x)/x = 1
    let sin_x = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Divide,
        left: Box::new(sin_x),
        right: Box::new(Expression::Variable("x".to_string())),
    };
    let point = Expression::Number(Number::Integer(BigInt::from(0)));
    
    let result = yuf.limit(&expr, "x", &point).unwrap();
    match result {
        Expression::Number(Number::Integer(n)) => {
            assert_eq!(n, BigInt::from(1));
        }
        _ => panic!("期望得到数字 1"),
    }
    
    // 测试 lim(x->0) (1-cos(x))/x = 0
    let cos_x = Expression::UnaryOp {
        op: UnaryOperator::Cos,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    let one_minus_cos = Expression::BinaryOp {
        op: BinaryOperator::Subtract,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        right: Box::new(cos_x),
    };
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Divide,
        left: Box::new(one_minus_cos),
        right: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.limit(&expr, "x", &point).unwrap();
    match result {
        Expression::Number(Number::Integer(n)) => {
            assert_eq!(n, BigInt::from(0));
        }
        _ => panic!("期望得到数字 0"),
    }
}

#[test]
fn test_series_expansions() {
    let yuf = Yufmath::new();
    let point = Expression::Number(Number::Integer(BigInt::from(0)));
    
    // 测试 e^x 的泰勒级数展开
    let exp_x = Expression::UnaryOp {
        op: UnaryOperator::Exp,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.series(&exp_x, "x", &point, 3).unwrap();
    
    // 结果应该是加法表达式：1 + x + x²/2 + x³/6
    match result {
        Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到加法表达式"),
    }
    
    // 测试 sin(x) 的泰勒级数展开
    let sin_x = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.series(&sin_x, "x", &point, 5).unwrap();
    
    // 结果应该是加法表达式：x - x³/6 + x⁵/120
    match result {
        Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到加法表达式"),
    }
    
    // 测试 cos(x) 的泰勒级数展开
    let cos_x = Expression::UnaryOp {
        op: UnaryOperator::Cos,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.series(&cos_x, "x", &point, 4).unwrap();
    
    // 结果应该是加法表达式：1 - x²/2 + x⁴/24
    match result {
        Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到加法表达式"),
    }
}

#[test]
fn test_numerical_evaluation() {
    let yuf = Yufmath::new();
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 2.0);
    vars.insert("y".to_string(), 3.0);
    
    // 测试基本算术运算的数值计算
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Variable("y".to_string())),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - 5.0).abs() < 1e-10);
    
    // 测试幂运算的数值计算
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Power,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - 8.0).abs() < 1e-10);
    
    // 测试三角函数的数值计算
    vars.insert("x".to_string(), 0.0);
    
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - 0.0).abs() < 1e-10);
    
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Cos,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - 1.0).abs() < 1e-10);
}

#[test]
fn test_numerical_evaluation_with_constants() {
    let yuf = Yufmath::new();
    let vars = HashMap::new();
    
    // 测试数学常量的数值计算
    let expr = Expression::Constant(yufmath::core::MathConstant::Pi);
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - std::f64::consts::PI).abs() < 1e-10);
    
    let expr = Expression::Constant(yufmath::core::MathConstant::E);
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - std::f64::consts::E).abs() < 1e-10);
    
    // 测试包含常量的表达式
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::Constant(yufmath::core::MathConstant::Pi)),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    assert!((result - 2.0 * std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_numerical_evaluation_complex_expressions() {
    let yuf = Yufmath::new();
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 1.0);
    
    // 测试复杂表达式：sin(x) + cos(x) + e^x
    let sin_x = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let cos_x = Expression::UnaryOp {
        op: UnaryOperator::Cos,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let exp_x = Expression::UnaryOp {
        op: UnaryOperator::Exp,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let sin_plus_cos = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(sin_x),
        right: Box::new(cos_x),
    };
    
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(sin_plus_cos),
        right: Box::new(exp_x),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars).unwrap();
    let expected = 1.0_f64.sin() + 1.0_f64.cos() + 1.0_f64.exp();
    assert!((result - expected).abs() < 1e-10);
}

#[test]
fn test_numerical_evaluation_error_handling() {
    let yuf = Yufmath::new();
    let vars = HashMap::new();
    
    // 测试除零错误
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Divide,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(0)))),
    };
    
    let result = yuf.numerical_evaluate(&expr, &vars);
    assert!(result.is_err());
    
    // 测试未定义变量错误
    let expr = Expression::Variable("undefined_var".to_string());
    let result = yuf.numerical_evaluate(&expr, &vars);
    assert!(result.is_err());
}