//! # 算术运算单元测试
//!
//! 测试 Number 类型的算术运算功能。

#[cfg(test)]
mod tests {
    use super::super::Number;
    use num_bigint::BigInt;
    use num_rational::BigRational;
    use bigdecimal::BigDecimal;

    #[test]
    fn test_integer_addition() {
        let a = Number::integer(5);
        let b = Number::integer(3);
        let result = a + b;
        
        assert_eq!(result, Number::integer(8));
        assert_eq!(result.to_string(), "8");
    }

    #[test]
    fn test_integer_subtraction() {
        let a = Number::integer(10);
        let b = Number::integer(3);
        let result = a - b;
        
        assert_eq!(result, Number::integer(7));
        assert_eq!(result.to_string(), "7");
    }

    #[test]
    fn test_integer_multiplication() {
        let a = Number::integer(4);
        let b = Number::integer(6);
        let result = a * b;
        
        assert_eq!(result, Number::integer(24));
        assert_eq!(result.to_string(), "24");
    }

    #[test]
    fn test_integer_division() {
        // 整除情况
        let a = Number::integer(12);
        let b = Number::integer(3);
        let result = a / b;
        
        assert_eq!(result, Number::integer(4));
        assert_eq!(result.to_string(), "4");
        
        // 不能整除的情况，应该返回有理数
        let a = Number::integer(5);
        let b = Number::integer(2);
        let result = a / b;
        
        assert!(matches!(result, Number::Rational(_)));
        assert_eq!(result.to_string(), "5/2");
    }

    #[test]
    fn test_rational_arithmetic() {
        let a = Number::rational(1, 2); // 1/2
        let b = Number::rational(1, 3); // 1/3
        
        // 加法: 1/2 + 1/3 = 5/6
        let sum = a.clone() + b.clone();
        assert!(matches!(sum, Number::Rational(_)));
        assert_eq!(sum.to_string(), "5/6");
        
        // 减法: 1/2 - 1/3 = 1/6
        let diff = a.clone() - b.clone();
        assert!(matches!(diff, Number::Rational(_)));
        assert_eq!(diff.to_string(), "1/6");
        
        // 乘法: 1/2 * 1/3 = 1/6
        let product = a.clone() * b.clone();
        assert!(matches!(product, Number::Rational(_)));
        assert_eq!(product.to_string(), "1/6");
        
        // 除法: 1/2 / 1/3 = 3/2
        let quotient = a / b;
        assert!(matches!(quotient, Number::Rational(_)));
        assert_eq!(quotient.to_string(), "3/2");
    }

    #[test]
    fn test_mixed_type_arithmetic() {
        let int_num = Number::integer(2);
        let rational_num = Number::rational(1, 2);
        
        // 整数 + 有理数 = 有理数
        let result = int_num.clone() + rational_num.clone();
        assert!(matches!(result, Number::Rational(_)));
        assert_eq!(result.to_string(), "5/2");
        
        // 整数 * 有理数 = 有理数
        let result = int_num * rational_num;
        assert!(matches!(result, Number::Rational(_)));
        assert_eq!(result.to_string(), "1");
    }

    #[test]
    fn test_complex_arithmetic() {
        let a = Number::complex(Number::integer(1), Number::integer(2)); // 1 + 2i
        let b = Number::complex(Number::integer(3), Number::integer(4)); // 3 + 4i
        
        // 复数加法: (1+2i) + (3+4i) = 4+6i
        let sum = a.clone() + b.clone();
        assert!(matches!(sum, Number::Complex { .. }));
        assert_eq!(sum.to_string(), "4+6i");
        
        // 复数减法: (1+2i) - (3+4i) = -2-2i
        let diff = a.clone() - b.clone();
        assert!(matches!(diff, Number::Complex { .. }));
        assert_eq!(diff.to_string(), "-2-2i");
        
        // 复数乘法: (1+2i) * (3+4i) = (3-8) + (4+6)i = -5+10i
        let product = a.clone() * b.clone();
        assert!(matches!(product, Number::Complex { .. }));
        assert_eq!(product.to_string(), "-5+10i");
    }

    #[test]
    fn test_complex_division() {
        let a = Number::complex(Number::integer(1), Number::integer(1)); // 1 + i
        let b = Number::complex(Number::integer(1), Number::integer(-1)); // 1 - i
        
        // (1+i) / (1-i) = [(1+i)(1+i)] / [(1-i)(1+i)] = (1+2i-1) / (1+1) = 2i/2 = i
        let result = a / b;
        assert!(matches!(result, Number::Complex { .. }));
        assert_eq!(result.to_string(), "i");
    }

    #[test]
    fn test_zero_operations() {
        let zero = Number::zero();
        let five = Number::integer(5);
        
        // 零加任何数
        assert_eq!(zero.clone() + five.clone(), five);
        assert_eq!(five.clone() + zero.clone(), five);
        
        // 零减任何数
        assert_eq!(zero.clone() - five.clone(), Number::integer(-5));
        assert_eq!(five.clone() - zero.clone(), five);
        
        // 零乘任何数
        assert_eq!(zero.clone() * five.clone(), zero);
        assert_eq!(five.clone() * zero.clone(), zero);
        
        // 零除以任何非零数
        assert_eq!(zero.clone() / five.clone(), zero);
    }

    #[test]
    fn test_one_operations() {
        let one = Number::one();
        let five = Number::integer(5);
        
        // 一乘任何数
        assert_eq!(one.clone() * five.clone(), five);
        assert_eq!(five.clone() * one.clone(), five);
        
        // 任何数除以一
        assert_eq!(five.clone() / one.clone(), five);
    }

    #[test]
    fn test_division_by_zero() {
        let five = Number::integer(5);
        let zero = Number::zero();
        
        // 除零应该返回符号表示
        let result = five / zero;
        assert!(matches!(result, Number::Symbolic(_)));
    }

    #[test]
    fn test_negation() {
        let positive = Number::integer(5);
        let negative = -positive.clone();
        
        assert_eq!(negative, Number::integer(-5));
        assert_eq!(negative.to_string(), "-5");
        
        // 双重否定
        let double_neg = -negative;
        assert_eq!(double_neg, positive);
        
        // 复数否定
        let complex_num = Number::complex(Number::integer(3), Number::integer(4));
        let neg_complex = -complex_num;
        assert_eq!(neg_complex.to_string(), "-3-4i");
    }

    #[test]
    fn test_reference_operations() {
        let a = Number::integer(3);
        let b = Number::integer(4);
        
        // 测试引用类型的运算
        let sum = &a + &b;
        assert_eq!(sum, Number::integer(7));
        
        let diff = &a - &b;
        assert_eq!(diff, Number::integer(-1));
        
        let product = &a * &b;
        assert_eq!(product, Number::integer(12));
        
        let quotient = &a / &b;
        assert_eq!(quotient.to_string(), "3/4");
        
        // 确保原始值没有被移动
        assert_eq!(a, Number::integer(3));
        assert_eq!(b, Number::integer(4));
    }

    #[test]
    fn test_precision_preservation() {
        // 测试精确计算的保持
        let a = Number::rational(1, 3);
        let b = Number::rational(2, 3);
        
        let sum = a + b;
        // 1/3 + 2/3 = 1，但结果是有理数形式 1/1
        assert!(matches!(sum, Number::Rational(_)));
        assert_eq!(sum.to_string(), "1"); // Display trait 会简化 1/1 为 1
        
        // 测试大数运算
        let big_a = Number::integer(BigInt::from(2).pow(100));
        let big_b = Number::integer(BigInt::from(2).pow(100));
        
        let big_sum = big_a + big_b;
        assert_eq!(big_sum, Number::integer(BigInt::from(2).pow(101)));
    }

    #[test]
    fn test_float_arithmetic() {
        let a = Number::float(2.5);
        let b = Number::float(1.5);
        
        let sum = a.clone() + b.clone();
        assert_eq!(sum, Number::float(4.0));
        
        let product = a * b;
        assert_eq!(product, Number::float(3.75));
    }

    #[test]
    fn test_real_arithmetic() {
        let a = Number::real(BigDecimal::from(25) / BigDecimal::from(10)); // 2.5
        let b = Number::real(BigDecimal::from(15) / BigDecimal::from(10)); // 1.5
        
        let sum = a.clone() + b.clone();
        assert!(matches!(sum, Number::Real(_)));
        
        let product = a * b;
        assert!(matches!(product, Number::Real(_)));
    }

    #[test]
    fn test_arithmetic_properties() {
        let a = Number::integer(3);
        let b = Number::integer(5);
        let c = Number::integer(7);
        
        // 交换律测试
        assert_eq!(a.clone() + b.clone(), b.clone() + a.clone());
        assert_eq!(a.clone() * b.clone(), b.clone() * a.clone());
        
        // 结合律测试
        assert_eq!((a.clone() + b.clone()) + c.clone(), a.clone() + (b.clone() + c.clone()));
        assert_eq!((a.clone() * b.clone()) * c.clone(), a.clone() * (b.clone() * c.clone()));
        
        // 分配律测试
        assert_eq!(a.clone() * (b.clone() + c.clone()), (a.clone() * b) + (a * c));
    }

    #[test]
    fn test_complex_real_mixed() {
        let real_num = Number::integer(5);
        let complex_num = Number::complex(Number::integer(2), Number::integer(3));
        
        // 实数 + 复数
        let result = real_num.clone() + complex_num.clone();
        assert_eq!(result.to_string(), "7+3i");
        
        // 复数 + 实数
        let result = complex_num + real_num;
        assert_eq!(result.to_string(), "7+3i");
    }

    #[test]
    fn test_edge_cases() {
        // 测试非常大的数
        let big_num = Number::integer(BigInt::from(10).pow(1000));
        let result = big_num.clone() + Number::one();
        assert!(result.is_positive());
        assert!(result.is_integer());
        
        // 测试非常小的有理数
        let small_rational = Number::rational(1, BigInt::from(10).pow(100));
        let result = small_rational.clone() + small_rational;
        assert!(matches!(result, Number::Rational(_)));
        assert!(result.is_positive());
    }
}