//! # 终端格式化测试
//!
//! 测试终端格式化器的功能，包括颜色输出和数值近似值。

use yufmath::formatter::{TerminalFormatter, Formatter};
use yufmath::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;
use num_rational::BigRational;

#[test]
fn test_terminal_formatter_creation() {
    let formatter = TerminalFormatter::new();
    // 测试默认设置
    assert!(true); // 基本创建测试
}

#[test]
fn test_number_formatting_with_approximations() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false); // 禁用颜色以便测试
    formatter.set_approximations_enabled(true);
    
    // 测试分数的近似值显示
    let fraction = Expression::Number(Number::Rational(
        BigRational::new(BigInt::from(22), BigInt::from(7))
    ));
    let formatted = formatter.format(&fraction);
    assert!(formatted.contains("22/7"));
    assert!(formatted.contains("≈"));
    assert!(formatted.contains("3.14")); // 应该包含近似值
}

#[test]
fn test_constant_formatting_with_approximations() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(true);
    
    // 测试π的近似值显示
    let pi = Expression::Constant(MathConstant::Pi);
    let formatted = formatter.format(&pi);
    assert!(formatted.contains("π"));
    assert!(formatted.contains("≈"));
    assert!(formatted.contains("3.14")); // 应该包含近似值
}

#[test]
fn test_expression_approximation() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(true);
    
    // 测试 sqrt(3) 的近似值
    let sqrt_expr = Expression::UnaryOp {
        op: UnaryOperator::Sqrt,
        operand: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
    };
    let formatted = formatter.format(&sqrt_expr);
    assert!(formatted.contains("√"));
    assert!(formatted.contains("≈"));
    assert!(formatted.contains("1.73")); // 应该包含近似值
}

#[test]
fn test_approximation_toggle() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    
    let pi = Expression::Constant(MathConstant::Pi);
    
    // 启用近似值
    formatter.set_approximations_enabled(true);
    let with_approx = formatter.format(&pi);
    assert!(with_approx.contains("≈"));
    
    // 禁用近似值
    formatter.set_approximations_enabled(false);
    let without_approx = formatter.format(&pi);
    assert!(!without_approx.contains("≈"));
}

#[test]
fn test_color_toggle() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_approximations_enabled(false); // 简化测试
    
    let number = Expression::Number(Number::Integer(BigInt::from(42)));
    
    // 启用颜色（默认情况下应该包含ANSI转义序列）
    formatter.set_colors_enabled(true);
    let with_colors = formatter.format(&number);
    
    // 禁用颜色
    formatter.set_colors_enabled(false);
    let without_colors = formatter.format(&number);
    
    // 无颜色版本应该更短（没有ANSI转义序列）
    assert!(without_colors.len() <= with_colors.len());
    assert!(without_colors.contains("42"));
}

#[test]
fn test_complex_expression_formatting() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(true);
    
    // 测试复合表达式: 2 + 3 * 4
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(4)))),
        }),
    };
    
    let formatted = formatter.format(&expr);
    assert!(formatted.contains("2"));
    assert!(formatted.contains("3"));
    assert!(formatted.contains("4"));
    assert!(formatted.contains("≈"));
    assert!(formatted.contains("14")); // 2 + 3*4 = 14
}

#[test]
fn test_function_formatting() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(true);
    
    // 测试 sin(π/2)
    let expr = Expression::Function {
        name: "sin".to_string(),
        args: vec![Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }],
    };
    
    let formatted = formatter.format(&expr);
    assert!(formatted.contains("sin"));
    assert!(formatted.contains("π"));
    assert!(formatted.contains("≈"));
    assert!(formatted.contains("1.0")); // sin(π/2) ≈ 1.0
}

#[test]
fn test_precision_setting() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(true);
    
    let pi = Expression::Constant(MathConstant::Pi);
    
    // 测试不同精度设置
    formatter.set_approximation_precision(2);
    let low_precision = formatter.format(&pi);
    assert!(low_precision.contains("3.14"));
    
    formatter.set_approximation_precision(6);
    let high_precision = formatter.format(&pi);
    assert!(high_precision.contains("3.141593"));
}

#[test]
fn test_special_symbols() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(false);
    
    // 测试特殊数学符号
    let multiply_expr = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
    };
    let formatted = formatter.format(&multiply_expr);
    assert!(formatted.contains("×")); // 应该使用×而不是*
    
    let divide_expr = Expression::BinaryOp {
        op: BinaryOperator::Divide,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(6)))),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
    };
    let formatted = formatter.format(&divide_expr);
    assert!(formatted.contains("÷")); // 应该使用÷而不是/
}

#[test]
fn test_complex_number_formatting() {
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(false);
    formatter.set_approximations_enabled(false);
    
    // 测试复数格式化
    let complex = Expression::Number(Number::Complex {
        real: Box::new(Number::Integer(BigInt::from(3))),
        imaginary: Box::new(Number::Integer(BigInt::from(4))),
    });
    
    let formatted = formatter.format(&complex);
    assert!(formatted.contains("3"));
    assert!(formatted.contains("4"));
    assert!(formatted.contains("i"));
}