//! # 表达式简化测试
//!
//! 测试表达式简化功能的正确性。

#[cfg(test)]
mod tests {
    use crate::engine::simplify::Simplifier;
    use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
    use num_bigint::BigInt;

    /// 创建测试用的简化器
    fn create_simplifier() -> Simplifier {
        Simplifier::new()
    }

    #[test]
    fn test_basic_arithmetic_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 0 + x = x
        let expr = Expression::add(
            Expression::Number(Number::zero()),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 x + 0 = x
        let expr = Expression::add(
            Expression::variable("x"),
            Expression::Number(Number::zero())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 1 * x = x
        let expr = Expression::multiply(
            Expression::Number(Number::one()),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 x * 1 = x
        let expr = Expression::multiply(
            Expression::variable("x"),
            Expression::Number(Number::one())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 0 * x = 0
        let expr = Expression::multiply(
            Expression::Number(Number::zero()),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::zero()));
    }

    #[test]
    fn test_constant_folding() {
        let mut simplifier = create_simplifier();

        // 测试 2 + 3 = 5
        let expr = Expression::add(
            Expression::Number(Number::integer(2)),
            Expression::Number(Number::integer(3))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(5)));

        // 测试 5 - 2 = 3
        let expr = Expression::subtract(
            Expression::Number(Number::integer(5)),
            Expression::Number(Number::integer(2))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(3)));

        // 测试 3 * 4 = 12
        let expr = Expression::multiply(
            Expression::Number(Number::integer(3)),
            Expression::Number(Number::integer(4))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(12)));

        // 测试 8 / 2 = 4
        let expr = Expression::divide(
            Expression::Number(Number::integer(8)),
            Expression::Number(Number::integer(2))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(4)));
    }

    #[test]
    fn test_like_terms_combination() {
        let mut simplifier = create_simplifier();

        // 测试 x + x = 2x
        let expr = Expression::add(
            Expression::variable("x"),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::multiply(
            Expression::Number(Number::integer(2)),
            Expression::variable("x")
        );
        assert_eq!(result, expected);

        // 测试 2x + 3x = 5x
        let expr = Expression::add(
            Expression::multiply(
                Expression::Number(Number::integer(2)),
                Expression::variable("x")
            ),
            Expression::multiply(
                Expression::Number(Number::integer(3)),
                Expression::variable("x")
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::multiply(
            Expression::Number(Number::integer(5)),
            Expression::variable("x")
        );
        assert_eq!(result, expected);

        // 测试 5x - 2x = 3x
        let expr = Expression::subtract(
            Expression::multiply(
                Expression::Number(Number::integer(5)),
                Expression::variable("x")
            ),
            Expression::multiply(
                Expression::Number(Number::integer(2)),
                Expression::variable("x")
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::multiply(
            Expression::Number(Number::integer(3)),
            Expression::variable("x")
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 x^0 = 1
        let expr = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::zero())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::one()));

        // 测试 x^1 = x
        let expr = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::one())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 1^x = 1
        let expr = Expression::power(
            Expression::Number(Number::one()),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::one()));

        // 测试 2^3 = 8
        let expr = Expression::power(
            Expression::Number(Number::integer(2)),
            Expression::Number(Number::integer(3))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(8)));
    }

    #[test]
    fn test_negation_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 -(-x) = x
        let expr = Expression::negate(
            Expression::negate(Expression::variable("x"))
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 -(a + b) = -a - b
        let expr = Expression::negate(
            Expression::add(
                Expression::variable("a"),
                Expression::variable("b")
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::subtract(
            Expression::negate(Expression::variable("a")),
            Expression::variable("b")
        );
        assert_eq!(result, expected);

        // 测试 -(a - b) = b - a
        let expr = Expression::negate(
            Expression::subtract(
                Expression::variable("a"),
                Expression::variable("b")
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::subtract(
            Expression::variable("b"),
            Expression::variable("a")
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_absolute_value_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 |5| = 5
        let expr = Expression::abs(Expression::Number(Number::integer(5)));
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(5)));

        // 测试 |-3| = 3
        let expr = Expression::abs(Expression::Number(Number::integer(-3)));
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(3)));

        // 测试 |-x| = |x|
        let expr = Expression::abs(
            Expression::negate(Expression::variable("x"))
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::abs(Expression::variable("x"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_division_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 x / 1 = x
        let expr = Expression::divide(
            Expression::variable("x"),
            Expression::Number(Number::one())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 x / x = 1
        let expr = Expression::divide(
            Expression::variable("x"),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::one()));

        // 测试 0 / x = 0
        let expr = Expression::divide(
            Expression::Number(Number::zero()),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::zero()));

        // 测试 x / -1 = -x
        let expr = Expression::divide(
            Expression::variable("x"),
            Expression::Number(Number::integer(-1))
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::negate(Expression::variable("x"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_power_combination() {
        let mut simplifier = create_simplifier();

        // 测试 x * x = x^2
        let expr = Expression::multiply(
            Expression::variable("x"),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::integer(2))
        );
        assert_eq!(result, expected);

        // 测试 x^2 * x^3 = x^5
        let expr = Expression::multiply(
            Expression::power(
                Expression::variable("x"),
                Expression::Number(Number::integer(2))
            ),
            Expression::power(
                Expression::variable("x"),
                Expression::Number(Number::integer(3))
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::integer(5))
        );
        assert_eq!(result, expected);

        // 测试 x^5 / x^2 = x^3
        let expr = Expression::divide(
            Expression::power(
                Expression::variable("x"),
                Expression::Number(Number::integer(5))
            ),
            Expression::power(
                Expression::variable("x"),
                Expression::Number(Number::integer(2))
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::integer(3))
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_expression_simplification() {
        let mut simplifier = create_simplifier();

        // 测试 (x + 0) * 1 + 0 = x
        let expr = Expression::add(
            Expression::multiply(
                Expression::add(
                    Expression::variable("x"),
                    Expression::Number(Number::zero())
                ),
                Expression::Number(Number::one())
            ),
            Expression::Number(Number::zero())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::variable("x"));

        // 测试 2 * (3 + 4) = 14
        let expr = Expression::multiply(
            Expression::Number(Number::integer(2)),
            Expression::add(
                Expression::Number(Number::integer(3)),
                Expression::Number(Number::integer(4))
            )
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::integer(14)));
    }

    #[test]
    fn test_canonical_form() {
        let mut simplifier = create_simplifier();

        // 测试交换律：x + 2 应该变成 2 + x
        let expr = Expression::add(
            Expression::variable("x"),
            Expression::Number(Number::integer(2))
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::add(
            Expression::Number(Number::integer(2)),
            Expression::variable("x")
        );
        assert_eq!(result, expected);

        // 测试交换律：x * 3 应该变成 3 * x
        let expr = Expression::multiply(
            Expression::variable("x"),
            Expression::Number(Number::integer(3))
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::multiply(
            Expression::Number(Number::integer(3)),
            Expression::variable("x")
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mathematical_constants() {
        let mut simplifier = create_simplifier();

        // 测试 π + 0 = π
        let expr = Expression::add(
            Expression::constant(MathConstant::Pi),
            Expression::Number(Number::zero())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::constant(MathConstant::Pi));

        // 测试 e * 1 = e
        let expr = Expression::multiply(
            Expression::constant(MathConstant::E),
            Expression::Number(Number::one())
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::constant(MathConstant::E));

        // 测试 i * i = -1 (通过常量折叠)
        let expr = Expression::multiply(
            Expression::constant(MathConstant::I),
            Expression::constant(MathConstant::I)
        );
        let result = simplifier.simplify(&expr).unwrap();
        // 由于 i 被转换为复数，i * i 应该简化为 -1
        assert_eq!(result, Expression::Number(Number::integer(-1)));
    }

    #[test]
    fn test_edge_cases() {
        let mut simplifier = create_simplifier();

        // 测试 x - x = 0
        let expr = Expression::subtract(
            Expression::variable("x"),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        assert_eq!(result, Expression::Number(Number::zero()));

        // 测试 -1 * x = -x
        let expr = Expression::multiply(
            Expression::Number(Number::integer(-1)),
            Expression::variable("x")
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::negate(Expression::variable("x"));
        assert_eq!(result, expected);

        // 测试 (x^2)^3 = x^6
        let expr = Expression::power(
            Expression::power(
                Expression::variable("x"),
                Expression::Number(Number::integer(2))
            ),
            Expression::Number(Number::integer(3))
        );
        let result = simplifier.simplify(&expr).unwrap();
        let expected = Expression::power(
            Expression::variable("x"),
            Expression::Number(Number::integer(6))
        );
        assert_eq!(result, expected);
    }
}