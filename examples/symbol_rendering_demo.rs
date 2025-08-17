//! # 符号渲染演示
//!
//! 演示修复后的符号渲染效果，特别是乘法符号的正确显示。

use yufmath::core::{Expression, Number, BinaryOperator};
use yufmath::formatter::{Formatter, StandardFormatter, LaTeXFormatter, MathMLFormatter};
use num_bigint::BigInt;

fn main() {
    println!("=== Yufmath 符号渲染演示 ===\n");
    
    // 创建问题表达式：(x/2)*x
    let problematic_expr = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        right: Box::new(Expression::Variable("x".to_string())),
    };
    
    // 创建其他测试表达式
    let simple_multiply = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::Variable("x".to_string())),
    };
    
    let complex_expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::Variable("a".to_string())),
                right: Box::new(Expression::Variable("b".to_string())),
            }),
            right: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::Variable("c".to_string())),
                right: Box::new(Expression::Variable("d".to_string())),
            }),
        }),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
    };
    
    let expressions = vec![
        ("(x/2)*x", &problematic_expr),
        ("2*x", &simple_multiply),
        ("(a+b)*(c+d)+1", &complex_expr),
    ];
    
    // 创建格式化器
    let formatters: Vec<(&str, Box<dyn Formatter>)> = vec![
        ("标准格式", Box::new(StandardFormatter::new())),
        ("LaTeX 格式", Box::new(LaTeXFormatter::new())),
        ("MathML 格式", Box::new(MathMLFormatter::new())),
    ];
    
    for (expr_name, expr) in expressions {
        println!("表达式: {}", expr_name);
        println!("{}", "=".repeat(50));
        
        for (formatter_name, formatter) in &formatters {
            let result = formatter.format(expr);
            println!("{}: {}", formatter_name, result);
        }
        
        println!();
    }
    
    println!("修复说明:");
    println!("- 修复前：(x/2)*x 在标准格式中被错误渲染为 'x / 2x'，造成歧义");
    println!("- 修复后：(x/2)*x 在标准格式中正确渲染为 'x / 2 * x'，保持清晰");
    println!("- LaTeX 和 MathML 格式化器本来就是正确的，没有受到影响");
    println!("- 简单的乘法（如 2*x）仍然可以省略乘法符号显示为 '2x'");
}