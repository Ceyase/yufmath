//! # 语法分析器
//!
//! 将词法单元序列转换为抽象语法树。

use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{ParseError, lexer::{Lexer, Token}};
use num_bigint::BigInt;
use num_rational::BigRational;
use bigdecimal::BigDecimal;
use std::str::FromStr;

/// 语法分析器
pub struct SyntaxParser {
    lexer: Lexer,
    current_token: Token,
}

impl SyntaxParser {
    /// 创建新的语法分析器
    pub fn new(input: String) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        
        Ok(Self {
            lexer,
            current_token,
        })
    }
    
    /// 解析表达式
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        if matches!(self.current_token, Token::EndOfInput) {
            return Err(ParseError::EmptyExpression);
        }
        
        let expr = self.parse_expression()?;
        
        // 确保输入已经完全消费
        if !matches!(self.current_token, Token::EndOfInput) {
            return Err(ParseError::syntax(
                self.lexer.position(),
                format!("意外的标记: {:?}", self.current_token)
            ));
        }
        
        Ok(expr)
    }
    
    /// 解析表达式（处理所有优先级）
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_logical_or()
    }
    
    /// 解析逻辑或表达式 (优先级 1)
    fn parse_logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_logical_and()?;
        
        while let Token::Operator(op) = &self.current_token {
            if op == "||" {
                self.advance()?;
                let right = self.parse_logical_and()?;
                left = Expression::binary_op(BinaryOperator::Or, left, right);
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// 解析逻辑与表达式 (优先级 2)
    fn parse_logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_equality()?;
        
        while let Token::Operator(op) = &self.current_token {
            if op == "&&" {
                self.advance()?;
                let right = self.parse_equality()?;
                left = Expression::binary_op(BinaryOperator::And, left, right);
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    /// 解析相等性表达式 (优先级 3)
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_comparison()?;
        
        while let Token::Operator(op) = &self.current_token {
            let binary_op = match op.as_str() {
                "==" => BinaryOperator::Equal,
                "!=" => BinaryOperator::NotEqual,
                _ => break,
            };
            
            self.advance()?;
            let right = self.parse_comparison()?;
            left = Expression::binary_op(binary_op, left, right);
        }
        
        Ok(left)
    }
    
    /// 解析比较表达式 (优先级 4)
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_term()?;
        
        while let Token::Operator(op) = &self.current_token {
            let binary_op = match op.as_str() {
                "<" => BinaryOperator::Less,
                "<=" => BinaryOperator::LessEqual,
                ">" => BinaryOperator::Greater,
                ">=" => BinaryOperator::GreaterEqual,
                _ => break,
            };
            
            self.advance()?;
            let right = self.parse_term()?;
            left = Expression::binary_op(binary_op, left, right);
        }
        
        Ok(left)
    }
    
    /// 解析项表达式 (优先级 6: 加法和减法)
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_factor()?;
        
        while let Token::Operator(op) = &self.current_token {
            let binary_op = match op.as_str() {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Subtract,
                _ => break,
            };
            
            self.advance()?;
            let right = self.parse_factor()?;
            left = Expression::binary_op(binary_op, left, right);
        }
        
        Ok(left)
    }
    
    /// 解析因子表达式 (优先级 7: 乘法、除法、取模)
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_power()?;
        
        while let Token::Operator(op) = &self.current_token {
            let binary_op = match op.as_str() {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                "%" => BinaryOperator::Modulo,
                _ => break,
            };
            
            self.advance()?;
            let right = self.parse_power()?;
            left = Expression::binary_op(binary_op, left, right);
        }
        
        Ok(left)
    }
    
    /// 解析幂表达式 (优先级 9: 右结合)
    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let left = self.parse_unary()?;
        
        if let Token::Operator(op) = &self.current_token {
            if op == "^" || op == "**" {
                self.advance()?;
                let right = self.parse_power()?; // 右结合
                return Ok(Expression::binary_op(BinaryOperator::Power, left, right));
            }
        }
        
        Ok(left)
    }
    
    /// 解析一元表达式
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        match &self.current_token {
            Token::Operator(op) => {
                let unary_op = match op.as_str() {
                    "-" => UnaryOperator::Negate,
                    "+" => UnaryOperator::Plus,
                    "!" => UnaryOperator::Not,
                    _ => return self.parse_primary(),
                };
                
                self.advance()?;
                let operand = self.parse_unary()?;
                Ok(Expression::unary_op(unary_op, operand))
            }
            _ => self.parse_primary(),
        }
    }
    
    /// 解析基本表达式
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match &self.current_token.clone() {
            Token::Number(num_str) => {
                self.advance()?;
                self.parse_number(num_str)
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance()?;
                
                // 检查是否是函数调用
                if matches!(self.current_token, Token::LeftParen) {
                    self.parse_function_call(name)
                } else {
                    // 检查是否是数学常量
                    if let Some(constant) = MathConstant::from_str(&name) {
                        Ok(Expression::constant(constant))
                    } else {
                        Ok(Expression::variable(name))
                    }
                }
            }
            Token::LeftParen => {
                self.advance()?; // 消费 '('
                let expr = self.parse_expression()?;
                
                if !matches!(self.current_token, Token::RightParen) {
                    return Err(ParseError::unmatched_parenthesis(self.lexer.position()));
                }
                
                self.advance()?; // 消费 ')'
                Ok(expr)
            }
            Token::LeftBracket => {
                self.parse_matrix_or_vector()
            }
            Token::EndOfInput => {
                Err(ParseError::UnexpectedEndOfInput)
            }
            _ => {
                Err(ParseError::syntax(
                    self.lexer.position(),
                    format!("意外的标记: {:?}", self.current_token)
                ))
            }
        }
    }
    
    /// 解析数值
    fn parse_number(&self, num_str: &str) -> Result<Expression, ParseError> {
        // 尝试解析为整数
        if !num_str.contains('.') && !num_str.contains('e') && !num_str.contains('E') {
            if let Ok(int_val) = BigInt::from_str(num_str) {
                return Ok(Expression::number(Number::Integer(int_val)));
            }
        }
        
        // 尝试解析为有理数（对于有限小数）
        if num_str.contains('.') && !num_str.contains('e') && !num_str.contains('E') {
            if let Ok(decimal) = BigDecimal::from_str(num_str) {
                // 尝试转换为有理数
                if let Some(rational) = self.decimal_to_rational(&decimal) {
                    return Ok(Expression::number(Number::Rational(rational)));
                } else {
                    return Ok(Expression::number(Number::Real(decimal)));
                }
            }
        }
        
        // 对于科学记数法，使用 BigDecimal
        if let Ok(decimal) = BigDecimal::from_str(num_str) {
            Ok(Expression::number(Number::Real(decimal)))
        } else {
            Err(ParseError::invalid_number(num_str.to_string()))
        }
    }
    
    /// 将 BigDecimal 转换为 BigRational（如果可能）
    fn decimal_to_rational(&self, decimal: &BigDecimal) -> Option<BigRational> {
        // 简单实现：对于有限小数，转换为分数
        let decimal_str = decimal.to_string();
        if let Some(dot_pos) = decimal_str.find('.') {
            let integer_part = &decimal_str[..dot_pos];
            let fractional_part = &decimal_str[dot_pos + 1..];
            
            if let (Ok(int_part), Ok(frac_part)) = (
                BigInt::from_str(integer_part),
                BigInt::from_str(fractional_part)
            ) {
                let denominator = BigInt::from(10).pow(fractional_part.len() as u32);
                let numerator = int_part * &denominator + frac_part;
                return Some(BigRational::new(numerator, denominator));
            }
        }
        None
    }
    
    /// 解析函数调用
    fn parse_function_call(&mut self, name: String) -> Result<Expression, ParseError> {
        // 消费 '('
        if !matches!(self.current_token, Token::LeftParen) {
            return Err(ParseError::syntax(
                self.lexer.position(),
                "期望 '(' 开始函数参数列表".to_string()
            ));
        }
        self.advance()?;
        
        let mut args = Vec::new();
        
        // 处理空参数列表
        if matches!(self.current_token, Token::RightParen) {
            self.advance()?;
            return Ok(Expression::function(name, args));
        }
        
        // 解析参数列表
        loop {
            args.push(self.parse_expression()?);
            
            match &self.current_token {
                Token::Comma => {
                    self.advance()?;
                    continue;
                }
                Token::RightParen => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError::syntax(
                        self.lexer.position(),
                        "期望 ',' 或 ')' 在函数参数列表中".to_string()
                    ));
                }
            }
        }
        
        Ok(Expression::function(name, args))
    }
    
    /// 解析矩阵或向量
    fn parse_matrix_or_vector(&mut self) -> Result<Expression, ParseError> {
        // 消费 '['
        self.advance()?;
        
        // 处理空矩阵/向量
        if matches!(self.current_token, Token::RightBracket) {
            self.advance()?;
            return Ok(Expression::Vector(Vec::new()));
        }
        
        // 检查第一个元素是否是 '['（表示这是一个矩阵）
        if matches!(self.current_token, Token::LeftBracket) {
            // 这是一个矩阵
            let mut rows = Vec::new();
            
            // 解析第一行
            rows.push(self.parse_matrix_row()?);
            
            // 解析剩余行
            while matches!(self.current_token, Token::Comma) {
                self.advance()?; // 消费 ','
                
                if matches!(self.current_token, Token::RightBracket) {
                    break; // 允许尾随逗号
                }
                
                rows.push(self.parse_matrix_row()?);
            }
            
            if !matches!(self.current_token, Token::RightBracket) {
                return Err(ParseError::syntax(
                    self.lexer.position(),
                    "期望 ']' 结束矩阵".to_string()
                ));
            }
            self.advance()?;
            
            Expression::matrix(rows).map_err(|e| ParseError::syntax(self.lexer.position(), e))
        } else {
            // 这是一个向量
            let mut elements = Vec::new();
            
            // 解析第一个元素
            elements.push(self.parse_expression()?);
            
            // 解析剩余元素
            while matches!(self.current_token, Token::Comma) {
                self.advance()?; // 消费 ','
                
                if matches!(self.current_token, Token::RightBracket) {
                    break; // 允许尾随逗号
                }
                
                elements.push(self.parse_expression()?);
            }
            
            if !matches!(self.current_token, Token::RightBracket) {
                return Err(ParseError::syntax(
                    self.lexer.position(),
                    "期望 ']' 结束向量".to_string()
                ));
            }
            self.advance()?;
            
            Ok(Expression::Vector(elements))
        }
    }
    
    /// 解析矩阵行
    fn parse_matrix_row(&mut self) -> Result<Vec<Expression>, ParseError> {
        if !matches!(self.current_token, Token::LeftBracket) {
            return Err(ParseError::syntax(
                self.lexer.position(),
                "期望 '[' 开始矩阵行".to_string()
            ));
        }
        self.advance()?; // 消费 '['
        
        let mut elements = Vec::new();
        
        // 处理空行
        if matches!(self.current_token, Token::RightBracket) {
            self.advance()?;
            return Ok(elements);
        }
        
        // 解析元素
        loop {
            elements.push(self.parse_expression()?);
            
            match &self.current_token {
                Token::Comma => {
                    self.advance()?;
                    continue;
                }
                Token::RightBracket => {
                    self.advance()?;
                    break;
                }
                _ => {
                    return Err(ParseError::syntax(
                        self.lexer.position(),
                        "期望 ',' 或 ']' 在矩阵行中".to_string()
                    ));
                }
            }
        }
        
        Ok(elements)
    }
    
    /// 前进到下一个标记
    fn advance(&mut self) -> Result<(), ParseError> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }
}