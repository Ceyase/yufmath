//! # 快速终端演示
//!
//! 快速展示终端交互模式的主要改进功能。

use yufmath::formatter::{TerminalFormatter, Formatter};
use yufmath::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;
use num_rational::BigRational;
use colored::*;

fn main() {
    println!("{}", "Yufmath 终端交互模式演示".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_black());
    
    let mut formatter = TerminalFormatter::new();
    
    // 演示各种数学表达式
    let examples = vec![
        ("基本算术", create_arithmetic_expr()),
        ("平方根", create_sqrt_expr()),
        ("三角函数", create_trig_expr()),
        ("复合表达式", create_complex_expr()),
        ("分数", create_fraction_expr()),
        ("常量", create_constant_expr()),
    ];
    
    println!("\n{}", "彩色输出 + 数值近似值演示:".bright_yellow().bold());
    for (name, expr) in &examples {
        println!("  {}: {}", 
            name.bright_white().bold(), 
            formatter.format(expr));
    }
    
    // 演示颜色开关
    println!("\n{}", "颜色开关对比:".bright_yellow().bold());
    let demo_expr = &examples[3].1; // 使用复合表达式
    
    formatter.set_colors_enabled(true);
    println!("  {}: {}", 
        "彩色模式".bright_green(), 
        formatter.format(demo_expr));
    
    formatter.set_colors_enabled(false);
    println!("  {}: {}", 
        "无色模式".bright_red(), 
        formatter.format(demo_expr));
    
    // 演示近似值开关
    formatter.set_colors_enabled(true);
    println!("\n{}", "🔍 近似值开关对比:".bright_yellow().bold());
    let sqrt_expr = &examples[1].1; // 使用平方根表达式
    
    formatter.set_approximations_enabled(true);
    println!("  {}: {}", 
        "显示近似值".bright_green(), 
        formatter.format(sqrt_expr));
    
    formatter.set_approximations_enabled(false);
    println!("  {}: {}", 
        "隐藏近似值".bright_red(), 
        formatter.format(sqrt_expr));
    
    // 演示精度控制
    formatter.set_approximations_enabled(true);
    println!("\n{}", "精度控制演示:".bright_yellow().bold());
    let pi_expr = &examples[5].1; // 使用常量表达式
    
    for precision in [2, 4, 8] {
        formatter.set_approximation_precision(precision);
        println!("  {}: {}", 
            format!("精度 {}", precision).bright_cyan(), 
            formatter.format(pi_expr));
    }
    
    println!("\n{}", "使用提示:".bright_yellow().bold());
    println!("  • 运行 {} 启动交互模式", "cargo run --bin yufmath interactive".green());
    println!("  • 在交互模式中输入 {} 查看所有命令", "help".green());
    println!("  • 使用 {} 切换颜色输出", "colors".green());
    println!("  • 使用 {} 切换近似值显示", "approx".green());
    println!("  • 使用 {} 设置近似值精度", "approx_precision <n>".green());
}

fn create_arithmetic_expr() -> Expression {
    Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(4)))),
        }),
    }
}

fn create_sqrt_expr() -> Expression {
    Expression::UnaryOp {
        op: UnaryOperator::Sqrt,
        operand: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
    }
}

fn create_trig_expr() -> Expression {
    Expression::Function {
        name: "sin".to_string(),
        args: vec![Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }],
    }
}

fn create_complex_expr() -> Expression {
    Expression::BinaryOp {
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
    }
}

fn create_fraction_expr() -> Expression {
    Expression::Number(Number::Rational(
        BigRational::new(BigInt::from(22), BigInt::from(7))
    ))
}

fn create_constant_expr() -> Expression {
    Expression::Constant(MathConstant::Pi)
}