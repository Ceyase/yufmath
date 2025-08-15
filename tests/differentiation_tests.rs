//! # 求导功能集成测试
//!
//! 测试求导功能在整个系统中的集成情况。

use yufmath::api::Yufmath;
use yufmath::core::{Expression, Number, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;

#[test]
fn test_differentiate_basic_expressions() {
    let yuf = Yufmath::new();
    
    // 测试常数求导
    let expr = Expression::Number(Number::Integer(BigInt::from(5)));
    let result = yuf.differentiate(&expr, "x").unwrap();
    match result {
        Expression::Number(Number::Integer(n)) => assert_eq!(n, BigInt::from(0)),
        _ => panic!("期望得到 0"),
    }
    
    // 测试变量求导
    let expr = Expression::Variable("x".to_string());
    let result = yuf.differentiate(&expr, "x").unwrap();
    match result {
        Expression::Number(Number::Integer(n)) => assert_eq!(n, BigInt::from(1)),
        _ => panic!("期望得到 1"),
    }
}

#[test]
fn test_differentiate_polynomial() {
    let yuf = Yufmath::new();
    
    // 测试 x^2 的求导
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Power,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是乘法表达式（2 * x^1 * 1）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到乘法表达式"),
    }
}

#[test]
fn test_differentiate_trigonometric() {
    let yuf = Yufmath::new();
    
    // 测试 sin(x) 的求导
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是 cos(x) * 1
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::UnaryOp { op: UnaryOperator::Cos, .. }, 
                 Expression::Number(Number::Integer(n))) => {
                    assert_eq!(n, &BigInt::from(1));
                }
                _ => panic!("期望得到 cos(x) * 1"),
            }
        }
        _ => panic!("期望得到乘法表达式"),
    }
}

#[test]
fn test_differentiate_logarithmic() {
    let yuf = Yufmath::new();
    
    // 测试 ln(x) 的求导
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Ln,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是 1 / x
    match result {
        Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::Number(Number::Integer(n)), Expression::Variable(var)) => {
                    assert_eq!(n, &BigInt::from(1));
                    assert_eq!(var, "x");
                }
                _ => panic!("期望得到 1 / x"),
            }
        }
        _ => panic!("期望得到除法表达式"),
    }
}

#[test]
fn test_differentiate_product_rule() {
    let yuf = Yufmath::new();
    
    // 测试 x * sin(x) 的求导（乘法法则）
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::UnaryOp {
            op: UnaryOperator::Sin,
            operand: Box::new(Expression::Variable("x".to_string())),
        }),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是加法表达式（乘法法则：u'v + uv'）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到加法表达式（乘法法则）"),
    }
}

#[test]
fn test_differentiate_quotient_rule() {
    let yuf = Yufmath::new();
    
    // 测试 x / (x + 1) 的求导（除法法则）
    let numerator = Expression::Variable("x".to_string());
    let denominator = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
    };
    
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Divide,
        left: Box::new(numerator),
        right: Box::new(denominator),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是除法表达式（除法法则）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到除法表达式（除法法则）"),
    }
}

#[test]
fn test_differentiate_chain_rule() {
    let yuf = Yufmath::new();
    
    // 测试 sin(x^2) 的求导（链式法则）
    let inner = Expression::BinaryOp {
        op: BinaryOperator::Power,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
    };
    
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Sin,
        operand: Box::new(inner),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是乘法表达式（链式法则）
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
            // 验证结构正确
        }
        _ => panic!("期望得到乘法表达式（链式法则）"),
    }
}

#[test]
fn test_differentiate_exponential() {
    let yuf = Yufmath::new();
    
    // 测试 e^x 的求导
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Exp,
        operand: Box::new(Expression::Variable("x".to_string())),
    };
    
    let result = yuf.differentiate(&expr, "x").unwrap();
    
    // 结果应该是 e^x * 1
    match result {
        Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
            match (left.as_ref(), right.as_ref()) {
                (Expression::UnaryOp { op: UnaryOperator::Exp, .. }, 
                 Expression::Number(Number::Integer(n))) => {
                    assert_eq!(n, &BigInt::from(1));
                }
                _ => panic!("期望得到 e^x * 1"),
            }
        }
        _ => panic!("期望得到乘法表达式"),
    }
}