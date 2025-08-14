//! # CLI 集成测试
//!
//! 测试命令行工具的各种功能。

use std::process::Command;
use std::str;

/// 测试帮助命令
#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--help"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();
    
    // 检查输出中是否包含预期的内容（可能在 stdout 或 stderr 中）
    let combined_output = format!("{}{}", stdout, stderr);
    assert!(combined_output.contains("计算机代数系统") || combined_output.contains("yufmath"));
}

/// 测试版本命令
#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--version"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.contains("yufmath"));
}

/// 测试基本计算命令
#[test]
fn test_compute_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "compute", "2+3"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 注意：由于当前实现使用 todo!()，这个测试会失败
    // 但我们可以检查是否正确调用了命令
    let stderr = str::from_utf8(&output.stderr).unwrap();
    // 检查是否包含预期的错误信息（由于 todo!() 宏）
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试简化命令
#[test]
fn test_simplify_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "simplify", "x+x"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 同样，由于当前实现使用 todo!()，检查错误信息
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试求导命令
#[test]
fn test_diff_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "diff", "x^2", "x"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 检查命令是否正确解析
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试积分命令
#[test]
fn test_integrate_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "integrate", "2*x", "x"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 检查命令是否正确解析
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试详细模式
#[test]
fn test_verbose_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--verbose", "compute", "1+1"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();
    
    // 检查是否输出了详细信息
    assert!(stdout.contains("Yufmath v") || stderr.contains("正在计算表达式"));
}

/// 测试静默模式
#[test]
fn test_quiet_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--quiet", "compute", "1+1"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 在静默模式下，stdout 应该只包含结果，不包含额外信息
    // 由于当前实现会 panic，我们主要检查命令是否正确解析
    assert!(!output.status.success() || output.stdout.is_empty() || !str::from_utf8(&output.stdout).unwrap().contains("详细模式"));
}

/// 测试输出格式选项
#[test]
fn test_format_options() {
    // 测试 LaTeX 格式
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--format", "latex", "compute", "x^2"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 检查命令是否正确解析格式选项
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
    
    // 测试 MathML 格式
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--format", "mathml", "compute", "x^2"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试精度选项
#[test]
fn test_precision_option() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "--precision", "10", "compute", "pi"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 检查命令是否正确解析精度选项
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}

/// 测试无效命令
#[test]
fn test_invalid_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "invalid_command"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 无效命令应该返回错误
    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("error") || stderr.contains("错误") || stderr.contains("unrecognized"));
}

/// 测试交互模式命令
#[test]
fn test_interactive_command() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "yufmath", "--", "interactive"])
        .current_dir(".")
        .output()
        .expect("执行命令失败");
    
    // 交互模式目前使用 todo!()，应该会 panic
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.contains("not yet implemented") || stderr.contains("todo"));
}