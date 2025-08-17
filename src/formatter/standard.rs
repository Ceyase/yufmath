//! # 标准格式化器
//!
//! 将表达式格式化为标准数学记号。

use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{Formatter, FormatOptions};
use num_traits::ToPrimitive;

/// 标准格式化器
pub struct StandardFormatter {
    options: FormatOptions,
}

impl StandardFormatter {
    /// 创建新的标准格式化器
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
        }
    }
    
    /// 格式化数值
    fn format_number(&self, number: &Number) -> String {
        match number {
            Number::Integer(i) => i.to_string(),
            Number::Rational(r) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    // 分母为1时，显示为整数
                    r.numer().to_string()
                } else {
                    format!("{}/{}", r.numer(), r.denom())
                }
            }
            Number::Real(r) => {
                if let Some(precision) = self.options.precision {
                    format!("{:.prec$}", r.to_f64().unwrap_or(0.0), prec = precision)
                } else {
                    r.to_string()
                }
            }
            Number::Complex { real, imaginary } => {
                let real_str = self.format_number(real);
                let imag_str = self.format_number(imaginary);
                
                // 处理复数的显示格式
                match (real.is_zero(), imaginary.is_zero()) {
                    (true, true) => "0".to_string(),
                    (true, false) => {
                        if imag_str == "1" {
                            "i".to_string()
                        } else if imag_str == "-1" {
                            "-i".to_string()
                        } else {
                            format!("{}i", imag_str)
                        }
                    }
                    (false, true) => real_str,
                    (false, false) => {
                        let imag_part = if imag_str == "1" {
                            "i".to_string()
                        } else if imag_str == "-1" {
                            "-i".to_string()
                        } else if imag_str.starts_with('-') {
                            format!("{}i", imag_str)
                        } else {
                            format!("+{}i", imag_str)
                        };
                        format!("{}{}", real_str, imag_part)
                    }
                }
            }
            Number::Constant(constant) => self.format_constant(constant),
            Number::Symbolic(expr) => self.format(expr),
            Number::Float(f) => {
                if let Some(precision) = self.options.precision {
                    format!("{:.prec$}", f, prec = precision)
                } else {
                    f.to_string()
                }
            }
        }
    }
    
    /// 格式化数学常量
    fn format_constant(&self, constant: &MathConstant) -> String {
        constant.symbol().to_string()
    }
    
    /// 格式化二元运算符
    fn format_binary_operator(&self, op: &BinaryOperator) -> String {
        match op {
            BinaryOperator::Add => "+".to_string(),
            BinaryOperator::Subtract => "-".to_string(),
            BinaryOperator::Multiply => "*".to_string(),
            BinaryOperator::Divide => "/".to_string(),
            BinaryOperator::Power => "^".to_string(),
            BinaryOperator::Modulo => "%".to_string(),
            BinaryOperator::Equal => "==".to_string(),
            BinaryOperator::NotEqual => "!=".to_string(),
            BinaryOperator::Less => "<".to_string(),
            BinaryOperator::LessEqual => "<=".to_string(),
            BinaryOperator::Greater => ">".to_string(),
            BinaryOperator::GreaterEqual => ">=".to_string(),
            BinaryOperator::And => "&&".to_string(),
            BinaryOperator::Or => "||".to_string(),
            BinaryOperator::Union => "∪".to_string(),
            BinaryOperator::Intersection => "∩".to_string(),
            BinaryOperator::SetDifference => "\\".to_string(),
            BinaryOperator::MatrixMultiply => "@".to_string(),
            BinaryOperator::CrossProduct => "×".to_string(),
            BinaryOperator::DotProduct => "·".to_string(),
        }
    }
    
    /// 格式化一元运算符
    fn format_unary_operator(&self, op: &UnaryOperator) -> String {
        match op {
            UnaryOperator::Negate => "-".to_string(),
            UnaryOperator::Plus => "+".to_string(),
            UnaryOperator::Sqrt => "√".to_string(),
            UnaryOperator::Abs => "abs".to_string(),
            UnaryOperator::Sin => "sin".to_string(),
            UnaryOperator::Cos => "cos".to_string(),
            UnaryOperator::Tan => "tan".to_string(),
            UnaryOperator::Asin => "asin".to_string(),
            UnaryOperator::Acos => "acos".to_string(),
            UnaryOperator::Atan => "atan".to_string(),
            UnaryOperator::Sinh => "sinh".to_string(),
            UnaryOperator::Cosh => "cosh".to_string(),
            UnaryOperator::Tanh => "tanh".to_string(),
            UnaryOperator::Asinh => "asinh".to_string(),
            UnaryOperator::Acosh => "acosh".to_string(),
            UnaryOperator::Atanh => "atanh".to_string(),
            UnaryOperator::Ln => "ln".to_string(),
            UnaryOperator::Log10 => "log10".to_string(),
            UnaryOperator::Log2 => "log2".to_string(),
            UnaryOperator::Exp => "exp".to_string(),
            UnaryOperator::Factorial => "!".to_string(),
            UnaryOperator::Gamma => "Γ".to_string(),
            UnaryOperator::Not => "!".to_string(),
            UnaryOperator::Real => "Re".to_string(),
            UnaryOperator::Imaginary => "Im".to_string(),
            UnaryOperator::Conjugate => "*".to_string(),
            UnaryOperator::Argument => "arg".to_string(),
            UnaryOperator::Transpose => "T".to_string(),
            UnaryOperator::Determinant => "det".to_string(),
            UnaryOperator::Inverse => "inv".to_string(),
            UnaryOperator::Trace => "tr".to_string(),
        }
    }
    
    /// 检查是否需要括号
    fn needs_parentheses(&self, expr: &Expression, parent_op: Option<&BinaryOperator>, is_right: bool) -> bool {
        if !self.options.use_parentheses {
            return false;
        }
        
        match (expr, parent_op) {
            (Expression::BinaryOp { op, .. }, Some(parent)) => {
                let expr_precedence = self.get_precedence(op);
                let parent_precedence = self.get_precedence(parent);
                
                // 如果当前运算符优先级低于父运算符，需要括号
                if expr_precedence < parent_precedence {
                    return true;
                }
                
                // 如果优先级相同，检查结合性
                if expr_precedence == parent_precedence {
                    match parent {
                        BinaryOperator::Subtract | BinaryOperator::Divide | BinaryOperator::Power => {
                            // 右结合或非结合运算符，右操作数需要括号
                            return is_right;
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    
    /// 获取运算符优先级
    fn get_precedence(&self, op: &BinaryOperator) -> u8 {
        match op {
            BinaryOperator::Or => 1,
            BinaryOperator::And => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual 
            | BinaryOperator::Less | BinaryOperator::LessEqual 
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual => 3,
            BinaryOperator::Union | BinaryOperator::Intersection | BinaryOperator::SetDifference => 4,
            BinaryOperator::Add | BinaryOperator::Subtract => 5,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo 
            | BinaryOperator::DotProduct | BinaryOperator::CrossProduct => 6,
            BinaryOperator::Power => 7,
            BinaryOperator::MatrixMultiply => 8,
        }
    }
    
    /// 格式化二元运算表达式
    fn format_binary_op(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> String {
        let left_str = if self.needs_parentheses(left, Some(op), false) {
            format!("({})", self.format(left))
        } else {
            self.format(left)
        };
        
        let right_str = if self.needs_parentheses(right, Some(op), true) {
            format!("({})", self.format(right))
        } else {
            self.format(right)
        };
        
        let op_str = self.format_binary_operator(op);
        
        // 特殊处理某些运算符的格式
        match op {
            BinaryOperator::Power => {
                format!("{}^{}", left_str, right_str)
            }
            BinaryOperator::Multiply => {
                // 智能乘法符号处理
                if self.should_omit_multiply_symbol(left, right) {
                    format!("{}{}", left_str, right_str)
                } else {
                    format!("{} {} {}", left_str, op_str, right_str)
                }
            }
            BinaryOperator::Add => {
                // 特殊处理 a + (-b) -> a - b
                match right {
                    Expression::UnaryOp { op: UnaryOperator::Negate, operand } => {
                        let right_str = if self.needs_parentheses(operand, Some(&BinaryOperator::Subtract), true) {
                            format!("({})", self.format(operand))
                        } else {
                            self.format(operand)
                        };
                        format!("{} - {}", left_str, right_str)
                    }
                    Expression::Number(n) if n.is_negative() => {
                        // 处理负数：a + (-5) -> a - 5
                        let positive_n = -n.clone();
                        let right_str = self.format(&Expression::Number(positive_n));
                        format!("{} - {}", left_str, right_str)
                    }
                    _ => {
                        format!("{} {} {}", left_str, op_str, right_str)
                    }
                }
            }
            _ => {
                format!("{} {} {}", left_str, op_str, right_str)
            }
        }
    }
    
    /// 判断是否应该省略乘法符号
    fn should_omit_multiply_symbol(&self, left: &Expression, right: &Expression) -> bool {
        match (left, right) {
            // 数字 * 变量：2x
            (Expression::Number(_), Expression::Variable(_)) => true,
            // 数字 * 函数：2sin(x)
            (Expression::Number(_), Expression::Function { .. }) => true,
            // 变量 * 变量：xy
            (Expression::Variable(_), Expression::Variable(_)) => true,
            // 变量 * 函数：x*sin(y) -> xsin(y)
            (Expression::Variable(_), Expression::Function { .. }) => true,
            // 函数 * 变量：sin(x)*y -> sin(x)y
            (Expression::Function { .. }, Expression::Variable(_)) => true,
            // 对于复杂表达式，保持明确的乘法符号以避免歧义
            // 例如：(x/2)*x 不应该变成 x/2x
            _ => false,
        }
    }
    
    /// 格式化一元运算表达式
    fn format_unary_op(&self, op: &UnaryOperator, operand: &Expression) -> String {
        let op_str = self.format_unary_operator(op);
        let operand_str = self.format(operand);
        
        match op {
            // 前缀运算符
            UnaryOperator::Negate | UnaryOperator::Plus | UnaryOperator::Not => {
                if matches!(operand, Expression::BinaryOp { .. }) && self.options.use_parentheses {
                    format!("{}({})", op_str, operand_str)
                } else {
                    format!("{}{}", op_str, operand_str)
                }
            }
            // 后缀运算符
            UnaryOperator::Factorial | UnaryOperator::Transpose => {
                format!("{}{}", operand_str, op_str)
            }
            // 函数形式运算符
            UnaryOperator::Sqrt => {
                format!("√({})", operand_str)
            }
            UnaryOperator::Abs => {
                format!("|{}|", operand_str)
            }
            UnaryOperator::Conjugate => {
                format!("{}*", operand_str)
            }
            // 其他函数形式
            _ => {
                format!("{}({})", op_str, operand_str)
            }
        }
    }
    
    /// 格式化函数调用
    fn format_function(&self, name: &str, args: &[Expression]) -> String {
        let args_str: Vec<String> = args.iter().map(|arg| self.format(arg)).collect();
        format!("{}({})", name, args_str.join(", "))
    }
    
    /// 格式化矩阵
    fn format_matrix(&self, matrix: &[Vec<Expression>]) -> String {
        let rows: Vec<String> = matrix.iter().map(|row| {
            let elements: Vec<String> = row.iter().map(|elem| self.format(elem)).collect();
            format!("[{}]", elements.join(", "))
        }).collect();
        format!("[{}]", rows.join(", "))
    }
    
    /// 格式化向量
    fn format_vector(&self, vector: &[Expression]) -> String {
        let elements: Vec<String> = vector.iter().map(|elem| self.format(elem)).collect();
        format!("[{}]", elements.join(", "))
    }
    
    /// 格式化集合
    fn format_set(&self, set: &[Expression]) -> String {
        let elements: Vec<String> = set.iter().map(|elem| self.format(elem)).collect();
        format!("{{{}}}", elements.join(", "))
    }
    
    /// 格式化区间
    fn format_interval(&self, start: &Expression, end: &Expression, start_inclusive: bool, end_inclusive: bool) -> String {
        let start_bracket = if start_inclusive { "[" } else { "(" };
        let end_bracket = if end_inclusive { "]" } else { ")" };
        format!("{}{}, {}{}", start_bracket, self.format(start), self.format(end), end_bracket)
    }
}

impl Default for StandardFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for StandardFormatter {
    fn format(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(number) => self.format_number(number),
            Expression::Variable(name) => name.clone(),
            Expression::Constant(constant) => self.format_constant(constant),
            Expression::BinaryOp { op, left, right } => {
                self.format_binary_op(op, left, right)
            }
            Expression::UnaryOp { op, operand } => {
                self.format_unary_op(op, operand)
            }
            Expression::Function { name, args } => {
                self.format_function(name, args)
            }
            Expression::Matrix(matrix) => {
                self.format_matrix(matrix)
            }
            Expression::Vector(vector) => {
                self.format_vector(vector)
            }
            Expression::Set(set) => {
                self.format_set(set)
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                self.format_interval(start, end, *start_inclusive, *end_inclusive)
            }
        }
    }
    
    fn set_options(&mut self, options: FormatOptions) {
        self.options = options;
    }
}

