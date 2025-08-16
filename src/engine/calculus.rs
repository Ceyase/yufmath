//! # 微积分引擎
//!
//! 实现符号求导、积分和其他微积分运算功能。

use crate::core::{Expression, Number, BinaryOperator, UnaryOperator, MathConstant};
use super::ComputeError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

/// 微积分运算引擎
pub struct CalculusEngine;

impl CalculusEngine {
    /// 创建新的微积分引擎
    pub fn new() -> Self {
        Self
    }
    
    /// 计算极限
    pub fn limit(&self, expr: &Expression, var: &str, point: &Expression) -> Result<Expression, ComputeError> {
        // 这是一个简化的极限计算实现
        // 实际的极限计算需要更复杂的算法
        
        // 检查特殊的极限情况
        match expr {
            // sin(x)/x 当 x -> 0 时的极限为 1
            Expression::BinaryOp { 
                op: BinaryOperator::Divide, 
                left, 
                right 
            } => {
                if let (
                    Expression::UnaryOp { op: UnaryOperator::Sin, operand: sin_operand },
                    Expression::Variable(var_name)
                ) = (left.as_ref(), right.as_ref()) {
                    if var_name == var && sin_operand.as_ref() == &Expression::Variable(var.to_string()) {
                        if let Expression::Number(Number::Integer(n)) = point {
                            if n == &BigInt::from(0) {
                                // lim(x->0) sin(x)/x = 1
                                return Ok(Expression::Number(Number::Integer(BigInt::from(1))));
                            }
                        }
                    }
                }
                
                // (1-cos(x))/x 当 x -> 0 时的极限为 0
                if let (
                    Expression::BinaryOp { 
                        op: BinaryOperator::Subtract, 
                        left: one, 
                        right: cos_expr 
                    },
                    Expression::Variable(var_name)
                ) = (left.as_ref(), right.as_ref()) {
                    if var_name == var {
                        if let (
                            Expression::Number(Number::Integer(n)),
                            Expression::UnaryOp { op: UnaryOperator::Cos, operand: cos_operand }
                        ) = (one.as_ref(), cos_expr.as_ref()) {
                            if n == &BigInt::from(1) && cos_operand.as_ref() == &Expression::Variable(var.to_string()) {
                                if let Expression::Number(Number::Integer(point_val)) = point {
                                    if point_val == &BigInt::from(0) {
                                        // lim(x->0) (1-cos(x))/x = 0
                                        return Ok(Expression::Number(Number::Integer(BigInt::from(0))));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 其他情况尝试直接代入
            _ => {
                // 对于简单情况，尝试直接代入
                if let Ok(result) = self.substitute_and_evaluate(expr, var, point) {
                    return Ok(result);
                }
            }
        }
        
        Err(ComputeError::UnsupportedOperation { 
            operation: format!("计算极限 lim({} -> {}) {}", var, self.format_expression(point), self.format_expression(expr))
        })
    }
    
    /// 级数展开
    pub fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) -> Result<Expression, ComputeError> {
        // 这是一个简化的泰勒级数展开实现
        // 实际的级数展开需要更复杂的算法
        
        match expr {
            // e^x 的泰勒级数：e^x = 1 + x + x²/2! + x³/3! + ...
            Expression::UnaryOp { op: UnaryOperator::Exp, operand } => {
                if let Expression::Variable(operand_var) = operand.as_ref() {
                    if operand_var == var {
                        if let Expression::Number(Number::Integer(n)) = point {
                            if n == &BigInt::from(0) {
                                // 在 x = 0 处展开 e^x
                                return self.exp_series_at_zero(var, order);
                            }
                        }
                    }
                }
            }
            
            // sin(x) 的泰勒级数：sin(x) = x - x³/3! + x⁵/5! - ...
            Expression::UnaryOp { op: UnaryOperator::Sin, operand } => {
                if let Expression::Variable(operand_var) = operand.as_ref() {
                    if operand_var == var {
                        if let Expression::Number(Number::Integer(n)) = point {
                            if n == &BigInt::from(0) {
                                // 在 x = 0 处展开 sin(x)
                                return self.sin_series_at_zero(var, order);
                            }
                        }
                    }
                }
            }
            
            // cos(x) 的泰勒级数：cos(x) = 1 - x²/2! + x⁴/4! - ...
            Expression::UnaryOp { op: UnaryOperator::Cos, operand } => {
                if let Expression::Variable(operand_var) = operand.as_ref() {
                    if operand_var == var {
                        if let Expression::Number(Number::Integer(n)) = point {
                            if n == &BigInt::from(0) {
                                // 在 x = 0 处展开 cos(x)
                                return self.cos_series_at_zero(var, order);
                            }
                        }
                    }
                }
            }
            
            _ => {}
        }
        
        Err(ComputeError::UnsupportedOperation { 
            operation: format!("级数展开 {} 在 {} = {} 处", self.format_expression(expr), var, self.format_expression(point))
        })
    }
    
    /// 数值计算
    pub fn numerical_evaluate(&self, expr: &Expression, vars: &std::collections::HashMap<String, f64>) -> Result<f64, ComputeError> {
        match expr {
            Expression::Number(n) => {
                match n {
                    Number::Integer(i) => Ok(i.to_f64().unwrap_or(f64::NAN)),
                    Number::Rational(r) => Ok(r.to_f64().unwrap_or(f64::NAN)),
                    Number::Real(_) => Ok(0.0), // 简化处理
                    Number::Complex { .. } => Err(ComputeError::UnsupportedOperation { 
                        operation: "复数的数值计算".to_string() 
                    }),
                    Number::Symbolic(_) => Err(ComputeError::UnsupportedOperation { 
                        operation: "符号数值的数值计算".to_string() 
                    }),
                    Number::Float(f) => Ok(*f),
                    Number::Constant(c) => {
                        // 处理嵌套的数学常量
                        match c {
                            MathConstant::Pi => Ok(std::f64::consts::PI),
                            MathConstant::E => Ok(std::f64::consts::E),
                            MathConstant::I => Err(ComputeError::UnsupportedOperation { 
                                operation: "虚数单位的数值计算".to_string() 
                            }),
                            MathConstant::EulerGamma => Ok(0.5772156649015329),
                            MathConstant::GoldenRatio => Ok(1.618033988749895),
                            MathConstant::Catalan => Ok(0.915965594177219),
                            MathConstant::PositiveInfinity => Ok(f64::INFINITY),
                            MathConstant::NegativeInfinity => Ok(f64::NEG_INFINITY),
                            MathConstant::Undefined => Ok(f64::NAN),
                        }
                    }
                }
            }
            
            Expression::Variable(name) => {
                vars.get(name).copied().ok_or_else(|| ComputeError::UndefinedVariable { 
                    name: name.clone() 
                })
            }
            
            Expression::Constant(c) => {
                match c {
                    MathConstant::Pi => Ok(std::f64::consts::PI),
                    MathConstant::E => Ok(std::f64::consts::E),
                    MathConstant::I => Err(ComputeError::UnsupportedOperation { 
                        operation: "虚数单位的数值计算".to_string() 
                    }),
                    MathConstant::EulerGamma => Ok(0.5772156649015329),
                    MathConstant::GoldenRatio => Ok(1.618033988749895),
                    MathConstant::Catalan => Ok(0.915965594177219),
                    MathConstant::PositiveInfinity => Ok(f64::INFINITY),
                    MathConstant::NegativeInfinity => Ok(f64::NEG_INFINITY),
                    MathConstant::Undefined => Ok(f64::NAN),
                }
            }
            
            Expression::BinaryOp { op, left, right } => {
                let left_val = self.numerical_evaluate(left, vars)?;
                let right_val = self.numerical_evaluate(right, vars)?;
                
                match op {
                    BinaryOperator::Add => Ok(left_val + right_val),
                    BinaryOperator::Subtract => Ok(left_val - right_val),
                    BinaryOperator::Multiply => Ok(left_val * right_val),
                    BinaryOperator::Divide => {
                        if right_val == 0.0 {
                            Err(ComputeError::DivisionByZero)
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    BinaryOperator::Power => Ok(left_val.powf(right_val)),
                    _ => Err(ComputeError::UnsupportedOperation { 
                        operation: format!("数值计算二元运算 {:?}", op) 
                    }),
                }
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_val = self.numerical_evaluate(operand, vars)?;
                
                match op {
                    UnaryOperator::Negate => Ok(-operand_val),
                    UnaryOperator::Plus => Ok(operand_val),
                    UnaryOperator::Sqrt => Ok(operand_val.sqrt()),
                    UnaryOperator::Abs => Ok(operand_val.abs()),
                    UnaryOperator::Sin => Ok(operand_val.sin()),
                    UnaryOperator::Cos => Ok(operand_val.cos()),
                    UnaryOperator::Tan => Ok(operand_val.tan()),
                    UnaryOperator::Asin => Ok(operand_val.asin()),
                    UnaryOperator::Acos => Ok(operand_val.acos()),
                    UnaryOperator::Atan => Ok(operand_val.atan()),
                    UnaryOperator::Sinh => Ok(operand_val.sinh()),
                    UnaryOperator::Cosh => Ok(operand_val.cosh()),
                    UnaryOperator::Tanh => Ok(operand_val.tanh()),
                    UnaryOperator::Asinh => Ok(operand_val.asinh()),
                    UnaryOperator::Acosh => Ok(operand_val.acosh()),
                    UnaryOperator::Atanh => Ok(operand_val.atanh()),
                    UnaryOperator::Ln => Ok(operand_val.ln()),
                    UnaryOperator::Log10 => Ok(operand_val.log10()),
                    UnaryOperator::Log2 => Ok(operand_val.log2()),
                    UnaryOperator::Exp => Ok(operand_val.exp()),
                    UnaryOperator::Factorial => {
                        if operand_val >= 0.0 && operand_val.fract() == 0.0 {
                            let n = operand_val as u32;
                            if n <= 170 { // 避免溢出
                                Ok(self.factorial(n) as f64)
                            } else {
                                Ok(f64::INFINITY)
                            }
                        } else {
                            Err(ComputeError::UnsupportedOperation { 
                                operation: "非整数的阶乘".to_string() 
                            })
                        }
                    }
                    _ => Err(ComputeError::UnsupportedOperation { 
                        operation: format!("数值计算一元运算 {:?}", op) 
                    }),
                }
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("数值计算 {:?} 类型", expr) 
            }),
        }
    }
    
    /// 对表达式积分
    pub fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, ComputeError> {
        match expr {
            // 常数的积分：∫c dx = cx + C
            Expression::Number(n) => {
                if n == &Number::Integer(BigInt::from(0)) {
                    // ∫0 dx = C (这里简化为 0，实际应该包含积分常数)
                    Ok(Expression::Number(Number::Integer(BigInt::from(0))))
                } else {
                    // ∫c dx = cx
                    Ok(Expression::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(expr.clone()),
                        right: Box::new(Expression::Variable(var.to_string())),
                    })
                }
            }
            
            // 数学常量的积分
            Expression::Constant(_) => {
                // ∫c dx = cx
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(expr.clone()),
                    right: Box::new(Expression::Variable(var.to_string())),
                })
            }
            
            // 变量的积分
            Expression::Variable(name) => {
                if name == var {
                    // ∫x dx = x²/2
                    let x_squared = Expression::BinaryOp {
                        op: BinaryOperator::Power,
                        left: Box::new(Expression::Variable(var.to_string())),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    };
                    
                    Ok(Expression::BinaryOp {
                        op: BinaryOperator::Divide,
                        left: Box::new(x_squared),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    })
                } else {
                    // ∫y dx = yx (y 是其他变量，视为常数)
                    Ok(Expression::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(expr.clone()),
                        right: Box::new(Expression::Variable(var.to_string())),
                    })
                }
            }
            
            // 二元运算的积分
            Expression::BinaryOp { op, left, right } => {
                self.integrate_binary_op(op, left, right, var)
            }
            
            // 一元运算的积分
            Expression::UnaryOp { op, operand } => {
                self.integrate_unary_op(op, operand, var)
            }
            
            // 函数的积分
            Expression::Function { name, args } => {
                self.integrate_function(name, args, var)
            }
            
            // 其他类型暂不支持
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对 {:?} 类型积分", expr) 
            }),
        }
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
    
    /// 对二元运算积分
    fn integrate_binary_op(
        &self, 
        op: &BinaryOperator, 
        left: &Expression, 
        right: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        match op {
            // 加法法则: ∫(u + v) dx = ∫u dx + ∫v dx
            BinaryOperator::Add => {
                let left_integral = self.integrate(left, var)?;
                let right_integral = self.integrate(right, var)?;
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(left_integral),
                    right: Box::new(right_integral),
                })
            }
            
            // 减法法则: ∫(u - v) dx = ∫u dx - ∫v dx
            BinaryOperator::Subtract => {
                let left_integral = self.integrate(left, var)?;
                let right_integral = self.integrate(right, var)?;
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(left_integral),
                    right: Box::new(right_integral),
                })
            }
            
            // 常数乘法法则: ∫(c * u) dx = c * ∫u dx (当 c 是常数时)
            BinaryOperator::Multiply => {
                if self.is_constant_with_respect_to(left, var) {
                    // c * u 的形式
                    let right_integral = self.integrate(right, var)?;
                    Ok(Expression::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: left.clone().into(),
                        right: Box::new(right_integral),
                    })
                } else if self.is_constant_with_respect_to(right, var) {
                    // u * c 的形式
                    let left_integral = self.integrate(left, var)?;
                    Ok(Expression::BinaryOp {
                        op: BinaryOperator::Multiply,
                        left: Box::new(left_integral),
                        right: right.clone().into(),
                    })
                } else {
                    // 一般乘法积分比较复杂，这里暂不支持
                    Err(ComputeError::UnsupportedOperation { 
                        operation: "一般乘法积分暂不支持".to_string() 
                    })
                }
            }
            
            // 幂函数积分: ∫x^n dx = x^(n+1)/(n+1) (n ≠ -1)
            BinaryOperator::Power => {
                self.integrate_power(left, right, var)
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对二元运算 {:?} 积分", op) 
            }),
        }
    }
    
    /// 对幂函数积分
    fn integrate_power(
        &self, 
        base: &Expression, 
        exponent: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        // 检查是否为 x^n 的形式
        if let Expression::Variable(base_var) = base {
            if base_var == var && self.is_constant_with_respect_to(exponent, var) {
                // ∫x^n dx = x^(n+1)/(n+1) (n ≠ -1)
                
                // 检查是否为 x^(-1) = 1/x 的情况
                if let Expression::Number(Number::Integer(n)) = exponent {
                    if n == &BigInt::from(-1) {
                        // ∫x^(-1) dx = ∫(1/x) dx = ln|x|
                        return Ok(Expression::UnaryOp {
                            op: UnaryOperator::Ln,
                            operand: Box::new(Expression::UnaryOp {
                                op: UnaryOperator::Abs,
                                operand: Box::new(Expression::Variable(var.to_string())),
                            }),
                        });
                    }
                }
                
                // n + 1
                let new_exponent = Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: exponent.clone().into(),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                };
                
                // x^(n+1)
                let power_term = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(Expression::Variable(var.to_string())),
                    right: Box::new(new_exponent.clone()),
                };
                
                // x^(n+1) / (n+1)
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(power_term),
                    right: Box::new(new_exponent),
                })
            } else {
                Err(ComputeError::UnsupportedOperation { 
                    operation: "复杂幂函数积分暂不支持".to_string() 
                })
            }
        } else {
            Err(ComputeError::UnsupportedOperation { 
                operation: "复杂幂函数积分暂不支持".to_string() 
            })
        }
    }
    
    /// 对一元运算积分
    fn integrate_unary_op(
        &self, 
        op: &UnaryOperator, 
        operand: &Expression, 
        var: &str
    ) -> Result<Expression, ComputeError> {
        match op {
            // 负号: ∫(-u) dx = -∫u dx
            UnaryOperator::Negate => {
                let operand_integral = self.integrate(operand, var)?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(operand_integral),
                })
            }
            
            // 正号: ∫(+u) dx = ∫u dx
            UnaryOperator::Plus => self.integrate(operand, var),
            
            // 三角函数积分
            UnaryOperator::Sin => {
                // ∫sin(x) dx = -cos(x)
                if let Expression::Variable(operand_var) = operand {
                    if operand_var == var {
                        return Ok(Expression::UnaryOp {
                            op: UnaryOperator::Negate,
                            operand: Box::new(Expression::UnaryOp {
                                op: UnaryOperator::Cos,
                                operand: Box::new(Expression::Variable(var.to_string())),
                            }),
                        });
                    }
                }
                Err(ComputeError::UnsupportedOperation { 
                    operation: "复合三角函数积分暂不支持".to_string() 
                })
            }
            
            UnaryOperator::Cos => {
                // ∫cos(x) dx = sin(x)
                if let Expression::Variable(operand_var) = operand {
                    if operand_var == var {
                        return Ok(Expression::UnaryOp {
                            op: UnaryOperator::Sin,
                            operand: Box::new(Expression::Variable(var.to_string())),
                        });
                    }
                }
                Err(ComputeError::UnsupportedOperation { 
                    operation: "复合三角函数积分暂不支持".to_string() 
                })
            }
            
            // 指数函数积分
            UnaryOperator::Exp => {
                // ∫e^x dx = e^x
                if let Expression::Variable(operand_var) = operand {
                    if operand_var == var {
                        return Ok(Expression::UnaryOp {
                            op: UnaryOperator::Exp,
                            operand: Box::new(Expression::Variable(var.to_string())),
                        });
                    }
                }
                Err(ComputeError::UnsupportedOperation { 
                    operation: "复合指数函数积分暂不支持".to_string() 
                })
            }
            
            // 1/x 的积分
            UnaryOperator::Ln => {
                Err(ComputeError::UnsupportedOperation { 
                    operation: "ln 函数的积分需要分部积分".to_string() 
                })
            }
            
            _ => Err(ComputeError::UnsupportedOperation { 
                operation: format!("对一元运算 {:?} 积分", op) 
            }),
        }
    }
    
    /// 对函数积分
    fn integrate_function(
        &self, 
        name: &str, 
        args: &[Expression], 
        var: &str
    ) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Err(ComputeError::UnsupportedOperation { 
                operation: format!("函数 {} 的多参数积分暂不支持", name) 
            });
        }
        
        let arg = &args[0];
        
        // 检查参数是否为简单变量
        if let Expression::Variable(arg_var) = arg {
            if arg_var == var {
                // 简单情况：函数参数就是积分变量
                match name {
                    // 三角函数积分
                    "sin" => {
                        // ∫sin(x) dx = -cos(x)
                        Ok(Expression::UnaryOp {
                            op: UnaryOperator::Negate,
                            operand: Box::new(Expression::Function {
                                name: "cos".to_string(),
                                args: vec![arg.clone()],
                            }),
                        })
                    }
                    
                    "cos" => {
                        // ∫cos(x) dx = sin(x)
                        Ok(Expression::Function {
                            name: "sin".to_string(),
                            args: vec![arg.clone()],
                        })
                    }
                    
                    "tan" => {
                        // ∫tan(x) dx = -ln|cos(x)|
                        let cos_x = Expression::Function {
                            name: "cos".to_string(),
                            args: vec![arg.clone()],
                        };
                        
                        let abs_cos = Expression::Function {
                            name: "abs".to_string(),
                            args: vec![cos_x],
                        };
                        
                        let ln_abs_cos = Expression::Function {
                            name: "ln".to_string(),
                            args: vec![abs_cos],
                        };
                        
                        Ok(Expression::UnaryOp {
                            op: UnaryOperator::Negate,
                            operand: Box::new(ln_abs_cos),
                        })
                    }
                    
                    // 双曲函数积分
                    "sinh" => {
                        // ∫sinh(x) dx = cosh(x)
                        Ok(Expression::Function {
                            name: "cosh".to_string(),
                            args: vec![arg.clone()],
                        })
                    }
                    
                    "cosh" => {
                        // ∫cosh(x) dx = sinh(x)
                        Ok(Expression::Function {
                            name: "sinh".to_string(),
                            args: vec![arg.clone()],
                        })
                    }
                    
                    // 指数函数积分
                    "exp" => {
                        // ∫e^x dx = e^x
                        Ok(Expression::Function {
                            name: "exp".to_string(),
                            args: vec![arg.clone()],
                        })
                    }
                    
                    // 对数函数积分（部分积分）
                    "ln" | "log" => {
                        // ∫ln(x) dx = x*ln(x) - x
                        let x_ln_x = Expression::BinaryOp {
                            op: BinaryOperator::Multiply,
                            left: Box::new(arg.clone()),
                            right: Box::new(Expression::Function {
                                name: "ln".to_string(),
                                args: vec![arg.clone()],
                            }),
                        };
                        
                        Ok(Expression::BinaryOp {
                            op: BinaryOperator::Subtract,
                            left: Box::new(x_ln_x),
                            right: Box::new(arg.clone()),
                        })
                    }
                    
                    // 平方根函数积分
                    "sqrt" => {
                        // ∫√x dx = (2/3)x^(3/2)
                        let three_halves = Expression::BinaryOp {
                            op: BinaryOperator::Divide,
                            left: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
                            right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                        };
                        
                        let x_power_three_halves = Expression::BinaryOp {
                            op: BinaryOperator::Power,
                            left: Box::new(arg.clone()),
                            right: Box::new(three_halves),
                        };
                        
                        let two_thirds = Expression::BinaryOp {
                            op: BinaryOperator::Divide,
                            left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                            right: Box::new(Expression::Number(Number::Integer(BigInt::from(3)))),
                        };
                        
                        Ok(Expression::BinaryOp {
                            op: BinaryOperator::Multiply,
                            left: Box::new(two_thirds),
                            right: Box::new(x_power_three_halves),
                        })
                    }
                    
                    _ => Err(ComputeError::UnsupportedOperation { 
                        operation: format!("对函数 {} 积分", name) 
                    }),
                }
            } else {
                // 参数是其他变量，视为常数
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(Expression::Function {
                        name: name.to_string(),
                        args: args.to_vec(),
                    }),
                    right: Box::new(Expression::Variable(var.to_string())),
                })
            }
        } else {
            // 复杂参数的情况，需要使用换元积分法
            Err(ComputeError::UnsupportedOperation { 
                operation: format!("对复杂参数函数 {} 积分", name) 
            })
        }
    }
    
    /// 对函数求导
    fn differentiate_function(
        &self, 
        name: &str, 
        args: &[Expression], 
        var: &str
    ) -> Result<Expression, ComputeError> {
        if args.len() != 1 {
            return Err(ComputeError::UnsupportedOperation { 
                operation: format!("函数 {} 的多参数求导暂不支持", name) 
            });
        }
        
        let arg = &args[0];
        let arg_diff = self.differentiate(arg, var)?;
        
        match name {
            // 三角函数求导
            "sin" => {
                // (sin u)' = cos u * u'
                let cos_arg = Expression::Function {
                    name: "cos".to_string(),
                    args: vec![arg.clone()],
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(cos_arg),
                    right: Box::new(arg_diff),
                })
            }
            
            "cos" => {
                // (cos u)' = -sin u * u'
                let sin_arg = Expression::Function {
                    name: "sin".to_string(),
                    args: vec![arg.clone()],
                };
                
                let neg_sin = Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(sin_arg),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(neg_sin),
                    right: Box::new(arg_diff),
                })
            }
            
            "tan" => {
                // (tan u)' = sec^2 u * u' = u' / cos^2 u
                let cos_arg = Expression::Function {
                    name: "cos".to_string(),
                    args: vec![arg.clone()],
                };
                
                let cos_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(cos_arg),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(cos_squared),
                })
            }
            
            // 反三角函数求导
            "asin" => {
                // (arcsin u)' = u' / √(1 - u²)
                let u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(arg.clone()),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                let one_minus_u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                    right: Box::new(u_squared),
                };
                
                let sqrt_term = Expression::Function {
                    name: "sqrt".to_string(),
                    args: vec![one_minus_u_squared],
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(sqrt_term),
                })
            }
            
            "acos" => {
                // (arccos u)' = -u' / √(1 - u²)
                let u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(arg.clone()),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                let one_minus_u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                    right: Box::new(u_squared),
                };
                
                let sqrt_term = Expression::Function {
                    name: "sqrt".to_string(),
                    args: vec![one_minus_u_squared],
                };
                
                let neg_arg_diff = Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand: Box::new(arg_diff),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(neg_arg_diff),
                    right: Box::new(sqrt_term),
                })
            }
            
            "atan" => {
                // (arctan u)' = u' / (1 + u²)
                let u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(arg.clone()),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                let one_plus_u_squared = Expression::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(1)))),
                    right: Box::new(u_squared),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(one_plus_u_squared),
                })
            }
            
            // 双曲函数求导
            "sinh" => {
                // (sinh u)' = cosh u * u'
                let cosh_arg = Expression::Function {
                    name: "cosh".to_string(),
                    args: vec![arg.clone()],
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(cosh_arg),
                    right: Box::new(arg_diff),
                })
            }
            
            "cosh" => {
                // (cosh u)' = sinh u * u'
                let sinh_arg = Expression::Function {
                    name: "sinh".to_string(),
                    args: vec![arg.clone()],
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(sinh_arg),
                    right: Box::new(arg_diff),
                })
            }
            
            "tanh" => {
                // (tanh u)' = sech² u * u' = u' / cosh² u
                let cosh_arg = Expression::Function {
                    name: "cosh".to_string(),
                    args: vec![arg.clone()],
                };
                
                let cosh_squared = Expression::BinaryOp {
                    op: BinaryOperator::Power,
                    left: Box::new(cosh_arg),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(cosh_squared),
                })
            }
            
            // 对数函数求导
            "ln" | "log" => {
                // (ln u)' = u' / u
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(arg.clone()),
                })
            }
            
            "log10" => {
                // (log₁₀ u)' = u' / (u * ln(10))
                let ln_10 = Expression::Function {
                    name: "ln".to_string(),
                    args: vec![Expression::Number(Number::Integer(BigInt::from(10)))],
                };
                
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(arg.clone()),
                    right: Box::new(ln_10),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(denominator),
                })
            }
            
            "log2" => {
                // (log₂ u)' = u' / (u * ln(2))
                let ln_2 = Expression::Function {
                    name: "ln".to_string(),
                    args: vec![Expression::Number(Number::Integer(BigInt::from(2)))],
                };
                
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(arg.clone()),
                    right: Box::new(ln_2),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(denominator),
                })
            }
            
            // 指数函数求导
            "exp" => {
                // (e^u)' = e^u * u'
                let exp_arg = Expression::Function {
                    name: "exp".to_string(),
                    args: vec![arg.clone()],
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(exp_arg),
                    right: Box::new(arg_diff),
                })
            }
            
            // 平方根函数求导
            "sqrt" => {
                // (√u)' = u' / (2√u)
                let sqrt_arg = Expression::Function {
                    name: "sqrt".to_string(),
                    args: vec![arg.clone()],
                };
                
                let denominator = Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    right: Box::new(sqrt_arg),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg_diff),
                    right: Box::new(denominator),
                })
            }
            
            // 绝对值函数求导
            "abs" => {
                // |u|' = u' * sign(u) (简化处理)
                let sign_u = Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(arg.clone()),
                    right: Box::new(Expression::Function {
                        name: "abs".to_string(),
                        args: vec![arg.clone()],
                    }),
                };
                
                Ok(Expression::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(arg_diff),
                    right: Box::new(sign_u),
                })
            }
            
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
    
    /// 辅助方法：代入并求值
    fn substitute_and_evaluate(&self, expr: &Expression, var: &str, value: &Expression) -> Result<Expression, ComputeError> {
        match expr {
            Expression::Variable(name) => {
                if name == var {
                    Ok(value.clone())
                } else {
                    Ok(expr.clone())
                }
            }
            Expression::BinaryOp { op, left, right } => {
                let left_sub = self.substitute_and_evaluate(left, var, value)?;
                let right_sub = self.substitute_and_evaluate(right, var, value)?;
                Ok(Expression::BinaryOp {
                    op: op.clone(),
                    left: Box::new(left_sub),
                    right: Box::new(right_sub),
                })
            }
            Expression::UnaryOp { op, operand } => {
                let operand_sub = self.substitute_and_evaluate(operand, var, value)?;
                Ok(Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(operand_sub),
                })
            }
            _ => Ok(expr.clone()),
        }
    }
    
    /// 辅助方法：格式化表达式为字符串（简化版）
    fn format_expression(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(n) => format!("{:?}", n),
            Expression::Variable(name) => name.clone(),
            Expression::Constant(c) => format!("{:?}", c),
            Expression::BinaryOp { op, left, right } => {
                format!("({} {:?} {})", 
                    self.format_expression(left), 
                    op, 
                    self.format_expression(right))
            }
            Expression::UnaryOp { op, operand } => {
                format!("{:?}({})", op, self.format_expression(operand))
            }
            _ => format!("{:?}", expr),
        }
    }
    
    /// e^x 在 x=0 处的泰勒级数展开
    fn exp_series_at_zero(&self, var: &str, order: usize) -> Result<Expression, ComputeError> {
        let mut terms = Vec::new();
        
        for n in 0..=order {
            if n == 0 {
                // 第0项：1
                terms.push(Expression::Number(Number::Integer(BigInt::from(1))));
            } else {
                // 第n项：x^n / n!
                let x_power = if n == 1 {
                    Expression::Variable(var.to_string())
                } else {
                    Expression::BinaryOp {
                        op: BinaryOperator::Power,
                        left: Box::new(Expression::Variable(var.to_string())),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(n as i64)))),
                    }
                };
                
                let factorial_n = self.factorial(n as u32);
                let term = Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(x_power),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(factorial_n)))),
                };
                
                terms.push(term);
            }
        }
        
        // 将所有项相加
        let mut result = terms[0].clone();
        for term in terms.into_iter().skip(1) {
            result = Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(result),
                right: Box::new(term),
            };
        }
        
        Ok(result)
    }
    
    /// sin(x) 在 x=0 处的泰勒级数展开
    fn sin_series_at_zero(&self, var: &str, order: usize) -> Result<Expression, ComputeError> {
        let mut terms = Vec::new();
        
        for n in 0..=order {
            if n % 2 == 1 { // 只有奇数项
                let power = n;
                let factorial_n = self.factorial(n as u32);
                let sign = if (n - 1) / 2 % 2 == 0 { 1 } else { -1 };
                
                let x_power = if power == 1 {
                    Expression::Variable(var.to_string())
                } else {
                    Expression::BinaryOp {
                        op: BinaryOperator::Power,
                        left: Box::new(Expression::Variable(var.to_string())),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(power as i64)))),
                    }
                };
                
                let mut term = Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(x_power),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(factorial_n)))),
                };
                
                if sign == -1 {
                    term = Expression::UnaryOp {
                        op: UnaryOperator::Negate,
                        operand: Box::new(term),
                    };
                }
                
                terms.push(term);
            }
        }
        
        if terms.is_empty() {
            return Ok(Expression::Number(Number::Integer(BigInt::from(0))));
        }
        
        // 将所有项相加
        let mut result = terms[0].clone();
        for term in terms.into_iter().skip(1) {
            result = Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(result),
                right: Box::new(term),
            };
        }
        
        Ok(result)
    }
    
    /// cos(x) 在 x=0 处的泰勒级数展开
    fn cos_series_at_zero(&self, var: &str, order: usize) -> Result<Expression, ComputeError> {
        let mut terms = Vec::new();
        
        for n in 0..=order {
            if n % 2 == 0 { // 只有偶数项
                let power = n;
                let factorial_n = self.factorial(n as u32);
                let sign = if n / 2 % 2 == 0 { 1 } else { -1 };
                
                let x_power = if power == 0 {
                    Expression::Number(Number::Integer(BigInt::from(1)))
                } else if power == 2 {
                    Expression::BinaryOp {
                        op: BinaryOperator::Power,
                        left: Box::new(Expression::Variable(var.to_string())),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(2)))),
                    }
                } else {
                    Expression::BinaryOp {
                        op: BinaryOperator::Power,
                        left: Box::new(Expression::Variable(var.to_string())),
                        right: Box::new(Expression::Number(Number::Integer(BigInt::from(power as i64)))),
                    }
                };
                
                let mut term = Expression::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(x_power),
                    right: Box::new(Expression::Number(Number::Integer(BigInt::from(factorial_n)))),
                };
                
                if sign == -1 {
                    term = Expression::UnaryOp {
                        op: UnaryOperator::Negate,
                        operand: Box::new(term),
                    };
                }
                
                terms.push(term);
            }
        }
        
        if terms.is_empty() {
            return Ok(Expression::Number(Number::Integer(BigInt::from(1))));
        }
        
        // 将所有项相加
        let mut result = terms[0].clone();
        for term in terms.into_iter().skip(1) {
            result = Expression::BinaryOp {
                op: BinaryOperator::Add,
                left: Box::new(result),
                right: Box::new(term),
            };
        }
        
        Ok(result)
    }
    
    /// 计算阶乘
    fn factorial(&self, n: u32) -> u64 {
        if n == 0 || n == 1 {
            1
        } else {
            (2..=n as u64).product()
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
    
    // 积分测试
    
    #[test]
    fn test_integrate_constants() {
        let engine = CalculusEngine::new();
        
        // 常数的积分：∫5 dx = 5x
        let result = engine.integrate(&int(5), "x").unwrap();
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert_eq!(*left, int(5));
                assert_eq!(*right, var("x"));
            }
            _ => panic!("期望得到 5x"),
        }
        
        // 零的积分：∫0 dx = 0
        let result = engine.integrate(&int(0), "x").unwrap();
        assert_eq!(result, int(0));
        
        // 数学常量的积分：∫π dx = πx
        let pi = Expression::Constant(MathConstant::Pi);
        let result = engine.integrate(&pi, "x").unwrap();
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert_eq!(*left, pi);
                assert_eq!(*right, var("x"));
            }
            _ => panic!("期望得到 πx"),
        }
    }
    
    #[test]
    fn test_integrate_variables() {
        let engine = CalculusEngine::new();
        
        // x 对 x 的积分：∫x dx = x²/2
        let result = engine.integrate(&var("x"), "x").unwrap();
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }, 
                     Expression::Number(Number::Integer(n))) => {
                        assert_eq!(**base, var("x"));
                        assert_eq!(**exp, int(2));
                        assert_eq!(n, &BigInt::from(2));
                    }
                    _ => panic!("期望得到 x²/2"),
                }
            }
            _ => panic!("期望得到除法表达式"),
        }
        
        // y 对 x 的积分：∫y dx = yx
        let result = engine.integrate(&var("y"), "x").unwrap();
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert_eq!(*left, var("y"));
                assert_eq!(*right, var("x"));
            }
            _ => panic!("期望得到 yx"),
        }
    }
    
    #[test]
    fn test_integrate_addition() {
        let engine = CalculusEngine::new();
        
        // (x + 5) 的积分：∫(x + 5) dx = x²/2 + 5x
        let expr = binop(BinaryOperator::Add, var("x"), int(5));
        let result = engine.integrate(&expr, "x").unwrap();
        
        // 结果应该是加法表达式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_integrate_power() {
        let engine = CalculusEngine::new();
        
        // x² 的积分：∫x² dx = x³/3
        let expr = binop(BinaryOperator::Power, var("x"), int(2));
        let result = engine.integrate(&expr, "x").unwrap();
        
        match result {
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (Expression::BinaryOp { op: BinaryOperator::Power, left: base, right: exp }, 
                     Expression::BinaryOp { op: BinaryOperator::Add, .. }) => {
                        assert_eq!(**base, var("x"));
                        // 指数应该是 2 + 1 = 3
                    }
                    _ => panic!("期望得到 x³/(2+1) 的形式"),
                }
            }
            _ => panic!("期望得到除法表达式"),
        }
    }
    
    #[test]
    fn test_integrate_power_negative_one() {
        let engine = CalculusEngine::new();
        
        // x^(-1) 的积分：∫x^(-1) dx = ∫(1/x) dx = ln|x|
        let expr = binop(BinaryOperator::Power, var("x"), int(-1));
        let result = engine.integrate(&expr, "x").unwrap();
        
        match result {
            Expression::UnaryOp { op: UnaryOperator::Ln, operand } => {
                match operand.as_ref() {
                    Expression::UnaryOp { op: UnaryOperator::Abs, operand: inner } => {
                        assert_eq!(**inner, var("x"));
                    }
                    _ => panic!("期望得到 ln|x|"),
                }
            }
            _ => panic!("期望得到 ln 表达式"),
        }
    }
    
    #[test]
    fn test_integrate_trigonometric() {
        let engine = CalculusEngine::new();
        
        // sin(x) 的积分：∫sin(x) dx = -cos(x)
        let sin_x = unop(UnaryOperator::Sin, var("x"));
        let result = engine.integrate(&sin_x, "x").unwrap();
        
        match result {
            Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                match operand.as_ref() {
                    Expression::UnaryOp { op: UnaryOperator::Cos, operand: inner } => {
                        assert_eq!(**inner, var("x"));
                    }
                    _ => panic!("期望得到 -cos(x)"),
                }
            }
            _ => panic!("期望得到负号表达式"),
        }
        
        // cos(x) 的积分：∫cos(x) dx = sin(x)
        let cos_x = unop(UnaryOperator::Cos, var("x"));
        let result = engine.integrate(&cos_x, "x").unwrap();
        
        match result {
            Expression::UnaryOp { op: UnaryOperator::Sin, operand } => {
                assert_eq!(operand.as_ref(), &var("x"));
            }
            _ => panic!("期望得到 sin(x)"),
        }
    }
    
    #[test]
    fn test_integrate_exponential() {
        let engine = CalculusEngine::new();
        
        // e^x 的积分：∫e^x dx = e^x
        let exp_x = unop(UnaryOperator::Exp, var("x"));
        let result = engine.integrate(&exp_x, "x").unwrap();
        
        match result {
            Expression::UnaryOp { op: UnaryOperator::Exp, operand } => {
                assert_eq!(operand.as_ref(), &var("x"));
            }
            _ => panic!("期望得到 e^x"),
        }
    }
    
    #[test]
    fn test_integrate_constant_multiplication() {
        let engine = CalculusEngine::new();
        
        // 3x 的积分：∫3x dx = 3 * (x²/2) = 3x²/2
        let expr = binop(BinaryOperator::Multiply, int(3), var("x"));
        let result = engine.integrate(&expr, "x").unwrap();
        
        // 结果应该是乘法表达式
        match result {
            Expression::BinaryOp { op: BinaryOperator::Multiply, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到乘法表达式"),
        }
    }
    
    // 高级微积分功能测试
    
    #[test]
    fn test_limit_sin_x_over_x() {
        let engine = CalculusEngine::new();
        
        // lim(x->0) sin(x)/x = 1
        let sin_x = unop(UnaryOperator::Sin, var("x"));
        let expr = binop(BinaryOperator::Divide, sin_x, var("x"));
        let point = int(0);
        
        let result = engine.limit(&expr, "x", &point).unwrap();
        assert_eq!(result, int(1));
    }
    
    #[test]
    fn test_limit_one_minus_cos_x_over_x() {
        let engine = CalculusEngine::new();
        
        // lim(x->0) (1-cos(x))/x = 0
        let cos_x = unop(UnaryOperator::Cos, var("x"));
        let one_minus_cos = binop(BinaryOperator::Subtract, int(1), cos_x);
        let expr = binop(BinaryOperator::Divide, one_minus_cos, var("x"));
        let point = int(0);
        
        let result = engine.limit(&expr, "x", &point).unwrap();
        assert_eq!(result, int(0));
    }
    
    #[test]
    fn test_series_exp_at_zero() {
        let engine = CalculusEngine::new();
        
        // e^x 在 x=0 处的泰勒级数展开（前3项）
        let exp_x = unop(UnaryOperator::Exp, var("x"));
        let point = int(0);
        
        let result = engine.series(&exp_x, "x", &point, 2).unwrap();
        
        // 结果应该是加法表达式：1 + x + x²/2
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_series_sin_at_zero() {
        let engine = CalculusEngine::new();
        
        // sin(x) 在 x=0 处的泰勒级数展开（前2项）
        let sin_x = unop(UnaryOperator::Sin, var("x"));
        let point = int(0);
        
        let result = engine.series(&sin_x, "x", &point, 3).unwrap();
        
        // 结果应该是减法表达式：x - x³/6
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_series_cos_at_zero() {
        let engine = CalculusEngine::new();
        
        // cos(x) 在 x=0 处的泰勒级数展开（前3项）
        let cos_x = unop(UnaryOperator::Cos, var("x"));
        let point = int(0);
        
        let result = engine.series(&cos_x, "x", &point, 4).unwrap();
        
        // 结果应该是加法表达式：1 - x²/2 + x⁴/24
        match result {
            Expression::BinaryOp { op: BinaryOperator::Add, .. } => {
                // 验证结构正确
            }
            _ => panic!("期望得到加法表达式"),
        }
    }
    
    #[test]
    fn test_numerical_evaluate_basic() {
        let engine = CalculusEngine::new();
        let mut vars = std::collections::HashMap::new();
        vars.insert("x".to_string(), 2.0);
        
        // 数值计算：x + 3 = 2 + 3 = 5
        let expr = binop(BinaryOperator::Add, var("x"), int(3));
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
        
        // 数值计算：x² = 2² = 4
        let expr = binop(BinaryOperator::Power, var("x"), int(2));
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - 4.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_numerical_evaluate_trigonometric() {
        let engine = CalculusEngine::new();
        let mut vars = std::collections::HashMap::new();
        vars.insert("x".to_string(), 0.0);
        
        // 数值计算：sin(0) = 0
        let expr = unop(UnaryOperator::Sin, var("x"));
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
        
        // 数值计算：cos(0) = 1
        let expr = unop(UnaryOperator::Cos, var("x"));
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_numerical_evaluate_constants() {
        let engine = CalculusEngine::new();
        let vars = std::collections::HashMap::new();
        
        // 数值计算：π ≈ 3.14159
        let expr = Expression::Constant(MathConstant::Pi);
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
        
        // 数值计算：e ≈ 2.71828
        let expr = Expression::Constant(MathConstant::E);
        let result = engine.numerical_evaluate(&expr, &vars).unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }
    
    #[test]
    fn test_factorial() {
        let engine = CalculusEngine::new();
        
        assert_eq!(engine.factorial(0), 1);
        assert_eq!(engine.factorial(1), 1);
        assert_eq!(engine.factorial(2), 2);
        assert_eq!(engine.factorial(3), 6);
        assert_eq!(engine.factorial(4), 24);
        assert_eq!(engine.factorial(5), 120);
    }
}