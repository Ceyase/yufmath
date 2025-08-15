//! # LaTeX 格式化器
//!
//! 将表达式格式化为 LaTeX 格式。

use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{Formatter, FormatOptions};
use num_traits::ToPrimitive;

/// LaTeX 格式化器
pub struct LaTeXFormatter {
    options: FormatOptions,
}

impl LaTeXFormatter {
    /// 创建新的 LaTeX 格式化器
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
                    r.numer().to_string()
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
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
        match constant {
            MathConstant::Pi => "\\pi".to_string(),
            MathConstant::E => "e".to_string(),
            MathConstant::I => "i".to_string(),
            MathConstant::EulerGamma => "\\gamma".to_string(),
            MathConstant::GoldenRatio => "\\phi".to_string(),
            MathConstant::Catalan => "G".to_string(),
            MathConstant::PositiveInfinity => "\\infty".to_string(),
            MathConstant::NegativeInfinity => "-\\infty".to_string(),
            MathConstant::Undefined => "\\text{undefined}".to_string(),
        }
    }
    
    /// 格式化二元运算表达式
    fn format_binary_op(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> String {
        let left_str = self.format(left);
        let right_str = self.format(right);
        
        match op {
            BinaryOperator::Add => format!("{} + {}", left_str, right_str),
            BinaryOperator::Subtract => format!("{} - {}", left_str, right_str),
            BinaryOperator::Multiply => {
                if self.should_omit_multiply_symbol(left, right) {
                    format!("{}{}", left_str, right_str)
                } else {
                    format!("{} \\cdot {}", left_str, right_str)
                }
            }
            BinaryOperator::Divide => format!("\\frac{{{}}}{{{}}}", left_str, right_str),
            BinaryOperator::Power => format!("{}^{{{}}}", left_str, right_str),
            BinaryOperator::Modulo => format!("{} \\bmod {}", left_str, right_str),
            BinaryOperator::Equal => format!("{} = {}", left_str, right_str),
            BinaryOperator::NotEqual => format!("{} \\neq {}", left_str, right_str),
            BinaryOperator::Less => format!("{} < {}", left_str, right_str),
            BinaryOperator::LessEqual => format!("{} \\leq {}", left_str, right_str),
            BinaryOperator::Greater => format!("{} > {}", left_str, right_str),
            BinaryOperator::GreaterEqual => format!("{} \\geq {}", left_str, right_str),
            BinaryOperator::And => format!("{} \\land {}", left_str, right_str),
            BinaryOperator::Or => format!("{} \\lor {}", left_str, right_str),
            BinaryOperator::Union => format!("{} \\cup {}", left_str, right_str),
            BinaryOperator::Intersection => format!("{} \\cap {}", left_str, right_str),
            BinaryOperator::SetDifference => format!("{} \\setminus {}", left_str, right_str),
            BinaryOperator::MatrixMultiply => format!("{} {}", left_str, right_str),
            BinaryOperator::CrossProduct => format!("{} \\times {}", left_str, right_str),
            BinaryOperator::DotProduct => format!("{} \\cdot {}", left_str, right_str),
        }
    }
    
    /// 判断是否应该省略乘法符号
    fn should_omit_multiply_symbol(&self, left: &Expression, right: &Expression) -> bool {
        match (left, right) {
            (Expression::Number(_), Expression::Variable(_)) => true,
            (Expression::Number(_), Expression::Function { .. }) => true,
            (Expression::Variable(_), Expression::Variable(_)) => true,
            (Expression::Variable(_), Expression::Function { .. }) => true,
            (Expression::Function { .. }, Expression::Variable(_)) => true,
            _ => false,
        }
    }
    
    /// 格式化一元运算表达式
    fn format_unary_op(&self, op: &UnaryOperator, operand: &Expression) -> String {
        let operand_str = self.format(operand);
        
        match op {
            UnaryOperator::Negate => format!("-{}", operand_str),
            UnaryOperator::Plus => format!("+{}", operand_str),
            UnaryOperator::Sqrt => format!("\\sqrt{{{}}}", operand_str),
            UnaryOperator::Abs => format!("\\left|{}\\right|", operand_str),
            UnaryOperator::Sin => format!("\\sin\\left({}\\right)", operand_str),
            UnaryOperator::Cos => format!("\\cos\\left({}\\right)", operand_str),
            UnaryOperator::Tan => format!("\\tan\\left({}\\right)", operand_str),
            UnaryOperator::Asin => format!("\\arcsin\\left({}\\right)", operand_str),
            UnaryOperator::Acos => format!("\\arccos\\left({}\\right)", operand_str),
            UnaryOperator::Atan => format!("\\arctan\\left({}\\right)", operand_str),
            UnaryOperator::Sinh => format!("\\sinh\\left({}\\right)", operand_str),
            UnaryOperator::Cosh => format!("\\cosh\\left({}\\right)", operand_str),
            UnaryOperator::Tanh => format!("\\tanh\\left({}\\right)", operand_str),
            UnaryOperator::Asinh => format!("\\text{{asinh}}\\left({}\\right)", operand_str),
            UnaryOperator::Acosh => format!("\\text{{acosh}}\\left({}\\right)", operand_str),
            UnaryOperator::Atanh => format!("\\text{{atanh}}\\left({}\\right)", operand_str),
            UnaryOperator::Ln => format!("\\ln\\left({}\\right)", operand_str),
            UnaryOperator::Log10 => format!("\\log_{{10}}\\left({}\\right)", operand_str),
            UnaryOperator::Log2 => format!("\\log_2\\left({}\\right)", operand_str),
            UnaryOperator::Exp => format!("\\exp\\left({}\\right)", operand_str),
            UnaryOperator::Factorial => format!("{}!", operand_str),
            UnaryOperator::Gamma => format!("\\Gamma\\left({}\\right)", operand_str),
            UnaryOperator::Not => format!("\\neg {}", operand_str),
            UnaryOperator::Real => format!("\\text{{Re}}\\left({}\\right)", operand_str),
            UnaryOperator::Imaginary => format!("\\text{{Im}}\\left({}\\right)", operand_str),
            UnaryOperator::Conjugate => format!("\\overline{{{}}}", operand_str),
            UnaryOperator::Argument => format!("\\arg\\left({}\\right)", operand_str),
            UnaryOperator::Transpose => format!("{}^T", operand_str),
            UnaryOperator::Determinant => format!("\\det\\left({}\\right)", operand_str),
            UnaryOperator::Inverse => format!("{}^{{-1}}", operand_str),
            UnaryOperator::Trace => format!("\\text{{tr}}\\left({}\\right)", operand_str),
        }
    }
    
    /// 格式化函数调用
    fn format_function(&self, name: &str, args: &[Expression]) -> String {
        let args_str: Vec<String> = args.iter().map(|arg| self.format(arg)).collect();
        format!("\\text{{{}}}\\left({}\\right)", name, args_str.join(", "))
    }
    
    /// 格式化矩阵
    fn format_matrix(&self, matrix: &[Vec<Expression>]) -> String {
        let rows: Vec<String> = matrix.iter().map(|row| {
            let elements: Vec<String> = row.iter().map(|elem| self.format(elem)).collect();
            elements.join(" & ")
        }).collect();
        
        format!("\\begin{{pmatrix}}\n{}\n\\end{{pmatrix}}", rows.join(" \\\\\n"))
    }
    
    /// 格式化向量
    fn format_vector(&self, vector: &[Expression]) -> String {
        let elements: Vec<String> = vector.iter().map(|elem| self.format(elem)).collect();
        format!("\\begin{{pmatrix}} {} \\end{{pmatrix}}", elements.join(" \\\\ "))
    }
    
    /// 格式化集合
    fn format_set(&self, set: &[Expression]) -> String {
        let elements: Vec<String> = set.iter().map(|elem| self.format(elem)).collect();
        format!("\\left\\{{{}\\right\\}}", elements.join(", "))
    }
    
    /// 格式化区间
    fn format_interval(&self, start: &Expression, end: &Expression, start_inclusive: bool, end_inclusive: bool) -> String {
        let start_bracket = if start_inclusive { "\\left[" } else { "\\left(" };
        let end_bracket = if end_inclusive { "\\right]" } else { "\\right)" };
        format!("{}{}, {}{}", start_bracket, self.format(start), self.format(end), end_bracket)
    }
}

impl Default for LaTeXFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for LaTeXFormatter {
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