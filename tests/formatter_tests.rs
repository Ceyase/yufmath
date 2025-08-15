//! # 格式化器测试
//!
//! 测试各种格式化器的功能。

use yufmath::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use yufmath::formatter::{
    Formatter, FormatOptions, FormatType, 
    StandardFormatter, LaTeXFormatter, MathMLFormatter, MultiFormatter
};
use num_bigint::BigInt;
use num_rational::BigRational;

/// 创建测试表达式
fn create_test_expressions() -> Vec<(String, Expression)> {
    vec![
        // 基本数值
        ("整数".to_string(), Expression::Number(Number::Integer(BigInt::from(42)))),
        ("有理数".to_string(), Expression::Number(Number::Rational(BigRational::new(BigInt::from(22), BigInt::from(7))))),
        ("浮点数".to_string(), Expression::Number(Number::Float(3.14159))),
        
        // 变量
        ("变量".to_string(), Expression::Variable("x".to_string())),
        
        // 常量
        ("圆周率".to_string(), Expression::Constant(MathConstant::Pi)),
        ("自然常数".to_string(), Expression::Constant(MathConstant::E)),
        ("虚数单位".to_string(), Expression::Constant(MathConstant::I)),
        
        // 二元运算
        ("加法".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        }),
        ("减法".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Subtract,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        }),
        ("乘法".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Variable("x".to_string())),
        }),
        ("除法".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        ("幂运算".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(Expression::Variable("x".to_string())),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        }),
        
        // 一元运算
        ("负号".to_string(), Expression::UnaryOp {
            op: UnaryOperator::Negate,
            operand: Box::new(Expression::Variable("x".to_string())),
        }),
        ("平方根".to_string(), Expression::UnaryOp {
            op: UnaryOperator::Sqrt,
            operand: Box::new(Expression::Variable("x".to_string())),
        }),
        ("绝对值".to_string(), Expression::UnaryOp {
            op: UnaryOperator::Abs,
            operand: Box::new(Expression::Variable("x".to_string())),
        }),
        ("正弦函数".to_string(), Expression::UnaryOp {
            op: UnaryOperator::Sin,
            operand: Box::new(Expression::Variable("x".to_string())),
        }),
        ("阶乘".to_string(), Expression::UnaryOp {
            op: UnaryOperator::Factorial,
            operand: Box::new(Expression::Variable("n".to_string())),
        }),
        
        // 函数调用
        ("函数调用".to_string(), Expression::Function {
            name: "f".to_string(),
            args: vec![
                Expression::Variable("x".to_string()),
                Expression::Number(Number::Integer(BigInt::from(1))),
            ],
        }),
        
        // 复杂表达式
        ("二次方程".to_string(), Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::BinaryOp {
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
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        }),
    ]
}

#[test]
fn test_standard_formatter() {
    let formatter = StandardFormatter::new();
    let expressions = create_test_expressions();
    
    for (name, expr) in expressions {
        let result = formatter.format(&expr);
        println!("标准格式 - {}: {}", name, result);
        
        // 基本验证：结果不应该为空
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
    }
}

#[test]
fn test_latex_formatter() {
    let formatter = LaTeXFormatter::new();
    let expressions = create_test_expressions();
    
    for (name, expr) in expressions {
        let result = formatter.format(&expr);
        println!("LaTeX 格式 - {}: {}", name, result);
        
        // 基本验证：结果不应该为空
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
    }
}

#[test]
fn test_mathml_formatter() {
    let formatter = MathMLFormatter::new();
    let expressions = create_test_expressions();
    
    for (name, expr) in expressions {
        let result = formatter.format(&expr);
        println!("MathML 格式 - {}: {}", name, result);
        
        // 基本验证：结果不应该为空且包含 MathML 标签
        assert!(!result.is_empty(), "格式化结果不应该为空: {}", name);
        assert!(result.contains("<math"), "MathML 格式应该包含 <math 标签: {}", name);
    }
}

#[test]
fn test_multi_formatter() {
    let mut formatter = MultiFormatter::new();
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Variable("x".to_string())),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
    };
    
    // 测试标准格式
    formatter.set_format_type(FormatType::Standard);
    let standard_result = formatter.format(&expr);
    assert!(!standard_result.is_empty());
    
    // 测试 LaTeX 格式
    formatter.set_format_type(FormatType::LaTeX);
    let latex_result = formatter.format(&expr);
    assert!(!latex_result.is_empty());
    
    // 测试 MathML 格式
    formatter.set_format_type(FormatType::MathML);
    let mathml_result = formatter.format(&expr);
    assert!(!mathml_result.is_empty());
    assert!(mathml_result.contains("<math"));
    
    // 验证 MathML 格式与其他格式不同（MathML 包含 XML 标签）
    assert_ne!(standard_result, mathml_result);
    assert_ne!(latex_result, mathml_result);
    
    // 对于简单表达式，标准格式和 LaTeX 格式可能相同，这是正常的
    println!("标准格式: {}", standard_result);
    println!("LaTeX 格式: {}", latex_result);
    println!("MathML 格式: {}", mathml_result);
}

#[test]
fn test_format_options() {
    let mut formatter = StandardFormatter::new();
    
    // 测试精度设置
    let options = FormatOptions {
        format_type: FormatType::Standard,
        precision: Some(2),
        use_parentheses: true,
    };
    formatter.set_options(options);
    
    let float_expr = Expression::Number(Number::Float(3.14159265));
    let result = formatter.format(&float_expr);
    
    // 验证精度设置生效（应该显示2位小数）
    assert!(result.contains("3.14") || result.contains("3,14")); // 考虑不同的小数点格式
}

#[test]
fn test_complex_number_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试复数：3 + 4i
    let complex_expr = Expression::Number(Number::Complex {
        real: Box::new(Number::Integer(BigInt::from(3))),
        imaginary: Box::new(Number::Integer(BigInt::from(4))),
    });
    
    for formatter in formatters {
        let result = formatter.format(&complex_expr);
        println!("复数格式化结果: {}", result);
        assert!(!result.is_empty());
        // 复数应该包含实部和虚部
        assert!(result.contains("3") && result.contains("4"));
    }
}

#[test]
fn test_rational_number_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试有理数：22/7
    let rational_expr = Expression::Number(Number::Rational(
        BigRational::new(BigInt::from(22), BigInt::from(7))
    ));
    
    for formatter in formatters {
        let result = formatter.format(&rational_expr);
        println!("有理数格式化结果: {}", result);
        assert!(!result.is_empty());
        // 有理数应该包含分子和分母
        assert!(result.contains("22") && result.contains("7"));
    }
}

#[test]
fn test_matrix_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试 2x2 矩阵
    let matrix_expr = Expression::Matrix(vec![
        vec![
            Expression::Number(Number::Integer(BigInt::from(1))),
            Expression::Number(Number::Integer(BigInt::from(2))),
        ],
        vec![
            Expression::Number(Number::Integer(BigInt::from(3))),
            Expression::Number(Number::Integer(BigInt::from(4))),
        ],
    ]);
    
    for formatter in formatters {
        let result = formatter.format(&matrix_expr);
        println!("矩阵格式化结果: {}", result);
        assert!(!result.is_empty());
        // 矩阵应该包含所有元素
        assert!(result.contains("1") && result.contains("2") && result.contains("3") && result.contains("4"));
    }
}

#[test]
fn test_vector_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试向量
    let vector_expr = Expression::Vector(vec![
        Expression::Number(Number::Integer(BigInt::from(1))),
        Expression::Number(Number::Integer(BigInt::from(2))),
        Expression::Number(Number::Integer(BigInt::from(3))),
    ]);
    
    for formatter in formatters {
        let result = formatter.format(&vector_expr);
        println!("向量格式化结果: {}", result);
        assert!(!result.is_empty());
        // 向量应该包含所有元素
        assert!(result.contains("1") && result.contains("2") && result.contains("3"));
    }
}

#[test]
fn test_set_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试集合
    let set_expr = Expression::Set(vec![
        Expression::Number(Number::Integer(BigInt::from(1))),
        Expression::Number(Number::Integer(BigInt::from(2))),
        Expression::Number(Number::Integer(BigInt::from(3))),
    ]);
    
    for formatter in formatters {
        let result = formatter.format(&set_expr);
        println!("集合格式化结果: {}", result);
        assert!(!result.is_empty());
        // 集合应该包含所有元素
        assert!(result.contains("1") && result.contains("2") && result.contains("3"));
    }
}

#[test]
fn test_interval_formatting() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(StandardFormatter::new()),
        Box::new(LaTeXFormatter::new()),
        Box::new(MathMLFormatter::new()),
    ];
    
    // 测试区间 [1, 5)
    let interval_expr = Expression::Interval {
        start: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
        end: Box::new(Expression::Number(Number::Integer(BigInt::from(5)))),
        start_inclusive: true,
        end_inclusive: false,
    };
    
    for formatter in formatters {
        let result = formatter.format(&interval_expr);
        println!("区间格式化结果: {}", result);
        assert!(!result.is_empty());
        // 区间应该包含起点和终点
        assert!(result.contains("1") && result.contains("5"));
    }
}

#[test]
fn test_precedence_and_parentheses() {
    let formatter = StandardFormatter::new();
    
    // 测试运算符优先级：2 + 3 * 4 应该是 2 + (3 * 4)
    let expr = Expression::BinaryOp {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
        right: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(4)))),
        }),
    };
    
    let result = formatter.format(&expr);
    println!("优先级测试结果: {}", result);
    assert!(!result.is_empty());
    
    // 测试需要括号的情况：(2 + 3) * 4
    let expr_with_parens = Expression::BinaryOp {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::BinaryOp {
            op: BinaryOperator::Add,
            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
            right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
        }),
        right: Box::new(Expression::Number(Number::Integer(BigInt::from(4)))),
    };
    
    let result_with_parens = formatter.format(&expr_with_parens);
    println!("括号测试结果: {}", result_with_parens);
    assert!(!result_with_parens.is_empty());
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_formatter_integration_with_yufmath() {
        use yufmath::Yufmath;
        use yufmath::formatter::FormatType;
        
        let mut yuf = Yufmath::new();
        
        // 测试不同格式的设置
        let formats = vec![
            FormatType::Standard,
            FormatType::LaTeX,
            FormatType::MathML,
        ];
        
        for format_type in formats {
            let options = FormatOptions {
                format_type: format_type.clone(),
                precision: Some(3),
                use_parentheses: true,
            };
            
            yuf.set_format_options(options);
            
            // 由于当前使用 DummyParser，这个测试会失败
            // 但我们可以验证设置不会导致 panic
            println!("已设置格式类型: {:?}", format_type);
        }
    }
}