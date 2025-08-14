//! # 词法分析器
//!
//! 将输入字符串分解为词法单元（tokens）。

use super::ParseError;

/// 词法单元类型
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 数值（支持整数、小数、科学记数法）
    Number(String),
    /// 标识符（变量名或函数名）
    Identifier(String),
    /// 运算符
    Operator(String),
    /// 左括号
    LeftParen,
    /// 右括号
    RightParen,
    /// 左方括号
    LeftBracket,
    /// 右方括号
    RightBracket,
    /// 逗号
    Comma,
    /// 输入结束
    EndOfInput,
}

/// 词法分析器
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// 创建新的词法分析器
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        
        Self {
            input: chars,
            position: 0,
            current_char,
        }
    }
    
    /// 获取下一个词法单元
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        loop {
            match self.current_char {
                None => return Ok(Token::EndOfInput),
                Some(ch) if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                Some(ch) if ch.is_ascii_digit() => {
                    return self.read_number();
                }
                Some(ch) if ch.is_alphabetic() || ch == '_' => {
                    return self.read_identifier();
                }
                Some('(') => {
                    self.advance();
                    return Ok(Token::LeftParen);
                }
                Some(')') => {
                    self.advance();
                    return Ok(Token::RightParen);
                }
                Some('[') => {
                    self.advance();
                    return Ok(Token::LeftBracket);
                }
                Some(']') => {
                    self.advance();
                    return Ok(Token::RightBracket);
                }
                Some(',') => {
                    self.advance();
                    return Ok(Token::Comma);
                }
                Some(_) => {
                    return self.read_operator();
                }
            }
        }
    }
    
    /// 读取数值（支持整数、小数、科学记数法）
    fn read_number(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        let mut number_str = String::new();
        let mut has_dot = false;
        let mut has_e = false;
        
        // 读取数字部分
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else if ch == '.' && !has_dot && !has_e {
                has_dot = true;
                number_str.push(ch);
                self.advance();
                
                // 确保小数点后至少有一个数字
                if let Some(next_ch) = self.current_char {
                    if !next_ch.is_ascii_digit() {
                        return Err(ParseError::invalid_number(number_str));
                    }
                } else {
                    return Err(ParseError::invalid_number(number_str));
                }
            } else if (ch == 'e' || ch == 'E') && !has_e {
                has_e = true;
                number_str.push(ch);
                self.advance();
                
                // 处理指数符号
                if let Some(sign_ch) = self.current_char {
                    if sign_ch == '+' || sign_ch == '-' {
                        number_str.push(sign_ch);
                        self.advance();
                    }
                }
                
                // 确保指数部分至少有一个数字
                if let Some(exp_ch) = self.current_char {
                    if !exp_ch.is_ascii_digit() {
                        return Err(ParseError::invalid_number(number_str));
                    }
                } else {
                    return Err(ParseError::invalid_number(number_str));
                }
            } else {
                break;
            }
        }
        
        // 验证数值格式
        if number_str.is_empty() {
            return Err(ParseError::syntax(start_pos, "空数值"));
        }
        
        // 验证数值是否有效
        self.validate_number(&number_str)?;
        
        Ok(Token::Number(number_str))
    }
    
    /// 验证数值格式
    fn validate_number(&self, number_str: &str) -> Result<(), ParseError> {
        // 尝试解析为浮点数来验证格式
        if number_str.parse::<f64>().is_err() {
            return Err(ParseError::invalid_number(number_str.to_string()));
        }
        Ok(())
    }
    
    /// 读取标识符（变量名或函数名）
    fn read_identifier(&mut self) -> Result<Token, ParseError> {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        if identifier.is_empty() {
            return Err(ParseError::syntax(self.position, "空标识符"));
        }
        
        Ok(Token::Identifier(identifier))
    }
    
    /// 读取运算符
    fn read_operator(&mut self) -> Result<Token, ParseError> {
        let start_pos = self.position;
        
        match self.current_char {
            Some('+') => {
                self.advance();
                Ok(Token::Operator("+".to_string()))
            }
            Some('-') => {
                self.advance();
                Ok(Token::Operator("-".to_string()))
            }
            Some('*') => {
                self.advance();
                // 检查是否是 **（幂运算）
                if let Some('*') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("**".to_string()))
                } else {
                    Ok(Token::Operator("*".to_string()))
                }
            }
            Some('/') => {
                self.advance();
                Ok(Token::Operator("/".to_string()))
            }
            Some('^') => {
                self.advance();
                Ok(Token::Operator("^".to_string()))
            }
            Some('%') => {
                self.advance();
                Ok(Token::Operator("%".to_string()))
            }
            Some('=') => {
                self.advance();
                // 检查是否是 ==
                if let Some('=') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("==".to_string()))
                } else {
                    Ok(Token::Operator("=".to_string()))
                }
            }
            Some('!') => {
                self.advance();
                // 检查是否是 !=
                if let Some('=') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("!=".to_string()))
                } else {
                    Ok(Token::Operator("!".to_string()))
                }
            }
            Some('<') => {
                self.advance();
                // 检查是否是 <=
                if let Some('=') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("<=".to_string()))
                } else {
                    Ok(Token::Operator("<".to_string()))
                }
            }
            Some('>') => {
                self.advance();
                // 检查是否是 >=
                if let Some('=') = self.current_char {
                    self.advance();
                    Ok(Token::Operator(">=".to_string()))
                } else {
                    Ok(Token::Operator(">".to_string()))
                }
            }
            Some('&') => {
                self.advance();
                // 检查是否是 &&
                if let Some('&') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("&&".to_string()))
                } else {
                    Err(ParseError::unexpected_character(start_pos, '&'))
                }
            }
            Some('|') => {
                self.advance();
                // 检查是否是 ||
                if let Some('|') = self.current_char {
                    self.advance();
                    Ok(Token::Operator("||".to_string()))
                } else {
                    Ok(Token::Operator("|".to_string()))
                }
            }
            Some(ch) => {
                Err(ParseError::unexpected_character(start_pos, ch))
            }
            None => {
                Err(ParseError::UnexpectedEndOfInput)
            }
        }
    }
    
    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// 前进到下一个字符
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).copied();
    }
    
    /// 获取当前位置
    pub fn position(&self) -> usize {
        self.position
    }
    
    /// 预览下一个字符（不移动位置）
    fn peek(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
}