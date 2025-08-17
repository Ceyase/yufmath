//! 测试增强化简功能的示例

use yufmath::Yufmath;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试增强化简功能...\n");
    
    let mut yuf = Yufmath::new();
    
    // 检查增强化简功能是否启用
    println!("增强化简功能状态: {}", yuf.is_enhanced_simplify_enabled());
    
    // 测试基本表达式
    println!("=== 测试基本表达式 ===");
    let test_cases = vec![
        "2 + 3",
        "x + 0",
        "1 * x",
        "x^1",
    ];
    
    for case in test_cases {
        println!("输入: {}", case);
        match yuf.compute(case) {
            Ok(result) => println!("输出: {}", result),
            Err(e) => println!("错误: {}", e),
        }
        println!();
    }
    
    // 测试分数化简
    println!("=== 测试分数化简 ===");
    let fraction_cases = vec![
        "1/2 + 1/3",
        "2/4",
        "6/9",
    ];
    
    for case in fraction_cases {
        println!("输入: {}", case);
        match yuf.compute(case) {
            Ok(result) => println!("输出: {}", result),
            Err(e) => println!("错误: {}", e),
        }
        println!();
    }
    
    // 测试关闭增强化简功能
    println!("=== 测试关闭增强化简功能 ===");
    yuf.set_enhanced_simplify(false);
    println!("增强化简功能状态: {}", yuf.is_enhanced_simplify_enabled());
    
    println!("输入: 1/2 + 1/3");
    match yuf.compute("1/2 + 1/3") {
        Ok(result) => println!("输出: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    // 重新启用增强化简功能
    println!("\n=== 重新启用增强化简功能 ===");
    yuf.set_enhanced_simplify(true);
    println!("增强化简功能状态: {}", yuf.is_enhanced_simplify_enabled());
    
    println!("输入: 1/2 + 1/3");
    match yuf.compute("1/2 + 1/3") {
        Ok(result) => println!("输出: {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    Ok(())
}