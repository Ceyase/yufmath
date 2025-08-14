//! # 表达式类型系统测试
//!
//! 测试表达式的构造、验证和类型推断功能。

use super::*;
use crate::core::{Number, MathConstant, BinaryOperator, UnaryOperator, ExprType, NumericType};
use num_bigint::BigInt;

#[cfg(test)]
mod expression_construction_tests {
    use super::*;

    #[test]
    fn test_basic_expression_construction() {
        // 测试数值表达式
        let num_expr = Expression::number(Number::integer(42));
        assert!(matches!(num_expr, Expression::Number(_)));
        
        // 测试变量表达式
        let var_expr = Expression::variable("x");
        assert!(matches!(var_expr, Expression::Variable(ref name) if name == "x"));
        
        // 测试常量表达式
        let const_expr = Expression::constant(MathConstant::Pi);
        assert!(matches!(const_expr, Expression::Constant(MathConstant::Pi)));
    }

    #[test]
    fn test_binary_operation_construction() {
        let x = Expression::variable("x");
        let y = Expression::variable("y");
        
        // 测试加法
        let add_expr = Expression::add(x.clone(), y.clone());
        match add_expr {
            Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
                assert_eq!(*left, x);
                assert_eq!(*right, y);
            }
            _ => panic!("期望加法表达式"),
        }
        
        // 测试乘法
        let mul_expr = Expression::multiply(x.clone(), y.clone());
        match mul_expr {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert_eq!(*left, x);
                assert_eq!(*right, y);
            }
            _ => panic!("期望乘法表达式"),
        }
        
        // 测试幂运算
        let pow_expr = Expression::power(x.clone(), Expression::number(Number::integer(2)));
        match pow_expr {
            Expression::BinaryOp { op: BinaryOperator::Power, left, right } => {
                assert_eq!(*left, x);
                assert!(matches!(*right, Expression::Number(_)));
            }
            _ => panic!("期望幂运算表达式"),
        }
    }

    #[test]
    fn test_unary_operation_construction() {
        let x = Expression::variable("x");
        
        // 测试负号
        let neg_expr = Expression::negate(x.clone());
        match neg_expr {
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                assert_eq!(*operand, x);
            }
            _ => panic!("期望负号表达式"),
        }
        
        // 测试平方根
        let sqrt_expr = Expression::sqrt(x.clone());
        match sqrt_expr {
            Expression::UnaryOp { op: UnaryOperator::Sqrt, operand } => {
                assert_eq!(*operand, x);
            }
            _ => panic!("期望平方根表达式"),
        }
        
        // 测试三角函数
        let sin_expr = Expression::sin(x.clone());
        match sin_expr {
            Expression::UnaryOp { op: UnaryOperator::Sin, operand } => {
                assert_eq!(*operand, x);
            }
            _ => panic!("期望正弦函数表达式"),
        }
    }

    #[test]
    fn test_matrix_construction() {
        // 测试有效矩阵
        let matrix_data = vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ];
        let matrix_expr = Expression::matrix(matrix_data.clone()).unwrap();
        match matrix_expr {
            Expression::Matrix(rows) => {
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0].len(), 2);
                assert_eq!(rows[1].len(), 2);
            }
            _ => panic!("期望矩阵表达式"),
        }
        
        // 测试空矩阵（应该失败）
        let empty_matrix = Expression::matrix(vec![]);
        assert!(empty_matrix.is_err());
        
        // 测试不规则矩阵（应该失败）
        let irregular_matrix = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1))],
            vec![Expression::number(Number::integer(2)), Expression::number(Number::integer(3))],
        ]);
        assert!(irregular_matrix.is_err());
    }

    #[test]
    fn test_vector_construction() {
        // 测试有效向量
        let vector_data = vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ];
        let vector_expr = Expression::vector(vector_data.clone()).unwrap();
        match vector_expr {
            Expression::Vector(elements) => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("期望向量表达式"),
        }
        
        // 测试空向量（应该失败）
        let empty_vector = Expression::vector(vec![]);
        assert!(empty_vector.is_err());
    }

    #[test]
    fn test_set_construction() {
        let set_data = vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ];
        let set_expr = Expression::set(set_data.clone());
        match set_expr {
            Expression::Set(elements) => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("期望集合表达式"),
        }
    }

    #[test]
    fn test_interval_construction() {
        let start = Expression::number(Number::integer(0));
        let end = Expression::number(Number::integer(10));
        
        // 测试闭区间
        let closed_interval = Expression::interval(start.clone(), end.clone(), true, true);
        match closed_interval {
            Expression::Interval { start_inclusive, end_inclusive, .. } => {
                assert!(start_inclusive);
                assert!(end_inclusive);
            }
            _ => panic!("期望区间表达式"),
        }
        
        // 测试开区间
        let open_interval = Expression::interval(start, end, false, false);
        match open_interval {
            Expression::Interval { start_inclusive, end_inclusive, .. } => {
                assert!(!start_inclusive);
                assert!(!end_inclusive);
            }
            _ => panic!("期望区间表达式"),
        }
    }
}

#[cfg(test)]
mod expression_properties_tests {
    use super::*;

    #[test]
    fn test_is_constant() {
        // 常量表达式
        let const_expr = Expression::number(Number::integer(42));
        assert!(const_expr.is_constant());
        
        let pi_expr = Expression::constant(MathConstant::Pi);
        assert!(pi_expr.is_constant());
        
        // 变量表达式
        let var_expr = Expression::variable("x");
        assert!(!var_expr.is_constant());
        
        // 包含变量的复合表达式
        let mixed_expr = Expression::add(
            Expression::number(Number::integer(1)),
            Expression::variable("x")
        );
        assert!(!mixed_expr.is_constant());
        
        // 纯常量的复合表达式
        let const_compound = Expression::add(
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2))
        );
        assert!(const_compound.is_constant());
    }

    #[test]
    fn test_get_variables() {
        // 无变量表达式
        let const_expr = Expression::number(Number::integer(42));
        assert_eq!(const_expr.get_variables(), Vec::<String>::new());
        
        // 单变量表达式
        let var_expr = Expression::variable("x");
        assert_eq!(var_expr.get_variables(), vec!["x"]);
        
        // 多变量表达式
        let multi_var = Expression::add(
            Expression::multiply(
                Expression::variable("x"),
                Expression::variable("y")
            ),
            Expression::variable("z")
        );
        let mut vars = multi_var.get_variables();
        vars.sort();
        assert_eq!(vars, vec!["x", "y", "z"]);
        
        // 重复变量（应该去重）
        let dup_var = Expression::add(
            Expression::variable("x"),
            Expression::variable("x")
        );
        assert_eq!(dup_var.get_variables(), vec!["x"]);
    }

    #[test]
    fn test_complexity() {
        // 简单表达式
        let simple = Expression::number(Number::integer(1));
        assert_eq!(simple.complexity(), 1);
        
        let var = Expression::variable("x");
        assert_eq!(var.complexity(), 1);
        
        // 二元运算
        let binary = Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        );
        assert_eq!(binary.complexity(), 3); // 1 + 1 + 1
        
        // 嵌套表达式
        let nested = Expression::add(
            Expression::multiply(
                Expression::variable("x"),
                Expression::variable("y")
            ),
            Expression::number(Number::integer(1))
        );
        assert_eq!(nested.complexity(), 5); // ((1 + 1) + 1) + 1
        
        // 矩阵表达式
        let matrix = Expression::matrix(vec![
            vec![Expression::variable("x"), Expression::number(Number::integer(1))],
            vec![Expression::number(Number::integer(2)), Expression::variable("y")],
        ]).unwrap();
        assert_eq!(matrix.complexity(), 5); // 1 + (1 + 1 + 1 + 1)
    }
}

#[cfg(test)]
mod type_inference_tests {
    use super::*;

    #[test]
    fn test_basic_type_inference() {
        // 数值类型推断
        let int_expr = Expression::number(Number::integer(42));
        assert_eq!(int_expr.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        let rational_expr = Expression::number(Number::rational(1, 2));
        assert_eq!(rational_expr.infer_type(), ExprType::Numeric(NumericType::Rational));
        
        let complex_expr = Expression::number(Number::complex(
            Number::integer(1),
            Number::integer(1)
        ));
        assert_eq!(complex_expr.infer_type(), ExprType::Numeric(NumericType::Complex));
        
        // 变量类型推断
        let var_expr = Expression::variable("x");
        assert_eq!(var_expr.infer_type(), ExprType::Symbolic);
        
        // 常量类型推断
        let pi_expr = Expression::constant(MathConstant::Pi);
        assert_eq!(pi_expr.infer_type(), ExprType::Numeric(NumericType::Real));
        
        let i_expr = Expression::constant(MathConstant::I);
        assert_eq!(i_expr.infer_type(), ExprType::Numeric(NumericType::Complex));
    }

    #[test]
    fn test_binary_operation_type_inference() {
        let int1 = Expression::number(Number::integer(1));
        let int2 = Expression::number(Number::integer(2));
        let rational = Expression::number(Number::rational(1, 2));
        let complex = Expression::number(Number::complex(Number::integer(1), Number::integer(1)));
        
        // 整数 + 整数 = 整数
        let int_add = Expression::add(int1.clone(), int2.clone());
        assert_eq!(int_add.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 整数 + 有理数 = 有理数
        let mixed_add = Expression::add(int1.clone(), rational.clone());
        assert_eq!(mixed_add.infer_type(), ExprType::Numeric(NumericType::Rational));
        
        // 整数 + 复数 = 复数
        let complex_add = Expression::add(int1.clone(), complex.clone());
        assert_eq!(complex_add.infer_type(), ExprType::Numeric(NumericType::Complex));
        
        // 整数 ^ 整数 = 有理数（因为可能产生分数）
        let power = Expression::power(int1.clone(), int2.clone());
        assert_eq!(power.infer_type(), ExprType::Numeric(NumericType::Rational));
        
        // 整数 % 整数 = 整数
        let modulo = Expression::binary_op(BinaryOperator::Modulo, int1.clone(), int2.clone());
        assert_eq!(modulo.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 比较运算返回整数（布尔值）
        let comparison = Expression::binary_op(BinaryOperator::Less, int1.clone(), int2.clone());
        assert_eq!(comparison.infer_type(), ExprType::Numeric(NumericType::Integer));
    }

    #[test]
    fn test_unary_operation_type_inference() {
        let int_expr = Expression::number(Number::integer(42));
        let complex_expr = Expression::number(Number::complex(
            Number::integer(3),
            Number::integer(4)
        ));
        
        // 负号保持类型
        let neg = Expression::negate(int_expr.clone());
        assert_eq!(neg.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 平方根：整数 -> 实数
        let sqrt = Expression::sqrt(int_expr.clone());
        assert_eq!(sqrt.infer_type(), ExprType::Numeric(NumericType::Real));
        
        // 绝对值：复数 -> 实数
        let abs_complex = Expression::abs(complex_expr.clone());
        assert_eq!(abs_complex.infer_type(), ExprType::Numeric(NumericType::Real));
        
        // 三角函数：实数 -> 实数
        let sin = Expression::sin(int_expr.clone());
        assert_eq!(sin.infer_type(), ExprType::Numeric(NumericType::Real));
        
        // 三角函数：复数 -> 复数
        let sin_complex = Expression::sin(complex_expr.clone());
        assert_eq!(sin_complex.infer_type(), ExprType::Numeric(NumericType::Complex));
        
        // 阶乘：整数 -> 整数
        let factorial = Expression::unary_op(UnaryOperator::Factorial, int_expr.clone());
        assert_eq!(factorial.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 实部：复数 -> 实数
        let real_part = Expression::unary_op(UnaryOperator::Real, complex_expr.clone());
        assert_eq!(real_part.infer_type(), ExprType::Numeric(NumericType::Real));
    }

    #[test]
    fn test_matrix_vector_type_inference() {
        // 矩阵类型推断
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ]).unwrap();
        assert_eq!(matrix.infer_type(), ExprType::Matrix(2, 2, Box::new(ExprType::Numeric(NumericType::Integer))));
        
        // 向量类型推断
        let vector = Expression::vector(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]).unwrap();
        assert_eq!(vector.infer_type(), ExprType::Vector(3, Box::new(ExprType::Numeric(NumericType::Integer))));
        
        // 矩阵转置
        let transpose = Expression::unary_op(UnaryOperator::Transpose, matrix.clone());
        assert_eq!(transpose.infer_type(), ExprType::Matrix(2, 2, Box::new(ExprType::Numeric(NumericType::Integer))));
        
        // 矩阵行列式
        let det = Expression::unary_op(UnaryOperator::Determinant, matrix.clone());
        assert_eq!(det.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 向量点积
        let dot_product = Expression::binary_op(BinaryOperator::DotProduct, vector.clone(), vector.clone());
        assert_eq!(dot_product.infer_type(), ExprType::Numeric(NumericType::Integer));
    }

    #[test]
    fn test_set_interval_type_inference() {
        // 集合类型推断
        let set = Expression::set(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]);
        assert_eq!(set.infer_type(), ExprType::Set(Box::new(ExprType::Numeric(NumericType::Integer))));
        
        // 区间类型推断
        let interval = Expression::interval(
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(10)),
            true,
            false
        );
        assert_eq!(interval.infer_type(), ExprType::Interval(Box::new(ExprType::Numeric(NumericType::Integer))));
        
        // 集合运算
        let union = Expression::binary_op(BinaryOperator::Union, set.clone(), set.clone());
        assert_eq!(union.infer_type(), ExprType::Set(Box::new(ExprType::Numeric(NumericType::Integer))));
    }

    #[test]
    fn test_function_type_inference() {
        let x = Expression::variable("x");
        
        // 已知函数的类型推断
        let sin_func = Expression::function("sin", vec![x.clone()]);
        assert_eq!(sin_func.infer_type(), ExprType::Symbolic); // 符号函数
        
        let max_func = Expression::function("max", vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
        ]);
        assert_eq!(max_func.infer_type(), ExprType::Numeric(NumericType::Integer));
        
        // 未知函数
        let unknown_func = Expression::function("unknown", vec![x]);
        assert_eq!(unknown_func.infer_type(), ExprType::Unknown);
    }
}

#[cfg(test)]
mod expression_validation_tests {
    use super::*;

    #[test]
    fn test_basic_validation() {
        // 基本表达式应该通过验证
        let simple_expr = Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        );
        assert!(simple_expr.validate().is_ok());
        
        // 嵌套表达式应该通过验证
        let nested_expr = Expression::multiply(
            Expression::add(Expression::variable("x"), Expression::number(Number::integer(1))),
            Expression::subtract(Expression::variable("y"), Expression::number(Number::integer(2)))
        );
        assert!(nested_expr.validate().is_ok());
    }

    #[test]
    fn test_matrix_validation() {
        // 有效矩阵应该通过验证
        let valid_matrix = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ]).unwrap();
        assert!(valid_matrix.validate().is_ok());
        
        // 矩阵运算的验证
        let matrix_a = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ]).unwrap();
        
        let matrix_b = Expression::matrix(vec![
            vec![Expression::number(Number::integer(5)), Expression::number(Number::integer(6))],
            vec![Expression::number(Number::integer(7)), Expression::number(Number::integer(8))],
        ]).unwrap();
        
        // 兼容的矩阵乘法
        let valid_mult = Expression::binary_op(BinaryOperator::MatrixMultiply, matrix_a.clone(), matrix_b.clone());
        assert!(valid_mult.validate().is_ok());
        
        // 方阵的行列式
        let det = Expression::unary_op(UnaryOperator::Determinant, matrix_a.clone());
        assert!(det.validate().is_ok());
        
        // 方阵的逆矩阵
        let inv = Expression::unary_op(UnaryOperator::Inverse, matrix_a.clone());
        assert!(inv.validate().is_ok());
    }

    #[test]
    fn test_vector_validation() {
        let vector_a = Expression::vector(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]).unwrap();
        
        let vector_b = Expression::vector(vec![
            Expression::number(Number::integer(4)),
            Expression::number(Number::integer(5)),
            Expression::number(Number::integer(6)),
        ]).unwrap();
        
        // 相同维度的向量点积
        let dot_product = Expression::binary_op(BinaryOperator::DotProduct, vector_a.clone(), vector_b.clone());
        assert!(dot_product.validate().is_ok());
        
        // 3维向量的叉积
        let cross_product = Expression::binary_op(BinaryOperator::CrossProduct, vector_a.clone(), vector_b.clone());
        assert!(cross_product.validate().is_ok());
        
        // 不同维度的向量点积（应该失败）
        let vector_2d = Expression::vector(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
        ]).unwrap();
        
        let invalid_dot = Expression::binary_op(BinaryOperator::DotProduct, vector_a.clone(), vector_2d.clone());
        assert!(invalid_dot.validate().is_err());
        
        // 非3维向量的叉积（应该失败）
        let invalid_cross = Expression::binary_op(BinaryOperator::CrossProduct, vector_a.clone(), vector_2d);
        assert!(invalid_cross.validate().is_err());
    }

    #[test]
    fn test_special_function_validation() {
        let int_expr = Expression::number(Number::integer(5));
        let float_expr = Expression::number(Number::float(3.14));
        
        // 阶乘只能应用于整数
        let valid_factorial = Expression::unary_op(UnaryOperator::Factorial, int_expr.clone());
        assert!(valid_factorial.validate().is_ok());
        
        let invalid_factorial = Expression::unary_op(UnaryOperator::Factorial, float_expr.clone());
        assert!(invalid_factorial.validate().is_err());
        
        // 矩阵运算只能应用于矩阵
        let non_matrix = Expression::variable("x");
        let invalid_det = Expression::unary_op(UnaryOperator::Determinant, non_matrix.clone());
        assert!(invalid_det.validate().is_err());
        
        let invalid_inv = Expression::unary_op(UnaryOperator::Inverse, non_matrix);
        assert!(invalid_inv.validate().is_err());
    }

    #[test]
    fn test_nested_validation() {
        // 嵌套表达式的验证应该递归进行
        let inner_invalid = Expression::unary_op(
            UnaryOperator::Factorial,
            Expression::number(Number::float(3.14))
        );
        
        let outer_expr = Expression::add(
            Expression::variable("x"),
            inner_invalid
        );
        
        // 外层表达式应该因为内层的无效表达式而验证失败
        assert!(outer_expr.validate().is_err());
    }
}

#[cfg(test)]
mod expression_evaluation_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_number_evaluation() {
        // 数值表达式应该直接返回数值
        let num_expr = Expression::number(Number::integer(42));
        let result = num_expr.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(42));
        
        // 常量表达式
        let pi_expr = Expression::constant(MathConstant::Pi);
        let result = pi_expr.evaluate_exact().unwrap();
        // Pi 应该返回符号表示
        assert!(matches!(result, Number::Symbolic(_)));
        
        // 虚数单位
        let i_expr = Expression::constant(MathConstant::I);
        let result = i_expr.evaluate_exact().unwrap();
        assert_eq!(result, Number::i());
    }

    #[test]
    fn test_variable_substitution() {
        let x = Expression::variable("x");
        let y = Expression::variable("y");
        
        // 创建表达式 x + y
        let expr = Expression::add(x.clone(), y.clone());
        
        // 创建变量映射
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Number::integer(3));
        vars.insert("y".to_string(), Number::integer(4));
        
        // 求值
        let result = expr.evaluate(&vars).unwrap();
        assert_eq!(result, Number::integer(7));
    }

    #[test]
    fn test_arithmetic_evaluation() {
        // 测试基本算术运算
        let expr1 = Expression::add(
            Expression::number(Number::integer(3)),
            Expression::number(Number::integer(4))
        );
        assert_eq!(expr1.evaluate_exact().unwrap(), Number::integer(7));
        
        let expr2 = Expression::subtract(
            Expression::number(Number::integer(10)),
            Expression::number(Number::integer(3))
        );
        assert_eq!(expr2.evaluate_exact().unwrap(), Number::integer(7));
        
        let expr3 = Expression::multiply(
            Expression::number(Number::integer(6)),
            Expression::number(Number::integer(7))
        );
        assert_eq!(expr3.evaluate_exact().unwrap(), Number::integer(42));
        
        let expr4 = Expression::divide(
            Expression::number(Number::integer(15)),
            Expression::number(Number::integer(3))
        );
        assert_eq!(expr4.evaluate_exact().unwrap(), Number::integer(5));
    }

    #[test]
    fn test_power_evaluation() {
        // 整数的小幂次可以精确计算
        let expr = Expression::power(
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3))
        );
        assert_eq!(expr.evaluate_exact().unwrap(), Number::integer(8));
        
        // 0的正数次幂
        let expr_zero = Expression::power(
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(5))
        );
        assert_eq!(expr_zero.evaluate_exact().unwrap(), Number::zero());
        
        // 任何数的0次幂
        let expr_zero_exp = Expression::power(
            Expression::number(Number::integer(42)),
            Expression::number(Number::integer(0))
        );
        assert_eq!(expr_zero_exp.evaluate_exact().unwrap(), Number::one());
        
        // 任何数的1次幂
        let expr_one_exp = Expression::power(
            Expression::number(Number::integer(42)),
            Expression::number(Number::integer(1))
        );
        assert_eq!(expr_one_exp.evaluate_exact().unwrap(), Number::integer(42));
    }

    #[test]
    fn test_unary_operation_evaluation() {
        // 负号
        let neg_expr = Expression::negate(Expression::number(Number::integer(5)));
        assert_eq!(neg_expr.evaluate_exact().unwrap(), Number::integer(-5));
        
        // 绝对值
        let abs_expr = Expression::abs(Expression::number(Number::integer(-7)));
        assert_eq!(abs_expr.evaluate_exact().unwrap(), Number::integer(7));
        
        // 阶乘
        let fact_expr = Expression::unary_op(
            UnaryOperator::Factorial,
            Expression::number(Number::integer(5))
        );
        assert_eq!(fact_expr.evaluate_exact().unwrap(), Number::integer(120));
        
        // 平方根（完全平方数）
        let sqrt_expr = Expression::sqrt(Expression::number(Number::integer(16)));
        assert_eq!(sqrt_expr.evaluate_exact().unwrap(), Number::integer(4));
        
        // 平方根（非完全平方数，应该返回符号表示）
        let sqrt_expr2 = Expression::sqrt(Expression::number(Number::integer(2)));
        let result = sqrt_expr2.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
    }

    #[test]
    fn test_complex_number_evaluation() {
        let complex_num = Number::complex(Number::integer(3), Number::integer(4));
        let expr = Expression::number(complex_num.clone());
        
        // 复数的实部
        let real_expr = Expression::unary_op(UnaryOperator::Real, expr.clone());
        assert_eq!(real_expr.evaluate_exact().unwrap(), Number::integer(3));
        
        // 复数的虚部
        let imag_expr = Expression::unary_op(UnaryOperator::Imaginary, expr.clone());
        assert_eq!(imag_expr.evaluate_exact().unwrap(), Number::integer(4));
        
        // 复数的共轭
        let conj_expr = Expression::unary_op(UnaryOperator::Conjugate, expr.clone());
        let expected_conj = Number::complex(Number::integer(3), Number::integer(-4));
        assert_eq!(conj_expr.evaluate_exact().unwrap(), expected_conj);
    }

    #[test]
    fn test_comparison_evaluation() {
        // 等于
        let eq_expr = Expression::binary_op(
            BinaryOperator::Equal,
            Expression::number(Number::integer(5)),
            Expression::number(Number::integer(5))
        );
        assert_eq!(eq_expr.evaluate_exact().unwrap(), Number::one());
        
        let neq_expr = Expression::binary_op(
            BinaryOperator::Equal,
            Expression::number(Number::integer(5)),
            Expression::number(Number::integer(3))
        );
        assert_eq!(neq_expr.evaluate_exact().unwrap(), Number::zero());
        
        // 小于
        let lt_expr = Expression::binary_op(
            BinaryOperator::Less,
            Expression::number(Number::integer(3)),
            Expression::number(Number::integer(5))
        );
        assert_eq!(lt_expr.evaluate_exact().unwrap(), Number::one());
    }

    #[test]
    fn test_logical_evaluation() {
        // 逻辑与
        let and_expr = Expression::binary_op(
            BinaryOperator::And,
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(1))
        );
        assert_eq!(and_expr.evaluate_exact().unwrap(), Number::one());
        
        let and_expr_false = Expression::binary_op(
            BinaryOperator::And,
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(0))
        );
        assert_eq!(and_expr_false.evaluate_exact().unwrap(), Number::zero());
        
        // 逻辑或
        let or_expr = Expression::binary_op(
            BinaryOperator::Or,
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(1))
        );
        assert_eq!(or_expr.evaluate_exact().unwrap(), Number::one());
    }

    #[test]
    fn test_function_evaluation() {
        // max 函数
        let max_expr = Expression::function("max", vec![
            Expression::number(Number::integer(3)),
            Expression::number(Number::integer(7)),
            Expression::number(Number::integer(5)),
        ]);
        assert_eq!(max_expr.evaluate_exact().unwrap(), Number::integer(7));
        
        // min 函数
        let min_expr = Expression::function("min", vec![
            Expression::number(Number::integer(3)),
            Expression::number(Number::integer(7)),
            Expression::number(Number::integer(5)),
        ]);
        assert_eq!(min_expr.evaluate_exact().unwrap(), Number::integer(3));
        
        // abs 函数
        let abs_func = Expression::function("abs", vec![
            Expression::number(Number::integer(-10)),
        ]);
        assert_eq!(abs_func.evaluate_exact().unwrap(), Number::integer(10));
    }

    #[test]
    fn test_modulo_evaluation() {
        let mod_expr = Expression::binary_op(
            BinaryOperator::Modulo,
            Expression::number(Number::integer(17)),
            Expression::number(Number::integer(5))
        );
        assert_eq!(mod_expr.evaluate_exact().unwrap(), Number::integer(2));
    }

    #[test]
    fn test_nested_expression_evaluation() {
        // (2 + 3) * (4 - 1)
        let nested_expr = Expression::multiply(
            Expression::add(
                Expression::number(Number::integer(2)),
                Expression::number(Number::integer(3))
            ),
            Expression::subtract(
                Expression::number(Number::integer(4)),
                Expression::number(Number::integer(1))
            )
        );
        assert_eq!(nested_expr.evaluate_exact().unwrap(), Number::integer(15));
    }

    #[test]
    fn test_variable_substitution_complex() {
        // 创建表达式 (x + y) * z
        let expr = Expression::multiply(
            Expression::add(
                Expression::variable("x"),
                Expression::variable("y")
            ),
            Expression::variable("z")
        );
        
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Number::integer(2));
        vars.insert("y".to_string(), Number::integer(3));
        vars.insert("z".to_string(), Number::integer(4));
        
        let result = expr.evaluate(&vars).unwrap();
        assert_eq!(result, Number::integer(20));
    }

    #[test]
    fn test_error_cases() {
        // 除零错误
        let div_zero = Expression::divide(
            Expression::number(Number::integer(5)),
            Expression::number(Number::integer(0))
        );
        assert!(div_zero.evaluate_exact().is_err());
        
        // 未定义变量
        let undefined_var = Expression::variable("undefined");
        assert!(undefined_var.evaluate_exact().is_err());
        
        // 负数阶乘
        let neg_factorial = Expression::unary_op(
            UnaryOperator::Factorial,
            Expression::number(Number::integer(-1))
        );
        assert!(neg_factorial.evaluate_exact().is_err());
        
        // 0的负数次幂
        let zero_neg_power = Expression::power(
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(-1))
        );
        assert!(zero_neg_power.evaluate_exact().is_err());
    }

    #[test]
    fn test_symbolic_results() {
        // 三角函数应该返回符号表示
        let sin_expr = Expression::sin(Expression::number(Number::integer(1)));
        let result = sin_expr.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
        
        // 对数函数应该返回符号表示
        let ln_expr = Expression::ln(Expression::number(Number::integer(2)));
        let result = ln_expr.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
        
        // 大数幂运算应该返回符号表示
        let big_power = Expression::power(
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(1000))
        );
        let result = big_power.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
    }

    #[test]
    fn test_matrix_vector_evaluation() {
        // 矩阵求值
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ]).unwrap();
        let result = matrix.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
        
        // 向量求值
        let vector = Expression::vector(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]).unwrap();
        let result = vector.evaluate_exact().unwrap();
        assert!(matches!(result, Number::Symbolic(_)));
    }

    #[test]
    fn test_is_evaluable() {
        // 纯数值表达式应该可求值
        let simple_expr = Expression::add(
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2))
        );
        assert!(simple_expr.is_evaluable());
        
        // 包含未定义变量的表达式不可求值
        let var_expr = Expression::add(
            Expression::variable("x"),
            Expression::number(Number::integer(1))
        );
        assert!(!var_expr.is_evaluable());
        
        // 包含除零的表达式不可求值
        let div_zero = Expression::divide(
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(0))
        );
        assert!(!div_zero.is_evaluable());
    }

    #[test]
    fn test_try_to_number() {
        // 可求值的表达式
        let simple_expr = Expression::multiply(
            Expression::number(Number::integer(3)),
            Expression::number(Number::integer(4))
        );
        let result = simple_expr.try_to_number();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Number::integer(12));
        
        // 不可求值的表达式
        let var_expr = Expression::variable("x");
        let result = var_expr.try_to_number();
        assert!(result.is_none());
    }
}

#[cfg(test)]
mod expression_display_tests {
    use super::*;

    #[test]
    fn test_basic_display() {
        // 数值显示
        let num = Expression::number(Number::integer(42));
        assert_eq!(format!("{}", num), "42");
        
        // 变量显示
        let var = Expression::variable("x");
        assert_eq!(format!("{}", var), "x");
        
        // 常量显示
        let pi = Expression::constant(MathConstant::Pi);
        assert_eq!(format!("{}", pi), "π");
    }

    #[test]
    fn test_binary_operation_display() {
        let x = Expression::variable("x");
        let y = Expression::variable("y");
        let two = Expression::number(Number::integer(2));
        
        // 基本二元运算
        let add = Expression::add(x.clone(), y.clone());
        assert_eq!(format!("{}", add), "x + y");
        
        let mul = Expression::multiply(x.clone(), two.clone());
        assert_eq!(format!("{}", mul), "x * 2");
        
        let pow = Expression::power(x.clone(), two.clone());
        assert_eq!(format!("{}", pow), "x ^ 2");
        
        // 运算符优先级和括号
        let complex_expr = Expression::add(
            Expression::multiply(x.clone(), y.clone()),
            Expression::number(Number::integer(1))
        );
        assert_eq!(format!("{}", complex_expr), "x * y + 1");
        
        // 需要括号的情况
        let with_parens = Expression::multiply(
            Expression::add(x.clone(), y.clone()),
            Expression::number(Number::integer(2))
        );
        assert_eq!(format!("{}", with_parens), "(x + y) * 2");
    }

    #[test]
    fn test_unary_operation_display() {
        let x = Expression::variable("x");
        
        // 前缀一元运算符
        let neg = Expression::negate(x.clone());
        assert_eq!(format!("{}", neg), "-x");
        
        // 函数形式的一元运算符
        let sin = Expression::sin(x.clone());
        assert_eq!(format!("{}", sin), "sin(x)");
        
        let sqrt = Expression::sqrt(x.clone());
        assert_eq!(format!("{}", sqrt), "√(x)");
        
        // 后缀一元运算符
        let factorial = Expression::unary_op(UnaryOperator::Factorial, x.clone());
        assert_eq!(format!("{}", factorial), "x!");
        
        let transpose = Expression::unary_op(UnaryOperator::Transpose, x.clone());
        assert_eq!(format!("{}", transpose), "x^T");
    }

    #[test]
    fn test_composite_structure_display() {
        // 矩阵显示
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::integer(1)), Expression::number(Number::integer(2))],
            vec![Expression::number(Number::integer(3)), Expression::number(Number::integer(4))],
        ]).unwrap();
        assert_eq!(format!("{}", matrix), "[[1, 2], [3, 4]]");
        
        // 向量显示
        let vector = Expression::vector(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]).unwrap();
        assert_eq!(format!("{}", vector), "[1, 2, 3]");
        
        // 集合显示
        let set = Expression::set(vec![
            Expression::number(Number::integer(1)),
            Expression::number(Number::integer(2)),
            Expression::number(Number::integer(3)),
        ]);
        assert_eq!(format!("{}", set), "{1, 2, 3}");
        
        // 区间显示
        let closed_interval = Expression::interval(
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(10)),
            true,
            true
        );
        assert_eq!(format!("{}", closed_interval), "[0, 10]");
        
        let open_interval = Expression::interval(
            Expression::number(Number::integer(0)),
            Expression::number(Number::integer(10)),
            false,
            false
        );
        assert_eq!(format!("{}", open_interval), "(0, 10)");
    }

    #[test]
    fn test_function_display() {
        let x = Expression::variable("x");
        let y = Expression::variable("y");
        
        // 单参数函数
        let sin_func = Expression::function("sin", vec![x.clone()]);
        assert_eq!(format!("{}", sin_func), "sin(x)");
        
        // 多参数函数
        let max_func = Expression::function("max", vec![x.clone(), y.clone()]);
        assert_eq!(format!("{}", max_func), "max(x, y)");
        
        // 无参数函数
        let rand_func = Expression::function("random", vec![]);
        assert_eq!(format!("{}", rand_func), "random()");
    }
}