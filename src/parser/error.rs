//! # 解析错误定义
//!
//! 定义表达式解析过程中可能出现的各种错误类型。

use thiserror::Error;

/// 解析错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseError {
    /// 语法错误
    #[error("语法错误：位置 {pos}，{message}")]
    Syntax { pos: usize, message: String },
    
    /// 未知函数
    #[error("未知函数：{name}")]
    UnknownFunction { name: String },
    
    /// 参数数量错误
    #[error("参数数量错误：期望 {expected}，实际 {actual}")]
    ArgumentCount { expected: usize, actual: usize },
    
    /// 无效的数值格式
    #[error("无效的数值格式：{value}")]
    InvalidNumber { value: String },
    
    /// 无效的变量名
    #[error("无效的变量名：{name}")]
    InvalidVariable { name: String },
    
    /// 括号不匹配
    #[error("括号不匹配：位置 {pos}")]
    UnmatchedParenthesis { pos: usize },
    
    /// 意外的输入结束
    #[error("意外的输入结束")]
    UnexpectedEndOfInput,
    
    /// 意外的字符
    #[error("意外的字符：位置 {pos}，字符 '{ch}'")]
    UnexpectedCharacter { pos: usize, ch: char },
    
    /// 空表达式
    #[error("空表达式")]
    EmptyExpression,
}

impl ParseError {
    /// 创建语法错误
    pub fn syntax(pos: usize, message: impl Into<String>) -> Self {
        ParseError::Syntax {
            pos,
            message: message.into(),
        }
    }
    
    /// 创建未知函数错误
    pub fn unknown_function(name: impl Into<String>) -> Self {
        ParseError::UnknownFunction {
            name: name.into(),
        }
    }
    
    /// 创建参数数量错误
    pub fn argument_count(expected: usize, actual: usize) -> Self {
        ParseError::ArgumentCount { expected, actual }
    }
    
    /// 创建无效数值错误
    pub fn invalid_number(value: impl Into<String>) -> Self {
        ParseError::InvalidNumber {
            value: value.into(),
        }
    }
    
    /// 创建无效变量名错误
    pub fn invalid_variable(name: impl Into<String>) -> Self {
        ParseError::InvalidVariable {
            name: name.into(),
        }
    }
    
    /// 创建括号不匹配错误
    pub fn unmatched_parenthesis(pos: usize) -> Self {
        ParseError::UnmatchedParenthesis { pos }
    }
    
    /// 创建意外字符错误
    pub fn unexpected_character(pos: usize, ch: char) -> Self {
        ParseError::UnexpectedCharacter { pos, ch }
    }
    
    /// 获取错误位置（如果有）
    pub fn position(&self) -> Option<usize> {
        match self {
            ParseError::Syntax { pos, .. } => Some(*pos),
            ParseError::UnmatchedParenthesis { pos } => Some(*pos),
            ParseError::UnexpectedCharacter { pos, .. } => Some(*pos),
            _ => None,
        }
    }
}