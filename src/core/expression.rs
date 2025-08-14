//! # 表达式数据结构
//!
//! 定义数学表达式的核心数据结构，支持各种数学运算和操作。

use super::{Number, MathConstant, BinaryOperator, UnaryOperator};
use std::fmt::{self, Display};

/// 数学表达式的核心数据结构
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// 数值常量
    Number(Number),
    /// 变量
    Variable(String),
    /// 数学常量
    Constant(MathConstant),
    /// 二元运算
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// 一元运算
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    /// 函数调用
    Function {
        name: String,
        args: Vec<Expression>,
    },
    /// 矩阵表达式
    Matrix(Vec<Vec<Expression>>),
    /// 向量表达式
    Vector(Vec<Expression>),
    /// 集合表达式
    Set(Vec<Expression>),
    /// 区间表达式
    Interval {
        start: Box<Expression>,
        end: Box<Expression>,
        start_inclusive: bool,
        end_inclusive: bool,
    },
}

impl Expression {
    /// 创建数值表达式
    pub fn number(n: Number) -> Self {
        Expression::Number(n)
    }
    
    /// 创建变量表达式
    pub fn variable(name: impl Into<String>) -> Self {
        Expression::Variable(name.into())
    }
    
    /// 创建常量表达式
    pub fn constant(c: MathConstant) -> Self {
        Expression::Constant(c)
    }
    
    /// 创建二元运算表达式
    pub fn binary_op(op: BinaryOperator, left: Expression, right: Expression) -> Self {
        Expression::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    
    /// 创建一元运算表达式
    pub fn unary_op(op: UnaryOperator, operand: Expression) -> Self {
        Expression::UnaryOp {
            op,
            operand: Box::new(operand),
        }
    }
    
    /// 创建函数调用表达式
    pub fn function(name: impl Into<String>, args: Vec<Expression>) -> Self {
        Expression::Function {
            name: name.into(),
            args,
        }
    }
    
    /// 检查表达式是否为常量
    pub fn is_constant(&self) -> bool {
        match self {
            Expression::Number(_) | Expression::Constant(_) => true,
            Expression::Variable(_) => false,
            Expression::BinaryOp { left, right, .. } => {
                left.is_constant() && right.is_constant()
            }
            Expression::UnaryOp { operand, .. } => operand.is_constant(),
            Expression::Function { args, .. } => args.iter().all(|arg| arg.is_constant()),
            Expression::Matrix(rows) => {
                rows.iter().all(|row| row.iter().all(|expr| expr.is_constant()))
            }
            Expression::Vector(elements) => {
                elements.iter().all(|expr| expr.is_constant())
            }
            Expression::Set(elements) => {
                elements.iter().all(|expr| expr.is_constant())
            }
            Expression::Interval { start, end, .. } => {
                start.is_constant() && end.is_constant()
            }
        }
    }
    
    /// 获取表达式中的所有变量
    pub fn get_variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_variables(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }
    
    /// 递归收集变量名
    fn collect_variables(&self, vars: &mut Vec<String>) {
        match self {
            Expression::Variable(name) => vars.push(name.clone()),
            Expression::BinaryOp { left, right, .. } => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            }
            Expression::UnaryOp { operand, .. } => {
                operand.collect_variables(vars);
            }
            Expression::Function { args, .. } => {
                for arg in args {
                    arg.collect_variables(vars);
                }
            }
            Expression::Matrix(rows) => {
                for row in rows {
                    for expr in row {
                        expr.collect_variables(vars);
                    }
                }
            }
            Expression::Vector(elements) => {
                for expr in elements {
                    expr.collect_variables(vars);
                }
            }
            Expression::Set(elements) => {
                for expr in elements {
                    expr.collect_variables(vars);
                }
            }
            Expression::Interval { start, end, .. } => {
                start.collect_variables(vars);
                end.collect_variables(vars);
            }
            _ => {} // 数值和常量不包含变量
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Constant(c) => write!(f, "{}", c.symbol()),
            Expression::BinaryOp { op, left, right } => {
                // 根据运算符优先级决定是否需要括号
                let needs_left_parens = match left.as_ref() {
                    Expression::BinaryOp { op: left_op, .. } => {
                        left_op.precedence() < op.precedence() ||
                        (left_op.precedence() == op.precedence() && op.is_right_associative())
                    }
                    _ => false,
                };
                
                let needs_right_parens = match right.as_ref() {
                    Expression::BinaryOp { op: right_op, .. } => {
                        right_op.precedence() < op.precedence() ||
                        (right_op.precedence() == op.precedence() && !op.is_right_associative())
                    }
                    _ => false,
                };
                
                if needs_left_parens {
                    write!(f, "({})", left)?;
                } else {
                    write!(f, "{}", left)?;
                }
                
                write!(f, " {} ", op.symbol())?;
                
                if needs_right_parens {
                    write!(f, "({})", right)?;
                } else {
                    write!(f, "{}", right)?;
                }
                
                Ok(())
            }
            Expression::UnaryOp { op, operand } => {
                match op {
                    UnaryOperator::Negate => write!(f, "-{}", operand),
                    UnaryOperator::Plus => write!(f, "+{}", operand),
                    UnaryOperator::Factorial => write!(f, "{}!", operand),
                    UnaryOperator::Transpose => write!(f, "{}^T", operand),
                    UnaryOperator::Conjugate => write!(f, "{}*", operand),
                    _ => {
                        // 函数形式的一元运算符
                        write!(f, "{}({})", op.symbol(), operand)
                    }
                }
            }
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Matrix(rows) => {
                write!(f, "[")?;
                for (i, row) in rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "[")?;
                    for (j, elem) in row.iter().enumerate() {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", elem)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Expression::Vector(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Expression::Set(elements) => {
                write!(f, "{{")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "}}")
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                let left_bracket = if *start_inclusive { "[" } else { "(" };
                let right_bracket = if *end_inclusive { "]" } else { ")" };
                write!(f, "{}{}, {}{}", left_bracket, start, end, right_bracket)
            }
        }
    }
}