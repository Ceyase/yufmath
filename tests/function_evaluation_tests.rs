//! # 函数求值测试
//!
//! 测试基本数学函数的计算功能。

use yufmath::Yufmath;

#[test]
fn test_logarithm_functions() {
    let yuf = Yufmath::new();
    
    // ln(e) = 1
    let result = yuf.compute("ln(e)").unwrap();
    assert_eq!(result, "1");
    
    // ln(1) = 0
    let result = yuf.compute("ln(1)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_trigonometric_functions() {
    let yuf = Yufmath::new();
    
    // sin(0) = 0
    let result = yuf.compute("sin(0)").unwrap();
    assert_eq!(result, "0");
    
    // sin(pi/2) = 1
    let result = yuf.compute("sin(pi/2)").unwrap();
    assert_eq!(result, "1");
    
    // cos(0) = 1
    let result = yuf.compute("cos(0)").unwrap();
    assert_eq!(result, "1");
    
    // cos(pi/2) = 0
    let result = yuf.compute("cos(pi/2)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_exponential_functions() {
    let yuf = Yufmath::new();
    
    // exp(0) = 1
    let result = yuf.compute("exp(0)").unwrap();
    assert_eq!(result, "1");
    
    // exp(1) = e
    let result = yuf.compute("exp(1)").unwrap();
    assert_eq!(result, "e");
}

#[test]
fn test_square_root_functions() {
    let yuf = Yufmath::new();
    
    // sqrt(0) = 0
    let result = yuf.compute("sqrt(0)").unwrap();
    assert_eq!(result, "0");
    
    // sqrt(1) = 1
    let result = yuf.compute("sqrt(1)").unwrap();
    assert_eq!(result, "1");
    
    // sqrt(4) = 2
    let result = yuf.compute("sqrt(4)").unwrap();
    assert_eq!(result, "2");
    
    // sqrt(9) = 3
    let result = yuf.compute("sqrt(9)").unwrap();
    assert_eq!(result, "3");
    
    // sqrt(16) = 4
    let result = yuf.compute("sqrt(16)").unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_factorial_functions() {
    let yuf = Yufmath::new();
    
    // factorial(0) = 1
    let result = yuf.compute("factorial(0)").unwrap();
    assert_eq!(result, "1");
    
    // factorial(1) = 1
    let result = yuf.compute("factorial(1)").unwrap();
    assert_eq!(result, "1");
    
    // factorial(5) = 120
    let result = yuf.compute("factorial(5)").unwrap();
    assert_eq!(result, "120");
    
    // factorial(10) = 3628800
    let result = yuf.compute("factorial(10)").unwrap();
    assert_eq!(result, "3628800");
}

#[test]
fn test_absolute_value_functions() {
    let yuf = Yufmath::new();
    
    // abs(0) = 0
    let result = yuf.compute("abs(0)").unwrap();
    assert_eq!(result, "0");
    
    // abs(5) = 5
    let result = yuf.compute("abs(5)").unwrap();
    assert_eq!(result, "5");
    
    // abs(-5) = 5
    let result = yuf.compute("abs(-5)").unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_special_trigonometric_values() {
    let yuf = Yufmath::new();
    
    // sin(pi/4) = sqrt(2)/2
    let result = yuf.compute("sin(pi/4)").unwrap();
    assert!(result.contains("sqrt(2)") && result.contains("/ 2"));
    
    // sin(pi/6) = 1/2
    let result = yuf.compute("sin(pi/6)").unwrap();
    assert_eq!(result, "1/2");
    
    // cos(pi/4) = sqrt(2)/2
    let result = yuf.compute("cos(pi/4)").unwrap();
    assert!(result.contains("sqrt(2)") && result.contains("/ 2"));
    
    // cos(pi/3) = 1/2
    let result = yuf.compute("cos(pi/3)").unwrap();
    assert_eq!(result, "1/2");
}

#[test]
fn test_function_composition() {
    let yuf = Yufmath::new();
    
    // exp(ln(x)) = x (这里 x 是符号，应该保持不变)
    let result = yuf.compute("exp(ln(x))").unwrap();
    assert_eq!(result, "x");
    
    // ln(exp(x)) = x (这里 x 是符号，应该保持不变)
    let result = yuf.compute("ln(exp(x))").unwrap();
    assert_eq!(result, "x");
}

#[test]
fn test_numerical_functions() {
    let yuf = Yufmath::new();
    
    // 测试数值计算
    let result = yuf.compute("sin(1.5708)").unwrap(); // 约等于 pi/2
    // 应该接近 1
    if let Ok(val) = result.parse::<f64>() {
        assert!((val - 1.0).abs() < 0.01);
    }
    
    let result = yuf.compute("ln(2.71828)").unwrap(); // 约等于 e
    // 应该接近 1
    if let Ok(val) = result.parse::<f64>() {
        assert!((val - 1.0).abs() < 0.01);
    }
}