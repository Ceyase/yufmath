//! # 数值类型单元测试
//!
//! 测试 Number 类型的各种功能和方法。

#[cfg(test)]
mod tests {
    use super::super::Number;
    use num_bigint::BigInt;
    use num_rational::BigRational;
    use bigdecimal::BigDecimal;

    #[test]
    fn test_number_creation() {
        // 测试整数创建
        let int_num = Number::integer(42);
        assert!(matches!(int_num, Number::Integer(_)));
        assert_eq!(int_num.to_string(), "42");

        // 测试有理数创建
        let rational_num = Number::rational(3, 4);
        assert!(matches!(rational_num, Number::Rational(_)));
        assert_eq!(rational_num.to_string(), "3/4");

        // 测试实数创建
        let real_num = Number::real(BigDecimal::from(314) / BigDecimal::from(100));
        assert!(matches!(real_num, Number::Real(_)));

        // 测试复数创建
        let complex_num = Number::complex(Number::integer(1), Number::integer(2));
        assert!(matches!(complex_num, Number::Complex { .. }));
        assert_eq!(complex_num.to_string(), "1+2i");

        // 测试浮点数创建
        let float_num = Number::float(2.5);
        assert!(matches!(float_num, Number::Float(_)));
        assert_eq!(float_num.to_string(), "2.5");
    }

    #[test]
    fn test_special_numbers() {
        // 测试零
        let zero = Number::zero();
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());

        // 测试一
        let one = Number::one();
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(one.is_positive());
        assert!(!one.is_negative());

        // 测试负一
        let neg_one = Number::neg_one();
        assert!(!neg_one.is_zero());
        assert!(!neg_one.is_one());
        assert!(!neg_one.is_positive());
        assert!(neg_one.is_negative());

        // 测试虚数单位
        let i = Number::i();
        assert!(i.is_complex());
        assert!(!i.is_real());
        assert_eq!(i.to_string(), "i");
    }

    #[test]
    fn test_type_checking() {
        let int_num = Number::integer(5);
        let rational_num = Number::rational(3, 4);
        let real_num = Number::real(BigDecimal::from(314) / BigDecimal::from(100));
        let complex_num = Number::complex(Number::integer(1), Number::integer(2));
        let float_num = Number::float(2.5);

        // 测试整数检查
        assert!(int_num.is_integer());
        assert!(int_num.is_rational());
        assert!(int_num.is_real());
        assert!(!int_num.is_complex());

        // 测试有理数检查
        assert!(!rational_num.is_integer());
        assert!(rational_num.is_rational());
        assert!(rational_num.is_real());
        assert!(!rational_num.is_complex());

        // 测试实数检查
        assert!(!real_num.is_integer());
        assert!(!real_num.is_rational());
        assert!(real_num.is_real());
        assert!(!real_num.is_complex());

        // 测试复数检查
        assert!(!complex_num.is_integer());
        assert!(!complex_num.is_rational());
        assert!(!complex_num.is_real());
        assert!(complex_num.is_complex());

        // 测试浮点数检查
        assert!(float_num.is_real());
        assert!(!float_num.is_exact());
    }

    #[test]
    fn test_precision_checking() {
        let exact_int = Number::integer(42);
        let exact_rational = Number::rational(3, 4);
        let exact_real = Number::real(BigDecimal::from(3));
        let inexact_float = Number::float(3.14);

        assert!(exact_int.is_exact());
        assert!(exact_rational.is_exact());
        assert!(exact_real.is_exact());
        assert!(!inexact_float.is_exact());
    }

    #[test]
    fn test_to_exact_conversion() {
        // 测试浮点数到精确表示的转换
        let float_num = Number::float(0.5);
        let exact_num = float_num.to_exact();
        
        // 0.5 应该能够精确转换为有理数 1/2
        assert!(exact_num.is_exact());
        if let Number::Rational(r) = exact_num {
            assert_eq!(r.numer(), &BigInt::from(1));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
    }

    #[test]
    fn test_number_operations() {
        let num1 = Number::integer(5);
        let num2 = Number::integer(-3);

        // 测试绝对值
        assert_eq!(num1.abs().unwrap().to_string(), "5");
        assert_eq!(num2.abs().unwrap().to_string(), "3");

        // 测试取负
        assert_eq!(num1.neg().to_string(), "-5");
        assert_eq!(num2.neg().to_string(), "3");
    }

    #[test]
    fn test_complex_number_display() {
        // 测试纯实数复数
        let real_complex = Number::complex(Number::integer(5), Number::zero());
        assert_eq!(real_complex.to_string(), "5");

        // 测试纯虚数复数
        let pure_imaginary = Number::complex(Number::zero(), Number::integer(3));
        assert_eq!(pure_imaginary.to_string(), "3i");

        // 测试虚数单位
        let unit_imaginary = Number::complex(Number::zero(), Number::one());
        assert_eq!(unit_imaginary.to_string(), "i");

        // 测试负虚数单位
        let neg_unit_imaginary = Number::complex(Number::zero(), Number::neg_one());
        assert_eq!(neg_unit_imaginary.to_string(), "-i");

        // 测试一般复数
        let general_complex = Number::complex(Number::integer(2), Number::integer(3));
        assert_eq!(general_complex.to_string(), "2+3i");

        // 测试负虚部复数
        let neg_imag_complex = Number::complex(Number::integer(2), Number::integer(-3));
        assert_eq!(neg_imag_complex.to_string(), "2-3i");
    }

    #[test]
    fn test_rational_display() {
        // 测试整数形式的有理数
        let int_rational = Number::rational(6, 1);
        assert_eq!(int_rational.to_string(), "6");

        // 测试一般有理数
        let general_rational = Number::rational(3, 4);
        assert_eq!(general_rational.to_string(), "3/4");

        // 测试负有理数
        let neg_rational = Number::rational(-5, 3);
        assert_eq!(neg_rational.to_string(), "-5/3");
    }

    #[test]
    fn test_type_promotion() {
        let int_a = Number::integer(2);
        let int_b = Number::integer(3);
        let rational_a = Number::rational(1, 2);
        let real_a = Number::real(BigDecimal::from(314) / BigDecimal::from(100));

        // 测试相同类型
        let (promoted_a, promoted_b) = Number::promote_types(&int_a, &int_b);
        assert!(matches!(promoted_a, Number::Integer(_)));
        assert!(matches!(promoted_b, Number::Integer(_)));

        // 测试整数到有理数的提升
        let (promoted_a, promoted_b) = Number::promote_types(&int_a, &rational_a);
        assert!(matches!(promoted_a, Number::Rational(_)));
        assert!(matches!(promoted_b, Number::Rational(_)));

        // 测试整数到实数的提升
        let (promoted_a, promoted_b) = Number::promote_types(&int_a, &real_a);
        assert!(matches!(promoted_a, Number::Real(_)));
        assert!(matches!(promoted_b, Number::Real(_)));
    }

    #[test]
    fn test_conversion_methods() {
        // 测试整数转换
        let int_num = Number::integer(42);
        assert_eq!(int_num.to_integer(), Some(BigInt::from(42)));

        // 测试有理数转换
        let rational_num = Number::rational(3, 4);
        let expected_rational = BigRational::new(BigInt::from(3), BigInt::from(4));
        assert_eq!(rational_num.to_rational(), Some(expected_rational));

        // 测试整数形式的有理数转换
        let int_as_rational = Number::rational(6, 1);
        assert_eq!(int_as_rational.to_integer(), Some(BigInt::from(6)));

        // 测试复数的实部转换
        let complex_real = Number::complex(Number::integer(5), Number::zero());
        assert_eq!(complex_real.to_integer(), Some(BigInt::from(5)));
    }

    #[test]
    fn test_approximate_values() {
        let int_num = Number::integer(42);
        assert_eq!(int_num.approximate(), 42.0);

        let rational_num = Number::rational(1, 2);
        assert_eq!(rational_num.approximate(), 0.5);

        let float_num = Number::float(3.14);
        assert_eq!(float_num.approximate(), 3.14);

        // 测试复数的模长
        let complex_num = Number::complex(Number::integer(3), Number::integer(4));
        assert_eq!(complex_num.approximate(), 5.0); // |3+4i| = 5
    }

    #[test]
    fn test_from_conversions() {
        // 测试 From trait 实现
        let from_i32: Number = 42i32.into();
        assert!(matches!(from_i32, Number::Integer(_)));

        let from_i64: Number = 42i64.into();
        assert!(matches!(from_i64, Number::Integer(_)));

        let from_f64: Number = 3.14f64.into();
        assert!(matches!(from_f64, Number::Float(_)));

        let from_bigint: Number = BigInt::from(100).into();
        assert!(matches!(from_bigint, Number::Integer(_)));

        let from_rational: Number = BigRational::new(BigInt::from(3), BigInt::from(4)).into();
        assert!(matches!(from_rational, Number::Rational(_)));

        let from_decimal: Number = (BigDecimal::from(25) / BigDecimal::from(10)).into();
        assert!(matches!(from_decimal, Number::Real(_)));
    }

    #[test]
    fn test_edge_cases() {
        // 测试零的各种表示
        let zero_int = Number::integer(0);
        let zero_rational = Number::rational(0, 1);
        let zero_float = Number::float(0.0);
        
        assert!(zero_int.is_zero());
        assert!(zero_rational.is_zero());
        assert!(zero_float.is_zero());

        // 测试一的各种表示
        let one_int = Number::integer(1);
        let one_rational = Number::rational(1, 1);
        let one_float = Number::float(1.0);
        
        assert!(one_int.is_one());
        assert!(one_rational.is_one());
        assert!(one_float.is_one());

        // 测试大数
        let big_int = Number::integer(BigInt::from(2).pow(100));
        assert!(big_int.is_positive());
        assert!(big_int.is_integer());
        assert!(big_int.is_exact());
    }
}