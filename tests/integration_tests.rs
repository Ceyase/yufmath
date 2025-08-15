//! # 积分功能集成测试
//!
//! 测试积分功能在整个系统中的集成情况。

use yufmath::api::Yufmath;
use yufmath::core::{Expression, Number, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;

#[test]
fn test_integrate_basic_expressions() {
    let yuf = Yufmath::new();
    
    // 测试常数积分
    let expr = Expression::Number(Number::Integer(BigInt::from(5)));
    let result = yuf.integrate(&expr, "x").unwrap();
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::Number(Number::Integer(n)), Expression::Variable(var)) => {
                    assert_eq!(n, &BigInt::from(5));
                    assert_eq!(var, "x");
                }
                _ => panic!("期望得到 5x"),
            }
        }
        _ => panic!("期望得到乘法表达式"),
    }
    
    // 测试变量积分
    let expr = Expression::Variable("x".to_string());
    let result = yuf.integrate(&expr, "x").unwrap();
    match result {
        Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到除法表达式"),
    }
}

#[test]
fn test_integrate_polynomial() {
    let yuf = Yufmath::new();
    
    // 测试 x^2 的积分
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Power,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是除法表达式（x^3 / 3）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到除法表达式"),
    }
}

#[test]
fn test_integrate_reciprocal() {
    let yuf = Yufmath::new();
    
    // 测试 x^(-1) = 1/x 的积分
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Power,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(-1)))),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是 ln|x|
    match result {
        Expression::UnaryOp { op: UnaryOperator::Ln, operand } => {
            match operand.as_ref() {
                Expression::UnaryOp { op: UnaryOperator::Abs, operand: inner } => {
                    match inner.as_ref() {
                        Expression::Variable(var) => assert_eq!(var, "x"),
                        _ => panic!("期望得到变量 x"),
                    }
                }
                _ => panic!("期望得到绝对值"),
            }
        }
        _ => panic!("期望得到 ln 表达式"),
    }
}

#[test]
fn test_integrate_trigonometric() {
    let yuf = Yufmath::new();
    
    // 测试 sin(x) 的积分
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是 -cos(x)
    match result {
        Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
            match operand.as_ref() {
                Expression::UnaryOp { op: UnaryOperator::Cos, operand: inner } => {
                    match inner.as_ref() {
                        Expression::Variable(var) => assert_eq!(var, "x"),
                        _ => panic!("期望得到变量 x"),
                    }
                }
                _ => panic!("期望得到 cos(x)"),
            }
        }
        _ => panic!("期望得到负号表达式"),
    }
    
    // 测试 cos(x) 的积分
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Cos,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是 sin(x)
    match result {
        Expression::UnaryOp { op: UnaryOperator::Sin, operand } => {
            match operand.as_ref() {
                Expression::Variable(var) => assert_eq!(var, "x"),
                _ => panic!("期望得到变量 x"),
            }
        }
        _ => panic!("期望得到 sin(x)"),
    }
}

#[test]
fn test_integrate_exponential() {
    let yuf = Yufmath::new();
    
    // 测试 e^x 的积分
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Exp,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是 e^x
    match result {
        Expression::UnaryOp { op: UnaryOperator::Exp, operand } => {
            match operand.as_ref() {
                Expression::Variable(var) => assert_eq!(var, "x"),
                _ => panic!("期望得到变量 x"),
            }
        }
        _ => panic!("期望得到 e^x"),
    }
}

#[test]
fn test_integrate_sum() {
    let yuf = Yufmath::new();
    
    // 测试 (x + 3) 的积分
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是加法表达式（积分的线性性）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到加法表达式"),
    }
}

#[test]
fn test_integrate_constant_multiple() {
    let yuf = Yufmath::new();
    
    // 测试 3x 的积分
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
        right: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是乘法表达式（常数倍法则）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到乘法表达式"),
    }
}

#[test]
fn test_integrate_other_variable() {
    let yuf = Yufmath::new();
    
    // 测试 y 对 x 的积分（y 视为常数）
    let expr = Expression::Variable("y".to_string());
    let result = yuf.integrate(&expr, "x").unwrap();
    
    // 结果应该是 yx
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::Variable(var1), Expression::Variable(var2)) => {
                    assert_eq!(var1, "y");
                    assert_eq!(var2, "x");
                }
                _ => panic!("期望得到 yx"),
            }
        }
        _ => panic!("期望得到乘法表达式"),
    }
}