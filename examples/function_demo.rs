//! # 基础函数支持演示
//!
//! 演示 Yufmath 库的基础数学函数功能，包括三角函数、指数函数、对数函数等。

use yufmath::core::{Expression, Number, MathConstant};
use std::collections::HashMap;

fn main() {
    println!("=== Yufmath 基础函数支持演示 ===\n");

    // 三角函数演示
    println!("1. 三角函数:");
    
    // sin(0) = 0
    let sin_0 = Expression::function("sin", vec![Expression::Number(Number::integer(0))]);
    println!("sin(0) = {}", sin_0.evaluate_exact().unwrap());
    
    // cos(0) = 1
    let cos_0 = Expression::function("cos", vec![Expression::Number(Number::integer(0))]);
    println!("cos(0) = {}", cos_0.evaluate_exact().unwrap());
    
    // sin(π) = 0
    let sin_pi = Expression::function("sin", vec![Expression::Constant(MathConstant::Pi)]);
    println!("sin(π) = {}", sin_pi.evaluate_exact().unwrap());
    
    // cos(π) = -1
    let cos_pi = Expression::function("cos", vec![Expression::Constant(MathConstant::Pi)]);
    println!("cos(π) = {}", cos_pi.evaluate_exact().unwrap());
    
    // sin(π/2) = 1
    let pi_half = Expression::BinaryOp {
        op: yufmath::core::BinaryOperator::Divide,
        left: Box::new(Expression::Constant(MathConstant::Pi)),
        right: Box::new(Expression::Number(Number::integer(2))),
    };
    let sin_pi_half = Expression::function("sin", vec![pi_half.clone()]);
    println!("sin(π/2) = {}", sin_pi_half.evaluate_exact().unwrap());
    
    // cos(π/2) = 0
    let cos_pi_half = Expression::function("cos", vec![pi_half]);
    println!("cos(π/2) = {}", cos_pi_half.evaluate_exact().unwrap());
    
    // tan(π/4) = 1
    let pi_quarter = Expression::BinaryOp {
        op: yufmath::core::BinaryOperator::Divide,
        left: Box::new(Expression::Constant(MathConstant::Pi)),
        right: Box::new(Expression::Number(Number::integer(4))),
    };
    let tan_pi_quarter = Expression::function("tan", vec![pi_quarter]);
    println!("tan(π/4) = {}", tan_pi_quarter.evaluate_exact().unwrap());
    
    println!();

    // 指数和对数函数演示
    println!("2. 指数和对数函数:");
    
    // exp(0) = 1
    let exp_0 = Expression::function("exp", vec![Expression::Number(Number::integer(0))]);
    println!("exp(0) = {}", exp_0.evaluate_exact().unwrap());
    
    // exp(1) = e
    let exp_1 = Expression::function("exp", vec![Expression::Number(Number::integer(1))]);
    println!("exp(1) = {}", exp_1.evaluate_exact().unwrap());
    
    // exp(i*π) = -1 (欧拉公式)
    let i_pi = Expression::BinaryOp {
        op: yufmath::core::BinaryOperator::Multiply,
        left: Box::new(Expression::Constant(MathConstant::I)),
        right: Box::new(Expression::Constant(MathConstant::Pi)),
    };
    let exp_i_pi = Expression::function("exp", vec![i_pi]);
    println!("exp(i*π) = {}", exp_i_pi.evaluate_exact().unwrap());
    
    // ln(1) = 0
    let ln_1 = Expression::function("ln", vec![Expression::Number(Number::integer(1))]);
    println!("ln(1) = {}", ln_1.evaluate_exact().unwrap());
    
    // ln(e) = 1
    let ln_e = Expression::function("ln", vec![Expression::Constant(MathConstant::E)]);
    println!("ln(e) = {}", ln_e.evaluate_exact().unwrap());
    
    // log10(10) = 1
    let log10_10 = Expression::function("log10", vec![Expression::Number(Number::integer(10))]);
    println!("log10(10) = {}", log10_10.evaluate_exact().unwrap());
    
    println!();

    // 平方根和幂函数演示
    println!("3. 平方根和幂函数:");
    
    // sqrt(4) = 2
    let sqrt_4 = Expression::function("sqrt", vec![Expression::Number(Number::integer(4))]);
    println!("sqrt(4) = {}", sqrt_4.evaluate_exact().unwrap());
    
    // sqrt(9) = 3
    let sqrt_9 = Expression::function("sqrt", vec![Expression::Number(Number::integer(9))]);
    println!("sqrt(9) = {}", sqrt_9.evaluate_exact().unwrap());
    
    // pow(2, 3) = 8
    let pow_2_3 = Expression::function("pow", vec![
        Expression::Number(Number::integer(2)),
        Expression::Number(Number::integer(3))
    ]);
    println!("pow(2, 3) = {}", pow_2_3.evaluate_exact().unwrap());
    
    // pow(3, 2) = 9
    let pow_3_2 = Expression::function("pow", vec![
        Expression::Number(Number::integer(3)),
        Expression::Number(Number::integer(2))
    ]);
    println!("pow(3, 2) = {}", pow_3_2.evaluate_exact().unwrap());
    
    println!();

    // 双曲函数演示
    println!("4. 双曲函数:");
    
    // sinh(0) = 0
    let sinh_0 = Expression::function("sinh", vec![Expression::Number(Number::integer(0))]);
    println!("sinh(0) = {}", sinh_0.evaluate_exact().unwrap());
    
    // cosh(0) = 1
    let cosh_0 = Expression::function("cosh", vec![Expression::Number(Number::integer(0))]);
    println!("cosh(0) = {}", cosh_0.evaluate_exact().unwrap());
    
    // tanh(0) = 0
    let tanh_0 = Expression::function("tanh", vec![Expression::Number(Number::integer(0))]);
    println!("tanh(0) = {}", tanh_0.evaluate_exact().unwrap());
    
    println!();

    // 变量替换演示
    println!("5. 包含变量的函数:");
    
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), Number::integer(0));
    
    // sin(x) where x = 0
    let sin_x = Expression::function("sin", vec![Expression::variable("x")]);
    println!("sin(x) where x = 0: {}", sin_x.evaluate(&vars).unwrap());
    
    // cos(x) where x = 0
    let cos_x = Expression::function("cos", vec![Expression::variable("x")]);
    println!("cos(x) where x = 0: {}", cos_x.evaluate(&vars).unwrap());
    
    // 改变变量值
    vars.insert("x".to_string(), Number::integer(1));
    
    // exp(x) where x = 1
    let exp_x = Expression::function("exp", vec![Expression::variable("x")]);
    println!("exp(x) where x = 1: {}", exp_x.evaluate(&vars).unwrap());
    
    println!();

    // 符号结果演示
    println!("6. 符号结果（无法精确计算的情况）:");
    
    // sin(2) 返回符号表示
    let sin_2 = Expression::function("sin", vec![Expression::Number(Number::integer(2))]);
    println!("sin(2) = {}", sin_2.evaluate_exact().unwrap());
    
    // exp(2) 返回符号表示
    let exp_2 = Expression::function("exp", vec![Expression::Number(Number::integer(2))]);
    println!("exp(2) = {}", exp_2.evaluate_exact().unwrap());
    
    // ln(2) 返回符号表示
    let ln_2 = Expression::function("ln", vec![Expression::Number(Number::integer(2))]);
    println!("ln(2) = {}", ln_2.evaluate_exact().unwrap());
    
    // sqrt(2) 返回符号表示
    let sqrt_2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
    println!("sqrt(2) = {}", sqrt_2.evaluate_exact().unwrap());
    
    println!();

    // 嵌套函数演示
    println!("7. 嵌套函数:");
    
    // sin(cos(0)) = sin(1)
    let nested_func = Expression::function("sin", vec![
        Expression::function("cos", vec![Expression::Number(Number::integer(0))])
    ]);
    println!("sin(cos(0)) = {}", nested_func.evaluate_exact().unwrap());
    
    // exp(ln(5)) = 5
    let exp_ln_5 = Expression::function("exp", vec![
        Expression::function("ln", vec![Expression::Number(Number::integer(5))])
    ]);
    println!("exp(ln(5)) = {}", exp_ln_5.evaluate_exact().unwrap());
    
    println!("\n=== 演示完成 ===");
}