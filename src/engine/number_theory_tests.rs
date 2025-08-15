//! # 数论和组合数学功能测试
//!
//! 测试数论相关算法的正确性，包括最大公约数、最小公倍数、
//! 素数判断、质因数分解、二项式系数、排列组合和统计函数。

#[cfg(test)]
mod tests {
    use crate::engine::number_theory::NumberTheoryEngine;
    use crate::core::{Expression, Number};
    use num_bigint::BigInt;
    use num_rational::BigRational;

    fn create_engine() -> NumberTheoryEngine {
        NumberTheoryEngine::new()
    }

    #[test]
    fn test_gcd_integers() {
        let engine = create_engine();
        
        // 测试基本的最大公约数计算
        let a = Expression::Number(Number::Integer(BigInt::from(48)));
        let b = Expression::Number(Number::Integer(BigInt::from(18)));
        
        let result = engine.gcd(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(6))));
        
        // 测试互质数
        let a = Expression::Number(Number::Integer(BigInt::from(17)));
        let b = Expression::Number(Number::Integer(BigInt::from(13)));
        
        let result = engine.gcd(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(1))));
        
        // 测试一个数为零的情况
        let a = Expression::Number(Number::Integer(BigInt::from(0)));
        let b = Expression::Number(Number::Integer(BigInt::from(5)));
        
        let result = engine.gcd(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(5))));
    }

    #[test]
    fn test_gcd_rationals() {
        let engine = create_engine();
        
        // 测试有理数的最大公约数
        let a = Expression::Number(Number::Rational(BigRational::new(BigInt::from(6), BigInt::from(4))));
        let b = Expression::Number(Number::Rational(BigRational::new(BigInt::from(9), BigInt::from(6))));
        
        let result = engine.gcd(&a, &b).unwrap();
        // gcd(6/4, 9/6) = gcd(3/2, 3/2) = 3/2
        let expected = Expression::Number(Number::Rational(BigRational::new(BigInt::from(3), BigInt::from(2))));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lcm_integers() {
        let engine = create_engine();
        
        // 测试基本的最小公倍数计算
        let a = Expression::Number(Number::Integer(BigInt::from(12)));
        let b = Expression::Number(Number::Integer(BigInt::from(8)));
        
        let result = engine.lcm(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(24))));
        
        // 测试互质数
        let a = Expression::Number(Number::Integer(BigInt::from(7)));
        let b = Expression::Number(Number::Integer(BigInt::from(11)));
        
        let result = engine.lcm(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(77))));
        
        // 测试一个数为零的情况
        let a = Expression::Number(Number::Integer(BigInt::from(0)));
        let b = Expression::Number(Number::Integer(BigInt::from(5)));
        
        let result = engine.lcm(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(0))));
    }

    #[test]
    fn test_is_prime() {
        let engine = create_engine();
        
        // 测试小素数
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in primes {
            let n = Expression::Number(Number::Integer(BigInt::from(p)));
            assert!(engine.is_prime(&n).unwrap(), "{} 应该是素数", p);
        }
        
        // 测试合数
        let composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
        for c in composites {
            let n = Expression::Number(Number::Integer(BigInt::from(c)));
            assert!(!engine.is_prime(&n).unwrap(), "{} 不应该是素数", c);
        }
        
        // 测试边界情况
        let n = Expression::Number(Number::Integer(BigInt::from(1)));
        assert!(!engine.is_prime(&n).unwrap(), "1 不是素数");
        
        let n = Expression::Number(Number::Integer(BigInt::from(0)));
        assert!(!engine.is_prime(&n).unwrap(), "0 不是素数");
    }

    #[test]
    fn test_prime_factors() {
        let engine = create_engine();
        
        // 测试小数的质因数分解
        let n = Expression::Number(Number::Integer(BigInt::from(12)));
        let factors = engine.prime_factors(&n).unwrap();
        let expected = vec![
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(3))),
        ];
        assert_eq!(factors, expected);
        
        // 测试素数的质因数分解
        let n = Expression::Number(Number::Integer(BigInt::from(17)));
        let factors = engine.prime_factors(&n).unwrap();
        let expected = vec![Expression::Number(Number::Integer(BigInt::from(17)))];
        assert_eq!(factors, expected);
        
        // 测试较大数的质因数分解
        let n = Expression::Number(Number::Integer(BigInt::from(60)));
        let factors = engine.prime_factors(&n).unwrap();
        let expected = vec![
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(3))),
            Expression::Number(Number::Integer(BigInt::from(5))),
        ];
        assert_eq!(factors, expected);
    }

    #[test]
    fn test_binomial_coefficient() {
        let engine = create_engine();
        
        // 测试基本的二项式系数
        let n = Expression::Number(Number::Integer(BigInt::from(5)));
        let k = Expression::Number(Number::Integer(BigInt::from(2)));
        
        let result = engine.binomial(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(10))));
        
        // 测试边界情况：C(n, 0) = 1
        let n = Expression::Number(Number::Integer(BigInt::from(10)));
        let k = Expression::Number(Number::Integer(BigInt::from(0)));
        
        let result = engine.binomial(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(1))));
        
        // 测试边界情况：C(n, n) = 1
        let n = Expression::Number(Number::Integer(BigInt::from(7)));
        let k = Expression::Number(Number::Integer(BigInt::from(7)));
        
        let result = engine.binomial(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(1))));
        
        // 测试对称性：C(n, k) = C(n, n-k)
        let n = Expression::Number(Number::Integer(BigInt::from(8)));
        let k1 = Expression::Number(Number::Integer(BigInt::from(3)));
        let k2 = Expression::Number(Number::Integer(BigInt::from(5)));
        
        let result1 = engine.binomial(&n, &k1).unwrap();
        let result2 = engine.binomial(&n, &k2).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_permutation() {
        let engine = create_engine();
        
        // 测试基本的排列数
        let n = Expression::Number(Number::Integer(BigInt::from(5)));
        let k = Expression::Number(Number::Integer(BigInt::from(3)));
        
        let result = engine.permutation(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(60))));
        
        // 测试边界情况：P(n, 0) = 1
        let n = Expression::Number(Number::Integer(BigInt::from(10)));
        let k = Expression::Number(Number::Integer(BigInt::from(0)));
        
        let result = engine.permutation(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(1))));
        
        // 测试边界情况：P(n, n) = n!
        let n = Expression::Number(Number::Integer(BigInt::from(4)));
        let k = Expression::Number(Number::Integer(BigInt::from(4)));
        
        let result = engine.permutation(&n, &k).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(24)))); // 4! = 24
    }

    #[test]
    fn test_mean() {
        let engine = create_engine();
        
        // 测试整数的平均值
        let values = vec![
            Expression::Number(Number::Integer(BigInt::from(1))),
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(3))),
            Expression::Number(Number::Integer(BigInt::from(4))),
            Expression::Number(Number::Integer(BigInt::from(5))),
        ];
        
        let result = engine.mean(&values).unwrap();
        assert_eq!(result, Expression::Number(Number::Rational(BigRational::new(BigInt::from(15), BigInt::from(5)))));
        
        // 测试有理数的平均值
        let values = vec![
            Expression::Number(Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(2)))),
            Expression::Number(Number::Rational(BigRational::new(BigInt::from(3), BigInt::from(4)))),
        ];
        
        let result = engine.mean(&values).unwrap();
        // (1/2 + 3/4) / 2 = (2/4 + 3/4) / 2 = 5/4 / 2 = 5/8
        let expected = Expression::Number(Number::Rational(BigRational::new(BigInt::from(5), BigInt::from(8))));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_variance() {
        let engine = create_engine();
        
        // 测试简单数据的方差
        let values = vec![
            Expression::Number(Number::Integer(BigInt::from(1))),
            Expression::Number(Number::Integer(BigInt::from(2))),
            Expression::Number(Number::Integer(BigInt::from(3))),
        ];
        
        let result = engine.variance(&values).unwrap();
        // 平均值 = 2, 方差 = ((1-2)² + (2-2)² + (3-2)²) / 3 = (1 + 0 + 1) / 3 = 2/3
        let expected = Expression::Number(Number::Rational(BigRational::new(BigInt::from(2), BigInt::from(3))));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_standard_deviation() {
        let engine = create_engine();
        
        // 测试完全平方数的标准差
        let values = vec![
            Expression::Number(Number::Integer(BigInt::from(0))),
            Expression::Number(Number::Integer(BigInt::from(4))),
        ];
        
        let result = engine.standard_deviation(&values).unwrap();
        // 平均值 = 2, 方差 = ((0-2)² + (4-2)²) / 2 = (4 + 4) / 2 = 4
        // 标准差 = sqrt(4) = 2
        // 由于我们的实现可能返回有理数形式，我们检查数值是否相等
        match result {
            Expression::Number(Number::Integer(n)) => {
                assert_eq!(n, BigInt::from(2));
            }
            Expression::Number(Number::Rational(r)) => {
                assert_eq!(r, BigRational::new(BigInt::from(2), BigInt::from(1)));
            }
            _ => panic!("期望得到数值结果"),
        }
    }

    #[test]
    fn test_error_cases() {
        let engine = create_engine();
        
        // 测试不支持的类型
        let a = Expression::Variable("x".to_string());
        let b = Expression::Number(Number::Integer(BigInt::from(5)));
        
        assert!(engine.gcd(&a, &b).is_err());
        assert!(engine.lcm(&a, &b).is_err());
        assert!(engine.is_prime(&a).is_err());
        assert!(engine.prime_factors(&a).is_err());
        assert!(engine.binomial(&a, &b).is_err());
        assert!(engine.permutation(&a, &b).is_err());
        
        // 测试空列表的平均值
        let empty_values: Vec<Expression> = vec![];
        assert!(engine.mean(&empty_values).is_err());
        
        // 测试单个值的方差
        let single_value = vec![Expression::Number(Number::Integer(BigInt::from(5)))];
        assert!(engine.variance(&single_value).is_err());
        
        // 测试质因数分解的边界情况
        let zero = Expression::Number(Number::Integer(BigInt::from(0)));
        assert!(engine.prime_factors(&zero).is_err());
        
        let one = Expression::Number(Number::Integer(BigInt::from(1)));
        assert!(engine.prime_factors(&one).is_err());
    }

    #[test]
    fn test_large_numbers() {
        let engine = create_engine();
        
        // 测试大数的最大公约数
        let a = Expression::Number(Number::Integer(BigInt::from(123456789)));
        let b = Expression::Number(Number::Integer(BigInt::from(987654321)));
        
        let result = engine.gcd(&a, &b).unwrap();
        // 这两个数的最大公约数是 9
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(9))));
        
        // 测试大数的二项式系数
        let n = Expression::Number(Number::Integer(BigInt::from(20)));
        let k = Expression::Number(Number::Integer(BigInt::from(10)));
        
        let result = engine.binomial(&n, &k).unwrap();
        // C(20, 10) = 184756
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(184756))));
    }

    #[test]
    fn test_negative_numbers() {
        let engine = create_engine();
        
        // 测试负数的最大公约数（应该返回正值）
        let a = Expression::Number(Number::Integer(BigInt::from(-12)));
        let b = Expression::Number(Number::Integer(BigInt::from(8)));
        
        let result = engine.gcd(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(4))));
        
        // 测试负数的最小公倍数（应该返回正值）
        let result = engine.lcm(&a, &b).unwrap();
        assert_eq!(result, Expression::Number(Number::Integer(BigInt::from(24))));
    }

    #[test]
    fn test_rational_arithmetic() {
        let engine = create_engine();
        
        // 测试有理数运算的精确性
        let values = vec![
            Expression::Number(Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(3)))),
            Expression::Number(Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(6)))),
            Expression::Number(Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(2)))),
        ];
        
        let result = engine.mean(&values).unwrap();
        // (1/3 + 1/6 + 1/2) / 3 = (2/6 + 1/6 + 3/6) / 3 = 6/6 / 3 = 1/3
        let expected = Expression::Number(Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(3))));
        assert_eq!(result, expected);
    }
}