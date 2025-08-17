//! # 终端格式化器
//!
//! 专为终端交互模式设计的格式化器，支持颜色输出和数值近似值显示。

use crate::core::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::{Formatter, FormatOptions};
use ansi_term::Colour;
use num_traits::{ToPrimitive, Zero};
use std::f64::consts;

/// 终端格式化器
pub struct TerminalFormatter {
    options: FormatOptions,
    /// 是否启用颜色输出
    enable_colors: bool,
    /// 是否显示数值近似值
    show_approximations: bool,
    /// 近似值精度
    approximation_precision: usize,
}

impl TerminalFormatter {
    /// 创建新的终端格式化器
    pub fn new() -> Self {
        Self {
            options: FormatOptions::default(),
            enable_colors: true,
            // 默认禁用近似值显示以避免精度问题
            show_approximations: false,
            approximation_precision: 6,
        }
    }
    
    /// 设置是否启用颜色输出
    pub fn set_colors_enabled(&mut self, enabled: bool) {
        self.enable_colors = enabled;
    }
    
    /// 检查是否启用了颜色输出
    pub fn colors_enabled(&self) -> bool {
        self.enable_colors
    }
    
    /// 设置是否显示数值近似值
    pub fn set_approximations_enabled(&mut self, enabled: bool) {
        self.show_approximations = enabled;
    }
    
    /// 设置近似值精度
    pub fn set_approximation_precision(&mut self, precision: usize) {
        self.approximation_precision = precision;
    }
    
    /// 为数字着色
    fn colorize_number(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Cyan.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为变量着色
    fn colorize_variable(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Green.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为常量着色
    fn colorize_constant(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Purple.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为运算符着色
    fn colorize_operator(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Yellow.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为函数名着色
    fn colorize_function(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Blue.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为括号着色
    fn colorize_parentheses(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::White.paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 为近似值着色
    fn colorize_approximation(&self, text: &str) -> String {
        if self.enable_colors {
            Colour::Black.bold().paint(text).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// 计算表达式的数值近似值
    fn calculate_approximation(&self, expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(number) => self.number_to_f64(number),
            Expression::Constant(constant) => self.constant_to_f64(constant),
            Expression::BinaryOp { op, left, right } => {
                let left_val = self.calculate_approximation(left)?;
                let right_val = self.calculate_approximation(right)?;
                self.apply_binary_op(op, left_val, right_val)
            }
            Expression::UnaryOp { op, operand } => {
                let operand_val = self.calculate_approximation(operand)?;
                self.apply_unary_op(op, operand_val)
            }
            Expression::Function { name, args } => {
                self.calculate_function_approximation(name, args)
            }
            _ => None, // 对于复杂类型暂不支持近似值计算
        }
    }
    
    /// 将数字转换为 f64
    fn number_to_f64(&self, number: &Number) -> Option<f64> {
        match number {
            Number::Integer(i) => i.to_f64(),
            Number::Rational(r) => {
                let numer = r.numer().to_f64()?;
                let denom = r.denom().to_f64()?;
                Some(numer / denom)
            }
            Number::Real(r) => r.to_f64(),
            Number::Float(f) => Some(*f),
            Number::Complex { real, imaginary } => {
                // 对于复数，如果虚部为0，返回实部
                if imaginary.is_zero() {
                    self.number_to_f64(real)
                } else {
                    None // 复数暂不支持简单的 f64 近似
                }
            }
            Number::Constant(constant) => self.constant_to_f64(constant),
            Number::Symbolic(_) => None, // 符号表达式需要递归计算
        }
    }
    
    /// 将常量转换为 f64
    fn constant_to_f64(&self, constant: &MathConstant) -> Option<f64> {
        match constant {
            MathConstant::Pi => Some(consts::PI),
            MathConstant::E => Some(consts::E),
            MathConstant::GoldenRatio => Some(1.618033988749895), // 黄金比例
            MathConstant::EulerGamma => Some(0.5772156649015329), // 欧拉-马歇罗尼常数
            MathConstant::Catalan => Some(0.915965594177219), // 卡塔兰常数
            MathConstant::I => None, // 虚数单位无法表示为实数
            MathConstant::PositiveInfinity => Some(f64::INFINITY),
            MathConstant::NegativeInfinity => Some(f64::NEG_INFINITY),
            MathConstant::Undefined => Some(f64::NAN),
        }
    }
    
    /// 应用二元运算
    fn apply_binary_op(&self, op: &BinaryOperator, left: f64, right: f64) -> Option<f64> {
        match op {
            BinaryOperator::Add => Some(left + right),
            BinaryOperator::Subtract => Some(left - right),
            BinaryOperator::Multiply => Some(left * right),
            BinaryOperator::Divide => {
                if right != 0.0 {
                    Some(left / right)
                } else {
                    None
                }
            }
            BinaryOperator::Power => Some(left.powf(right)),
            BinaryOperator::Modulo => Some(left % right),
            _ => None, // 其他运算符暂不支持
        }
    }
    
    /// 应用一元运算
    fn apply_unary_op(&self, op: &UnaryOperator, operand: f64) -> Option<f64> {
        match op {
            UnaryOperator::Negate => Some(-operand),
            UnaryOperator::Plus => Some(operand),
            UnaryOperator::Sqrt => {
                if operand >= 0.0 {
                    Some(operand.sqrt())
                } else {
                    None
                }
            }
            UnaryOperator::Abs => Some(operand.abs()),
            UnaryOperator::Sin => Some(operand.sin()),
            UnaryOperator::Cos => Some(operand.cos()),
            UnaryOperator::Tan => Some(operand.tan()),
            UnaryOperator::Asin => {
                if operand >= -1.0 && operand <= 1.0 {
                    Some(operand.asin())
                } else {
                    None
                }
            }
            UnaryOperator::Acos => {
                if operand >= -1.0 && operand <= 1.0 {
                    Some(operand.acos())
                } else {
                    None
                }
            }
            UnaryOperator::Atan => Some(operand.atan()),
            UnaryOperator::Sinh => Some(operand.sinh()),
            UnaryOperator::Cosh => Some(operand.cosh()),
            UnaryOperator::Tanh => Some(operand.tanh()),
            UnaryOperator::Asinh => Some(operand.asinh()),
            UnaryOperator::Acosh => {
                if operand >= 1.0 {
                    Some(operand.acosh())
                } else {
                    None
                }
            }
            UnaryOperator::Atanh => {
                if operand > -1.0 && operand < 1.0 {
                    Some(operand.atanh())
                } else {
                    None
                }
            }
            UnaryOperator::Ln => {
                if operand > 0.0 {
                    Some(operand.ln())
                } else {
                    None
                }
            }
            UnaryOperator::Log10 => {
                if operand > 0.0 {
                    Some(operand.log10())
                } else {
                    None
                }
            }
            UnaryOperator::Log2 => {
                if operand > 0.0 {
                    Some(operand.log2())
                } else {
                    None
                }
            }
            UnaryOperator::Exp => Some(operand.exp()),
            UnaryOperator::Factorial => {
                if operand >= 0.0 && operand.fract() == 0.0 && operand <= 170.0 {
                    // 简单的阶乘计算，限制在170以内避免溢出
                    let n = operand as u32;
                    let mut result = 1.0;
                    for i in 1..=n {
                        result *= i as f64;
                    }
                    Some(result)
                } else {
                    None
                }
            }
            _ => None, // 其他运算符暂不支持
        }
    }
    
    /// 计算函数的近似值
    fn calculate_function_approximation(&self, name: &str, args: &[Expression]) -> Option<f64> {
        match name {
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" |
            "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" |
            "ln" | "log" | "log10" | "log2" | "exp" | "sqrt" | "abs" => {
                if args.len() == 1 {
                    let arg_val = self.calculate_approximation(&args[0])?;
                    match name {
                        "sin" => Some(arg_val.sin()),
                        "cos" => Some(arg_val.cos()),
                        "tan" => Some(arg_val.tan()),
                        "asin" => if arg_val >= -1.0 && arg_val <= 1.0 { Some(arg_val.asin()) } else { None },
                        "acos" => if arg_val >= -1.0 && arg_val <= 1.0 { Some(arg_val.acos()) } else { None },
                        "atan" => Some(arg_val.atan()),
                        "sinh" => Some(arg_val.sinh()),
                        "cosh" => Some(arg_val.cosh()),
                        "tanh" => Some(arg_val.tanh()),
                        "asinh" => Some(arg_val.asinh()),
                        "acosh" => if arg_val >= 1.0 { Some(arg_val.acosh()) } else { None },
                        "atanh" => if arg_val > -1.0 && arg_val < 1.0 { Some(arg_val.atanh()) } else { None },
                        "ln" | "log" => if arg_val > 0.0 { Some(arg_val.ln()) } else { None },
                        "log10" => if arg_val > 0.0 { Some(arg_val.log10()) } else { None },
                        "log2" => if arg_val > 0.0 { Some(arg_val.log2()) } else { None },
                        "exp" => Some(arg_val.exp()),
                        "sqrt" => if arg_val >= 0.0 { Some(arg_val.sqrt()) } else { None },
                        "abs" => Some(arg_val.abs()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            "pow" => {
                if args.len() == 2 {
                    let base = self.calculate_approximation(&args[0])?;
                    let exp = self.calculate_approximation(&args[1])?;
                    Some(base.powf(exp))
                } else {
                    None
                }
            }
            "max" | "min" => {
                if args.len() >= 2 {
                    let values: Option<Vec<f64>> = args.iter()
                        .map(|arg| self.calculate_approximation(arg))
                        .collect();
                    let values = values?;
                    match name {
                        "max" => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)).into(),
                        "min" => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)).into(),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// 格式化数值并添加近似值
    fn format_number_with_approximation(&self, number: &Number) -> String {
        let formatted = self.format_number_basic(number);
        
        if self.show_approximations {
            if let Some(approx) = self.number_to_f64(number) {
                // 检查是否需要显示近似值
                if self.should_show_approximation(number, approx) {
                    let approx_str = format!("{:.prec$}", approx, prec = self.approximation_precision);
                    let colored_approx = self.colorize_approximation(&format!(" ≈ {}", approx_str));
                    return format!("{}{}", formatted, colored_approx);
                }
            }
        }
        
        formatted
    }
    
    /// 判断是否应该显示近似值
    fn should_show_approximation(&self, number: &Number, approx: f64) -> bool {
        match number {
            Number::Integer(_) => false, // 整数不需要近似值
            Number::Float(_) => false,   // 浮点数本身就是近似值
            Number::Rational(r) => {
                // 对于分数，如果分母不是1且结果不是整数，显示近似值
                r.denom() != &num_bigint::BigInt::from(1) && approx.fract() != 0.0
            }
            Number::Real(_) => true,     // 任意精度实数显示近似值
            Number::Complex { imaginary, .. } => imaginary.is_zero(), // 只有实数部分时显示
            Number::Constant(_) => true, // 常量显示近似值
            Number::Symbolic(_) => true, // 符号表达式显示近似值
        }
    }
    
    /// 基础数值格式化（不含近似值）
    fn format_number_basic(&self, number: &Number) -> String {
        let text = match number {
            Number::Integer(i) => i.to_string(),
            Number::Rational(r) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
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
                let real_str = self.format_number_basic(real);
                let imag_str = self.format_number_basic(imaginary);
                
                match (real.is_zero(), imaginary.is_zero()) {
                    (true, true) => "0".to_string(),
                    (true, false) => {
                        if imag_str == "1" {
                            self.colorize_constant("i")
                        } else if imag_str == "-1" {
                            format!("-{}", self.colorize_constant("i"))
                        } else {
                            format!("{}{}", self.colorize_number(&imag_str), self.colorize_constant("i"))
                        }
                    }
                    (false, true) => self.colorize_number(&real_str),
                    (false, false) => {
                        let imag_part = if imag_str == "1" {
                            self.colorize_constant("i")
                        } else if imag_str == "-1" {
                            format!("-{}", self.colorize_constant("i"))
                        } else if imag_str.starts_with('-') {
                            format!("{}{}", self.colorize_number(&imag_str), self.colorize_constant("i"))
                        } else {
                            format!("+{}{}", self.colorize_number(&imag_str), self.colorize_constant("i"))
                        };
                        format!("{}{}", self.colorize_number(&real_str), imag_part)
                    }
                }
            }
            Number::Constant(constant) => self.colorize_constant(constant.symbol()),
            Number::Symbolic(expr) => self.format(expr),
            Number::Float(f) => {
                if let Some(precision) = self.options.precision {
                    format!("{:.prec$}", f, prec = precision)
                } else {
                    f.to_string()
                }
            }
        };
        
        // 对于非复数情况，应用数字颜色
        match number {
            Number::Complex { .. } => text, // 复数已经在上面处理了颜色
            Number::Constant(_) => text,    // 常量已经在上面处理了颜色
            Number::Symbolic(_) => text,    // 符号表达式递归处理
            _ => self.colorize_number(&text),
        }
    }
    
    /// 格式化二元运算符
    fn format_binary_operator(&self, op: &BinaryOperator) -> String {
        let symbol = match op {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "×",
            BinaryOperator::Divide => "÷",
            BinaryOperator::Power => "^",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "≠",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "≤",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => "≥",
            BinaryOperator::And => "∧",
            BinaryOperator::Or => "∨",
            BinaryOperator::Union => "∪",
            BinaryOperator::Intersection => "∩",
            BinaryOperator::SetDifference => "\\",
            BinaryOperator::MatrixMultiply => "@",
            BinaryOperator::CrossProduct => "×",
            BinaryOperator::DotProduct => "·",
        };
        self.colorize_operator(symbol)
    }
    
    /// 格式化一元运算符
    fn format_unary_operator(&self, op: &UnaryOperator) -> String {
        let symbol = match op {
            UnaryOperator::Negate => "-",
            UnaryOperator::Plus => "+",
            UnaryOperator::Sqrt => "√",
            UnaryOperator::Abs => "abs",
            UnaryOperator::Sin => "sin",
            UnaryOperator::Cos => "cos",
            UnaryOperator::Tan => "tan",
            UnaryOperator::Asin => "arcsin",
            UnaryOperator::Acos => "arccos",
            UnaryOperator::Atan => "arctan",
            UnaryOperator::Sinh => "sinh",
            UnaryOperator::Cosh => "cosh",
            UnaryOperator::Tanh => "tanh",
            UnaryOperator::Asinh => "arcsinh",
            UnaryOperator::Acosh => "arccosh",
            UnaryOperator::Atanh => "arctanh",
            UnaryOperator::Ln => "ln",
            UnaryOperator::Log10 => "log₁₀",
            UnaryOperator::Log2 => "log₂",
            UnaryOperator::Exp => "exp",
            UnaryOperator::Factorial => "!",
            UnaryOperator::Gamma => "Γ",
            UnaryOperator::Not => "¬",
            UnaryOperator::Real => "Re",
            UnaryOperator::Imaginary => "Im",
            UnaryOperator::Conjugate => "*",
            UnaryOperator::Argument => "arg",
            UnaryOperator::Transpose => "ᵀ",
            UnaryOperator::Determinant => "det",
            UnaryOperator::Inverse => "⁻¹",
            UnaryOperator::Trace => "tr",
        };
        
        match op {
            UnaryOperator::Negate | UnaryOperator::Plus => self.colorize_operator(symbol),
            _ => self.colorize_function(symbol),
        }
    }
    
    /// 检查是否需要括号（继承自标准格式化器的逻辑）
    fn needs_parentheses(&self, expr: &Expression, parent_op: Option<&BinaryOperator>, is_right: bool) -> bool {
        if !self.options.use_parentheses {
            return false;
        }
        
        match (expr, parent_op) {
            (Expression::BinaryOp { op, .. }, Some(parent)) => {
                let expr_precedence = self.get_precedence(op);
                let parent_precedence = self.get_precedence(parent);
                
                if expr_precedence < parent_precedence {
                    return true;
                }
                
                if expr_precedence == parent_precedence {
                    match parent {
                        BinaryOperator::Subtract | BinaryOperator::Divide | BinaryOperator::Power => {
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
            format!("{}{}{}", 
                self.colorize_parentheses("("), 
                self.format(left), 
                self.colorize_parentheses(")"))
        } else {
            self.format(left)
        };
        
        let right_str = if self.needs_parentheses(right, Some(op), true) {
            format!("{}{}{}", 
                self.colorize_parentheses("("), 
                self.format(right), 
                self.colorize_parentheses(")"))
        } else {
            self.format(right)
        };
        
        let op_str = self.format_binary_operator(op);
        
        // 特殊处理某些运算符的格式
        match op {
            BinaryOperator::Power => {
                format!("{}{}{}", left_str, op_str, right_str)
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
                        let right_str = self.format(operand);
                        format!("{} {} {}", left_str, self.colorize_operator("-"), right_str)
                    }
                    Expression::Number(n) if n.is_negative() => {
                        // 处理负数：a + (-5) -> a - 5
                        let positive_n = -n.clone();
                        let right_str = self.format(&Expression::Number(positive_n));
                        format!("{} {} {}", left_str, self.colorize_operator("-"), right_str)
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
        let op_str = self.format_unary_operator(op);
        let operand_str = self.format(operand);
        
        match op {
            // 前缀运算符
            UnaryOperator::Negate | UnaryOperator::Plus | UnaryOperator::Not => {
                if matches!(operand, Expression::BinaryOp { .. }) && self.options.use_parentheses {
                    format!("{}{}{}{}", op_str, 
                        self.colorize_parentheses("("), 
                        operand_str, 
                        self.colorize_parentheses(")"))
                } else {
                    format!("{}{}", op_str, operand_str)
                }
            }
            // 后缀运算符
            UnaryOperator::Factorial | UnaryOperator::Transpose => {
                format!("{}{}", operand_str, op_str)
            }
            // 特殊格式
            UnaryOperator::Sqrt => {
                format!("{}{}{}{}", op_str, 
                    self.colorize_parentheses("("), 
                    operand_str, 
                    self.colorize_parentheses(")"))
            }
            UnaryOperator::Abs => {
                format!("{}{}{}",
                    self.colorize_operator("|"),
                    operand_str,
                    self.colorize_operator("|"))
            }
            UnaryOperator::Conjugate => {
                format!("{}{}", operand_str, op_str)
            }
            // 其他函数形式
            _ => {
                format!("{}{}{}{}", op_str,
                    self.colorize_parentheses("("), 
                    operand_str, 
                    self.colorize_parentheses(")"))
            }
        }
    }
    
    /// 格式化函数调用
    fn format_function(&self, name: &str, args: &[Expression]) -> String {
        let args_str: Vec<String> = args.iter().map(|arg| self.format(arg)).collect();
        format!("{}{}{}{}", 
            self.colorize_function(name),
            self.colorize_parentheses("("),
            args_str.join(&format!("{} ", self.colorize_operator(","))),
            self.colorize_parentheses(")"))
    }
}

impl Default for TerminalFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for TerminalFormatter {
    fn format(&self, expr: &Expression) -> String {
        let formatted = match expr {
            Expression::Number(number) => self.format_number_with_approximation(number),
            Expression::Variable(name) => self.colorize_variable(name),
            Expression::Constant(constant) => {
                let symbol = self.colorize_constant(constant.symbol());
                if self.show_approximations {
                    if let Some(approx) = self.constant_to_f64(constant) {
                        let approx_str = format!("{:.prec$}", approx, prec = self.approximation_precision);
                        let colored_approx = self.colorize_approximation(&format!(" ≈ {}", approx_str));
                        format!("{}{}", symbol, colored_approx)
                    } else {
                        symbol
                    }
                } else {
                    symbol
                }
            }
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
                let rows: Vec<String> = matrix.iter().map(|row| {
                    let elements: Vec<String> = row.iter().map(|elem| self.format(elem)).collect();
                    format!("{}{}{}", 
                        self.colorize_parentheses("["),
                        elements.join(&format!("{} ", self.colorize_operator(","))),
                        self.colorize_parentheses("]"))
                }).collect();
                format!("{}{}{}", 
                    self.colorize_parentheses("["),
                    rows.join(&format!("{} ", self.colorize_operator(","))),
                    self.colorize_parentheses("]"))
            }
            Expression::Vector(vector) => {
                let elements: Vec<String> = vector.iter().map(|elem| self.format(elem)).collect();
                format!("{}{}{}", 
                    self.colorize_parentheses("["),
                    elements.join(&format!("{} ", self.colorize_operator(","))),
                    self.colorize_parentheses("]"))
            }
            Expression::Set(set) => {
                let elements: Vec<String> = set.iter().map(|elem| self.format(elem)).collect();
                format!("{}{}{}", 
                    self.colorize_parentheses("{"),
                    elements.join(&format!("{} ", self.colorize_operator(","))),
                    self.colorize_parentheses("}"))
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                let start_bracket = if *start_inclusive { "[" } else { "(" };
                let end_bracket = if *end_inclusive { "]" } else { ")" };
                format!("{}{}{} {}{}", 
                    self.colorize_parentheses(start_bracket),
                    self.format(start),
                    self.colorize_operator(","),
                    self.format(end),
                    self.colorize_parentheses(end_bracket))
            }
        };
        
        // 如果整个表达式可以计算近似值，且不是简单的数字或常量，添加近似值
        if self.show_approximations && !matches!(expr, Expression::Number(_) | Expression::Constant(_)) {
            if let Some(approx) = self.calculate_approximation(expr) {
                let approx_str = format!("{:.prec$}", approx, prec = self.approximation_precision);
                let colored_approx = self.colorize_approximation(&format!(" ≈ {}", approx_str));
                format!("{}{}", formatted, colored_approx)
            } else {
                formatted
            }
        } else {
            formatted
        }
    }
    
    fn set_options(&mut self, options: FormatOptions) {
        self.options = options;
    }
}