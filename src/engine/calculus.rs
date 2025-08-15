//! # 微积分引擎
//!
//! 实现符号求导、积分和其他微积分运算功能。

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
use super::ComputeError;
use num_bigint::BigInt;
use num_rational::BigRational;

/// 微积分运算引擎
pub struct CalculusEngine;

impl CalculusEngine {
    /// 创建新的微积分引擎
    pub fn new() -> Self {
        Self
    }
    
    /// 对表达式求导
    pub fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        match expr {
            // 常数的导数为 0
            Expression::Number(_) => Ok(Expression::Number(Number::Integer(BigInt::from(0)))),
            
            // 数学常量的导数为 0
            Expression::Constant(_) => Ok(Expression::Number(Number::Integer(BigInt::from(0)))),
            
            // 变量的导数
            Expression::Variable(name) => {
                if name == var {
                    // d/dx(x) = 1
                    Ok(Expression::Number(Number::Integer(BigInt::from(1))))
                } else {
                    // d/dx(y) = 0 (y 是其他变量)
                    Ok(Expression::Number(Number::Integer(BigInt::from(0))))
                }
            }
            
            // 二元运算的求导
            Expression::BinaryOp { op, left, right } => {
                self.differentiate_binary_op(op, left, right, var)
            }
            
            // 一元运算的求导
            Expression::UnaryOp { op, operand } => {
                self.differentiate_unary_op(op, operand, var)
            }
            
            // 函数的求导
            Expression::Function { name, args } => {
                self.differentiate_function(name, args, var)
            }
            
            // 其他类型暂不支持
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对 {:?} 类型求导", expr) 
            }),
        }
    }
    
    /// 对二元运算求导
    fn differentiate_binary_op(
        &self, 
        op: &BinaryOperator, 
        left: &Expression, 
        right: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        match op {
            // 加法法则: (u + v)' = u' + v'
            BinaryOperator::Add => {
                let left_diff = self.differentiate(left, var)?;
                let right_diff = self.differentiate(right, var)?;
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(left_diff),
                    right: Box::new(right_diff),
                })
            }
            
            // 减法法则: (u - v)' = u' - v'
            BinaryOperator::Subtract => {
                let left_diff = self.differentiate(left, var)?;
                let right_diff = self.differentiate(right, var)?;
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(left_diff),
                    right: Box::new(right_diff),
                })
            }
            
            // 乘法法则: (u * v)' = u' * v + u * v'
            BinaryOperator::Multiply => {
                let left_diff = self.differentiate(left, var)?;
                let right_diff = self.differentiate(right, var)?;
                
                let term1 = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(left_diff),
                    right: right.clone().into(),
                };
                
                let term2 = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: left.clone().into(),
                    right: Box::new(right_diff),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(term1),
                    right: Box::new(term2),
                })
            }
            
            // 除法法则: (u / v)' = (u' * v - u * v') / v^2
            BinaryOperator::Divide => {
                let left_diff = self.differentiate(left, var)?;
                let right_diff = self.differentiate(right, var)?;
                
                // u' * v
                let numerator_term1 = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(left_diff),
                    right: right.clone().into(),
                };
                
                // u * v'
                let numerator_term2 = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: left.clone().into(),
                    right: Box::new(right_diff),
                };
                
                // u' * v - u * v'
                let numerator = Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(numerator_term1),
                    right: Box::new(numerator_term2),
                };
                
                // v^2
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: right.clone().into(),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(numerator),
                    right: Box::new(denominator),
                })
            }
            
            // 幂函数法则: (u^v)' = u^v * (v' * ln(u) + v * u' / u)
            BinaryOperator::Power => {
                self.differentiate_power(left, right, var)
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对二元运算 {:?} 求导", op) 
            }),
        }
    }
    
    /// 对幂函数求导
    fn differentiate_power(
        &self, 
        base: &Expression, 
        exponent: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        // 检查指数是否为常数
        if self.is_constant_with_respect_to(exponent, var) {
            // 常数指数: (u^n)' = n * u^(n-1) * u'
            let base_diff = self.differentiate(base, var)?;
            
            // n - 1
            let new_exponent = Expression::BinaryOp {
                op: BinaryOperator::Subtract,
                left: exponent.clone().into(),
                right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
            };
            
            // u^(n-1)
            let power_term = Expression::BinaryOp {
                op: BinaryOperator::Power,
                left: base.clone().into(),
                right: Box::new(new_exponent),
            };
            
            // n * u^(n-1)
            let coeff_term = Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: exponent.clone().into(),
                right: Box::new(power_term),
            };
            
            // n * u^(n-1) * u'
            Ok(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(coeff_term),
                right: Box::new(base_diff),
            })
        } else if self.is_constant_with_respect_to(base, var) {
            // 常数底数: (a^u)' = a^u * ln(a) * u'
            let exponent_diff = self.differentiate(exponent, var)?;
            
            // a^u
            let power_term = Expression::BinaryOp {
                op: BinaryOperator::Power,
                left: base.clone().into(),
                right: exponent.clone().into(),
            };
            
            // ln(a)
            let ln_base = Expression::UnaryOp {
                op: UnaryOperator::Ln,
                operand: base.clone().into(),
            };
            
            // a^u * ln(a)
            let term1 = Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(power_term),
                right: Box::new(ln_base),
            };
            
            // a^u * ln(a) * u'
            Ok(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(term1),
                right: Box::new(exponent_diff),
            })
        } else {
            // 一般情况: (u^v)' = u^v * (v' * ln(u) + v * u' / u)
            let base_diff = self.differentiate(base, var)?;
            let exponent_diff = self.differentiate(exponent, var)?;
            
            // u^v
            let power_term = Expression::BinaryOp {
                op: BinaryOperator::Power,
                left: base.clone().into(),
                right: exponent.clone().into(),
            };
            
            // ln(u)
            let ln_base = Expression::UnaryOp {
                op: UnaryOperator::Ln,
                operand: base.clone().into(),
            };
            
            // v' * ln(u)
            let term1 = Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(exponent_diff),
                right: Box::new(ln_base),
            };
            
            // u' / u
            let term2_fraction = Expression::BinaryOp {
                op: BinaryOperator::Divide,
                left: Box::new(base_diff),
                right: base.clone().into(),
            };
            
            // v * u' / u
            let term2 = Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: exponent.clone().into(),
                right: Box::new(term2_fraction),
            };
            
            // v' * ln(u) + v * u' / u
            let bracket_term = Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            };
            
            // u^v * (v' * ln(u) + v * u' / u)
            Ok(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                left: Box::new(power_term),
                right: Box::new(bracket_term),
            })
        }
    }
    
    /// 对一元运算求导
    fn differentiate_unary_op(
        &self, 
        op: &UnaryOperator, 
        operand: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        let operand_diff = self.differentiate(operand, var)?;
        
        match op {
            // 负号: (-u)' = -u'
            UnaryOperator::Negate => {
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand_diff),
                })
            }
            
            // 正号: (+u)' = u'
            UnaryOperator::Plus => Ok(operand_diff),
            
            // 平方根: (√u)' = u' / (2√u)
            UnaryOperator::Sqrt => {
                let sqrt_operand = Expression::UnaryOp {
                    op: UnaryOperator::Sqrt,
                    operand: operand.clone().into(),
                };
                
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(sqrt_operand),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(operand_diff),
                    right: Box::new(denominator),
                })
            }
            
            // 绝对值: |u|' = u' * sign(u) (简化处理)
            UnaryOperator::Abs => {
                // 这里简化处理，实际应该考虑 u = 0 的情况
                let sign_u = Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: operand.clone().into(),
                    right: Box::new(Expression::UnaryOp {
                        op: UnaryOperator::Abs,
                        operand: operand.clone().into(),
                    }),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(operand_diff),
                    right: Box::new(sign_u),
                })
            }
            
            // 三角函数求导
            UnaryOperator::Sin => {
                // (sin u)' = cos u * u'
                let cos_operand = Expression::UnaryOp {
                    op: UnaryOperator::Cos,
                    operand: operand.clone().into(),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(cos_operand),
                    right: Box::new(operand_diff),
                })
            }
            
            UnaryOperator::Cos => {
                // (cos u)' = -sin u * u'
                let sin_operand = Expression::UnaryOp {
                    op: UnaryOperator::Sin,
                    operand: operand.clone().into(),
                };
                
                let neg_sin = Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(sin_operand),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(neg_sin),
                    right: Box::new(operand_diff),
                })
            }
            
            UnaryOperator::Tan => {
                // (tan u)' = sec^2 u * u' = u' / cos^2 u
                let cos_operand = Expression::UnaryOp {
                    op: UnaryOperator::Cos,
                    operand: operand.clone().into(),
                };
                
                let cos_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(cos_operand),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(operand_diff),
                    right: Box::new(cos_squared),
                })
            }
            
            // 对数函数求导
            UnaryOperator::Ln => {
                // (ln u)' = u' / u
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(operand_diff),
                    right: operand.clone().into(),
                })
            }
            
            UnaryOperator::Log10 => {
                // (log₁₀ u)' = u' / (u * ln(10))
                let ln_10 = Expression::UnaryOp {
                    op: UnaryOperator::Ln,
                    operand: Box::new(Expression::Number(Number::Integer(BigInt::from(10)))),
                };
                
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: operand.clone().into(),
                    right: Box::new(ln_10),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(operand_diff),
                    right: Box::new(denominator),
                })
            }
            
            // 指数函数求导
            UnaryOperator::Exp => {
                // (e^u)' = e^u * u'
                let exp_operand = Expression::UnaryOp {
                    op: UnaryOperator::Exp,
                    operand: operand.clone().into(),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(exp_operand),
                    right: Box::new(operand_diff),
                })
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对一元运算 {:?} 求导", op) 
            }),
        }
    }
    
    /// 对函数求导
    fn differentiate_function(
        &self, 
        name: &str, 
        args: &[Expression], 
        var: &str
    ) -> Result<Expression, ComputeError> {
        match name {
            // 对于函数调用，这里简化处理
            // 实际实现中应该支持更多内置函数
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对函数 {} 求导", name) 
            }),
        }
    }
    
    /// 检查表达式是否相对于给定变量为常数
    fn is_constant_with_respect_to(&self, expr: &Expression, var: &str) -> bool {
        match expr {
            Expression::Number(_) => true,
            Expression::Constant(_) => true,
            Expression::Variable(name) => name != var,
            Expression::BinaryOp { left, right, .. } => {
                self.is_constant_with_respect_to(left, var) && 
                self.is_constant_with_respect_to(right, var)
            }
            Expression::UnaryOp { operand, .. } => {
                self.is_constant_with_respect_to(operand, var)
            }
            Expression::Function { args, .. } => {
                args.iter().all(|arg| self.is_constant_with_respect_to(arg, var))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// 创建变量表达式
    fn var(name: &str) -> Expression {
        Expression::Variable(name.to_string())
    }
    
    /// 创建整数表达式
    fn int(value: i64) -> Expression {
        Expression::Number(Number::Integer(BigInt::from(value)))
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
        let engine = CalculusEngine::new();
        
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
        let engine = CalculusEngine::new();
        
        // x 对 x 的导数为 1
        let result = engine.differentiate(&var("x"), "x").unwrap();
        assert_eq!(result, int(1));
        
        // y 对 x 的导数为 0
        let result = engine.differentiate(&var("y"), "x").unwrap();
        assert_eq!(result, int(0));
    }
    
    #[test]
    fn test_differentiate_addition() {
        let engine = CalculusEngine::new();
        
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
    fn test_differentiate_power_constant_exponent() {
        let engine = CalculusEngine::new();
        
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
    fn test_differentiate_sin() {
        let engine = CalculusEngine::new();
        
        // sin(x) 的导数应该是 cos(x) * 1
        let sin_x = unop(UnaryOperator::Sin, var("x"));
        let result = engine.differentiate(&sin_x, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::UnaryOp { op: UnaryOperator::Cos, .. }, 
                     Expression::Number(Number::Integer(n))) => {
                        assert_eq!(n, &BigInt::from(1));
                    }
                    _ => panic!("期望得到 cos(x) * 1"),
                }
            }
            _ => panic!("期望得到乘法表达式"),
        }
    }
    
    #[test]
    fn test_differentiate_ln() {
        let engine = CalculusEngine::new();
        
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
    }
    
    #[test]
    fn test_is_constant_with_respect_to() {
        let engine = CalculusEngine::new();
        
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