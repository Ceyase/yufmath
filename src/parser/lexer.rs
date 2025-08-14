//! # 词法分析器
//!
//! 将输入字符串分解为词法单元（tokens）。

use super::ParseError;

/// 词法单元类型
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 数值
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
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    /// 创建新的词法分析器
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.chars().next();
        lexer
    }
    
    /// 获取下一个词法单元
    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        // 这里是占位符实现，将在后续任务中完成
        todo!("词法分析器将在解析器实现任务中完成")
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
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
        }
    }
}