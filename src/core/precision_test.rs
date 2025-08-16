//! 高精度小数测试
//! 
//! 测试极小数值的精确计算

#[cfg(test)]
mod tests {
    use super::super::Number;
    use bigdecimal::BigDecimal;
    use num_bigint::BigInt;
    use std::str::FromStr;

    #[test]
    fn test_tiny_decimal_addition() {
        println!("测试高精度小数计算问题");
        
        // 创建极小的数值：0.00000000000000000000000000000000000000000000000000000000000000000000001
        let tiny_decimal_str = "0.00000000000000000000000000000000000000000000000000000000000000000000001";
        let tiny_number = Number::real(BigDecimal::from_str(tiny_decimal_str).unwrap());
        
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
            println!("差异分析:");
            
            // 尝试分析差异
            match (&result, &expected) {
                (Number::Real(r1), Number::Real(r2)) => {
                    let diff = r1 - r2;
                    println!("数值差异: {}", diff);
                }
                _ => println!("类型不匹配"),
            }
        }
        
        // 断言应该相等
        assert_eq!(result, expected, "高精度小数加法计算不正确");
    }

    #[test]
    fn test_bigdecimal_precision() {
        println!("\n测试 BigDecimal 精度:");
        
        // 手动创建高精度的 BigDecimal
        let tiny_precise = BigDecimal::new(BigInt::from(1), 71); // 1 * 10^-71
        let point_one_precise = BigDecimal::new(BigInt::from(1), 1); // 1 * 10^-1 = 0.1
        
        println!("高精度极小数值: {}", tiny_precise);
        println!("高精度 0.1: {}", point_one_precise);
        
        let result_precise = &tiny_precise + &point_one_precise;
        println!("高精度计算结果: {}", result_precise);
        
        // 转换为 Number 类型测试
        let tiny_number = Number::real(tiny_precise);
        let point_one_number = Number::real(point_one_precise);
        
        let result_number = tiny_number + point_one_number;
        println!("Number 类型计算结果: {}", result_number);
        
        // 预期结果
        let expected_precise = BigDecimal::new(
            BigInt::from_str("10000000000000000000000000000000000000000000000000000000000000000000001").unwrap(),
            71
        );
        let expected_number = Number::real(expected_precise);
        
        println!("预期结果: {}", expected_number);
        
        assert_eq!(result_number, expected_number, "高精度 BigDecimal 计算不正确");
    }

    #[test]
    fn test_float_precision_issue() {
        println!("\n浮点数版本（展示精度问题）:");
        
        // 这个测试展示了浮点数的精度问题
        let tiny_float = 1e-70_f64;
        let result_float = tiny_float + 0.1;
        println!("浮点数结果: {:.70}", result_float);
        
        // 浮点数会丢失精度，结果应该就是 0.1
        assert_eq!(result_float, 0.1);
        
        // 但是用 BigDecimal 应该保持精度
        let tiny_decimal = BigDecimal::new(BigInt::from(1), 70);
        let point_one_decimal = BigDecimal::from_str("0.1").unwrap();
        let result_decimal = &tiny_decimal + &point_one_decimal;
        
        // BigDecimal 应该保持完整精度 - 70位小数精度
        let expected_decimal = BigDecimal::from_str("0.1000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
        
        println!("BigDecimal 极小数: {}", tiny_decimal);
        println!("BigDecimal 结果: {}", result_decimal);
        println!("BigDecimal 预期: {}", expected_decimal);
        
        assert_eq!(result_decimal, expected_decimal, "BigDecimal 应该保持完整精度");
    }

    #[test]
    fn test_number_type_precision() {
        // 测试 Number 类型是否正确处理高精度
        let tiny = Number::real(BigDecimal::new(BigInt::from(1), 50));
        let normal = Number::real(BigDecimal::from_str("0.5").unwrap());
        
        let result = tiny.clone() + normal.clone();
        
        // 预期结果应该是 0.5 + 1E-50
        let expected_decimal = BigDecimal::from_str("0.5").unwrap() + BigDecimal::new(BigInt::from(1), 50);
        let expected = Number::real(expected_decimal);
        
        println!("极小数: {}", tiny);
        println!("普通数: {}", normal);
        println!("计算结果: {}", result);
        println!("预期结果: {}", expected);
        
        assert_eq!(result, expected, "Number 类型应该保持高精度");
    }

    #[test]
    fn test_user_reported_precision_issue() {
        println!("\n测试用户报告的精度问题:");
        println!("0.00000000000000000000000000000000000000000000000000000000000000000000001+0.1=0.10000000000000000555111512312578270211815834045410156250000000000000001");
        println!("预期为: 0.10000000000000000000000000000000000000000000000000000000000000000000001");
        
        // 创建极小的数值：0.00000000000000000000000000000000000000000000000000000000000000000000001
        let tiny_decimal = BigDecimal::new(BigInt::from(1), 71);
        let tiny_number = Number::real(tiny_decimal);
        
        // 创建 0.1
        let point_one = Number::real(BigDecimal::from_str("0.1").unwrap());
        
        println!("极小数值: {}", tiny_number);
        println!("0.1: {}", point_one);
        
        // 执行加法运算
        let result = tiny_number + point_one;
        
        println!("实际计算结果: {}", result);
        
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
                    
                    // 检查 BigDecimal 层面的计算
                    println!("\n直接使用 BigDecimal 测试:");
                    let tiny_bd = BigDecimal::new(BigInt::from(1), 71);
                    let point_one_bd = BigDecimal::from_str("0.1").unwrap();
                    let result_bd = &tiny_bd + &point_one_bd;
                    let expected_bd = BigDecimal::from_str(expected_str).unwrap();
                    
                    println!("BigDecimal 极小数: {}", tiny_bd);
                    println!("BigDecimal 0.1: {}", point_one_bd);
                    println!("BigDecimal 结果: {}", result_bd);
                    println!("BigDecimal 预期: {}", expected_bd);
                    println!("BigDecimal 相等: {}", result_bd == expected_bd);
                }
                _ => println!("类型不匹配"),
            }
        }
        
        // 这个测试应该通过，如果失败说明高精度系统有问题
        assert_eq!(result, expected, "高精度小数加法计算应该正确");
    }

    #[test]
    fn test_rational_to_real_precision() {
        println!("\n测试有理数到实数转换的精度保持:");
        
        // 创建一个有理数：1/3
        let rational = Number::rational(1, 3);
        
        // 创建一个极小的实数
        let tiny_real = Number::real(BigDecimal::new(BigInt::from(1), 50));
        
        println!("有理数: {}", rational);
        println!("极小实数: {}", tiny_real);
        
        // 执行加法，这会触发类型提升
        let result = rational + tiny_real;
        
        println!("计算结果: {}", result);
        
        // 预期结果应该是精确的 1/3 + 极小数
        // 1/3 = 0.333333333333333333333333333333333333333333333333333...
        // 加上 1E-50 应该得到精确结果
        let one_third = BigDecimal::new(BigInt::from(1), 0) / BigDecimal::new(BigInt::from(3), 0);
        let tiny_part = BigDecimal::new(BigInt::from(1), 50);
        let expected_decimal = one_third + tiny_part;
        let expected = Number::real(expected_decimal);
        
        println!("预期结果: {}", expected);
        
        // 检查结果是否正确
        assert_eq!(result, expected, "有理数到实数的转换应该保持精度");
    }

    #[test]
    fn test_mixed_precision_addition() {
        println!("\n测试混合精度加法:");
        
        // 测试用户报告的具体情况：极小数 + 0.1
        // 但这次我们用不同的方式创建 0.1
        
        // 方式1：直接从字符串创建 BigDecimal
        let tiny1 = Number::real(BigDecimal::new(BigInt::from(1), 71));
        let point_one_str = Number::real(BigDecimal::from_str("0.1").unwrap());
        let result1 = tiny1 + point_one_str;
        
        // 方式2：从有理数 1/10 创建
        let tiny2 = Number::real(BigDecimal::new(BigInt::from(1), 71));
        let point_one_rational = Number::rational(1, 10);
        let result2 = tiny2 + point_one_rational;
        
        // 方式3：从浮点数创建（这个可能有精度问题）
        let tiny3 = Number::real(BigDecimal::new(BigInt::from(1), 71));
        let point_one_float = Number::float(0.1);
        let result3 = tiny3 + point_one_float;
        
        println!("方式1 (字符串): {}", result1);
        println!("方式2 (有理数): {}", result2);
        println!("方式3 (浮点数): {}", result3);
        
        // 预期结果
        let expected_str = "0.10000000000000000000000000000000000000000000000000000000000000000000001";
        let expected = Number::real(BigDecimal::from_str(expected_str).unwrap());
        
        println!("预期结果: {}", expected);
        
        // 方式1和方式2应该得到正确结果
        assert_eq!(result1, expected, "字符串方式应该保持精度");
        assert_eq!(result2, expected, "有理数方式应该保持精度");
        
        // 方式3可能有精度损失，我们只检查它不等于预期结果
        // （这个测试展示了浮点数的精度问题）
        if result3 != expected {
            println!("✓ 浮点数方式确实有精度损失，这是预期的");
        }
    }
}