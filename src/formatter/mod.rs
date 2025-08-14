//! # 表达式格式化器
//!
//! 本模块负责将内部表达式结构转换为各种输出格式，
//! 包括标准数学记号、LaTeX、MathML 等格式。

pub mod standard;
pub mod latex;
pub mod mathml;

use crate::core::Expression;

/// 输出格式类型
#[derive(Debug, Clone, PartialEq)]
pub enum FormatType {
    /// 标准数学记号
    Standard,
    /// LaTeX 格式
    LaTeX,
    /// MathML 格式
    MathML,
}

/// 格式化选项
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// 输出格式类型
    pub format_type: FormatType,
    /// 数值精度
    pub precision: Option<usize>,
    /// 是否使用括号
    pub use_parentheses: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            format_type: FormatType::Standard,
            precision: None,
            use_parentheses: true,
        }
    }
}

/// 表达式格式化器 trait
pub trait Formatter {
    /// 将表达式格式化为字符串
    fn format(&self, expr: &Expression) -> String;
    
    /// 设置输出格式选项
    fn set_options(&mut self, options: FormatOptions);
}