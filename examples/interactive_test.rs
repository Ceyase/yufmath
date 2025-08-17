//! # 交互模式测试
//!
//! 测试交互模式下的代数值显示功能

use yufmath::Yufmath;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 交互模式代数值显示测试 ===\n");
    
    let mut yuf = Yufmath::new();
    // 启用增强化简功能
    yuf.set_enhanced_simplify(true);
    
    // 测试数学常量
    println!("1. 数学常量测试:");
    test_constants(&mut yuf)?;
    
    // 测试三角函数
    println!("\n2. 三角函数测试:");
    test_trigonometric(&mut yuf)?;
    
    // 测试根号表达式
    println!("\n3. 根号表达式测试:");
    test_radicals(&mut yuf)?;
    
    // 测试复杂表达式
    println!("\n4. 复杂表达式测试:");
    test_complex_expressions(&mut yuf)?;
    
    println!("\n=== 测试完成 ===");
    Ok(())
}

fn test_constants(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        "pi",
        "e", 
        "pi/2",
        "pi/4",
        "2*pi",
        "e^2",
    ];
    
    for input in test_cases {
        println!("  {} ->", input);
        match yuf.compute(input) {
            Ok(result) => println!("    {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_trigonometric(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        "sin(pi/6)",
        "cos(pi/4)", 
        "tan(pi/3)",
        "sin(pi/2)",
        "cos(pi)",
        "sin(0)",
        "cos(0)",
    ];
    
    for input in test_cases {
        println!("  {} ->", input);
        match yuf.compute(input) {
            Ok(result) => println!("    {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_radicals(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        "sqrt(2)",
        "sqrt(3)",
        "sqrt(4)",
        "sqrt(8)",
        "sqrt(12)",
        "sqrt(18)",
    ];
    
    for input in test_cases {
        println!("  {} ->", input);
        match yuf.compute(input) {
            Ok(result) => println!("    {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_complex_expressions(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        "pi + e",
        "sqrt(2) + sqrt(3)",
        "sin(pi/4) + cos(pi/4)",
        "pi * sqrt(2)",
        "e^(pi/2)",
    ];
    
    for input in test_cases {
        println!("  {} ->", input);
        match yuf.compute(input) {
            Ok(result) => println!("    {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}