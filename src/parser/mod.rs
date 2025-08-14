//! # 表达式解析器
//!
//! 本模块实现数学表达式的词法分析和语法分析，
//! 将字符串形式的数学表达式转换为内部的表达式树结构。

pub mod lexer;
pub mod syntax;
pub mod error;

#[cfg(test)]
mod lexer_tests;

#[cfg(test)]
mod syntax_tests;

use crate::core::Expression;
pub use error::ParseError;

/// 表达式解析器 trait
pub trait Parser {
    /// 解析字符串为表达式
    fn parse(&self, input: &str) -> Result<Expression, ParseError>;
    
    /// 验证表达式语法
    fn validate(&self, input: &str) -> Result<(), ParseError>;
}