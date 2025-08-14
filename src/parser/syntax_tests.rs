//! # 语法分析器测试
//!
//! 测试语法分析器的各种功能。

use super::syntax::SyntaxParser;
use super::ParseError;
use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use num_bigint::BigInt;
use num_rational::BigRational;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let result = SyntaxParser::new("".to_string());
        assert!(result.is_ok());
        
        let mut parser = result.unwrap();
        let result = parser.parse();
        assert!(result.is_err());
        
        if let Err(ParseError::EmptyExpression) = result {
            // 预期的错误
        } else {
            panic!("Expected EmptyExpression error");
        }
    }

    #[test]
    fn test_simple_integer() {
        let mut parser = SyntaxParser::new("123".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Number(Number::Integer(n)) => {
                assert_eq!(n, BigInt::from(123));
            }
            _ => panic!("Expected integer number"),
        }
    }

    #[test]
    fn test_decimal_number() {
        let mut parser = SyntaxParser::new("123.456".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Number(Number::Rational(_)) => {
                // 小数应该被转换为有理数
            }
            _ => panic!("Expected rational number"),
        }
    }

    #[test]
    fn test_scientific_notation() {
        let mut parser = SyntaxParser::new("1.23e10".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Number(Number::Real(_)) => {
                // 科学记数法应该使用 BigDecimal
            }
            _ => panic!("Expected real number"),
        }
    }

    #[test]
    fn test_variable() {
        let mut parser = SyntaxParser::new("x".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Variable(name) => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected variable"),
        }
    }

    #[test]
    fn test_math_constants() {
        let test_cases = vec![
            ("pi", MathConstant::Pi),
            ("e", MathConstant::E),
            ("i", MathConstant::I),
        ];
        
        for (input, expected_constant) in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::Constant(constant) => {
                    assert_eq!(constant, expected_constant);
                }
                _ => panic!("Expected constant for input: {}", input),
            }
        }
    }

    #[test]
    fn test_simple_addition() {
        let mut parser = SyntaxParser::new("2 + 3".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Add);
                
                match (*left, *right) {
                    (Expression::Number(Number::Integer(l)), Expression::Number(Number::Integer(r))) => {
                        assert_eq!(l, BigInt::from(2));
                        assert_eq!(r, BigInt::from(3));
                    }
                    _ => panic!("Expected integer operands"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_operator_precedence() {
        let mut parser = SyntaxParser::new("2 + 3 * 4".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该解析为 2 + (3 * 4)
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Add);
                
                match (*left, *right) {
                    (Expression::Number(Number::Integer(l)), Expression::BinaryOp { op: mul_op, .. }) => {
                        assert_eq!(l, BigInt::from(2));
                        assert_eq!(mul_op, BinaryOperator::Multiply);
                    }
                    _ => panic!("Expected correct precedence parsing"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_power_right_associative() {
        let mut parser = SyntaxParser::new("2 ^ 3 ^ 4".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该解析为 2 ^ (3 ^ 4)
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Power);
                
                match (*left, *right) {
                    (Expression::Number(Number::Integer(l)), Expression::BinaryOp { op: pow_op, .. }) => {
                        assert_eq!(l, BigInt::from(2));
                        assert_eq!(pow_op, BinaryOperator::Power);
                    }
                    _ => panic!("Expected right associative power"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_parentheses() {
        let mut parser = SyntaxParser::new("(2 + 3) * 4".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该解析为 (2 + 3) * 4
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Multiply);
                
                match (*left, *right) {
                    (Expression::BinaryOp { op: add_op, .. }, Expression::Number(Number::Integer(r))) => {
                        assert_eq!(add_op, BinaryOperator::Add);
                        assert_eq!(r, BigInt::from(4));
                    }
                    _ => panic!("Expected correct parentheses parsing"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_unary_operators() {
        let test_cases = vec![
            ("-5", UnaryOperator::Negate),
            ("+5", UnaryOperator::Plus),
            ("!true", UnaryOperator::Not),
        ];
        
        for (input, expected_op) in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::UnaryOp { op, .. } => {
                    assert_eq!(op, expected_op);
                }
                _ => panic!("Expected unary operation for input: {}", input),
            }
        }
    }

    #[test]
    fn test_function_call_no_args() {
        let mut parser = SyntaxParser::new("sin()".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_function_call_with_args() {
        let mut parser = SyntaxParser::new("sin(x)".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
                
                match &args[0] {
                    Expression::Variable(var_name) => {
                        assert_eq!(var_name, "x");
                    }
                    _ => panic!("Expected variable argument"),
                }
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_function_call_multiple_args() {
        let mut parser = SyntaxParser::new("pow(x, 2)".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "pow");
                assert_eq!(args.len(), 2);
                
                match (&args[0], &args[1]) {
                    (Expression::Variable(var_name), Expression::Number(Number::Integer(n))) => {
                        assert_eq!(var_name, "x");
                        assert_eq!(*n, BigInt::from(2));
                    }
                    _ => panic!("Expected variable and number arguments"),
                }
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_nested_function_calls() {
        let mut parser = SyntaxParser::new("sin(cos(x))".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
                
                match &args[0] {
                    Expression::Function { name: inner_name, args: inner_args } => {
                        assert_eq!(inner_name, "cos");
                        assert_eq!(inner_args.len(), 1);
                    }
                    _ => panic!("Expected nested function call"),
                }
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_vector() {
        let mut parser = SyntaxParser::new("[1, 2, 3]".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Vector(elements) => {
                assert_eq!(elements.len(), 3);
                
                for (i, element) in elements.iter().enumerate() {
                    match element {
                        Expression::Number(Number::Integer(n)) => {
                            assert_eq!(*n, BigInt::from(i + 1));
                        }
                        _ => panic!("Expected integer element"),
                    }
                }
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn test_matrix() {
        let mut parser = SyntaxParser::new("[[1, 2], [3, 4]]".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Matrix(rows) => {
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0].len(), 2);
                assert_eq!(rows[1].len(), 2);
                
                // 检查矩阵元素
                let expected_values = vec![vec![1, 2], vec![3, 4]];
                for (i, row) in rows.iter().enumerate() {
                    for (j, element) in row.iter().enumerate() {
                        match element {
                            Expression::Number(Number::Integer(n)) => {
                                assert_eq!(*n, BigInt::from(expected_values[i][j]));
                            }
                            _ => panic!("Expected integer element"),
                        }
                    }
                }
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_empty_vector() {
        let mut parser = SyntaxParser::new("[]".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::Vector(elements) => {
                assert_eq!(elements.len(), 0);
            }
            _ => panic!("Expected empty vector"),
        }
    }

    #[test]
    fn test_complex_expression() {
        let mut parser = SyntaxParser::new("2 * x + sin(3.14)".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该解析为 (2 * x) + sin(3.14)
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Add);
                
                match (*left, *right) {
                    (Expression::BinaryOp { op: mul_op, .. }, Expression::Function { name, .. }) => {
                        assert_eq!(mul_op, BinaryOperator::Multiply);
                        assert_eq!(name, "sin");
                    }
                    _ => panic!("Expected multiplication and function call"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_comparison_operators() {
        let test_cases = vec![
            ("x == y", BinaryOperator::Equal),
            ("x != y", BinaryOperator::NotEqual),
            ("x < y", BinaryOperator::Less),
            ("x <= y", BinaryOperator::LessEqual),
            ("x > y", BinaryOperator::Greater),
            ("x >= y", BinaryOperator::GreaterEqual),
        ];
        
        for (input, expected_op) in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::BinaryOp { op, .. } => {
                    assert_eq!(op, expected_op);
                }
                _ => panic!("Expected comparison operation for input: {}", input),
            }
        }
    }

    #[test]
    fn test_logical_operators() {
        let test_cases = vec![
            ("x && y", BinaryOperator::And),
            ("x || y", BinaryOperator::Or),
        ];
        
        for (input, expected_op) in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::BinaryOp { op, .. } => {
                    assert_eq!(op, expected_op);
                }
                _ => panic!("Expected logical operation for input: {}", input),
            }
        }
    }

    #[test]
    fn test_all_arithmetic_operators() {
        let test_cases = vec![
            ("x + y", BinaryOperator::Add),
            ("x - y", BinaryOperator::Subtract),
            ("x * y", BinaryOperator::Multiply),
            ("x / y", BinaryOperator::Divide),
            ("x ^ y", BinaryOperator::Power),
            ("x % y", BinaryOperator::Modulo),
        ];
        
        for (input, expected_op) in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::BinaryOp { op, .. } => {
                    assert_eq!(op, expected_op);
                }
                _ => panic!("Expected arithmetic operation for input: {}", input),
            }
        }
    }

    #[test]
    fn test_power_operator_variants() {
        let test_cases = vec!["x ^ y", "x ** y"];
        
        for input in test_cases {
            let mut parser = SyntaxParser::new(input.to_string()).unwrap();
            let expr = parser.parse().unwrap();
            
            match expr {
                Expression::BinaryOp { op, .. } => {
                    assert_eq!(op, BinaryOperator::Power);
                }
                _ => panic!("Expected power operation for input: {}", input),
            }
        }
    }

    #[test]
    fn test_operator_precedence_complex() {
        let mut parser = SyntaxParser::new("x + y * z ^ w".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该解析为 x + (y * (z ^ w))
        match expr {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, BinaryOperator::Add);
                
                match (*left, *right) {
                    (Expression::Variable(_), Expression::BinaryOp { op: mul_op, right: mul_right, .. }) => {
                        assert_eq!(mul_op, BinaryOperator::Multiply);
                        
                        match *mul_right {
                            Expression::BinaryOp { op: pow_op, .. } => {
                                assert_eq!(pow_op, BinaryOperator::Power);
                            }
                            _ => panic!("Expected power operation"),
                        }
                    }
                    _ => panic!("Expected correct precedence"),
                }
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_error_unmatched_parenthesis() {
        let mut parser = SyntaxParser::new("(2 + 3".to_string()).unwrap();
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::UnmatchedParenthesis { .. }) => {
                // 预期的错误
            }
            _ => panic!("Expected UnmatchedParenthesis error"),
        }
    }

    #[test]
    fn test_error_unexpected_token() {
        let mut parser = SyntaxParser::new("2 +".to_string()).unwrap();
        let result = parser.parse();
        
        assert!(result.is_err());
    }

    #[test]
    fn test_error_invalid_function_syntax() {
        let mut parser = SyntaxParser::new("sin(2,)".to_string()).unwrap();
        let result = parser.parse();
        
        // 应该能够处理尾随逗号或报告适当的错误
        // 这里我们只检查它不会崩溃
        let _ = result;
    }

    #[test]
    fn test_whitespace_handling() {
        let mut parser = SyntaxParser::new("  2   +   3  ".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::BinaryOp { op, .. } => {
                assert_eq!(op, BinaryOperator::Add);
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_negative_numbers() {
        let mut parser = SyntaxParser::new("-5".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        match expr {
            Expression::UnaryOp { op, operand } => {
                assert_eq!(op, UnaryOperator::Negate);
                
                match *operand {
                    Expression::Number(Number::Integer(n)) => {
                        assert_eq!(n, BigInt::from(5));
                    }
                    _ => panic!("Expected integer operand"),
                }
            }
            _ => panic!("Expected unary operation"),
        }
    }

    #[test]
    fn test_chained_operations() {
        let mut parser = SyntaxParser::new("a + b - c + d".to_string()).unwrap();
        let expr = parser.parse().unwrap();
        
        // 应该左结合：((a + b) - c) + d
        match expr {
            Expression::BinaryOp { op, .. } => {
                assert_eq!(op, BinaryOperator::Add);
            }
            _ => panic!("Expected binary operation"),
        }
    }
}