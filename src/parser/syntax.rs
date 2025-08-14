//! # 语法分析器
//!
//! 将词法单元序列转换为抽象语法树。

use crate::core::Expression;
use super::{ParseError, lexer::Lexer};

/// 语法分析器
pub struct SyntaxParser {
    lexer: Lexer,
}

impl SyntaxParser {
    /// 创建新的语法分析器
    pub fn new(input: String) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
    
    /// 解析表达式
    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        // 这里是占位符实现，将在后续任务中完成
        todo!("语法分析器将在解析器实现任务中完成")
    }
}