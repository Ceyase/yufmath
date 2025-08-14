//! # 错误处理演示
//!
//! 展示 Yufmath 库的错误处理功能，包括用户友好的错误消息、
//! 修复建议和错误恢复策略。

use yufmath::api::error::{YufmathError, FormatError};
use yufmath::parser::ParseError;
use yufmath::engine::ComputeError;

fn main() {
    println!("🔧 Yufmath 错误处理系统演示\n");
    
    // 演示解析错误
    demonstrate_parse_errors();
    
    // 演示计算错误
    demonstrate_compute_errors();
    
    // 演示格式化错误
    demonstrate_format_errors();
    
    // 演示完整的错误报告
    demonstrate_error_reports();
}

/// 演示解析错误的处理
fn demonstrate_parse_errors() {
    println!("📝 解析错误演示");
    println!("{}", "=".repeat(50));
    
    // 语法错误
    let error = ParseError::syntax(5, "期望操作数");
    println!("1. 语法错误：");
    println!("   {}", error.user_friendly_message());
    println!("   建议：{:?}\n", error.suggestions());
    
    // 未知函数错误
    let error = ParseError::unknown_function("sine");
    println!("2. 未知函数错误：");
    println!("   {}", error.user_friendly_message());
    println!("   建议：{:?}\n", error.suggestions());
    
    // 括号不匹配错误
    let error = ParseError::unmatched_parenthesis(10);
    println!("3. 括号不匹配错误：");
    println!("   {}", error.user_friendly_message());
    println!("   建议：{:?}\n", error.suggestions());
    
    // 带上下文的错误格式化
    let error = ParseError::syntax(4, "期望操作数");
    let input = "2 + * 3";
    println!("4. 带上下文的错误显示：");
    println!("{}", error.format_with_context(input));
}

/// 演示计算错误的处理
fn demonstrate_compute_errors() {
    println!("🧮 计算错误演示");
    println!("{}", "=".repeat(50));
    
    // 除零错误
    let error = ComputeError::DivisionByZero;
    println!("1. 除零错误：");
    println!("   消息：{}", error.user_friendly_message());
    println!("   严重程度：{:?}", error.severity());
    println!("   可恢复：{}", error.is_recoverable());
    println!("   建议：{:?}\n", error.suggestions());
    
    // 未定义变量错误
    let error = ComputeError::undefined_variable("x");
    println!("2. 未定义变量错误：");
    println!("   消息：{}", error.user_friendly_message());
    println!("   严重程度：{:?}", error.severity());
    println!("   可恢复：{}", error.is_recoverable());
    println!("   建议：{:?}\n", error.suggestions());
    
    // 域错误
    let error = ComputeError::domain_error("负数的平方根");
    println!("3. 域错误：");
    println!("   消息：{}", error.user_friendly_message());
    println!("   严重程度：{:?}", error.severity());
    println!("   可恢复：{}", error.is_recoverable());
    println!("   建议：{:?}\n", error.suggestions());
}

/// 演示格式化错误的处理
fn demonstrate_format_errors() {
    println!("🎨 格式化错误演示");
    println!("{}", "=".repeat(50));
    
    // 不支持的格式错误
    let error = FormatError::unsupported_format("xml");
    println!("1. 不支持的格式错误：");
    println!("   {}\n", error);
    
    // 格式化失败错误
    let error = FormatError::format_failure("表达式过于复杂");
    println!("2. 格式化失败错误：");
    println!("   {}\n", error);
}

/// 演示完整的错误报告
fn demonstrate_error_reports() {
    println!("📋 完整错误报告演示");
    println!("{}", "=".repeat(50));
    
    // 解析错误的完整报告
    let parse_error = ParseError::syntax(5, "期望操作数");
    let error = YufmathError::Parse(parse_error);
    let input = "2 + * 3";
    println!("1. 解析错误的完整报告：");
    println!("{}", error.format_error_report(Some(input)));
    
    // 计算错误的完整报告
    let compute_error = ComputeError::DivisionByZero;
    let error = YufmathError::Compute(compute_error);
    println!("2. 计算错误的完整报告：");
    println!("{}", error.format_error_report(None));
    
    // 内部错误的完整报告
    let error = YufmathError::internal("空指针异常");
    println!("3. 内部错误的完整报告：");
    println!("{}", error.format_error_report(None));
}

/// 演示错误恢复策略
#[allow(dead_code)]
fn demonstrate_error_recovery() {
    println!("🔄 错误恢复策略演示");
    println!("{}", "=".repeat(50));
    
    let errors = vec![
        YufmathError::Parse(ParseError::syntax(0, "测试")),
        YufmathError::Compute(ComputeError::undefined_variable("x")),
        YufmathError::Compute(ComputeError::DivisionByZero),
        YufmathError::internal("测试内部错误"),
    ];
    
    for (i, error) in errors.iter().enumerate() {
        println!("错误 {}: {}", i + 1, error.user_friendly_message());
        println!("  严重程度: {:?}", error.severity());
        println!("  可恢复: {}", error.is_recoverable());
        
        if error.is_recoverable() {
            println!("  🔧 恢复策略: 根据建议修复后重试");
        } else {
            println!("  ⚠️  恢复策略: 需要程序重启或技术支持");
        }
        println!();
    }
}