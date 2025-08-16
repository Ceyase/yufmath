//! 简单测试高精度问题

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    println!("测试 BigDecimal 高精度计算");
    
    // 创建极小的数值：0.00000000000000000000000000000000000000000000000000000000000000000000001
    let tiny = BigDecimal::new(BigInt::from(1), 71);
    let point_one = BigDecimal::from_str("0.1").unwrap();
    
    println!("极小数值: {}", tiny);
    println!("0.1: {}", point_one);
    
    // 执行加法运算
    let result = &tiny + &point_one;
    
    println!("计算结果: {}", result);
    
    // 预期结果
    let expected = BigDecimal::from_str("0.10000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
    
    println!("预期结果: {}", expected);
    
    // 检查是否相等
    if result == expected {
        println!("✓ BigDecimal 计算结果正确！");
    } else {
        println!("✗ BigDecimal 计算结果不正确！");
        let diff = &result - &expected;
        println!("数值差异: {}", diff);
    }
}