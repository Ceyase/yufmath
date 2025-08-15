//! # 多项式运算测试

use super::*;
use crate::core::{Expression, Number};

#[test]
fn test_polynomial_term_creation() {
    // 测试常数项
    let const_term = PolynomialTerm::constant(Number::integer(5));
    assert!(const_term.is_constant());
    assert_eq!(const_term.degree(), 0);
    
    // 测试变量项
    let var_term = PolynomialTerm::variable("x".to_string(), 2, Number::integer(3));
    assert!(!var_term.is_constant());
    assert_eq!(var_term.degree(), 2);
    assert_eq!(var_term.degree_of("x"), 2);
    assert_eq!(var_term.degree_of("y"), 0);
}

#[test]
fn test_polynomial_term_multiplication() {
    // 3x^2 * 2y^3 = 6x^2y^3
    let term1 = PolynomialTerm::variable("x".to_string(), 2, Number::integer(3));
    let term2 = PolynomialTerm::variable("y".to_string(), 3, Number::integer(2));
    
    let result = term1.multiply(&term2);
    assert_eq!(result.coefficient, Number::integer(6));
    assert_eq!(result.degree_of("x"), 2);
    assert_eq!(result.degree_of("y"), 3);
    assert_eq!(result.degree(), 5);
}

#[test]
fn test_polynomial_term_division() {
    // 6x^3y^2 / 2xy = 3x^2y
    let mut vars1 = HashMap::new();
    vars1.insert("x".to_string(), 3);
    vars1.insert("y".to_string(), 2);
    let term1 = PolynomialTerm::new(Number::integer(6), vars1);
    
    let mut vars2 = HashMap::new();
    vars2.insert("x".to_string(), 1);
    vars2.insert("y".to_string(), 1);
    let term2 = PolynomialTerm::new(Number::integer(2), vars2);
    
    let result = term1.divide(&term2).unwrap();
    assert_eq!(result.coefficient, Number::integer(3));
    assert_eq!(result.degree_of("x"), 2);
    assert_eq!(result.degree_of("y"), 1);
}

#[test]
fn test_polynomial_creation_and_simplification() {
    // 创建多项式 3x^2 + 2x^2 + x - x = 5x^2
    let terms = vec![
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(3)),
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(2)),
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(1)),
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(-1)),
    ];
    
    let poly = Polynomial::new(terms);
    assert_eq!(poly.terms.len(), 1);
    assert_eq!(poly.terms[0].coefficient, Number::integer(5));
    assert_eq!(poly.terms[0].degree_of("x"), 2);
}

#[test]
fn test_polynomial_addition() {
    // (2x + 3) + (x^2 - x + 1) = x^2 + x + 4
    let poly1 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(2)),
        PolynomialTerm::constant(Number::integer(3)),
    ]);
    
    let poly2 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(1)),
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(-1)),
        PolynomialTerm::constant(Number::integer(1)),
    ]);
    
    let result = poly1.add(&poly2);
    assert_eq!(result.terms.len(), 3);
    
    // 检查结果项（按次数排序）
    assert_eq!(result.terms[0].degree_of("x"), 2);
    assert_eq!(result.terms[0].coefficient, Number::integer(1));
    
    assert_eq!(result.terms[1].degree_of("x"), 1);
    assert_eq!(result.terms[1].coefficient, Number::integer(1));
    
    assert_eq!(result.terms[2].degree_of("x"), 0);
    assert_eq!(result.terms[2].coefficient, Number::integer(4));
}

#[test]
fn test_polynomial_multiplication() {
    // (x + 1) * (x + 2) = x^2 + 3x + 2
    let poly1 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(1)),
        PolynomialTerm::constant(Number::integer(1)),
    ]);
    
    let poly2 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(1)),
        PolynomialTerm::constant(Number::integer(2)),
    ]);
    
    let result = poly1.multiply(&poly2);
    assert_eq!(result.terms.len(), 3);
    
    // 检查结果项
    assert_eq!(result.terms[0].degree_of("x"), 2);
    assert_eq!(result.terms[0].coefficient, Number::integer(1));
    
    assert_eq!(result.terms[1].degree_of("x"), 1);
    assert_eq!(result.terms[1].coefficient, Number::integer(3));
    
    assert_eq!(result.terms[2].degree_of("x"), 0);
    assert_eq!(result.terms[2].coefficient, Number::integer(2));
}

#[test]
fn test_polynomial_division() {
    // (x^2 + 3x + 2) / (x + 1) = x + 2
    let dividend = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(1)),
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(3)),
        PolynomialTerm::constant(Number::integer(2)),
    ]);
    
    let divisor = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(1)),
        PolynomialTerm::constant(Number::integer(1)),
    ]);
    
    let (quotient, remainder) = dividend.divide(&divisor).unwrap();
    
    // 检查商
    assert_eq!(quotient.terms.len(), 2);
    assert_eq!(quotient.terms[0].degree_of("x"), 1);
    assert_eq!(quotient.terms[0].coefficient, Number::integer(1));
    assert_eq!(quotient.terms[1].degree_of("x"), 0);
    assert_eq!(quotient.terms[1].coefficient, Number::integer(2));
    
    // 检查余式（应该为0）
    assert!(remainder.is_zero());
}

#[test]
fn test_polynomial_gcd() {
    // gcd(x^2 - 1, x^2 + 2x + 1) = x + 1
    // x^2 - 1 = (x-1)(x+1)
    // x^2 + 2x + 1 = (x+1)^2
    // 所以 gcd = x + 1
    let poly1 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(1)),
        PolynomialTerm::constant(Number::integer(-1)),
    ]);
    
    let poly2 = Polynomial::new(vec![
        PolynomialTerm::variable("x".to_string(), 2, Number::integer(1)),
        PolynomialTerm::variable("x".to_string(), 1, Number::integer(2)),
        PolynomialTerm::constant(Number::integer(1)),
    ]);
    
    // 先测试除法是否正确
    let (quotient, remainder) = poly2.divide(&poly1).unwrap();
    println!("poly2 / poly1 = {:?}, remainder = {:?}", quotient, remainder);
    
    let gcd = poly1.gcd(&poly2).unwrap();
    println!("GCD result: {:?}", gcd);
    
    // 检查最大公约数的次数应该是1
    assert_eq!(gcd.degree(), 1);
    
    // 检查 GCD 能整除两个多项式
    let (q1, r1) = poly1.divide(&gcd).unwrap();
    let (q2, r2) = poly2.divide(&gcd).unwrap();
    
    assert!(r1.is_zero(), "GCD 应该能整除 poly1");
    assert!(r2.is_zero(), "GCD 应该能整除 poly2");
}

#[test]
fn test_polynomial_engine_expand() {
    let engine = PolynomialEngine::new();
    
    // 展开 (x + 1)^2
    let expr = Expression::power(
        Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        ),
        Expression::number(Number::integer(2))
    );
    
    let expanded = engine.expand(&expr).unwrap();
    
    // 验证展开结果是多项式形式
    let poly = engine.expression_to_polynomial(&expanded).unwrap();
    assert_eq!(poly.terms.len(), 3);
    
    // 检查系数
    let mut found_x2 = false;
    let mut found_x1 = false;
    let mut found_const = false;
    
    for term in &poly.terms {
        match term.degree_of("x") {
            2 => {
                assert_eq!(term.coefficient, Number::integer(1));
                found_x2 = true;
            }
            1 => {
                assert_eq!(term.coefficient, Number::integer(2));
                found_x1 = true;
            }
            0 => {
                assert_eq!(term.coefficient, Number::integer(1));
                found_const = true;
            }
            _ => panic!("意外的项次数"),
        }
    }
    
    assert!(found_x2 && found_x1 && found_const);
}

#[test]
fn test_polynomial_engine_collect() {
    let engine = PolynomialEngine::new();
    
    // 收集 2x + 3y + x - y 中 x 的同类项
    let expr = Expression::add(
        Expression::add(
            Expression::multiply(
                Expression::number(Number::integer(2)),
                Expression::variable("x")
            ),
            Expression::multiply(
                Expression::number(Number::integer(3)),
                Expression::variable("y")
            )
        ),
        Expression::subtract(
            Expression::variable("x"),
            Expression::variable("y")
        )
    );
    
    let collected = engine.collect(&expr, "x").unwrap();
    
    // 验证收集结果
    let poly = engine.expression_to_polynomial(&collected).unwrap();
    
    // 应该有 x 项和 y 项
    let mut found_x = false;
    let mut found_y = false;
    
    for term in &poly.terms {
        if term.degree_of("x") == 1 && term.degree_of("y") == 0 {
            assert_eq!(term.coefficient, Number::integer(3)); // 2x + x = 3x
            found_x = true;
        } else if term.degree_of("x") == 0 && term.degree_of("y") == 1 {
            assert_eq!(term.coefficient, Number::integer(2)); // 3y - y = 2y
            found_y = true;
        }
    }
    
    assert!(found_x && found_y);
}

#[test]
fn test_polynomial_engine_factor() {
    let engine = PolynomialEngine::new();
    
    // 因式分解 6x^2 + 9x = 3x(2x + 3)
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
    if let Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } = factored {
        // 检查公因子部分
        let factor_poly = engine.expression_to_polynomial(&left).unwrap();
        let remaining_poly = engine.expression_to_polynomial(&right).unwrap();
        
        // 验证提取了公因子
        let reconstructed = factor_poly.multiply(&remaining_poly);
        let original_poly = engine.expression_to_polynomial(&expr).unwrap();
        
        // 重新构建的多项式应该等于原多项式
        assert_eq!(reconstructed.terms.len(), original_poly.terms.len());
    }
}

#[test]
fn test_polynomial_engine_divide() {
    let engine = PolynomialEngine::new();
    
    // 多项式除法 (x^2 + 3x + 2) / (x + 1)
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
    
    // 验证商
    let quotient_poly = engine.expression_to_polynomial(&quotient).unwrap();
    assert_eq!(quotient_poly.terms.len(), 2);
    
    // 验证余式为0
    let remainder_poly = engine.expression_to_polynomial(&remainder).unwrap();
    assert!(remainder_poly.is_zero());
}

#[test]
fn test_polynomial_engine_gcd() {
    let engine = PolynomialEngine::new();
    
    // 计算 gcd(x^2 - 1, x^2 + 2x + 1)
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
    
    // 验证最大公约数
    let gcd_poly = engine.expression_to_polynomial(&gcd).unwrap();
    assert_eq!(gcd_poly.degree(), 1); // 应该是一次多项式
}

#[test]
fn test_expression_to_polynomial_conversion() {
    let engine = PolynomialEngine::new();
    
    // 测试简单表达式转换
    let expr = Expression::add(
        Expression::multiply(
            Expression::number(Number::integer(2)),
            Expression::variable("x")
        ),
        Expression::number(Number::integer(3))
    );
    
    let poly = engine.expression_to_polynomial(&expr).unwrap();
    assert_eq!(poly.terms.len(), 2);
    
    // 测试转换回表达式
    let converted_back = poly.to_expression();
    let poly_back = engine.expression_to_polynomial(&converted_back).unwrap();
    
    // 应该保持一致
    assert_eq!(poly.terms.len(), poly_back.terms.len());
}

#[test]
fn test_multivariate_polynomial() {
    let engine = PolynomialEngine::new();
    
    // 测试多变量多项式 2xy + 3x + y - 1
    let expr = Expression::subtract(
        Expression::add(
            Expression::add(
                Expression::multiply(
                    Expression::multiply(
                        Expression::number(Number::integer(2)),
                        Expression::variable("x")
                    ),
                    Expression::variable("y")
                ),
                Expression::multiply(
                    Expression::number(Number::integer(3)),
                    Expression::variable("x")
                )
            ),
            Expression::variable("y")
        ),
        Expression::number(Number::integer(1))
    );
    
    let poly = engine.expression_to_polynomial(&expr).unwrap();
    assert_eq!(poly.terms.len(), 4);
    
    // 验证各项
    let mut found_xy = false;
    let mut found_x = false;
    let mut found_y = false;
    let mut found_const = false;
    
    for term in &poly.terms {
        if term.degree_of("x") == 1 && term.degree_of("y") == 1 {
            assert_eq!(term.coefficient, Number::integer(2));
            found_xy = true;
        } else if term.degree_of("x") == 1 && term.degree_of("y") == 0 {
            assert_eq!(term.coefficient, Number::integer(3));
            found_x = true;
        } else if term.degree_of("x") == 0 && term.degree_of("y") == 1 {
            assert_eq!(term.coefficient, Number::integer(1));
            found_y = true;
        } else if term.degree_of("x") == 0 && term.degree_of("y") == 0 {
            assert_eq!(term.coefficient, Number::integer(-1));
            found_const = true;
        }
    }
    
    assert!(found_xy && found_x && found_y && found_const);
}