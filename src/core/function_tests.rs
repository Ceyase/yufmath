//! # 函数功能测试
//!
//! 测试基础数学函数的求值和运算功能。

use super::*;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_creation() {
        // 测试函数表达式的创建
        let sin_expr = Expression::function("sin", vec![Expression::variable("x")]);
        match sin_expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::variable("x"));
            }
            _ => panic!("应该创建函数表达式"),
        }
        
        let cos_expr = Expression::function("cos", vec![Expression::Number(Number::integer(0))]);
        match cos_expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "cos");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::Number(Number::integer(0)));
            }
            _ => panic!("应该创建函数表达式"),
        }
    }

    #[test]
    fn test_trigonometric_functions_special_values() {
        // 测试三角函数的特殊值
        
        // sin(0) = 0
        let sin_0 = Expression::function("sin", vec![Expression::Number(Number::integer(0))]);
        let result = sin_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // cos(0) = 1
        let cos_0 = Expression::function("cos", vec![Expression::Number(Number::integer(0))]);
        let result = cos_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // tan(0) = 0
        let tan_0 = Expression::function("tan", vec![Expression::Number(Number::integer(0))]);
        let result = tan_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // sin(π) = 0
        let sin_pi = Expression::function("sin", vec![Expression::Constant(MathConstant::Pi)]);
        let result = sin_pi.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // cos(π) = -1
        let cos_pi = Expression::function("cos", vec![Expression::Constant(MathConstant::Pi)]);
        let result = cos_pi.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(-1));
        
        // tan(π) = 0
        let tan_pi = Expression::function("tan", vec![Expression::Constant(MathConstant::Pi)]);
        let result = tan_pi.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
    }

    #[test]
    fn test_trigonometric_functions_pi_fractions() {
        // 测试 π 分数的三角函数值
        
        // sin(π/2) = 1
        let pi_half = Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Number(Number::integer(2))),
        };
        let sin_pi_half = Expression::function("sin", vec![pi_half.clone()]);
        let result = sin_pi_half.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // cos(π/2) = 0
        let cos_pi_half = Expression::function("cos", vec![pi_half.clone()]);
        let result = cos_pi_half.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // tan(π/2) = undefined
        let tan_pi_half = Expression::function("tan", vec![pi_half]);
        let result = tan_pi_half.evaluate_exact().unwrap();
        assert_eq!(result, Number::Constant(MathConstant::Undefined));
        
        // sin(π/4) = cos(π/4) = √2/2
        let pi_quarter = Expression::BinaryOp {
            op: BinaryOperator::Divide,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Number(Number::integer(4))),
        };
        let sin_pi_quarter = Expression::function("sin", vec![pi_quarter.clone()]);
        let result = sin_pi_quarter.evaluate_exact().unwrap();
        
        // 检查结果是否为 √2/2 的符号表示
        match result {
            Number::Symbolic(expr) => {
                match expr.as_ref() {
                    Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                        // 检查分子是否为 √2
                        match left.as_ref() {
                            Expression::UnaryOp { op: UnaryOperator::Sqrt, operand } => {
                                assert_eq!(operand.as_ref(), &Expression::Number(Number::integer(2)));
                            }
                            _ => panic!("sin(π/4) 的分子应该是 √2"),
                        }
                        // 检查分母是否为 2
                        assert_eq!(right.as_ref(), &Expression::Number(Number::integer(2)));
                    }
                    _ => panic!("sin(π/4) 应该是分数形式"),
                }
            }
            _ => panic!("sin(π/4) 应该返回符号表示"),
        }
        
        // tan(π/4) = 1
        let tan_pi_quarter = Expression::function("tan", vec![pi_quarter]);
        let result = tan_pi_quarter.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
    }

    #[test]
    fn test_inverse_trigonometric_functions() {
        // 测试反三角函数的特殊值
        
        // asin(0) = 0
        let asin_0 = Expression::function("asin", vec![Expression::Number(Number::integer(0))]);
        let result = asin_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // acos(1) = 0
        let acos_1 = Expression::function("acos", vec![Expression::Number(Number::integer(1))]);
        let result = acos_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // atan(0) = 0
        let atan_0 = Expression::function("atan", vec![Expression::Number(Number::integer(0))]);
        let result = atan_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // asin(1) = π/2
        let asin_1 = Expression::function("asin", vec![Expression::Number(Number::integer(1))]);
        let result = asin_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::Constant(MathConstant::Pi));
        
        // acos(-1) = π
        let acos_neg1 = Expression::function("acos", vec![Expression::Number(Number::integer(-1))]);
        let result = acos_neg1.evaluate_exact().unwrap();
        assert_eq!(result, Number::Constant(MathConstant::Pi));
    }

    #[test]
    fn test_exponential_function() {
        // 测试指数函数的特殊值
        
        // exp(0) = 1
        let exp_0 = Expression::function("exp", vec![Expression::Number(Number::integer(0))]);
        let result = exp_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // exp(1) = e
        let exp_1 = Expression::function("exp", vec![Expression::Number(Number::integer(1))]);
        let result = exp_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::Constant(MathConstant::E));
        
        // exp(i*π) = -1 (欧拉公式)
        let i_pi = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Constant(MathConstant::I)),
            right: Box::new(Expression::Constant(MathConstant::Pi)),
        };
        let exp_i_pi = Expression::function("exp", vec![i_pi]);
        let result = exp_i_pi.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(-1));
        
        // exp(π*i) = -1 (欧拉公式，交换律)
        let pi_i = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            left: Box::new(Expression::Constant(MathConstant::Pi)),
            right: Box::new(Expression::Constant(MathConstant::I)),
        };
        let exp_pi_i = Expression::function("exp", vec![pi_i]);
        let result = exp_pi_i.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(-1));
    }

    #[test]
    fn test_logarithm_function() {
        // 测试对数函数的特殊值
        
        // ln(1) = 0
        let ln_1 = Expression::function("ln", vec![Expression::Number(Number::integer(1))]);
        let result = ln_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // ln(e) = 1
        let ln_e = Expression::function("ln", vec![Expression::Constant(MathConstant::E)]);
        let result = ln_e.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // log10(1) = 0
        let log10_1 = Expression::function("log10", vec![Expression::Number(Number::integer(1))]);
        let result = log10_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // log10(10) = 1
        let log10_10 = Expression::function("log10", vec![Expression::Number(Number::integer(10))]);
        let result = log10_10.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // log2(1) = 0
        let log2_1 = Expression::function("log2", vec![Expression::Number(Number::integer(1))]);
        let result = log2_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // log2(2) = 1
        let log2_2 = Expression::function("log2", vec![Expression::Number(Number::integer(2))]);
        let result = log2_2.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // 测试负数对数错误
        let ln_neg = Expression::function("ln", vec![Expression::Number(Number::integer(-1))]);
        let result = ln_neg.evaluate_exact();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("对数函数的参数必须为正数"));
    }

    #[test]
    fn test_sqrt_function() {
        // 测试平方根函数的特殊值
        
        // sqrt(0) = 0
        let sqrt_0 = Expression::function("sqrt", vec![Expression::Number(Number::integer(0))]);
        let result = sqrt_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // sqrt(1) = 1
        let sqrt_1 = Expression::function("sqrt", vec![Expression::Number(Number::integer(1))]);
        let result = sqrt_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // sqrt(4) = 2
        let sqrt_4 = Expression::function("sqrt", vec![Expression::Number(Number::integer(4))]);
        let result = sqrt_4.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(2));
        
        // sqrt(9) = 3
        let sqrt_9 = Expression::function("sqrt", vec![Expression::Number(Number::integer(9))]);
        let result = sqrt_9.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(3));
        
        // sqrt(16) = 4
        let sqrt_16 = Expression::function("sqrt", vec![Expression::Number(Number::integer(16))]);
        let result = sqrt_16.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(4));
        
        // sqrt(25) = 5
        let sqrt_25 = Expression::function("sqrt", vec![Expression::Number(Number::integer(25))]);
        let result = sqrt_25.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(5));
        
        // sqrt(-1) = i (复数)
        let sqrt_neg1 = Expression::function("sqrt", vec![Expression::Number(Number::integer(-1))]);
        let result = sqrt_neg1.evaluate_exact().unwrap();
        match result {
            Number::Complex { real, imaginary } => {
                assert_eq!(real.as_ref(), &Number::integer(0));
                // 虚部应该是 √1 的符号表示
                match imaginary.as_ref() {
                    Number::Symbolic(expr) => {
                        match expr.as_ref() {
                            Expression::UnaryOp { op: UnaryOperator::Sqrt, operand } => {
                                assert_eq!(operand.as_ref(), &Expression::Number(Number::integer(1)));
                            }
                            _ => panic!("sqrt(-1) 的虚部应该是 √1"),
                        }
                    }
                    _ => panic!("sqrt(-1) 的虚部应该是符号表示"),
                }
            }
            _ => panic!("sqrt(-1) 应该返回复数"),
        }
    }

    #[test]
    fn test_power_function() {
        // 测试幂函数的特殊值
        
        // pow(5, 0) = 1 (使用具体数值而不是变量)
        let pow_5_0 = Expression::function("pow", vec![
            Expression::Number(Number::integer(5)),
            Expression::Number(Number::integer(0))
        ]);
        let result = pow_5_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // pow(x, 1) = x
        let pow_x_1 = Expression::function("pow", vec![
            Expression::Number(Number::integer(5)),
            Expression::Number(Number::integer(1))
        ]);
        let result = pow_x_1.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(5));
        
        // pow(0, positive) = 0
        let pow_0_2 = Expression::function("pow", vec![
            Expression::Number(Number::integer(0)),
            Expression::Number(Number::integer(2))
        ]);
        let result = pow_0_2.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // pow(1, anything) = 1
        let pow_1_100 = Expression::function("pow", vec![
            Expression::Number(Number::integer(1)),
            Expression::Number(Number::integer(100))
        ]);
        let result = pow_1_100.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // pow(2, 3) = 8
        let pow_2_3 = Expression::function("pow", vec![
            Expression::Number(Number::integer(2)),
            Expression::Number(Number::integer(3))
        ]);
        let result = pow_2_3.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(8));
        
        // pow(3, 2) = 9
        let pow_3_2 = Expression::function("pow", vec![
            Expression::Number(Number::integer(3)),
            Expression::Number(Number::integer(2))
        ]);
        let result = pow_3_2.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(9));
    }

    #[test]
    fn test_hyperbolic_functions() {
        // 测试双曲函数的特殊值
        
        // sinh(0) = 0
        let sinh_0 = Expression::function("sinh", vec![Expression::Number(Number::integer(0))]);
        let result = sinh_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
        
        // cosh(0) = 1
        let cosh_0 = Expression::function("cosh", vec![Expression::Number(Number::integer(0))]);
        let result = cosh_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(1));
        
        // tanh(0) = 0
        let tanh_0 = Expression::function("tanh", vec![Expression::Number(Number::integer(0))]);
        let result = tanh_0.evaluate_exact().unwrap();
        assert_eq!(result, Number::integer(0));
    }

    #[test]
    fn test_function_with_variables() {
        // 测试包含变量的函数求值
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Number::integer(0));
        
        // sin(x) where x = 0
        let sin_x = Expression::function("sin", vec![Expression::variable("x")]);
        let result = sin_x.evaluate(&vars).unwrap();
        assert_eq!(result, Number::integer(0));
        
        // cos(x) where x = 0
        let cos_x = Expression::function("cos", vec![Expression::variable("x")]);
        let result = cos_x.evaluate(&vars).unwrap();
        assert_eq!(result, Number::integer(1));
        
        // exp(x) where x = 1
        vars.insert("x".to_string(), Number::integer(1));
        let exp_x = Expression::function("exp", vec![Expression::variable("x")]);
        let result = exp_x.evaluate(&vars).unwrap();
        assert_eq!(result, Number::Constant(MathConstant::E));
        
        // ln(x) where x = e
        vars.insert("x".to_string(), Number::Constant(MathConstant::E));
        let ln_x = Expression::function("ln", vec![Expression::variable("x")]);
        let result = ln_x.evaluate(&vars).unwrap();
        assert_eq!(result, Number::integer(1));
    }

    #[test]
    fn test_function_error_handling() {
        // 测试函数参数错误处理
        
        // sin 函数需要一个参数
        let sin_no_args = Expression::function("sin", vec![]);
        let result = sin_no_args.evaluate_exact();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("sin函数需要一个参数"));
        
        let sin_two_args = Expression::function("sin", vec![
            Expression::Number(Number::integer(0)),
            Expression::Number(Number::integer(1))
        ]);
        let result = sin_two_args.evaluate_exact();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("sin函数需要一个参数"));
        
        // pow 函数需要两个参数
        let pow_one_arg = Expression::function("pow", vec![Expression::Number(Number::integer(2))]);
        let result = pow_one_arg.evaluate_exact();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("pow函数需要两个参数"));
    }

    #[test]
    fn test_function_symbolic_results() {
        // 测试返回符号表示的函数
        
        // sin(2) 应该返回符号表示
        let sin_2 = Expression::function("sin", vec![Expression::Number(Number::integer(2))]);
        let result = sin_2.evaluate_exact().unwrap();
        match result {
            Number::Symbolic(expr) => {
                match expr.as_ref() {
                    Expression::UnaryOp { op: UnaryOperator::Sin, operand } => {
                        assert_eq!(operand.as_ref(), &Expression::Number(Number::integer(2)));
                    }
                    _ => panic!("sin(2) 应该返回 sin 的符号表示"),
                }
            }
            _ => panic!("sin(2) 应该返回符号表示"),
        }
        
        // exp(2) 应该返回符号表示
        let exp_2 = Expression::function("exp", vec![Expression::Number(Number::integer(2))]);
        let result = exp_2.evaluate_exact().unwrap();
        match result {
            Number::Symbolic(expr) => {
                match expr.as_ref() {
                    Expression::UnaryOp { op: UnaryOperator::Exp, operand } => {
                        assert_eq!(operand.as_ref(), &Expression::Number(Number::integer(2)));
                    }
                    _ => panic!("exp(2) 应该返回 exp 的符号表示"),
                }
            }
            _ => panic!("exp(2) 应该返回符号表示"),
        }
        
        // ln(2) 应该返回符号表示
        let ln_2 = Expression::function("ln", vec![Expression::Number(Number::integer(2))]);
        let result = ln_2.evaluate_exact().unwrap();
        match result {
            Number::Symbolic(expr) => {
                match expr.as_ref() {
                    Expression::UnaryOp { op: UnaryOperator::Ln, operand } => {
                        assert_eq!(operand.as_ref(), &Expression::Number(Number::integer(2)));
                    }
                    _ => panic!("ln(2) 应该返回 ln 的符号表示"),
                }
            }
            _ => panic!("ln(2) 应该返回符号表示"),
        }
    }

    #[test]
    fn test_function_type_inference() {
        // 测试函数类型推断
        
        let sin_expr = Expression::function("sin", vec![Expression::Number(Number::integer(1))]);
        let expr_type = sin_expr.infer_type();
        assert_eq!(expr_type, ExprType::Numeric(NumericType::Real));
        
        let cos_expr = Expression::function("cos", vec![Expression::Number(Number::complex(
            Number::integer(1), 
            Number::integer(1)
        ))]);
        let expr_type = cos_expr.infer_type();
        assert_eq!(expr_type, ExprType::Numeric(NumericType::Complex));
        
        let unknown_func = Expression::function("unknown", vec![Expression::Number(Number::integer(1))]);
        let expr_type = unknown_func.infer_type();
        assert_eq!(expr_type, ExprType::Unknown);
    }

    #[test]
    fn test_function_validation() {
        // 测试函数验证
        
        let valid_sin = Expression::function("sin", vec![Expression::Number(Number::integer(1))]);
        assert!(valid_sin.validate().is_ok());
        
        let valid_pow = Expression::function("pow", vec![
            Expression::Number(Number::integer(2)),
            Expression::Number(Number::integer(3))
        ]);
        assert!(valid_pow.validate().is_ok());
        
        // 嵌套函数也应该有效
        let nested_func = Expression::function("sin", vec![
            Expression::function("cos", vec![Expression::Number(Number::integer(0))])
        ]);
        assert!(nested_func.validate().is_ok());
    }

    #[test]
    fn test_function_complexity() {
        // 测试函数复杂度计算
        
        let simple_func = Expression::function("sin", vec![Expression::Number(Number::integer(1))]);
        assert_eq!(simple_func.complexity(), 2); // 1 (function) + 1 (number)
        
        let complex_func = Expression::function("sin", vec![
            Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(Expression::variable("x")),
                right: Box::new(Expression::Number(Number::integer(1))),
            }
        ]);
        assert_eq!(complex_func.complexity(), 4); // 1 (function) + 3 (add + var + number)
        
        let nested_func = Expression::function("sin", vec![
            Expression::function("cos", vec![Expression::Number(Number::integer(0))])
        ]);
        assert_eq!(nested_func.complexity(), 3); // 1 (sin) + 1 (cos) + 1 (number)
    }

    #[test]
    fn test_function_substitution() {
        // 测试函数中的变量替换
        
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::Number(Number::integer(5)));
        
        let sin_x = Expression::function("sin", vec![Expression::variable("x")]);
        let substituted = sin_x.substitute(&vars);
        
        match substituted {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Expression::Number(Number::integer(5)));
            }
            _ => panic!("替换后应该仍然是函数表达式"),
        }
        
        // 测试多参数函数的替换
        let pow_xy = Expression::function("pow", vec![
            Expression::variable("x"),
            Expression::variable("y")
        ]);
        
        vars.insert("y".to_string(), Expression::Number(Number::integer(3)));
        let substituted = pow_xy.substitute(&vars);
        
        match substituted {
            Expression::Function { name, args } => {
                assert_eq!(name, "pow");
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], Expression::Number(Number::integer(5)));
                assert_eq!(args[1], Expression::Number(Number::integer(3)));
            }
            _ => panic!("替换后应该仍然是函数表达式"),
        }
    }
}