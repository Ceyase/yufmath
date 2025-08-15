//! # 交互式模式测试
//!
//! 测试交互式 REPL 功能的各个方面。

use yufmath::cli::interactive::InteractiveSession;

#[test]
fn test_interactive_session_creation() {
    let session = InteractiveSession::new();
    // 测试会话创建成功
    // 由于 InteractiveSession 的字段是私有的，我们只能测试创建不会 panic
}

#[test]
fn test_help_command() {
    let mut session = InteractiveSession::new();
    
    let result = session.process_command("help").unwrap();
    assert!(result.contains("Yufmath 交互式计算器帮助"));
    assert!(result.contains("基本命令"));
    assert!(result.contains("数学运算"));
    
    // 测试 ? 别名
    let result = session.process_command("?").unwrap();
    assert!(result.contains("Yufmath 交互式计算器帮助"));
}

#[test]
fn test_quit_commands() {
    let mut session = InteractiveSession::new();
    
    // 测试各种退出命令
    let result = session.process_command("quit").unwrap();
    assert_eq!(result, "再见！");
    
    let result = session.process_command("exit").unwrap();
    assert_eq!(result, "再见！");
    
    let result = session.process_command("q").unwrap();
    assert_eq!(result, "再见！");
}

#[test]
fn test_clear_command() {
    let mut session = InteractiveSession::new();
    
    let result = session.process_command("clear").unwrap();
    assert_eq!(result, "变量已清空");
}

#[test]
fn test_variables_command() {
    let mut session = InteractiveSession::new();
    
    // 初始状态应该没有变量
    let result = session.process_command("vars").unwrap();
    assert_eq!(result, "没有定义变量");
    
    // 测试 variables 别名
    let result = session.process_command("variables").unwrap();
    assert_eq!(result, "没有定义变量");
}

#[test]
fn test_verbose_command() {
    let mut session = InteractiveSession::new();
    
    // 切换详细模式
    let result = session.process_command("verbose").unwrap();
    assert!(result.contains("详细模式"));
    
    // 再次切换
    let result = session.process_command("verbose").unwrap();
    assert!(result.contains("详细模式"));
}

#[test]
fn test_format_commands() {
    let mut session = InteractiveSession::new();
    
    // 测试设置标准格式
    let result = session.process_command("format standard").unwrap();
    assert!(result.contains("输出格式已设置为"));
    
    // 测试设置 LaTeX 格式
    let result = session.process_command("format latex").unwrap();
    assert!(result.contains("输出格式已设置为"));
    
    // 测试设置 MathML 格式
    let result = session.process_command("format mathml").unwrap();
    assert!(result.contains("输出格式已设置为"));
    
    // 测试无效格式
    let result = session.process_command("format invalid").unwrap();
    assert!(result.contains("无效的格式类型"));
}

#[test]
fn test_precision_commands() {
    let mut session = InteractiveSession::new();
    
    // 测试设置有效精度
    let result = session.process_command("precision 10").unwrap();
    assert!(result.contains("数值精度已设置为: 10"));
    
    // 测试无效精度
    let result = session.process_command("precision abc").unwrap();
    assert!(result.contains("无效的精度值"));
    
    let result = session.process_command("precision -5").unwrap();
    assert!(result.contains("无效的精度值"));
}

#[test]
fn test_empty_input() {
    let mut session = InteractiveSession::new();
    
    // 空输入应该返回空字符串
    let result = session.process_command("").unwrap();
    assert_eq!(result, "");
    
    let result = session.process_command("   ").unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_assignment_parsing() {
    let mut session = InteractiveSession::new();
    
    // 注意：由于当前的计算引擎是占位符实现，这些测试可能会失败
    // 但我们可以测试赋值语法的解析
    
    // 这个测试会因为 DummyParser 而失败，但我们可以测试它不会 panic
    let result = session.process_command("x = 5");
    // 应该返回错误，但不应该 panic
    assert!(result.is_err());
}

#[test]
fn test_mathematical_expressions() {
    let mut session = InteractiveSession::new();
    
    // 由于使用 DummyParser，这些会返回错误，但不应该 panic
    let result = session.process_command("2 + 3");
    assert!(result.is_err());
    
    let result = session.process_command("x^2 + 2*x + 1");
    assert!(result.is_err());
}

#[test]
fn test_case_insensitive_commands() {
    let mut session = InteractiveSession::new();
    
    // 测试命令的大小写不敏感
    let result1 = session.process_command("HELP").unwrap();
    let result2 = session.process_command("help").unwrap();
    assert_eq!(result1, result2);
    
    let result1 = session.process_command("QUIT").unwrap();
    let result2 = session.process_command("quit").unwrap();
    assert_eq!(result1, result2);
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_session_workflow() {
        let mut session = InteractiveSession::new();
        
        // 1. 查看帮助
        let help_result = session.process_command("help").unwrap();
        assert!(help_result.contains("帮助"));
        
        // 2. 设置格式
        let format_result = session.process_command("format latex").unwrap();
        assert!(format_result.contains("LaTeX"));
        
        // 3. 设置精度
        let precision_result = session.process_command("precision 15").unwrap();
        assert!(precision_result.contains("15"));
        
        // 4. 开启详细模式
        let verbose_result = session.process_command("verbose").unwrap();
        assert!(verbose_result.contains("详细模式"));
        
        // 5. 查看变量（应该为空）
        let vars_result = session.process_command("vars").unwrap();
        assert_eq!(vars_result, "没有定义变量");
        
        // 6. 清空变量
        let clear_result = session.process_command("clear").unwrap();
        assert_eq!(clear_result, "变量已清空");
    }
}