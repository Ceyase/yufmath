//! # 符号渲染测试
//!
//! 专门测试各种格式化器中可能存在的符号渲染问题，
//! 特别是括号、运算符优先级和乘法符号的正确显示。

use yufmath::core::{Expression, Number, BinaryOperator};
use yufmath::formatter::{Formatter, StandardFormatter, LaTeXFormatter, MathMLFormatter};
use num_bigint::BigInt;

/// 创建测试用的问题表达式
fn create_problematic_expressions() -> Vec<(String, Expression, Vec<String>)> {
    vec![
        // 问题1: (x/2)*x 不应该被渲染为 x/2x
        (
            "(x/2)*x 乘法符号问题".to_string(),
            Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(Expression::Variable("x".to_string())),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                }),
                right: Box::new(Expression::Variable("x".to_string())),
            },
            vec![
                "应该包含括号或明确的乘法符号".to_string(),
                "不应该是 x/2x 这种模糊形式".to_string(),
            ]
        ),
        
        // 问题2: 2*x/3 应该正确显示优先级
        (
            "2*x/3 优先级问题".to_string(),
            Expression::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(Expression::Variable("x".to_string())),
                }),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
            },
            vec![
                "应该明确显示为 (2*x)/3 或 2x/3".to_string(),
                "不应该产生歧义".to_string(),
            ]
        ),
        
        // 问题3: x/(2*y) 分母中的乘法
        (
            "x/(2*y) 分母乘法问题".to_string(),
            Expression::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(Expression::Variable("y".to_string())),
                }),
            },
            vec![
                "分母应该正确显示".to_string(),
                "LaTeX 应该使用 \\frac 格式".to_string(),
            ]
        ),
        
        // 问题4: (a+b)*(c+d) 括号保持
        (
            "(a+b)*(c+d) 括号保持问题".to_string(),
            Expression::BinaryOp {
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
            },
            vec![
                "应该保持括号以避免歧义".to_string(),
                "不应该省略必要的括号".to_string(),
            ]
        ),
        
        // 问题5: x^(2+1) 指数中的运算
        (
            "x^(2+1) 指数运算问题".to_string(),
            Expression::BinaryOp {
                op: BinaryOperator::Power,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                }),
            },
            vec![
                "指数部分应该正确显示".to_string(),
                "LaTeX 应该使用大括号包围复杂指数".to_string(),
            ]
        ),
        
        // 问题6: 根号中的复杂表达式
        (
            "√(x+1) 根号问题".to_string(),
            Expression::UnaryOp {
                op: yufmath::core::UnaryOperator::Sqrt,
                operand: Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(Expression::Variable("x".to_string())),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                }),
            },
            vec![
                "根号应该正确包围整个表达式".to_string(),
                "LaTeX 应该使用 \\sqrt{} 格式".to_string(),
            ]
        ),
    ]
}

#[test]
fn test_standard_formatter_symbol_rendering() {
    let formatter = StandardFormatter::new();
    let expressions = create_problematic_expressions();
    
    println!("\n=== 标准格式化器符号渲染测试 ===");
    
    for (name, expr, expectations) in expressions {
        let result = formatter.format(&expr);
        println!("\n测试: {}", name);
        println!("结果: {}", result);
        println!("期望:");
        for expectation in expectations {
            println!("  - {}", expectation);
        }
        
        // 基本验证
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
        
        // 特定问题检查
        match name.as_str() {
            s if s.contains("(x/2)*x") => {
                // 不应该是简单的 x/2x 形式
                assert!(!result.eq("x/2x"), "不应该渲染为模糊的 x/2x 形式");
                // 应该包含某种分隔符或括号
                assert!(
                    result.contains("(") || result.contains("*") || result.contains("·"),
                    "应该包含明确的运算符或括号: {}",
                    result
                );
            }
            s if s.contains("2*x/3") => {
                // 应该明确显示运算顺序
                assert!(
                    result.contains("2") && result.contains("x") && result.contains("3"),
                    "应该包含所有操作数: {}",
                    result
                );
            }
            s if s.contains("括号保持") => {
                // 应该保持必要的括号
                assert!(
                    result.contains("(") && result.contains(")"),
                    "应该保持括号: {}",
                    result
                );
            }
            _ => {}
        }
    }
}

#[test]
fn test_latex_formatter_symbol_rendering() {
    let formatter = LaTeXFormatter::new();
    let expressions = create_problematic_expressions();
    
    println!("\n=== LaTeX 格式化器符号渲染测试 ===");
    
    for (name, expr, expectations) in expressions {
        let result = formatter.format(&expr);
        println!("\n测试: {}", name);
        println!("结果: {}", result);
        println!("期望:");
        for expectation in expectations {
            println!("  - {}", expectation);
        }
        
        // 基本验证
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
        
        // LaTeX 特定检查
        match name.as_str() {
            s if s.contains("分母乘法") => {
                // 应该使用 \frac 格式
                assert!(result.contains("\\frac"), "应该使用 \\frac 格式: {}", result);
            }
            s if s.contains("指数运算") => {
                // 指数应该用大括号包围
                assert!(result.contains("^{"), "指数应该用大括号包围: {}", result);
            }
            s if s.contains("根号") => {
                // 应该使用 \sqrt 格式
                assert!(result.contains("\\sqrt"), "应该使用 \\sqrt 格式: {}", result);
            }
            s if s.contains("(x/2)*x") => {
                // LaTeX 中的除法应该用 \frac 或明确的符号
                assert!(
                    result.contains("\\frac") || result.contains("\\cdot") || result.contains("/"),
                    "应该有明确的运算符: {}",
                    result
                );
            }
            _ => {}
        }
    }
}

#[test]
fn test_mathml_formatter_symbol_rendering() {
    let formatter = MathMLFormatter::new();
    let expressions = create_problematic_expressions();
    
    println!("\n=== MathML 格式化器符号渲染测试 ===");
    
    for (name, expr, expectations) in expressions {
        let result = formatter.format(&expr);
        println!("\n测试: {}", name);
        println!("结果: {}", result);
        println!("期望:");
        for expectation in expectations {
            println!("  - {}", expectation);
        }
        
        // 基本验证
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
        assert!(result.contains("<math"), "应该包含 MathML 标签: {}", result);
        
        // MathML 特定检查
        match name.as_str() {
            s if s.contains("分母乘法") => {
                // 应该使用 <mfrac> 标签
                assert!(result.contains("<mfrac>"), "应该使用 <mfrac> 标签: {}", result);
            }
            s if s.contains("指数运算") => {
                // 应该使用 <msup> 标签
                assert!(result.contains("<msup>"), "应该使用 <msup> 标签: {}", result);
            }
            s if s.contains("根号") => {
                // 应该使用 <msqrt> 标签
                assert!(result.contains("<msqrt>"), "应该使用 <msqrt> 标签: {}", result);
            }
            s if s.contains("(x/2)*x") => {
                // 应该有明确的运算符标签
                assert!(
                    result.contains("<mo>") && (result.contains("&middot;") || result.contains("*")),
                    "应该有明确的运算符标签: {}",
                    result
                );
            }
            _ => {}
        }
    }
}

#[test]
fn test_multiplication_symbol_consistency() {
    let formatters: Vec<(&str, Box<dyn Formatter>)> = vec![
        ("Standard", Box::new(StandardFormatter::new())),
        ("LaTeX", Box::new(LaTeXFormatter::new())),
        ("MathML", Box::new(MathMLFormatter::new())),
    ];
    
    // 测试不同类型的乘法表达式
    let multiplication_cases = vec![
        // 数字 * 变量
        ("2*x", Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Variable("x".to_string())),
        }),
        // 变量 * 变量
        ("x*y", Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Variable("y".to_string())),
        }),
        // 复杂表达式 * 变量
        ("(x+1)*y", Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
            }),
            right: Box::new(Expression::Variable("y".to_string())),
        }),
    ];
    
    println!("\n=== 乘法符号一致性测试 ===");
    
    for (case_name, expr) in multiplication_cases {
        println!("\n测试用例: {}", case_name);
        
        for (formatter_name, formatter) in &formatters {
            let result = formatter.format(&expr);
            println!("  {}: {}", formatter_name, result);
            
            // 验证结果不为空
            assert!(!result.is_empty(), "{}格式化器结果不应该为空", formatter_name);
        }
    }
}

#[test]
fn test_division_rendering_consistency() {
    let formatters: Vec<(&str, Box<dyn Formatter>)> = vec![
        ("Standard", Box::new(StandardFormatter::new())),
        ("LaTeX", Box::new(LaTeXFormatter::new())),
        ("MathML", Box::new(MathMLFormatter::new())),
    ];
    
    // 测试不同类型的除法表达式
    let division_cases = vec![
        // 简单除法
        ("x/2", Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        // 复杂分子
        ("(x+1)/2", Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::Variable("x".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
            }),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        // 复杂分母
        ("x/(y+1)", Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::Variable("y".to_string())),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
            }),
        }),
    ];
    
    println!("\n=== 除法渲染一致性测试 ===");
    
    for (case_name, expr) in division_cases {
        println!("\n测试用例: {}", case_name);
        
        for (formatter_name, formatter) in &formatters {
            let result = formatter.format(&expr);
            println!("  {}: {}", formatter_name, result);
            
            // 验证结果不为空
            assert!(!result.is_empty(), "{}格式化器结果不应该为空", formatter_name);
            
            // 特定格式检查
            match formatter_name {
                &"LaTeX" => {
                    if case_name != "x/2" {
                        // 复杂的除法应该使用 \frac
                        assert!(
                            result.contains("\\frac") || result.contains("/"),
                            "LaTeX 复杂除法应该使用适当格式: {}",
                            result
                        );
                    }
                }
                &"MathML" => {
                    // MathML 应该使用 <mfrac> 标签
                    assert!(
                        result.contains("<mfrac>"),
                        "MathML 除法应该使用 <mfrac> 标签: {}",
                        result
                    );
                }
                _ => {}
            }
        }
    }
}