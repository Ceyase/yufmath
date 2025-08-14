//! # 标准格式化器
//!
//! 将表达式格式化为标准数学记号。

use crate::core::Expression;
use super::{Formatter, FormatOptions};

/// 标准格式化器
pub struct StandardFormatter {
    options: FormatOptions,
}

impl StandardFormatter {
    /// 创建新的标准格式化器
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
        }
    }
}

impl Formatter for StandardFormatter {
    fn format(&self, _expr: &Expression) -> String {
        // 占位符实现，将在后续任务中完成
        todo!("格式化功能将在后续任务中实现")
    }
    
    fn set_options(&mut self, options: FormatOptions) {
        self.options = options;
    }
}