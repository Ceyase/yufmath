//! # 平方根精度测试
//!
//! 测试平方根函数的精度处理，确保无理数不会被错误地转换为浮点数近似值。

use yufmath::Yufmath;

#[test]
fn test_sqrt_perfect_squares() {
    let yuf = Yufmath::new();
    
    // 测试完全平方数
    assert_eq!(yuf.compute("sqrt(0)").unwrap(), "0");
    assert_eq!(yuf.compute("sqrt(1)").unwrap(), "1");
    assert_eq!(yuf.compute("sqrt(4)").unwrap(), "2");
    assert_eq!(yuf.compute("sqrt(9)").unwrap(), "3");
    assert_eq!(yuf.compute("sqrt(16)").unwrap(), "4");
    assert_eq!(yuf.compute("sqrt(25)").unwrap(), "5");
    assert_eq!(yuf.compute("sqrt(100)").unwrap(), "10");
}

#[test]
fn test_sqrt_rational_perfect_squares() {
    let yuf = Yufmath::new();
    
    // 测试有理数的完全平方
    assert_eq!(yuf.compute("sqrt(4/9)").unwrap(), "2/3");
    assert_eq!(yuf.compute("sqrt(16/25)").unwrap(), "4/5");
    assert_eq!(yuf.compute("sqrt(1/4)").unwrap(), "1/2");
    assert_eq!(yuf.compute("sqrt(9/16)").unwrap(), "3/4");
}

#[test]
fn test_sqrt_irrational_numbers() {
    let yuf = Yufmath::new();
    
    // 测试无理数保持符号形式
    assert_eq!(yuf.compute("sqrt(2)").unwrap(), "sqrt(2)");
    assert_eq!(yuf.compute("sqrt(3)").unwrap(), "sqrt(3)");
    assert_eq!(yuf.compute("sqrt(5)").unwrap(), "sqrt(5)");
    assert_eq!(yuf.compute("sqrt(7)").unwrap(), "sqrt(7)");
    assert_eq!(yuf.compute("sqrt(10)").unwrap(), "sqrt(10)");
}

#[test]
fn test_sqrt_large_expressions() {
    let yuf = Yufmath::new();
    
    // 测试大数值表达式保持精确性
    assert_eq!(yuf.compute("sqrt(2)*1000000000000").unwrap(), "sqrt(2) * 1000000000000");
    assert_eq!(yuf.compute("sqrt(3)*999999999999").unwrap(), "sqrt(3) * 999999999999");
    
    // 测试完全平方数的大数值表达式
    assert_eq!(yuf.compute("sqrt(4)*1000000000000").unwrap(), "2000000000000");
}

#[test]
fn test_sqrt_rational_irrational() {
    let yuf = Yufmath::new();
    
    // 测试有理数的无理数平方根
    assert_eq!(yuf.compute("sqrt(2/3)").unwrap(), "sqrt(2/3)");
    assert_eq!(yuf.compute("sqrt(3/5)").unwrap(), "sqrt(3/5)");
    assert_eq!(yuf.compute("sqrt(7/11)").unwrap(), "sqrt(7/11)");
}

#[test]
fn test_sqrt_nested_expressions() {
    let yuf = Yufmath::new();
    
    // 测试嵌套表达式
    assert_eq!(yuf.compute("sqrt(2) + sqrt(3)").unwrap(), "sqrt(2) + sqrt(3)");
    assert_eq!(yuf.compute("sqrt(2) * sqrt(3)").unwrap(), "sqrt(2) * sqrt(3)");
    assert_eq!(yuf.compute("2 * sqrt(3)").unwrap(), "2sqrt(3)");
}

#[test]
fn test_sqrt_zero_and_one() {
    let yuf = Yufmath::new();
    
    // 测试特殊值
    assert_eq!(yuf.compute("sqrt(0)").unwrap(), "0");
    assert_eq!(yuf.compute("sqrt(1)").unwrap(), "1");
    assert_eq!(yuf.compute("sqrt(0) + 5").unwrap(), "5");
    assert_eq!(yuf.compute("sqrt(1) * 7").unwrap(), "7");
}

#[test]
fn test_sqrt_precision_preservation() {
    let yuf = Yufmath::new();
    
    // 确保无理数不会被转换为浮点数
    let result = yuf.compute("sqrt(2)").unwrap();
    assert!(!result.contains('.'), "sqrt(2) 不应该包含小数点，实际结果: {}", result);
    
    let result = yuf.compute("sqrt(3)").unwrap();
    assert!(!result.contains('.'), "sqrt(3) 不应该包含小数点，实际结果: {}", result);
    
    let result = yuf.compute("sqrt(5)").unwrap();
    assert!(!result.contains('.'), "sqrt(5) 不应该包含小数点，实际结果: {}", result);
}