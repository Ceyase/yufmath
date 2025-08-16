//! 测试高精度小数问题

use yufmath::core::Number;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    println!("测试高精度小数计算问题");
    
    // 创建极小的数值：0.00000000000000000000000000000000000000000000000000000000000000000000001
    let tiny_decimal = BigDecimal::new(BigInt::from(1), 71);
    let tiny_number = Number::real(tiny_decimal);
    
    // 创建 0.1
    let point_one = Number::real(BigDecimal::from_str("0.1").unwrap());
    
    println!("极小数值: {}", tiny_number);
    println!("0.1: {}", point_one);
    
    // 执行加法运算
    let result = tiny_number + point_one;
    
    println!("计算结果: {}", result);
    
    // 预期结果
    let expected_str = "0.10000000000000000000000000000000000000000000000000000000000000000000001";
    let expected = Number::real(BigDecimal::from_str(expected_str).unwrap());
    
    println!("预期结果: {}", expected);
    
    // 检查是否相等
    if result == expected {
        println!("✓ 计算结果正确！");
    } else {
        println!("✗ 计算结果不正确！");
        
        // 分析差异
        match (&result, &expected) {
            (Number::Real(r1), Number::Real(r2)) => {
                let diff = r1 - r2;
                println!("数值差异: {}", diff);
            }
            _ => println!("类型不匹配"),
        }
    }
}