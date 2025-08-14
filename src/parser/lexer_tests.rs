//! # 词法分析器测试
//!
//! 测试词法分析器的各种功能。

use super::lexer::{Lexer, Token};
use super::ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::EndOfInput);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \t\n  ".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::EndOfInput);
    }

    #[test]
    fn test_simple_integer() {
        let mut lexer = Lexer::new("123".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("123".to_string()));
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::EndOfInput);
    }

    #[test]
    fn test_decimal_number() {
        let mut lexer = Lexer::new("123.456".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("123.456".to_string()));
    }

    #[test]
    fn test_scientific_notation() {
        let mut lexer = Lexer::new("1.23e10".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("1.23e10".to_string()));
        
        let mut lexer = Lexer::new("1.23E-5".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("1.23E-5".to_string()));
        
        let mut lexer = Lexer::new("5e+3".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("5e+3".to_string()));
    }

    #[test]
    fn test_large_integer() {
        let large_number = "123456789012345678901234567890";
        let mut lexer = Lexer::new(large_number.to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number(large_number.to_string()));
    }

    #[test]
    fn test_high_precision_decimal() {
        let high_precision = "3.141592653589793238462643383279502884197169399375105820974944592307816406286";
        let mut lexer = Lexer::new(high_precision.to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number(high_precision.to_string()));
    }

    #[test]
    fn test_invalid_number_formats() {
        // 小数点后没有数字
        let mut lexer = Lexer::new("123.".to_string());
        let result = lexer.next_token();
        assert!(result.is_err());
        
        // 指数后没有数字
        let mut lexer = Lexer::new("123e".to_string());
        let result = lexer.next_token();
        assert!(result.is_err());
        
        // 指数符号后没有数字
        let mut lexer = Lexer::new("123e+".to_string());
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("x".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("x".to_string()));
        
        let mut lexer = Lexer::new("variable_name".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("variable_name".to_string()));
        
        let mut lexer = Lexer::new("sin".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("sin".to_string()));
        
        let mut lexer = Lexer::new("func123".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Identifier("func123".to_string()));
    }

    #[test]
    fn test_basic_operators() {
        let operators = vec![
            ("+", "+"),
            ("-", "-"),
            ("*", "*"),
            ("/", "/"),
            ("^", "^"),
            ("%", "%"),
            ("=", "="),
            ("!", "!"),
            ("<", "<"),
            (">", ">"),
            ("|", "|"),
        ];
        
        for (input, expected) in operators {
            let mut lexer = Lexer::new(input.to_string());
            let token = lexer.next_token().unwrap();
            assert_eq!(token, Token::Operator(expected.to_string()));
        }
    }

    #[test]
    fn test_compound_operators() {
        let operators = vec![
            ("**", "**"),
            ("==", "=="),
            ("!=", "!="),
            ("<=", "<="),
            (">=", ">="),
            ("&&", "&&"),
            ("||", "||"),
        ];
        
        for (input, expected) in operators {
            let mut lexer = Lexer::new(input.to_string());
            let token = lexer.next_token().unwrap();
            assert_eq!(token, Token::Operator(expected.to_string()));
        }
    }

    #[test]
    fn test_parentheses_and_brackets() {
        let mut lexer = Lexer::new("()[]".to_string());
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::LeftParen);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::RightParen);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::LeftBracket);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::RightBracket);
    }

    #[test]
    fn test_comma() {
        let mut lexer = Lexer::new(",".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Comma);
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("2 * x + sin(3.14)".to_string());
        
        let tokens = vec![
            Token::Number("2".to_string()),
            Token::Operator("*".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("+".to_string()),
            Token::Identifier("sin".to_string()),
            Token::LeftParen,
            Token::Number("3.14".to_string()),
            Token::RightParen,
            Token::EndOfInput,
        ];
        
        for expected_token in tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_mathematical_expression() {
        let mut lexer = Lexer::new("x^2 + 2*x + 1".to_string());
        
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Operator("^".to_string()),
            Token::Number("2".to_string()),
            Token::Operator("+".to_string()),
            Token::Number("2".to_string()),
            Token::Operator("*".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("+".to_string()),
            Token::Number("1".to_string()),
            Token::EndOfInput,
        ];
        
        for expected_token in tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_function_call() {
        let mut lexer = Lexer::new("sqrt(x + 1)".to_string());
        
        let tokens = vec![
            Token::Identifier("sqrt".to_string()),
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Operator("+".to_string()),
            Token::Number("1".to_string()),
            Token::RightParen,
            Token::EndOfInput,
        ];
        
        for expected_token in tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_matrix_notation() {
        let mut lexer = Lexer::new("[[1, 2], [3, 4]]".to_string());
        
        let tokens = vec![
            Token::LeftBracket,
            Token::LeftBracket,
            Token::Number("1".to_string()),
            Token::Comma,
            Token::Number("2".to_string()),
            Token::RightBracket,
            Token::Comma,
            Token::LeftBracket,
            Token::Number("3".to_string()),
            Token::Comma,
            Token::Number("4".to_string()),
            Token::RightBracket,
            Token::RightBracket,
            Token::EndOfInput,
        ];
        
        for expected_token in tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_unexpected_character() {
        let mut lexer = Lexer::new("@".to_string());
        let result = lexer.next_token();
        assert!(result.is_err());
        
        if let Err(ParseError::UnexpectedCharacter { pos, ch }) = result {
            assert_eq!(pos, 0);
            assert_eq!(ch, '@');
        } else {
            panic!("Expected UnexpectedCharacter error");
        }
    }

    #[test]
    fn test_invalid_ampersand() {
        let mut lexer = Lexer::new("&".to_string());
        let result = lexer.next_token();
        assert!(result.is_err());
        
        if let Err(ParseError::UnexpectedCharacter { pos, ch }) = result {
            assert_eq!(pos, 0);
            assert_eq!(ch, '&');
        } else {
            panic!("Expected UnexpectedCharacter error");
        }
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("123 + x".to_string());
        
        assert_eq!(lexer.position(), 0);
        
        let _token1 = lexer.next_token().unwrap(); // "123"
        let _token2 = lexer.next_token().unwrap(); // "+"
        let _token3 = lexer.next_token().unwrap(); // "x"
        
        // 位置应该在字符串末尾
        assert_eq!(lexer.position(), 7);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("  123   +   x  ".to_string());
        
        let tokens = vec![
            Token::Number("123".to_string()),
            Token::Operator("+".to_string()),
            Token::Identifier("x".to_string()),
            Token::EndOfInput,
        ];
        
        for expected_token in tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_zero_numbers() {
        let test_cases = vec![
            "0",
            "0.0",
            "0.000",
            "0e0",
            "0.0e0",
            "0E+0",
            "0.0E-0",
        ];
        
        for case in test_cases {
            let mut lexer = Lexer::new(case.to_string());
            let token = lexer.next_token().unwrap();
            assert_eq!(token, Token::Number(case.to_string()));
        }
    }

    #[test]
    fn test_edge_case_numbers() {
        // 非常小的数
        let mut lexer = Lexer::new("1e-100".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("1e-100".to_string()));
        
        // 非常大的数
        let mut lexer = Lexer::new("1e+100".to_string());
        let token = lexer.next_token().unwrap();
        assert_eq!(token, Token::Number("1e+100".to_string()));
    }
}