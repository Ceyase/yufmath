//! # 计算错误定义
//!
//! 定义数学计算过程中可能出现的各种错误类型。

use thiserror::Error;

/// 计算错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ComputeError {
    /// 除零错误
    #[error("除零错误")]
    DivisionByZero,
    
    /// 未定义变量
    #[error("未定义变量：{name}")]
    UndefinedVariable { name: String },
    
    /// 数值溢出
    #[error("数值溢出")]
    Overflow,
    
    /// 不支持的运算
    #[error("不支持的运算：{operation}")]
    UnsupportedOperation { operation: String },
    
    /// 域错误（如负数的平方根）
    #[error("域错误：{message}")]
    DomainError { message: String },
    
    /// 矩阵维度不匹配
    #[error("矩阵维度不匹配：{message}")]
    DimensionMismatch { message: String },
    
    /// 奇异矩阵（不可逆）
    #[error("奇异矩阵，无法求逆")]
    SingularMatrix,
    
    /// 收敛失败
    #[error("数值方法收敛失败：{message}")]
    ConvergenceFailure { message: String },
    
    /// 超时错误
    #[error("计算超时")]
    Timeout,
    
    /// 内存不足
    #[error("内存不足")]
    OutOfMemory,
    
    /// 用户取消
    #[error("用户取消计算")]
    Cancelled,
}

impl ComputeError {
    /// 创建未定义变量错误
    pub fn undefined_variable(name: impl Into<String>) -> Self {
        ComputeError::UndefinedVariable {
            name: name.into(),
        }
    }
    
    /// 创建不支持的运算错误
    pub fn unsupported_operation(operation: impl Into<String>) -> Self {
        ComputeError::UnsupportedOperation {
            operation: operation.into(),
        }
    }
    
    /// 创建域错误
    pub fn domain_error(message: impl Into<String>) -> Self {
        ComputeError::DomainError {
            message: message.into(),
        }
    }
    
    /// 创建维度不匹配错误
    pub fn dimension_mismatch(message: impl Into<String>) -> Self {
        ComputeError::DimensionMismatch {
            message: message.into(),
        }
    }
    
    /// 创建收敛失败错误
    pub fn convergence_failure(message: impl Into<String>) -> Self {
        ComputeError::ConvergenceFailure {
            message: message.into(),
        }
    }
}