//! # 终端交互演示
//!
//! 演示增强的终端交互模式，包括颜色输出和数值近似值显示。

use yufmath::formatter::{TerminalFormatter, Formatter, FormatOptions, FormatType};
use yufmath::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;
use num_rational::BigRational;
use colored::*;

fn main() {
    println!("{}", "🎨 Yufmath 终端格式化演示".bright_cyan().bold());
    println!("{}", "━".repeat(40).bright_black());
    
    let mut formatter = TerminalFormatter::new();
    formatter.set_colors_enabled(true);
    formatter.set_approximations_enabled(true);
    formatter.set_approximation_precision(6);
    
    // 演示数字格式化
    println!("\n{}", "📊 数字格式化:".bright_yellow().bold());
    demo_numbers(&formatter);
    
    // 演示常量格式化
    println!("\n{}", "🔢 数学常量:".bright_yellow().bold());
    demo_constants(&formatter);
    
    // 演示表达式格式化
    println!("\n{}", "🧮 数学表达式:".bright_yellow().bold());
    demo_expressions(&formatter);
    
    // 演示函数格式化
    println!("\n{}", "📐 数学函数:".bright_yellow().bold());
    demo_functions(&formatter);
    
    // 演示颜色开关
    println!("\n{}", "🎨 颜色开关演示:".bright_yellow().bold());
    demo_color_toggle(&mut formatter);
    
    // 演示近似值开关
    println!("\n{}", "🔍 近似值开关演示:".bright_yellow().bold());
    demo_approximation_toggle(&mut formatter);
}

fn demo_numbers(formatter: &TerminalFormatter) {
    let examples = vec![
        ("整数", Expression::Number(Number::Integer(BigInt::from(42)))),
        ("分数", Expression::Number(Number::Rational(BigRational::new(BigInt::from(22), BigInt::from(7))))),
        ("浮点数", Expression::Number(Number::Float(3.14159))),
        ("复数", Expression::Number(Number::Complex {
            real: Box::new(Number::Integer(BigInt::from(3))),
            imaginary: Box::new(Number::Integer(BigInt::from(4))),
        })),
    ];
    
    for (name, expr) in examples {
        println!("  {}: {}", name.bright_white(), formatter.format(&expr));
    }
}

fn demo_constants(formatter: &TerminalFormatter) {
    let constants = vec![
        ("圆周率", Expression::Constant(MathConstant::Pi)),
        ("自然常数", Expression::Constant(MathConstant::E)),
        ("黄金比例", Expression::Constant(MathConstant::Phi)),
        ("欧拉常数", Expression::Constant(MathConstant::Gamma)),
    ];
    
    for (name, expr) in constants {
        println!("  {}: {}", name.bright_white(), formatter.format(&expr));
    }
}

fn demo_expressions(formatter: &TerminalFormatter) {
    let examples = vec![
        ("加法", Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
        }),
        ("乘法", Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(6)))),
            right: Box::new(Expression::Variable("x".to_string())),
        }),
        ("幂运算", Expression::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        ("复合表达式", Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Power,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            }),
            right: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                right: Box::new(Expression::Variable("x".to_string())),
            }),
        }),
    ];
    
    for (name, expr) in examples {
        println!("  {}: {}", name.bright_white(), formatter.format(&expr));
    }
}

fn demo_functions(formatter: &TerminalFormatter) {
    let examples = vec![
        ("平方根", Expression::UnaryOp {
            op: UnaryOperator::Sqrt,
            operand: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
        }),
        ("正弦函数", Expression::Function {
            name: "sin".to_string(),
            args: vec![Expression::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(Expression::Constant(MathConstant::Pi)),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            }],
        }),
        ("自然对数", Expression::UnaryOp {
            op: UnaryOperator::Ln,
            operand: Box::new(Expression::Constant(MathConstant::E)),
        }),
        ("绝对值", Expression::UnaryOp {
            op: UnaryOperator::Abs,
            operand: Box::new(Expression::Number(Number::Integer(BigInt::from(-5)))),
        }),
    ];
    
    for (name, expr) in examples {
        println!("  {}: {}", name.bright_white(), formatter.format(&expr));
    }
}

fn demo_color_toggle(formatter: &mut TerminalFormatter) {
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Variable("x".to_string())),
        }),
        right: Box::new(Expression::Constant(MathConstant::Pi)),
    };
    
    formatter.set_colors_enabled(true);
    println!("  {}: {}", "彩色输出".bright_white(), formatter.format(&expr));
    
    formatter.set_colors_enabled(false);
    println!("  {}: {}", "无色输出".bright_white(), formatter.format(&expr));
    
    // 恢复彩色输出
    formatter.set_colors_enabled(true);
}

fn demo_approximation_toggle(formatter: &mut TerminalFormatter) {
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Sqrt,
        operand: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
    };
    
    formatter.set_approximations_enabled(true);
    println!("  {}: {}", "显示近似值".bright_white(), formatter.format(&expr));
    
    formatter.set_approximations_enabled(false);
    println!("  {}: {}", "隐藏近似值".bright_white(), formatter.format(&expr));
    
    // 恢复近似值显示
    formatter.set_approximations_enabled(true);
}