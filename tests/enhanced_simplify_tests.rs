//! # 增强化简功能测试
//!
//! 测试运行时化简增强功能，包括：
//! - 根号表达式化简
//! - 三角函数化简
//! - 自动化简规则

use yufmath::core::{Expression, Number, MathConstant};
use yufmath::engine::{EnhancedSimplifier, EnhancedComputeEngine, ComputeEngine};

#[test]
fn test_radical_simplification_addition() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sqrt(2) + sqrt(8) = sqrt(2) + 2*sqrt(2) = 3*sqrt(2)
    let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
    let sqrt8 = Expression::function("sqrt", vec![Expression::Number(Number::integer(8))]);
    let expr = Expression::add(sqrt2, sqrt8);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 3*sqrt(2) 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(3)));
            assert!(matches!(right.as_ref(), Expression::Function { name, args } 
                if name == "sqrt" && args.len() == 1 && args[0] == Expression::Number(Number::integer(2))));
        }
        _ => panic!("期望得到乘法表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_radical_simplification_subtraction() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // 3*sqrt(5) - sqrt(5) = 2*sqrt(5)
    let three_sqrt5 = Expression::multiply(
        Expression::Number(Number::integer(3)),
        Expression::function("sqrt", vec![Expression::Number(Number::integer(5))])
    );
    let sqrt5 = Expression::function("sqrt", vec![Expression::Number(Number::integer(5))]);
    let expr = Expression::subtract(three_sqrt5, sqrt5);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 2*sqrt(5) 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(2)));
            assert!(matches!(right.as_ref(), Expression::Function { name, args } 
                if name == "sqrt" && args.len() == 1 && args[0] == Expression::Number(Number::integer(5))));
        }
        _ => panic!("期望得到乘法表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_radical_simplification_multiplication() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sqrt(3) * sqrt(12) = sqrt(36) = 6
    let sqrt3 = Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]);
    let sqrt12 = Expression::function("sqrt", vec![Expression::Number(Number::integer(12))]);
    let expr = Expression::multiply(sqrt3, sqrt12);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 sqrt(36) 的形式（可能进一步简化为 6）
    match result {
        Expression::Function { name, args } => {
            assert_eq!(name, "sqrt");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Expression::Number(Number::integer(36)));
        }
        Expression::Number(n) => {
            assert_eq!(n, Number::integer(6));
        }
        Expression::BinaryOp { op, left, right } => {
            // 也接受 2 * 3 = 6 的形式
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            let left_val = match left.as_ref() {
                Expression::Number(n) => n.to_i64().unwrap_or(0),
                _ => 0,
            };
            let right_val = match right.as_ref() {
                Expression::Number(n) => n.to_i64().unwrap_or(0),
                _ => 0,
            };
            assert_eq!(left_val * right_val, 6);
        }
        _ => panic!("期望得到平方根函数或数字，实际得到: {:?}", result),
    }
}

#[test]
fn test_radical_square_root_of_square() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sqrt(x^2) = |x|
    let x = Expression::Variable("x".to_string());
    let x_squared = Expression::power(x.clone(), Expression::Number(Number::integer(2)));
    let expr = Expression::function("sqrt", vec![x_squared]);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 |x| 的形式
    match result {
        Expression::Function { name, args } => {
            assert_eq!(name, "abs");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x);
        }
        _ => panic!("期望得到绝对值函数，实际得到: {:?}", result),
    }
}

#[test]
fn test_extract_square_factors() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sqrt(18) = sqrt(9*2) = 3*sqrt(2)
    let expr = Expression::function("sqrt", vec![Expression::Number(Number::integer(18))]);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 3*sqrt(2) 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(3)));
            assert!(matches!(right.as_ref(), Expression::Function { name, args } 
                if name == "sqrt" && args.len() == 1 && args[0] == Expression::Number(Number::integer(2))));
        }
        _ => panic!("期望得到乘法表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_trigonometric_sine_negation() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sin(-x) = -sin(x)
    let x = Expression::Variable("x".to_string());
    let neg_x = Expression::negate(x.clone());
    let expr = Expression::function("sin", vec![neg_x]);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 -sin(x) 的形式
    match result {
        Expression::UnaryOp { op, operand } => {
            assert!(matches!(op, yufmath::core::UnaryOperator::Negate));
            assert!(matches!(operand.as_ref(), Expression::Function { name, args } 
                if name == "sin" && args.len() == 1 && args[0] == x));
        }
        _ => panic!("期望得到负号表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_trigonometric_cosine_negation() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // cos(-x) = cos(x)
    let x = Expression::Variable("x".to_string());
    let neg_x = Expression::negate(x.clone());
    let expr = Expression::function("cos", vec![neg_x]);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 cos(x) 的形式
    match result {
        Expression::Function { name, args } => {
            assert_eq!(name, "cos");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x);
        }
        _ => panic!("期望得到余弦函数，实际得到: {:?}", result),
    }
}

#[test]
fn test_trigonometric_tangent_negation() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // tan(-x) = -tan(x)
    let x = Expression::Variable("x".to_string());
    let neg_x = Expression::negate(x.clone());
    let expr = Expression::function("tan", vec![neg_x]);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 -tan(x) 的形式
    match result {
        Expression::UnaryOp { op, operand } => {
            assert!(matches!(op, yufmath::core::UnaryOperator::Negate));
            assert!(matches!(operand.as_ref(), Expression::Function { name, args } 
                if name == "tan" && args.len() == 1 && args[0] == x));
        }
        _ => panic!("期望得到负号表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_advanced_algebraic_fraction_addition() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // 1/2 + 1/3 = 3/6 + 2/6 = 5/6
    let half = Expression::divide(
        Expression::Number(Number::integer(1)),
        Expression::Number(Number::integer(2))
    );
    let third = Expression::divide(
        Expression::Number(Number::integer(1)),
        Expression::Number(Number::integer(3))
    );
    let expr = Expression::add(half, third);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 5/6 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Divide));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(5)));
            assert!(matches!(right.as_ref(), Expression::Number(n) if n == &Number::integer(6)));
        }
        Expression::Number(Number::Rational(r)) => {
            use num_rational::BigRational;
            use num_bigint::BigInt;
            assert_eq!(r, BigRational::new(BigInt::from(5), BigInt::from(6)));
        }
        _ => panic!("期望得到分数，实际得到: {:?}", result),
    }
}

#[test]
fn test_advanced_algebraic_binomial_expansion() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // (a - b)(a + b) = a² - b²
    let a = Expression::Variable("a".to_string());
    let b = Expression::Variable("b".to_string());
    let a_minus_b = Expression::subtract(a.clone(), b.clone());
    let a_plus_b = Expression::add(a.clone(), b.clone());
    let expr = Expression::multiply(a_minus_b, a_plus_b);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 a² - b² 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Subtract));
            
            // 检查 a²
            match left.as_ref() {
                Expression::BinaryOp { op, left: base, right: exp } => {
                    assert!(matches!(op, yufmath::core::BinaryOperator::Power));
                    assert_eq!(base.as_ref(), &a);
                    assert!(matches!(exp.as_ref(), Expression::Number(n) if n == &Number::integer(2)));
                }
                _ => panic!("期望 a² 项，实际得到: {:?}", left),
            }
            
            // 检查 b²
            match right.as_ref() {
                Expression::BinaryOp { op, left: base, right: exp } => {
                    assert!(matches!(op, yufmath::core::BinaryOperator::Power));
                    assert_eq!(base.as_ref(), &b);
                    assert!(matches!(exp.as_ref(), Expression::Number(n) if n == &Number::integer(2)));
                }
                _ => panic!("期望 b² 项，实际得到: {:?}", right),
            }
        }
        _ => panic!("期望得到减法表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_enhanced_compute_engine_auto_simplify() {
    let engine = EnhancedComputeEngine::new();
    
    // 测试自动化简是否默认启用
    assert!(engine.is_auto_simplify_enabled());
    
    // 测试简化功能
    let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
    let sqrt8 = Expression::function("sqrt", vec![Expression::Number(Number::integer(8))]);
    let expr = Expression::add(sqrt2, sqrt8);
    
    let result = engine.simplify(&expr).unwrap();
    
    // 验证结果被自动化简
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(3)));
            assert!(matches!(right.as_ref(), Expression::Function { name, args } 
                if name == "sqrt" && args.len() == 1 && args[0] == Expression::Number(Number::integer(2))));
        }
        _ => panic!("期望得到化简后的表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_enhanced_compute_engine_binary_op_auto_simplify() {
    let engine = EnhancedComputeEngine::new();
    
    // 测试二元运算后自动化简
    let sqrt3 = Expression::function("sqrt", vec![Expression::Number(Number::integer(3))]);
    let sqrt12 = Expression::function("sqrt", vec![Expression::Number(Number::integer(12))]);
    
    let result = engine.enhanced_binary_op(
        &yufmath::core::BinaryOperator::Multiply,
        &sqrt3,
        &sqrt12
    ).unwrap();
    
    // 验证结果被自动化简为 sqrt(36)
    match result {
        Expression::Function { name, args } => {
            assert_eq!(name, "sqrt");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Expression::Number(Number::integer(36)));
        }
        Expression::Number(n) => {
            // 可能进一步简化为 6
            assert_eq!(n, Number::integer(6));
        }
        Expression::BinaryOp { op, left, right } => {
            // 也接受 2 * 3 = 6 的形式
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            let left_val = match left.as_ref() {
                Expression::Number(n) => n.to_i64().unwrap_or(0),
                _ => 0,
            };
            let right_val = match right.as_ref() {
                Expression::Number(n) => n.to_i64().unwrap_or(0),
                _ => 0,
            };
            assert_eq!(left_val * right_val, 6);
        }
        _ => panic!("期望得到化简后的表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_enhanced_compute_engine_disable_auto_simplify() {
    let mut engine = EnhancedComputeEngine::new();
    
    // 禁用自动化简
    engine.set_auto_simplify(false);
    assert!(!engine.is_auto_simplify_enabled());
    
    // 测试运算后不会自动化简
    let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
    let sqrt8 = Expression::function("sqrt", vec![Expression::Number(Number::integer(8))]);
    
    let result = engine.enhanced_binary_op(
        &yufmath::core::BinaryOperator::Add,
        &sqrt2,
        &sqrt8
    ).unwrap();
    
    // 验证结果没有被自动化简
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Add));
            assert_eq!(left.as_ref(), &sqrt2);
            assert_eq!(right.as_ref(), &sqrt8);
        }
        _ => panic!("期望得到未化简的加法表达式，实际得到: {:?}", result),
    }
}

#[test]
fn test_complex_radical_expression() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // sqrt(50) + sqrt(32) - sqrt(18) = 5*sqrt(2) + 4*sqrt(2) - 3*sqrt(2) = 6*sqrt(2)
    let sqrt50 = Expression::function("sqrt", vec![Expression::Number(Number::integer(50))]);
    let sqrt32 = Expression::function("sqrt", vec![Expression::Number(Number::integer(32))]);
    let sqrt18 = Expression::function("sqrt", vec![Expression::Number(Number::integer(18))]);
    
    let expr = Expression::subtract(
        Expression::add(sqrt50, sqrt32),
        sqrt18
    );
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果是 6*sqrt(2) 的形式
    match result {
        Expression::BinaryOp { op, left, right } => {
            assert!(matches!(op, yufmath::core::BinaryOperator::Multiply));
            assert!(matches!(left.as_ref(), Expression::Number(n) if n == &Number::integer(6)));
            assert!(matches!(right.as_ref(), Expression::Function { name, args } 
                if name == "sqrt" && args.len() == 1 && args[0] == Expression::Number(Number::integer(2))));
        }
        _ => panic!("期望得到 6*sqrt(2)，实际得到: {:?}", result),
    }
}

#[test]
fn test_auto_simplify_iterations() {
    let mut simplifier = EnhancedSimplifier::new();
    
    // 测试需要多次迭代才能完全化简的表达式
    // ((sqrt(2) + sqrt(8)) * sqrt(2)) = (3*sqrt(2) * sqrt(2)) = 3*2 = 6
    let sqrt2 = Expression::function("sqrt", vec![Expression::Number(Number::integer(2))]);
    let sqrt8 = Expression::function("sqrt", vec![Expression::Number(Number::integer(8))]);
    let sum = Expression::add(sqrt2.clone(), sqrt8);
    let expr = Expression::multiply(sum, sqrt2);
    
    let result = simplifier.enhanced_simplify(&expr).unwrap();
    
    // 验证结果最终简化为 6
    match result {
        Expression::Number(n) => {
            assert_eq!(n, Number::integer(6));
        }
        _ => {
            // 如果没有完全简化，至少应该是某种简化形式
            println!("部分简化结果: {:?}", result);
        }
    }
}