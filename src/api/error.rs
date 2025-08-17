//! # 顶层错误定义
//!
//! 定义 Yufmath 库的顶层错误类型，整合各个模块的错误。

use thiserror::Error;
use crate::parser::ParseError;
use crate::engine::{ComputeError, ErrorSeverity};

/// 顶层错误类型
#[derive(Debug, Error)]
pub enum YufmathError {
    /// 解析错误
    #[error("解析错误: {0}")]
    Parse(#[from] ParseError),
    
    /// 计算错误
    #[error("计算错误: {0}")]
    Compute(#[from] ComputeError),
    
    /// 格式化错误
    #[error("格式化错误: {0}")]
    Format(#[from] FormatError),
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    /// 配置错误
    #[error("配置错误: {message}")]
    Config { message: String },
    
    /// 内部错误
    #[error("内部错误: {message}")]
    Internal { message: String },
}

/// 格式化错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum FormatError {
    /// 不支持的格式
    #[error("不支持的格式：{format}")]
    UnsupportedFormat { format: String },
    
    /// 格式化失败
    #[error("格式化失败：{message}")]
    FormatFailure { message: String },
}

impl YufmathError {
    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        YufmathError::Config {
            message: message.into(),
        }
    }
    
    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self {
        YufmathError::Internal {
            message: message.into(),
        }
    }
    
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            YufmathError::Parse(e) => e.user_friendly_message(),
            YufmathError::Compute(e) => e.user_friendly_message(),
            YufmathError::Format(e) => match e {
                FormatError::UnsupportedFormat { format } => {
                    format!("不支持的输出格式 '{}'。支持的格式：standard, latex, mathml", format)
                }
                FormatError::FormatFailure { message } => {
                    format!("格式化失败：{}。请检查表达式是否过于复杂", message)
                }
            },
            YufmathError::Io(e) => {
                format!("文件操作错误：{}。请检查文件路径和权限", e)
            }
            YufmathError::Config { message } => {
                format!("配置错误：{}。请检查配置文件或参数设置", message)
            }
            YufmathError::Internal { message } => {
                format!("内部错误：{}。这可能是程序缺陷，请报告此问题", message)
            }
        }
    }
    
    /// 获取修复建议
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            YufmathError::Parse(e) => e.suggestions(),
            YufmathError::Compute(e) => e.suggestions(),
            YufmathError::Format(e) => match e {
                FormatError::UnsupportedFormat { .. } => {
                    vec![
                        "使用支持的格式：standard, latex, mathml".to_string(),
                        "检查格式名称的拼写是否正确".to_string(),
                    ]
                }
                FormatError::FormatFailure { .. } => {
                    vec![
                        "尝试简化表达式".to_string(),
                        "检查表达式是否包含不支持的元素".to_string(),
                        "使用不同的输出格式".to_string(),
                    ]
                }
            },
            YufmathError::Io(_) => {
                vec![
                    "检查文件路径是否正确".to_string(),
                    "确保有足够的文件访问权限".to_string(),
                    "检查磁盘空间是否充足".to_string(),
                ]
            }
            YufmathError::Config { .. } => {
                vec![
                    "检查配置文件的语法是否正确".to_string(),
                    "确保所有必需的配置项都已设置".to_string(),
                    "参考文档了解正确的配置格式".to_string(),
                ]
            }
            YufmathError::Internal { .. } => {
                vec![
                    "这是程序内部错误，请报告给开发者".to_string(),
                    "尝试重启程序".to_string(),
                    "检查是否有可用的程序更新".to_string(),
                ]
            }
        }
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            YufmathError::Parse(_) => ErrorSeverity::Medium,
            YufmathError::Compute(e) => e.severity(),
            YufmathError::Format(_) => ErrorSeverity::Low,
            YufmathError::Io(_) => ErrorSeverity::Medium,
            YufmathError::Config { .. } => ErrorSeverity::Medium,
            YufmathError::Internal { .. } => ErrorSeverity::High,
        }
    }
    
    /// 检查错误是否可以恢复
    pub fn is_recoverable(&self) -> bool {
        match self {
            YufmathError::Parse(_) => true,
            YufmathError::Compute(e) => e.is_recoverable(),
            YufmathError::Format(_) => true,
            YufmathError::Io(_) => true,
            YufmathError::Config { .. } => true,
            YufmathError::Internal { .. } => false,
        }
    }
    
    /// 生成完整的错误报告
    pub fn format_error_report(&self, input: Option<&str>) -> String {
        let mut report = String::new();
        
        // 错误标题
        report.push_str(&format!("错误: {}\n", self.user_friendly_message()));
        
        // 如果是解析错误且有输入，显示位置信息
        if let (YufmathError::Parse(parse_error), Some(input_str)) = (self, input) {
            if let Some(pos) = parse_error.position() {
                if pos < input_str.len() {
                    report.push_str(&format!("\n输入：{}\n", input_str));
                    report.push_str(&format!("位置：{}{}\n", " ".repeat(pos + 3), "^"));
                }
            }
        }
        
        // 严重程度指示
        report.push_str(&format!("\n严重程度：{:?}\n", self.severity()));
        
        // 修复建议
        let suggestions = self.suggestions();
        if !suggestions.is_empty() {
            report.push_str("\n建议解决方案：\n");
            for (i, suggestion) in suggestions.iter().enumerate() {
                report.push_str(&format!("  {}. {}\n", i + 1, suggestion));
            }
        }
        
        // 恢复信息
        if self.is_recoverable() {
            report.push_str("\n此错误可以修复，请根据建议进行调整后重试\n");
        } else {
            report.push_str("\n此错误无法自动恢复，可能需要程序重启或联系技术支持\n");
        }
        
        report
    }
}

impl FormatError {
    /// 创建不支持的格式错误
    pub fn unsupported_format(format: impl Into<String>) -> Self {
        FormatError::UnsupportedFormat {
            format: format.into(),
        }
    }
    
    /// 创建格式化失败错误
    pub fn format_failure(message: impl Into<String>) -> Self {
        FormatError::FormatFailure {
            message: message.into(),
        }
    }
}