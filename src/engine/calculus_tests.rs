//! # 微积分引擎测试
//!
//! 测试符号求导、积分和其他微积分运算功能。

#[cfg(test)]
mod tests {
    use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
    use crate::engine::calculus::CalculusEngine;
    use num_bigint::BigInt;
    use num_rational::BigRational;
    
    /// 创建测试用的微积分引擎
    fn create_engine() -> CalculusEngine {
        CalculusEngine::new()
    }
    
    /// 创建变量表达式
    fn var(name: &str) -> Expression {
        Expression::Variable(name.to_string())
    }
    
    /// 创建整数表达式
    fn int(value: i64) -> Expression {
        Expression::Number(Number::Integer(BigInt::from(value)))
    }
    
    /// 创建有理数表达式
    fn rational(num: i64, den: i64) -> Expression {
        Expression::Number(Number::Rational(BigRational::new(
            BigInt::from(num), 
            BigInt::from(den)
        )))
    }
    
    /// 创建二元运算表达式
    fn binop(op: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    /// 创建一元运算表达式
    fn unop(op: UnaryOperator, operand: Expression) -> Expression {
        Expression::UnaryOp {
            op,
            operand: Box::new(operand),
        }
    }
    
    #[test]
    fn test_differentiate_constants() {
        let engine = create_engine();
        
        // 常数的导数为 0
        let result = engine.differentiate(&int(5), "x").unwrap();
        assert_eq!(result, int(0));
        
        // 数学常量的导数为 0
        let pi = Expression::Constant(MathConstant::Pi);
        let result = engine.differentiate(&pi, "x").unwrap();
        assert_eq!(result, int(0));
    }
    
    #[test]
    fn test_differentiate_variables() {
        let engine = create_engine();
        
        // x 对 x 的导数为 1
        let result = engine.differentiate(&var("x"), "x").unwrap();
        assert_eq!(result, int(1));
        
        // y 对 x 的导数为 0
        let result = engine.differentiate(&var("y"), "x").unwrap();
        assert_eq!(result, int(0));
    }
    
    #[test]
    fn test_differentiate_addition() {
        let engine = create_engine();
        
        // (x + 5) 的导数应该是 1 + 0 = 1
        let expr = binop(BinaryOperator::Add, var("x"), int(5));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是 1 + 0
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, left, right } => {
                assert_eq!(*left, int(1));
                assert_eq!(*right, int(0));
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_subtraction() {
        let engine = create_engine();
        
        // (x - 3) 的导数应该是 1 - 0 = 1
        let expr = binop(BinaryOperator::Subtract, var("x"), int(3));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是 1 - 0
        match result {
            Expression::BinaryOp { op: BinaryOperator::Subtract, left, right } => {
                assert_eq!(*left, int(1));
                assert_eq!(*right, int(0));
            }
            _ => panic!("期望得到减法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_multiplication() {
        let engine = create_engine();
        
        // (x * 3) 的导数应该是 1 * 3 + x * 0 = 3
        let expr = binop(BinaryOperator::Multiply, var("x"), int(3));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是乘法法则的形式: u' * v + u * v'
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式（乘法法则）"),
        }
    }
    
    #[test]
    fn test_differentiate_division() {
        let engine = create_engine();
        
        // (x / 2) 的导数应该是 (1 * 2 - x * 0) / 2^2 = 2 / 4 = 1/2
        let expr = binop(BinaryOperator::Divide, var("x"), int(2));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是除法法则的形式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到除法表达式（除法法则）"),
        }
    }
    
    #[test]
    fn test_differentiate_power_constant_exponent() {
        let engine = create_engine();
        
        // x^2 的导数应该是 2 * x^1 * 1 = 2x
        let expr = binop(BinaryOperator::Power, var("x"), int(2));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是乘法表达式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到乘法表达式（幂函数法则）"),
        }
    }
    
    #[test]
    fn test_differentiate_power_variable_exponent() {
        let engine = create_engine();
        
        // x^x 的导数应该使用一般幂函数法则
        let expr = binop(BinaryOperator::Power, var("x"), var("x"));
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是乘法表达式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到乘法表达式（一般幂函数法则）"),
        }
    }
    
    #[test]
    fn test_differentiate_trigonometric_functions() {
        let engine = create_engine();
        
        // sin(x) 的导数应该是 cos(x) * 1
        let sin_x = unop(UnaryOperator::Sin, var("x"));
        let result = engine.differentiate(&sin_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                match (&**left, &**right) {
                    (Expression::UnaryOp { op: UnaryOperator::Cos, .. }, 
                     Expression::Number(Number::Integer(n))) => {
                        assert_eq!(*n, BigInt::from(1));
                    }
                    _ => panic!("期望得到 cos(x) * 1"),
                }
            }
            _ => panic!("期望得到乘法表达式"),
        }
        
        // cos(x) 的导数应该是 -sin(x) * 1
        let cos_x = unop(UnaryOperator::Cos, var("x"));
        let result = engine.differentiate(&cos_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                match (&**left, &**right) {
                    (Expression::UnaryOp { op: UnaryOperator::Negate, .. }, 
                     Expression::Number(Number::Integer(n))) => {
                        assert_eq!(*n, BigInt::from(1));
                    }
                    _ => panic!("期望得到 -sin(x) * 1"),
                }
            }
            _ => panic!("期望得到乘法表达式"),
        }
        
        // tan(x) 的导数应该是 1 / cos^2(x)
        let tan_x = unop(UnaryOperator::Tan, var("x"));
        let result = engine.differentiate(&tan_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到除法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_logarithmic_functions() {
        let engine = create_engine();
        
        // ln(x) 的导数应该是 1 / x
        let ln_x = unop(UnaryOperator::Ln, var("x"));
        let result = engine.differentiate(&ln_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                assert_eq!(*left, int(1));
                assert_eq!(*right, var("x"));
            }
            _ => panic!("期望得到 1/x"),
        }
        
        // log₁₀(x) 的导数应该是 1 / (x * ln(10))
        let log10_x = unop(UnaryOperator::Log10, var("x"));
        let result = engine.differentiate(&log10_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到除法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_exponential_function() {
        let engine = create_engine();
        
        // e^x 的导数应该是 e^x * 1
        let exp_x = unop(UnaryOperator::Exp, var("x"));
        let result = engine.differentiate(&exp_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                match (&**left, &**right) {
                    (Expression::UnaryOp { op: UnaryOperator::Exp, .. }, 
                     Expression::Number(Number::Integer(n))) => {
                        assert_eq!(*n, BigInt::from(1));
                    }
                    _ => panic!("期望得到 e^x * 1"),
                }
            }
            _ => panic!("期望得到乘法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_sqrt() {
        let engine = create_engine();
        
        // √x 的导数应该是 1 / (2√x)
        let sqrt_x = unop(UnaryOperator::Sqrt, var("x"));
        let result = engine.differentiate(&sqrt_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到除法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_chain_rule() {
        let engine = create_engine();
        
        // sin(x^2) 的导数应该是 cos(x^2) * 2x
        let x_squared = binop(BinaryOperator::Power, var("x"), int(2));
        let sin_x_squared = unop(UnaryOperator::Sin, x_squared);
        let result = engine.differentiate(&sin_x_squared, "x").unwrap();
        
        // 结果应该是乘法表达式（链式法则）
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到乘法表达式（链式法则）"),
        }
    }
    
    #[test]
    fn test_differentiate_complex_expression() {
        let engine = create_engine();
        
        // (x^2 + 3x + 1) 的导数应该是 2x + 3
        let x_squared = binop(BinaryOperator::Power, var("x"), int(2));
        let three_x = binop(BinaryOperator::Multiply, int(3), var("x"));
        let temp = binop(BinaryOperator::Add, x_squared, three_x);
        let expr = binop(BinaryOperator::Add, temp, int(1));
        
        let result = engine.differentiate(&expr, "x").unwrap();
        
        // 结果应该是加法表达式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_is_constant_with_respect_to() {
        let engine = create_engine();
        
        // 常数相对于任何变量都是常数
        assert!(engine.is_constant_with_respect_to(&int(5), "x"));
        
        // 数学常量相对于任何变量都是常数
        let pi = Expression::Constant(MathConstant::Pi);
        assert!(engine.is_constant_with_respect_to(&pi, "x"));
        
        // 变量 x 相对于 x 不是常数
        assert!(!engine.is_constant_with_respect_to(&var("x"), "x"));
        
        // 变量 y 相对于 x 是常数
        assert!(engine.is_constant_with_respect_to(&var("y"), "x"));
        
        // 包含变量 x 的表达式相对于 x 不是常数
        let expr = binop(BinaryOperator::Add, var("x"), int(1));
        assert!(!engine.is_constant_with_respect_to(&expr, "x"));
        
        // 不包含变量 x 的表达式相对于 x 是常数
        let expr = binop(BinaryOperator::Add, var("y"), int(1));
        assert!(engine.is_constant_with_respect_to(&expr, "x"));
    }
}