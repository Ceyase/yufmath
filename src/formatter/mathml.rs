//! # MathML 格式化器
//!
//! 将表达式格式化为 MathML 格式。

use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{Formatter, FormatOptions};
use num_traits::ToPrimitive;

/// MathML 格式化器
pub struct MathMLFormatter {
    options: FormatOptions,
}

impl MathMLFormatter {
    /// 创建新的 MathML 格式化器
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
        }
    }
    
    /// 格式化数值
    fn format_number(&self, number: &Number) -> String {
        match number {
            Number::Integer(i) => format!("<mn>{}</mn>", i),
            Number::Rational(r) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    format!("<mn>{}</mn>", r.numer())
                } else {
                    format!(
                        "<mfrac><mn>{}</mn><mn>{}</mn></mfrac>",
                        r.numer(),
                        r.denom()
                    )
                }
            }
            Number::Real(r) => {
                let value = if let Some(precision) = self.options.precision {
                    format!("{:.prec$}", r.to_f64().unwrap_or(0.0), prec = precision)
                } else {
                    r.to_string()
                };
                format!("<mn>{}</mn>", value)
            }
            Number::Complex { real, imaginary } => {
                let real_str = self.format_number(real);
                let imag_str = self.format_number(imaginary);
                
                match (real.is_zero(), imaginary.is_zero()) {
                    (true, true) => "<mn>0</mn>".to_string(),
                    (true, false) => {
                        if imag_str == "<mn>1</mn>" {
                            "<mi>i</mi>".to_string()
                        } else if imag_str == "<mn>-1</mn>" {
                            "<mrow><mo>-</mo><mi>i</mi></mrow>".to_string()
                        } else {
                            format!("<mrow>{}<mi>i</mi></mrow>", imag_str)
                        }
                    }
                    (false, true) => real_str,
                    (false, false) => {
                        let imag_part = if imag_str == "<mn>1</mn>" {
                            "<mi>i</mi>".to_string()
                        } else if imag_str == "<mn>-1</mn>" {
                            "<mrow><mo>-</mo><mi>i</mi></mrow>".to_string()
                        } else if imag_str.contains("<mn>-") {
                            format!("<mrow>{}<mi>i</mi></mrow>", imag_str)
                        } else {
                            format!("<mrow><mo>+</mo>{}<mi>i</mi></mrow>", imag_str)
                        };
                        format!("<mrow>{}{}</mrow>", real_str, imag_part)
                    }
                }
            }
            Number::Constant(constant) => self.format_constant(constant),
            Number::Symbolic(expr) => self.format(expr),
            Number::Float(f) => {
                let value = if let Some(precision) = self.options.precision {
                    format!("{:.prec$}", f, prec = precision)
                } else {
                    f.to_string()
                };
                format!("<mn>{}</mn>", value)
            }
        }
    }
    
    /// 格式化数学常量
    fn format_constant(&self, constant: &MathConstant) -> String {
        match constant {
            MathConstant::Pi => "<mi>&pi;</mi>".to_string(),
            MathConstant::E => "<mi>e</mi>".to_string(),
            MathConstant::I => "<mi>i</mi>".to_string(),
            MathConstant::EulerGamma => "<mi>&gamma;</mi>".to_string(),
            MathConstant::GoldenRatio => "<mi>&phi;</mi>".to_string(),
            MathConstant::Catalan => "<mi>G</mi>".to_string(),
            MathConstant::PositiveInfinity => "<mi>&infin;</mi>".to_string(),
            MathConstant::NegativeInfinity => "<mrow><mo>-</mo><mi>&infin;</mi></mrow>".to_string(),
            MathConstant::Undefined => "<mtext>undefined</mtext>".to_string(),
        }
    }
    
    /// 格式化二元运算表达式
    fn format_binary_op(&self, op: &BinaryOperator, left: &Expression, right: &Expression) -> String {
        let left_str = self.format(left);
        let right_str = self.format(right);
        
        match op {
            BinaryOperator::Add => {
                format!("<mrow>{}<mo>+</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Subtract => {
                format!("<mrow>{}<mo>-</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Multiply => {
                if self.should_omit_multiply_symbol(left, right) {
                    format!("<mrow>{}{}</mrow>", left_str, right_str)
                } else {
                    format!("<mrow>{}<mo>&middot;</mo>{}</mrow>", left_str, right_str)
                }
            }
            BinaryOperator::Divide => {
                format!("<mfrac>{}{}</mfrac>", left_str, right_str)
            }
            BinaryOperator::Power => {
                format!("<msup>{}{}</msup>", left_str, right_str)
            }
            BinaryOperator::Modulo => {
                format!("<mrow>{}<mo>mod</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Equal => {
                format!("<mrow>{}<mo>=</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::NotEqual => {
                format!("<mrow>{}<mo>&ne;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Less => {
                format!("<mrow>{}<mo>&lt;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::LessEqual => {
                format!("<mrow>{}<mo>&le;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Greater => {
                format!("<mrow>{}<mo>&gt;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::GreaterEqual => {
                format!("<mrow>{}<mo>&ge;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::And => {
                format!("<mrow>{}<mo>&and;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Or => {
                format!("<mrow>{}<mo>&or;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Union => {
                format!("<mrow>{}<mo>&cup;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::Intersection => {
                format!("<mrow>{}<mo>&cap;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::SetDifference => {
                format!("<mrow>{}<mo>\\</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::MatrixMultiply => {
                format!("<mrow>{}{}</mrow>", left_str, right_str)
            }
            BinaryOperator::CrossProduct => {
                format!("<mrow>{}<mo>&times;</mo>{}</mrow>", left_str, right_str)
            }
            BinaryOperator::DotProduct => {
                format!("<mrow>{}<mo>&middot;</mo>{}</mrow>", left_str, right_str)
            }
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
            UnaryOperator::Negate => {
                format!("<mrow><mo>-</mo>{}</mrow>", operand_str)
            }
            UnaryOperator::Plus => {
                format!("<mrow><mo>+</mo>{}</mrow>", operand_str)
            }
            UnaryOperator::Sqrt => {
                format!("<msqrt>{}</msqrt>", operand_str)
            }
            UnaryOperator::Abs => {
                format!("<mrow><mo>|</mo>{}<mo>|</mo></mrow>", operand_str)
            }
            UnaryOperator::Sin => {
                format!("<mrow><mi>sin</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Cos => {
                format!("<mrow><mi>cos</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Tan => {
                format!("<mrow><mi>tan</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Asin => {
                format!("<mrow><mi>arcsin</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Acos => {
                format!("<mrow><mi>arccos</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Atan => {
                format!("<mrow><mi>arctan</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Sinh => {
                format!("<mrow><mi>sinh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Cosh => {
                format!("<mrow><mi>cosh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Tanh => {
                format!("<mrow><mi>tanh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Asinh => {
                format!("<mrow><mi>asinh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Acosh => {
                format!("<mrow><mi>acosh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Atanh => {
                format!("<mrow><mi>atanh</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Ln => {
                format!("<mrow><mi>ln</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Log10 => {
                format!("<mrow><msub><mi>log</mi><mn>10</mn></msub><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Log2 => {
                format!("<mrow><msub><mi>log</mi><mn>2</mn></msub><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Exp => {
                format!("<mrow><mi>exp</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Factorial => {
                format!("<mrow>{}<mo>!</mo></mrow>", operand_str)
            }
            UnaryOperator::Gamma => {
                format!("<mrow><mi>&Gamma;</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Not => {
                format!("<mrow><mo>&not;</mo>{}</mrow>", operand_str)
            }
            UnaryOperator::Real => {
                format!("<mrow><mi>Re</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Imaginary => {
                format!("<mrow><mi>Im</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Conjugate => {
                format!("<mover>{}<mo>¯</mo></mover>", operand_str)
            }
            UnaryOperator::Argument => {
                format!("<mrow><mi>arg</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Transpose => {
                format!("<msup>{}<mi>T</mi></msup>", operand_str)
            }
            UnaryOperator::Determinant => {
                format!("<mrow><mi>det</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
            UnaryOperator::Inverse => {
                format!("<msup>{}<mrow><mo>-</mo><mn>1</mn></mrow></msup>", operand_str)
            }
            UnaryOperator::Trace => {
                format!("<mrow><mi>tr</mi><mo>(</mo>{}<mo>)</mo></mrow>", operand_str)
            }
        }
    }
    
    /// 格式化函数调用
    fn format_function(&self, name: &str, args: &[Expression]) -> String {
        let args_str: Vec<String> = args.iter().map(|arg| self.format(arg)).collect();
        format!(
            "<mrow><mi>{}</mi><mo>(</mo>{}<mo>)</mo></mrow>",
            name,
            args_str.join("<mo>,</mo>")
        )
    }
    
    /// 格式化矩阵
    fn format_matrix(&self, matrix: &[Vec<Expression>]) -> String {
        let rows: Vec<String> = matrix.iter().map(|row| {
            let elements: Vec<String> = row.iter().map(|elem| {
                format!("<mtd>{}</mtd>", self.format(elem))
            }).collect();
            format!("<mtr>{}</mtr>", elements.join(""))
        }).collect();
        
        format!(
            "<mrow><mo>(</mo><mtable>{}</mtable><mo>)</mo></mrow>",
            rows.join("")
        )
    }
    
    /// 格式化向量
    fn format_vector(&self, vector: &[Expression]) -> String {
        let elements: Vec<String> = vector.iter().map(|elem| {
            format!("<mtd>{}</mtd>", self.format(elem))
        }).collect();
        
        format!(
            "<mrow><mo>(</mo><mtable>{}</mtable><mo>)</mo></mrow>",
            elements.iter().map(|elem| format!("<mtr>{}</mtr>", elem)).collect::<Vec<_>>().join("")
        )
    }
    
    /// 格式化集合
    fn format_set(&self, set: &[Expression]) -> String {
        let elements: Vec<String> = set.iter().map(|elem| self.format(elem)).collect();
        format!(
            "<mrow><mo>{{</mo>{}<mo>}}</mo></mrow>",
            elements.join("<mo>,</mo>")
        )
    }
    
    /// 格式化区间
    fn format_interval(&self, start: &Expression, end: &Expression, start_inclusive: bool, end_inclusive: bool) -> String {
        let start_bracket = if start_inclusive { "<mo>[</mo>" } else { "<mo>(</mo>" };
        let end_bracket = if end_inclusive { "<mo>]</mo>" } else { "<mo>)</mo>" };
        format!(
            "<mrow>{}{}<mo>,</mo>{}{}</mrow>",
            start_bracket,
            self.format(start),
            self.format(end),
            end_bracket
        )
    }
}

impl Default for MathMLFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for MathMLFormatter {
    fn format(&self, expr: &Expression) -> String {
        let content = match expr {
            Expression::Number(number) => self.format_number(number),
            Expression::Variable(name) => format!("<mi>{}</mi>", name),
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
        };
        
        // 包装在 math 标签中
        format!("<math xmlns=\"http://www.w3.org/1998/Math/MathML\">{}</math>", content)
    }
    
    fn set_options(&mut self, options: FormatOptions) {
        self.options = options;
    }
}