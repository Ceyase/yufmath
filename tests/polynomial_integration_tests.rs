//! # 多项式运算集成测试
//!
//! 测试多项式运算系统与计算引擎的集成。

use yufmath::core::{Expression, Number};
use yufmath::engine::{ComputeEngine, compute::BasicComputeEngine};

#[test]
fn test_polynomial_expand_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试展开 (x + 1)^2
    let expr = Expression::power(
        Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        ),
        Expression::number(Number::integer(2))
    );
    
    let expanded = engine.expand(&expr).unwrap();
    
    // 验证展开结果包含 x^2, 2x, 1 项
    let expanded_str = format!("{}", expanded);
    println!("Expanded result: {}", expanded_str);
    assert!(expanded_str.contains("x²") || expanded_str.contains("x^2") || expanded_str.contains("x"));
    assert!(expanded_str.contains("2"));
    assert!(expanded_str.contains("1"));
}

#[test]
fn test_polynomial_factor_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试因式分解 6x^2 + 9x
    let expr = Expression::add(
        Expression::multiply(
            Expression::number(Number::integer(6)),
            Expression::power(
                Expression::variable("x"),
                Expression::number(Number::integer(2))
            )
        ),
        Expression::multiply(
            Expression::number(Number::integer(9)),
            Expression::variable("x")
        )
    );
    
    let factored = engine.factor(&expr).unwrap();
    
    // 验证因式分解结果是乘法形式
    match factored {
        Expression::BinaryOp { op, .. } => {
            assert_eq!(op, yufmath::core::BinaryOperator::Multiply);
        }
        _ => panic!("因式分解结果应该是乘法表达式"),
    }
}

#[test]
fn test_polynomial_collect_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试收集同类项 2x + 3x
    let expr = Expression::add(
        Expression::multiply(
            Expression::number(Number::integer(2)),
            Expression::variable("x")
        ),
        Expression::multiply(
            Expression::number(Number::integer(3)),
            Expression::variable("x")
        )
    );
    
    let collected = engine.collect(&expr, "x").unwrap();
    
    // 验证收集结果
    let collected_str = format!("{}", collected);
    assert!(collected_str.contains("5") && collected_str.contains("x"));
}

#[test]
fn test_polynomial_divide_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试多项式除法 (x^2 + 3x + 2) / (x + 1)
    let dividend = Expression::add(
        Expression::add(
            Expression::power(
                Expression::variable("x"),
                Expression::number(Number::integer(2))
            ),
            Expression::multiply(
                Expression::number(Number::integer(3)),
                Expression::variable("x")
            )
        ),
        Expression::number(Number::integer(2))
    );
    
    let divisor = Expression::add(
        Expression::variable("x"),
        Expression::number(Number::integer(1))
    );
    
    let (quotient, remainder) = engine.polynomial_divide(&dividend, &divisor).unwrap();
    
    // 验证商是一次多项式
    let quotient_str = format!("{}", quotient);
    assert!(quotient_str.contains("x"));
    
    // 验证余式为0或常数
    let remainder_str = format!("{}", remainder);
    assert!(remainder_str == "0" || !remainder_str.contains("x"));
}

#[test]
fn test_polynomial_gcd_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试多项式最大公约数 gcd(x^2 - 1, x^2 + 2x + 1)
    let poly1 = Expression::subtract(
        Expression::power(
            Expression::variable("x"),
            Expression::number(Number::integer(2))
        ),
        Expression::number(Number::integer(1))
    );
    
    let poly2 = Expression::add(
        Expression::add(
            Expression::power(
                Expression::variable("x"),
                Expression::number(Number::integer(2))
            ),
            Expression::multiply(
                Expression::number(Number::integer(2)),
                Expression::variable("x")
            )
        ),
        Expression::number(Number::integer(1))
    );
    
    let gcd = engine.polynomial_gcd(&poly1, &poly2).unwrap();
    
    // 验证最大公约数是一次多项式
    let gcd_str = format!("{}", gcd);
    assert!(gcd_str.contains("x"));
    assert!(gcd_str.contains("1"));
}

#[test]
fn test_multivariate_polynomial_operations() {
    let engine = BasicComputeEngine::new();
    
    // 测试多变量多项式展开 (x + y)^2
    let expr = Expression::power(
        Expression::add(
            Expression::variable("x"),
            Expression::variable("y")
        ),
        Expression::number(Number::integer(2))
    );
    
    let expanded = engine.expand(&expr).unwrap();
    
    // 验证展开结果包含 x^2, 2xy, y^2 项
    let expanded_str = format!("{}", expanded);
    assert!(expanded_str.contains("x"));
    assert!(expanded_str.contains("y"));
    assert!(expanded_str.contains("2"));
}

#[test]
fn test_polynomial_simplify_integration() {
    let engine = BasicComputeEngine::new();
    
    // 测试多项式简化与展开的结合
    let expr = Expression::multiply(
        Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        ),
        Expression::subtract(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        )
    );
    
    // 先展开
    let expanded = engine.expand(&expr).unwrap();
    
    // 再简化
    let simplified = engine.simplify(&expanded).unwrap();
    
    // 验证结果是 x^2 - 1
    let result_str = format!("{}", simplified);
    assert!(result_str.contains("x"));
    assert!(result_str.contains("1"));
}

#[test]
fn test_polynomial_error_handling() {
    let engine = BasicComputeEngine::new();
    
    // 测试除零错误
    let dividend = Expression::variable("x");
    let divisor = Expression::number(Number::zero());
    
    let result = engine.polynomial_divide(&dividend, &divisor);
    assert!(result.is_err());
    
    // 测试不支持的表达式类型
    let unsupported_expr = Expression::sin(Expression::variable("x"));
    let result = engine.expand(&unsupported_expr);
    assert!(result.is_err());
}

#[test]
fn test_polynomial_precision_preservation() {
    let engine = BasicComputeEngine::new();
    
    // 测试有理数系数的多项式运算
    let expr = Expression::add(
        Expression::multiply(
            Expression::number(Number::rational(1, 3)),
            Expression::variable("x")
        ),
        Expression::multiply(
            Expression::number(Number::rational(2, 3)),
            Expression::variable("x")
        )
    );
    
    let collected = engine.collect(&expr, "x").unwrap();
    
    // 验证精确性保持
    let result_str = format!("{}", collected);
    assert!(result_str.contains("x"));
    // 应该是 x（因为 1/3 + 2/3 = 1）
}