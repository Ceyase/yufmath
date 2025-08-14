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
    
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            ComputeError::DivisionByZero => {
                "除零错误：不能除以零。请检查分母是否为零".to_string()
            }
            ComputeError::UndefinedVariable { name } => {
                format!("未定义变量 '{}'。请确保变量已被赋值或在表达式中定义", name)
            }
            ComputeError::Overflow => {
                "数值溢出：计算结果超出了数值范围。请尝试使用更小的数值或检查计算逻辑".to_string()
            }
            ComputeError::UnsupportedOperation { operation } => {
                format!("不支持的运算 '{}'。该运算可能尚未实现或不适用于当前数据类型", operation)
            }
            ComputeError::DomainError { message } => {
                format!("域错误：{}。请检查函数的输入值是否在有效范围内", message)
            }
            ComputeError::DimensionMismatch { message } => {
                format!("矩阵维度不匹配：{}。请检查矩阵的行列数是否符合运算要求", message)
            }
            ComputeError::SingularMatrix => {
                "奇异矩阵错误：矩阵不可逆（行列式为零）。请检查矩阵是否为满秩矩阵".to_string()
            }
            ComputeError::ConvergenceFailure { message } => {
                format!("数值方法收敛失败：{}。请尝试调整参数或使用其他方法", message)
            }
            ComputeError::Timeout => {
                "计算超时：计算时间超过了设定的限制。请尝试简化表达式或增加超时时间".to_string()
            }
            ComputeError::OutOfMemory => {
                "内存不足：计算需要的内存超过了可用内存。请尝试简化计算或释放内存".to_string()
            }
            ComputeError::Cancelled => {
                "计算已取消：用户主动取消了计算过程".to_string()
            }
        }
    }
    
    /// 获取修复建议
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            ComputeError::DivisionByZero => {
                vec![
                    "检查分母表达式是否可能为零".to_string(),
                    "使用条件判断来避免除零情况".to_string(),
                    "考虑使用极限来处理 0/0 类型的不定式".to_string(),
                ]
            }
            ComputeError::UndefinedVariable { name } => {
                vec![
                    format!("为变量 '{}' 赋值", name),
                    "检查变量名是否拼写正确".to_string(),
                    "确保变量在使用前已经定义".to_string(),
                ]
            }
            ComputeError::Overflow => {
                vec![
                    "使用更小的数值进行计算".to_string(),
                    "检查是否有无限循环或递归".to_string(),
                    "考虑使用任意精度数值类型".to_string(),
                    "分步计算以避免中间结果过大".to_string(),
                ]
            }
            ComputeError::UnsupportedOperation { operation } => {
                vec![
                    format!("检查运算 '{}' 是否适用于当前数据类型", operation),
                    "查看文档了解支持的运算列表".to_string(),
                    "考虑使用等价的支持运算".to_string(),
                ]
            }
            ComputeError::DomainError { .. } => {
                vec![
                    "检查函数输入值的有效范围".to_string(),
                    "对于平方根，确保输入为非负数".to_string(),
                    "对于对数，确保输入为正数".to_string(),
                    "对于反三角函数，确保输入在 [-1, 1] 范围内".to_string(),
                ]
            }
            ComputeError::DimensionMismatch { .. } => {
                vec![
                    "检查矩阵的行列数是否匹配运算要求".to_string(),
                    "矩阵乘法要求第一个矩阵的列数等于第二个矩阵的行数".to_string(),
                    "矩阵加减法要求两个矩阵具有相同的维度".to_string(),
                ]
            }
            ComputeError::SingularMatrix => {
                vec![
                    "检查矩阵是否为方阵".to_string(),
                    "确保矩阵的行列式不为零".to_string(),
                    "检查矩阵的行或列是否线性相关".to_string(),
                    "考虑使用伪逆或其他数值方法".to_string(),
                ]
            }
            ComputeError::ConvergenceFailure { .. } => {
                vec![
                    "增加迭代次数限制".to_string(),
                    "调整收敛精度要求".to_string(),
                    "检查初始值是否合适".to_string(),
                    "尝试使用不同的数值方法".to_string(),
                ]
            }
            ComputeError::Timeout => {
                vec![
                    "简化表达式以减少计算复杂度".to_string(),
                    "增加计算超时时间限制".to_string(),
                    "分解复杂计算为多个简单步骤".to_string(),
                ]
            }
            ComputeError::OutOfMemory => {
                vec![
                    "简化计算以减少内存使用".to_string(),
                    "释放不必要的变量和缓存".to_string(),
                    "使用流式计算处理大数据".to_string(),
                ]
            }
            ComputeError::Cancelled => {
                vec![
                    "如需继续计算，请重新启动".to_string(),
                ]
            }
        }
    }
    
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            ComputeError::DivisionByZero => ErrorSeverity::High,
            ComputeError::UndefinedVariable { .. } => ErrorSeverity::Medium,
            ComputeError::Overflow => ErrorSeverity::High,
            ComputeError::UnsupportedOperation { .. } => ErrorSeverity::Medium,
            ComputeError::DomainError { .. } => ErrorSeverity::High,
            ComputeError::DimensionMismatch { .. } => ErrorSeverity::Medium,
            ComputeError::SingularMatrix => ErrorSeverity::High,
            ComputeError::ConvergenceFailure { .. } => ErrorSeverity::Medium,
            ComputeError::Timeout => ErrorSeverity::Low,
            ComputeError::OutOfMemory => ErrorSeverity::High,
            ComputeError::Cancelled => ErrorSeverity::Low,
        }
    }
    
    /// 检查错误是否可以恢复
    pub fn is_recoverable(&self) -> bool {
        match self {
            ComputeError::UndefinedVariable { .. } => true,
            ComputeError::UnsupportedOperation { .. } => false,
            ComputeError::DimensionMismatch { .. } => true,
            ComputeError::ConvergenceFailure { .. } => true,
            ComputeError::Timeout => true,
            ComputeError::Cancelled => true,
            _ => false,
        }
    }
}

/// 错误严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// 低严重程度（警告）
    Low,
    /// 中等严重程度
    Medium,
    /// 高严重程度（严重错误）
    High,
}