//! # 表达式格式化器
//!
//! 本模块负责将内部表达式结构转换为各种输出格式，
//! 包括标准数学记号、LaTeX、MathML 等格式。

pub mod standard;
pub mod latex;
pub mod mathml;
pub mod terminal;

// 重新导出格式化器
pub use standard::StandardFormatter;
pub use latex::LaTeXFormatter;
pub use mathml::MathMLFormatter;
pub use terminal::TerminalFormatter;

use crate::core::Expression;

/// 输出格式类型
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FormatType {
    /// 标准数学记号
    Standard,
    /// 终端彩色格式
    Terminal,
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
pub trait Formatter: Send + Sync {
    /// 将表达式格式化为字符串
    fn format(&self, expr: &Expression) -> String;
    
    /// 设置输出格式选项
    fn set_options(&mut self, options: FormatOptions);
}

/// 格式化器工厂
pub struct FormatterFactory;

impl FormatterFactory {
    /// 根据格式类型创建格式化器
    pub fn create_formatter(format_type: FormatType) -> Box<dyn Formatter> {
        match format_type {
            FormatType::Standard => Box::new(StandardFormatter::new()),
            FormatType::Terminal => Box::new(TerminalFormatter::new()),
            FormatType::LaTeX => Box::new(LaTeXFormatter::new()),
            FormatType::MathML => Box::new(MathMLFormatter::new()),
        }
    }
}

/// 多格式化器 - 支持动态切换格式
pub struct MultiFormatter {
    current_formatter: Box<dyn Formatter>,
    options: FormatOptions,
}

impl MultiFormatter {
    /// 创建新的多格式化器
    pub fn new() -> Self {
        let options = FormatOptions::default();
        let formatter = FormatterFactory::create_formatter(options.format_type.clone());
        Self {
            current_formatter: formatter,
            options,
        }
    }
    
    /// 创建带指定格式的多格式化器
    pub fn with_format(format_type: FormatType) -> Self {
        let mut options = FormatOptions::default();
        options.format_type = format_type.clone();
        let formatter = FormatterFactory::create_formatter(format_type);
        Self {
            current_formatter: formatter,
            options,
        }
    }
    
    /// 切换格式类型
    pub fn set_format_type(&mut self, format_type: FormatType) {
        if self.options.format_type != format_type {
            self.options.format_type = format_type.clone();
            self.current_formatter = FormatterFactory::create_formatter(format_type);
            self.current_formatter.set_options(self.options.clone());
        }
    }
    
    /// 获取当前格式类型
    pub fn get_format_type(&self) -> &FormatType {
        &self.options.format_type
    }
}

impl Default for MultiFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for MultiFormatter {
    fn format(&self, expr: &Expression) -> String {
        self.current_formatter.format(expr)
    }
    
    fn set_options(&mut self, options: FormatOptions) {
        // 如果格式类型改变，需要重新创建格式化器
        if self.options.format_type != options.format_type {
            self.current_formatter = FormatterFactory::create_formatter(options.format_type.clone());
        }
        
        self.options = options.clone();
        self.current_formatter.set_options(options);
    }
}