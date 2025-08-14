//! # 顶层错误定义
//!
//! 定义 Yufmath 库的顶层错误类型，整合各个模块的错误。

use thiserror::Error;
use crate::parser::ParseError;
use crate::engine::ComputeError;

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