//! # 增强三角函数化简演示
//!
//! 演示增强的三角函数化简功能，包括诱导公式、辅助角公式等。

use yufmath::{Yufmath, Expression, Number, MathConstant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 增强三角函数化简演示 ===\n");
    
    let mut yuf = Yufmath::new();
    // 启用增强化简功能
    yuf.set_enhanced_simplify(true);
    
    // 测试诱导公式
    println!("1. 诱导公式测试:");
    test_induction_formulas(&mut yuf)?;
    
    // 测试特殊角度值
    println!("\n2. 特殊角度值测试:");
    test_special_angles(&mut yuf)?;
    
    // 测试三角恒等式
    println!("\n3. 三角恒等式测试:");
    test_trigonometric_identities(&mut yuf)?;
    
    // 测试反三角函数
    println!("\n4. 反三角函数测试:");
    test_inverse_trigonometric(&mut yuf)?;
    
    // 测试周期性
    println!("\n5. 周期性测试:");
    test_periodicity(&mut yuf)?;
    
    println!("\n=== 演示完成 ===");
    Ok(())
}

fn test_induction_formulas(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("sin(-x)", "应该简化为 -sin(x)"),
        ("cos(-x)", "应该简化为 cos(x)"),
        ("tan(-x)", "应该简化为 -tan(x)"),
        ("sin(pi - x)", "应该简化为 sin(x)"),
        ("cos(pi - x)", "应该简化为 -cos(x)"),
        ("sin(pi + x)", "应该简化为 -sin(x)"),
        ("cos(pi + x)", "应该简化为 -cos(x)"),
        ("tan(pi + x)", "应该简化为 tan(x)"),
        ("sin(pi/2 - x)", "应该简化为 cos(x)"),
        ("cos(pi/2 - x)", "应该简化为 sin(x)"),
        ("sin(pi/2 + x)", "应该简化为 cos(x)"),
        ("cos(pi/2 + x)", "应该简化为 -sin(x)"),
    ];
    
    for (input, description) in test_cases {
        println!("  {} -> {}", input, description);
        match yuf.compute(input) {
            Ok(result) => println!("    结果: {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_special_angles(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        // 正弦特殊角度
        ("sin(0)", "0"),
        ("sin(pi/6)", "1/2"),
        ("sin(pi/4)", "√2/2"),
        ("sin(pi/3)", "√3/2"),
        ("sin(pi/2)", "1"),
        ("sin(pi)", "0"),
        
        // 余弦特殊角度
        ("cos(0)", "1"),
        ("cos(pi/6)", "√3/2"),
        ("cos(pi/4)", "√2/2"),
        ("cos(pi/3)", "1/2"),
        ("cos(pi/2)", "0"),
        ("cos(pi)", "-1"),
        
        // 正切特殊角度
        ("tan(0)", "0"),
        ("tan(pi/6)", "1/√3"),
        ("tan(pi/4)", "1"),
        ("tan(pi/3)", "√3"),
        ("tan(pi)", "0"),
    ];
    
    for (input, expected) in test_cases {
        println!("  {} (期望: {})", input, expected);
        match yuf.compute(input) {
            Ok(result) => println!("    结果: {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_trigonometric_identities(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("sin(x)^2 + cos(x)^2", "应该简化为 1"),
        ("sin(x)/cos(x)", "应该简化为 tan(x)"),
        ("cos(x)/sin(x)", "应该简化为 1/tan(x)"),
        ("1 - sin(x)^2", "应该简化为 cos(x)^2"),
        ("1 - cos(x)^2", "应该简化为 sin(x)^2"),
    ];
    
    for (input, description) in test_cases {
        println!("  {} -> {}", input, description);
        match yuf.compute(input) {
            Ok(result) => println!("    结果: {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_inverse_trigonometric(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("asin(0)", "0"),
        ("asin(1)", "π/2"),
        ("asin(-1)", "-π/2"),
        ("acos(1)", "0"),
        ("acos(0)", "π/2"),
        ("acos(-1)", "π"),
        ("atan(0)", "0"),
        ("atan(1)", "π/4"),
        ("atan(-1)", "-π/4"),
        ("asin(sin(x))", "x (在定义域内)"),
        ("acos(cos(x))", "x (在定义域内)"),
        ("atan(tan(x))", "x (在定义域内)"),
    ];
    
    for (input, expected) in test_cases {
        println!("  {} (期望: {})", input, expected);
        match yuf.compute(input) {
            Ok(result) => println!("    结果: {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

fn test_periodicity(yuf: &mut Yufmath) -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("sin(x + 2*pi)", "应该简化为 sin(x)"),
        ("cos(x + 2*pi)", "应该简化为 cos(x)"),
        ("tan(x + pi)", "应该简化为 tan(x)"),
        ("sin(2*pi + x)", "应该简化为 sin(x)"),
        ("cos(2*pi + x)", "应该简化为 cos(x)"),
    ];
    
    for (input, description) in test_cases {
        println!("  {} -> {}", input, description);
        match yuf.compute(input) {
            Ok(result) => println!("    结果: {}", result),
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}