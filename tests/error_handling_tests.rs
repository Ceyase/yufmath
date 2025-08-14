//! # 错误处理测试
//!
//! 测试各种错误类型的处理、用户友好消息和修复建议。

use yufmath::api::error::{YufmathError, FormatError};
use yufmath::parser::ParseError;
use yufmath::engine::{ComputeError, ErrorSeverity};

/// 测试解析错误的用户友好消息
#[test]
fn test_parse_error_user_friendly_messages() {
    let error = ParseError::syntax(5, "期望操作数");
    assert!(error.user_friendly_message().contains("语法错误（位置 6）"));
    assert!(error.user_friendly_message().contains("期望操作数"));
    
    let error = ParseError::unknown_function("sine");
    let message = error.user_friendly_message();
    assert!(message.contains("未知函数 'sine'"));
    assert!(message.contains("sin, cos, tan"));
    
    let error = ParseError::argument_count(2, 3);
    let message = error.user_friendly_message();
    assert!(message.contains("期望 2 个参数"));
    assert!(message.contains("提供了 3 个"));
}

/// 测试解析错误的修复建议
#[test]
fn test_parse_error_suggestions() {
    let error = ParseError::unknown_function("sine");
    let suggestions = error.suggestions();
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.contains("sin")));
    
    let error = ParseError::unmatched_parenthesis(10);
    let suggestions = error.suggestions();
    assert!(suggestions.iter().any(|s| s.contains("括号")));
    
    let error = ParseError::invalid_number("3.14.15");
    let suggestions = error.suggestions();
    assert!(suggestions.iter().any(|s| s.contains("数值格式")));
}

/// 测试解析错误的位置跟踪
#[test]
fn test_parse_error_position_tracking() {
    let error = ParseError::syntax(5, "测试错误");
    assert_eq!(error.position(), Some(5));
    
    let error = ParseError::unmatched_parenthesis(10);
    assert_eq!(error.position(), Some(10));
    
    let error = ParseError::unknown_function("test");
    assert_eq!(error.position(), None);
}

/// 测试解析错误的上下文格式化
#[test]
fn test_parse_error_context_formatting() {
    let error = ParseError::syntax(5, "期望操作数");
    let input = "2 + * 3";
    let formatted = error.format_with_context(input);
    
    assert!(formatted.contains("错误："));
    assert!(formatted.contains("输入："));
    assert!(formatted.contains("位置："));
    assert!(formatted.contains("^"));
    assert!(formatted.contains("建议："));
}

/// 测试计算错误的用户友好消息
#[test]
fn test_compute_error_user_friendly_messages() {
    let error = ComputeError::DivisionByZero;
    let message = error.user_friendly_message();
    assert!(message.contains("除零错误"));
    assert!(message.contains("不能除以零"));
    
    let error = ComputeError::undefined_variable("x");
    let message = error.user_friendly_message();
    assert!(message.contains("未定义变量 'x'"));
    
    let error = ComputeError::domain_error("负数的平方根");
    let message = error.user_friendly_message();
    assert!(message.contains("域错误"));
    assert!(message.contains("负数的平方根"));
}

/// 测试计算错误的修复建议
#[test]
fn test_compute_error_suggestions() {
    let error = ComputeError::DivisionByZero;
    let suggestions = error.suggestions();
    assert!(suggestions.iter().any(|s| s.contains("分母")));
    
    let error = ComputeError::undefined_variable("x");
    let suggestions = error.suggestions();
    assert!(suggestions.iter().any(|s| s.contains("赋值")));
    
    let error = ComputeError::domain_error("负数的平方根");
    let suggestions = error.suggestions();
    assert!(suggestions.iter().any(|s| s.contains("非负数")));
}

/// 测试计算错误的严重程度
#[test]
fn test_compute_error_severity() {
    assert_eq!(ComputeError::DivisionByZero.severity(), ErrorSeverity::High);
    assert_eq!(ComputeError::undefined_variable("x").severity(), ErrorSeverity::Medium);
    assert_eq!(ComputeError::Timeout.severity(), ErrorSeverity::Low);
}

/// 测试计算错误的可恢复性
#[test]
fn test_compute_error_recoverability() {
    assert!(!ComputeError::DivisionByZero.is_recoverable());
    assert!(ComputeError::undefined_variable("x").is_recoverable());
    assert!(ComputeError::Timeout.is_recoverable());
    assert!(!ComputeError::Overflow.is_recoverable());
}

/// 测试格式化错误
#[test]
fn test_format_error() {
    let error = FormatError::unsupported_format("xml");
    assert!(format!("{}", error).contains("不支持的格式：xml"));
    
    let error = FormatError::format_failure("表达式过于复杂");
    assert!(format!("{}", error).contains("格式化失败：表达式过于复杂"));
}

/// 测试顶层错误的用户友好消息
#[test]
fn test_yufmath_error_user_friendly_messages() {
    let parse_error = ParseError::syntax(0, "测试");
    let error = YufmathError::Parse(parse_error);
    let message = error.user_friendly_message();
    assert!(message.contains("语法错误"));
    
    let compute_error = ComputeError::DivisionByZero;
    let error = YufmathError::Compute(compute_error);
    let message = error.user_friendly_message();
    assert!(message.contains("除零错误"));
    
    let format_error = FormatError::unsupported_format("xml");
    let error = YufmathError::Format(format_error);
    let message = error.user_friendly_message();
    assert!(message.contains("不支持的输出格式"));
}

/// 测试顶层错误的严重程度
#[test]
fn test_yufmath_error_severity() {
    let parse_error = ParseError::syntax(0, "测试");
    let error = YufmathError::Parse(parse_error);
    assert_eq!(error.severity(), ErrorSeverity::Medium);
    
    let compute_error = ComputeError::DivisionByZero;
    let error = YufmathError::Compute(compute_error);
    assert_eq!(error.severity(), ErrorSeverity::High);
    
    let format_error = FormatError::unsupported_format("xml");
    let error = YufmathError::Format(format_error);
    assert_eq!(error.severity(), ErrorSeverity::Low);
}

/// 测试顶层错误的可恢复性
#[test]
fn test_yufmath_error_recoverability() {
    let parse_error = ParseError::syntax(0, "测试");
    let error = YufmathError::Parse(parse_error);
    assert!(error.is_recoverable());
    
    let compute_error = ComputeError::DivisionByZero;
    let error = YufmathError::Compute(compute_error);
    assert!(!error.is_recoverable());
    
    let error = YufmathError::internal("测试内部错误");
    assert!(!error.is_recoverable());
}

/// 测试完整的错误报告生成
#[test]
fn test_error_report_generation() {
    let parse_error = ParseError::syntax(5, "期望操作数");
    let error = YufmathError::Parse(parse_error);
    let input = "2 + * 3";
    let report = error.format_error_report(Some(input));
    
    assert!(report.contains("🚫"));
    assert!(report.contains("输入："));
    assert!(report.contains("位置："));
    assert!(report.contains("^"));
    assert!(report.contains("严重程度"));
    assert!(report.contains("💡 建议解决方案"));
    assert!(report.contains("✅"));
}

/// 测试错误报告不包含输入时的情况
#[test]
fn test_error_report_without_input() {
    let compute_error = ComputeError::DivisionByZero;
    let error = YufmathError::Compute(compute_error);
    let report = error.format_error_report(None);
    
    assert!(report.contains("🚫"));
    assert!(report.contains("🔥")); // 高严重程度图标
    assert!(report.contains("💡 建议解决方案"));
    assert!(report.contains("❌")); // 不可恢复
}

/// 测试编辑距离算法（用于函数名建议）
#[test]
fn test_function_name_suggestions() {
    let error = ParseError::unknown_function("sine");
    let suggestions = error.suggestions();
    
    // 应该建议 "sin" 因为编辑距离很小
    assert!(suggestions.iter().any(|s| s.contains("sin")));
    
    let error = ParseError::unknown_function("cosine");
    let suggestions = error.suggestions();
    
    // 应该建议 "cos" 因为编辑距离较小
    assert!(suggestions.iter().any(|s| s.contains("cos")));
}

/// 测试配置错误和内部错误
#[test]
fn test_config_and_internal_errors() {
    let error = YufmathError::config("无效的精度设置");
    let message = error.user_friendly_message();
    assert!(message.contains("配置错误"));
    assert!(message.contains("无效的精度设置"));
    
    let error = YufmathError::internal("空指针异常");
    let message = error.user_friendly_message();
    assert!(message.contains("内部错误"));
    assert!(message.contains("程序缺陷"));
}

/// 测试错误链和转换
#[test]
fn test_error_conversion() {
    let parse_error = ParseError::syntax(0, "测试");
    let yufmath_error: YufmathError = parse_error.into();
    assert!(matches!(yufmath_error, YufmathError::Parse(_)));
    
    let compute_error = ComputeError::DivisionByZero;
    let yufmath_error: YufmathError = compute_error.into();
    assert!(matches!(yufmath_error, YufmathError::Compute(_)));
    
    let format_error = FormatError::unsupported_format("xml");
    let yufmath_error: YufmathError = format_error.into();
    assert!(matches!(yufmath_error, YufmathError::Format(_)));
}

/// 测试错误的 Debug 和 Display 实现
#[test]
fn test_error_display_and_debug() {
    let error = ParseError::syntax(5, "测试错误");
    let display_str = format!("{}", error);
    let debug_str = format!("{:?}", error);
    
    assert!(display_str.contains("语法错误"));
    assert!(debug_str.contains("Syntax"));
    
    let error = ComputeError::DivisionByZero;
    let display_str = format!("{}", error);
    assert!(display_str.contains("除零错误"));
}

/// 测试错误的克隆和相等性
#[test]
fn test_error_clone_and_equality() {
    let error1 = ParseError::syntax(5, "测试");
    let error2 = error1.clone();
    assert_eq!(error1, error2);
    
    let error1 = ComputeError::undefined_variable("x");
    let error2 = ComputeError::undefined_variable("x");
    assert_eq!(error1, error2);
    
    let error1 = ComputeError::undefined_variable("x");
    let error2 = ComputeError::undefined_variable("y");
    assert_ne!(error1, error2);
}