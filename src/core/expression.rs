//! # 表达式数据结构
//!
//! 定义数学表达式的核心数据结构，支持各种数学运算和操作。

use super::{Number, MathConstant, BinaryOperator, UnaryOperator, ExprType, NumericType};
use std::fmt::{self, Display};
use std::collections::HashMap;
use num_traits::{ToPrimitive, Zero, Signed};

/// 数学表达式的核心数据结构
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    
    /// 创建矩阵表达式
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Result<Self, String> {
        if rows.is_empty() {
            return Err("矩阵不能为空".to_string());
        }
        
        let cols = rows[0].len();
        if cols == 0 {
            return Err("矩阵行不能为空".to_string());
        }
        
        // 检查所有行的列数是否相同
        for (i, row) in rows.iter().enumerate() {
            if row.len() != cols {
                return Err(format!("矩阵第{}行的列数({})与第一行的列数({})不匹配", i + 1, row.len(), cols));
            }
        }
        
        Ok(Expression::Matrix(rows))
    }
    
    /// 创建向量表达式
    pub fn vector(elements: Vec<Expression>) -> Result<Self, String> {
        if elements.is_empty() {
            return Err("向量不能为空".to_string());
        }
        Ok(Expression::Vector(elements))
    }
    
    /// 创建集合表达式
    pub fn set(elements: Vec<Expression>) -> Self {
        Expression::Set(elements)
    }
    
    /// 创建区间表达式
    pub fn interval(start: Expression, end: Expression, start_inclusive: bool, end_inclusive: bool) -> Self {
        Expression::Interval {
            start: Box::new(start),
            end: Box::new(end),
            start_inclusive,
            end_inclusive,
        }
    }
    
    /// 创建加法表达式
    pub fn add(left: Expression, right: Expression) -> Self {
        Expression::binary_op(BinaryOperator::Add, left, right)
    }
    
    /// 创建减法表达式
    pub fn subtract(left: Expression, right: Expression) -> Self {
        Expression::binary_op(BinaryOperator::Subtract, left, right)
    }
    
    /// 创建乘法表达式
    pub fn multiply(left: Expression, right: Expression) -> Self {
        Expression::binary_op(BinaryOperator::Multiply, left, right)
    }
    
    /// 创建除法表达式
    pub fn divide(left: Expression, right: Expression) -> Self {
        Expression::binary_op(BinaryOperator::Divide, left, right)
    }
    
    /// 创建幂运算表达式
    pub fn power(base: Expression, exponent: Expression) -> Self {
        Expression::binary_op(BinaryOperator::Power, base, exponent)
    }
    
    /// 创建负号表达式
    pub fn negate(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Negate, operand)
    }
    
    /// 创建平方根表达式
    pub fn sqrt(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Sqrt, operand)
    }
    
    /// 创建绝对值表达式
    pub fn abs(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Abs, operand)
    }
    
    /// 创建正弦函数表达式
    pub fn sin(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Sin, operand)
    }
    
    /// 创建余弦函数表达式
    pub fn cos(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Cos, operand)
    }
    
    /// 创建正切函数表达式
    pub fn tan(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Tan, operand)
    }
    
    /// 创建自然对数表达式
    pub fn ln(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Ln, operand)
    }
    
    /// 创建指数函数表达式
    pub fn exp(operand: Expression) -> Self {
        Expression::unary_op(UnaryOperator::Exp, operand)
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
    
    /// 推断表达式的类型
    pub fn infer_type(&self) -> ExprType {
        match self {
            Expression::Number(n) => ExprType::Numeric(n.get_numeric_type()),
            Expression::Variable(_) => ExprType::Symbolic,
            Expression::Constant(c) => match c {
                MathConstant::I => ExprType::Numeric(NumericType::Complex),
                MathConstant::PositiveInfinity | MathConstant::NegativeInfinity => ExprType::Numeric(NumericType::Real),
                MathConstant::Undefined => ExprType::Unknown,
                _ => ExprType::Numeric(NumericType::Real),
            },
            Expression::BinaryOp { op, left, right } => {
                let left_type = left.infer_type();
                let right_type = right.infer_type();
                self.infer_binary_op_type(op, &left_type, &right_type)
            }
            Expression::UnaryOp { op, operand } => {
                let operand_type = operand.infer_type();
                self.infer_unary_op_type(op, &operand_type)
            }
            Expression::Function { name, args } => {
                let arg_types: Vec<ExprType> = args.iter().map(|arg| arg.infer_type()).collect();
                self.infer_function_type(name, &arg_types)
            }
            Expression::Matrix(rows) => {
                if rows.is_empty() {
                    return ExprType::Unknown;
                }
                let rows_count = rows.len();
                let cols_count = rows[0].len();
                
                // 推断矩阵元素的公共类型
                let mut element_type = ExprType::Unknown;
                for row in rows {
                    for elem in row {
                        let elem_type = elem.infer_type();
                        element_type = self.common_type(&element_type, &elem_type);
                    }
                }
                
                ExprType::Matrix(rows_count, cols_count, Box::new(element_type))
            }
            Expression::Vector(elements) => {
                if elements.is_empty() {
                    return ExprType::Unknown;
                }
                
                let mut element_type = ExprType::Unknown;
                for elem in elements {
                    let elem_type = elem.infer_type();
                    element_type = self.common_type(&element_type, &elem_type);
                }
                
                ExprType::Vector(elements.len(), Box::new(element_type))
            }
            Expression::Set(elements) => {
                let mut element_type = ExprType::Unknown;
                for elem in elements {
                    let elem_type = elem.infer_type();
                    element_type = self.common_type(&element_type, &elem_type);
                }
                
                ExprType::Set(Box::new(element_type))
            }
            Expression::Interval { start, end, .. } => {
                let start_type = start.infer_type();
                let end_type = end.infer_type();
                let common_type = self.common_type(&start_type, &end_type);
                
                ExprType::Interval(Box::new(common_type))
            }
        }
    }
    
    /// 推断二元运算的结果类型
    fn infer_binary_op_type(&self, op: &BinaryOperator, left_type: &ExprType, right_type: &ExprType) -> ExprType {
        match op {
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                match (left_type, right_type) {
                    (ExprType::Numeric(l), ExprType::Numeric(r)) => {
                        ExprType::Numeric(l.common_type(r))
                    }
                    _ => ExprType::Symbolic
                }
            }
            BinaryOperator::Power => {
                match (left_type, right_type) {
                    (ExprType::Numeric(NumericType::Integer), ExprType::Numeric(NumericType::Integer)) => {
                        ExprType::Numeric(NumericType::Rational)
                    }
                    (ExprType::Numeric(l), ExprType::Numeric(r)) => {
                        ExprType::Numeric(l.common_type(r))
                    }
                    _ => ExprType::Symbolic
                }
            }
            BinaryOperator::Modulo => {
                match (left_type, right_type) {
                    (ExprType::Numeric(NumericType::Integer), ExprType::Numeric(NumericType::Integer)) => {
                        ExprType::Numeric(NumericType::Integer)
                    }
                    _ => ExprType::Symbolic
                }
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::Less | 
            BinaryOperator::LessEqual | BinaryOperator::Greater | BinaryOperator::GreaterEqual => {
                ExprType::Numeric(NumericType::Integer) // 布尔值用整数表示
            }
            BinaryOperator::And | BinaryOperator::Or => {
                ExprType::Numeric(NumericType::Integer) // 布尔值用整数表示
            }
            BinaryOperator::Union | BinaryOperator::Intersection | BinaryOperator::SetDifference => {
                match (left_type, right_type) {
                    (ExprType::Set(l), ExprType::Set(r)) => {
                        let common_elem_type = self.common_type(l, r);
                        ExprType::Set(Box::new(common_elem_type))
                    }
                    _ => ExprType::Unknown
                }
            }
            BinaryOperator::MatrixMultiply => {
                match (left_type, right_type) {
                    (ExprType::Matrix(m, n, l_elem), ExprType::Matrix(p, q, r_elem)) => {
                        if n == p {
                            let common_elem_type = self.common_type(l_elem, r_elem);
                            ExprType::Matrix(*m, *q, Box::new(common_elem_type))
                        } else {
                            ExprType::Unknown // 矩阵维度不匹配
                        }
                    }
                    _ => ExprType::Unknown
                }
            }
            BinaryOperator::DotProduct => {
                match (left_type, right_type) {
                    (ExprType::Vector(m, l_elem), ExprType::Vector(n, r_elem)) => {
                        if m == n {
                            self.common_type(l_elem, r_elem)
                        } else {
                            ExprType::Unknown // 向量维度不匹配
                        }
                    }
                    _ => ExprType::Unknown
                }
            }
            BinaryOperator::CrossProduct => {
                match (left_type, right_type) {
                    (ExprType::Vector(3, l_elem), ExprType::Vector(3, r_elem)) => {
                        let common_elem_type = self.common_type(l_elem, r_elem);
                        ExprType::Vector(3, Box::new(common_elem_type))
                    }
                    _ => ExprType::Unknown // 叉积只适用于3维向量
                }
            }
        }
    }
    
    /// 推断一元运算的结果类型
    fn infer_unary_op_type(&self, op: &UnaryOperator, operand_type: &ExprType) -> ExprType {
        match op {
            UnaryOperator::Negate | UnaryOperator::Plus => operand_type.clone(),
            UnaryOperator::Abs => {
                match operand_type {
                    ExprType::Numeric(NumericType::Complex) => ExprType::Numeric(NumericType::Real),
                    _ => operand_type.clone()
                }
            }
            UnaryOperator::Sqrt => {
                match operand_type {
                    ExprType::Numeric(NumericType::Integer) => ExprType::Numeric(NumericType::Real),
                    ExprType::Numeric(NumericType::Rational) => ExprType::Numeric(NumericType::Real),
                    _ => operand_type.clone()
                }
            }
            UnaryOperator::Sin | UnaryOperator::Cos | UnaryOperator::Tan |
            UnaryOperator::Asin | UnaryOperator::Acos | UnaryOperator::Atan |
            UnaryOperator::Sinh | UnaryOperator::Cosh | UnaryOperator::Tanh |
            UnaryOperator::Asinh | UnaryOperator::Acosh | UnaryOperator::Atanh => {
                match operand_type {
                    ExprType::Numeric(NumericType::Complex) => ExprType::Numeric(NumericType::Complex),
                    ExprType::Numeric(_) => ExprType::Numeric(NumericType::Real),
                    _ => ExprType::Symbolic
                }
            }
            UnaryOperator::Ln | UnaryOperator::Log10 | UnaryOperator::Log2 => {
                match operand_type {
                    ExprType::Numeric(NumericType::Complex) => ExprType::Numeric(NumericType::Complex),
                    ExprType::Numeric(_) => ExprType::Numeric(NumericType::Real),
                    _ => ExprType::Symbolic
                }
            }
            UnaryOperator::Exp => {
                match operand_type {
                    ExprType::Numeric(NumericType::Complex) => ExprType::Numeric(NumericType::Complex),
                    ExprType::Numeric(_) => ExprType::Numeric(NumericType::Real),
                    _ => ExprType::Symbolic
                }
            }
            UnaryOperator::Factorial => {
                match operand_type {
                    ExprType::Numeric(NumericType::Integer) => ExprType::Numeric(NumericType::Integer),
                    _ => ExprType::Symbolic
                }
            }
            UnaryOperator::Gamma => ExprType::Numeric(NumericType::Real),
            UnaryOperator::Not => ExprType::Numeric(NumericType::Integer), // 布尔值用整数表示
            UnaryOperator::Real | UnaryOperator::Imaginary => ExprType::Numeric(NumericType::Real),
            UnaryOperator::Conjugate => operand_type.clone(),
            UnaryOperator::Argument => ExprType::Numeric(NumericType::Real),
            UnaryOperator::Transpose => {
                match operand_type {
                    ExprType::Matrix(m, n, elem_type) => ExprType::Matrix(*n, *m, elem_type.clone()),
                    ExprType::Vector(n, elem_type) => ExprType::Matrix(*n, 1, elem_type.clone()),
                    _ => ExprType::Unknown
                }
            }
            UnaryOperator::Determinant => {
                match operand_type {
                    ExprType::Matrix(m, n, elem_type) if m == n => elem_type.as_ref().clone(),
                    _ => ExprType::Unknown
                }
            }
            UnaryOperator::Inverse => {
                match operand_type {
                    ExprType::Matrix(m, n, elem_type) if m == n => {
                        ExprType::Matrix(*m, *n, elem_type.clone())
                    }
                    _ => ExprType::Unknown
                }
            }
            UnaryOperator::Trace => {
                match operand_type {
                    ExprType::Matrix(m, n, elem_type) if m == n => elem_type.as_ref().clone(),
                    _ => ExprType::Unknown
                }
            }
        }
    }
    
    /// 推断函数调用的结果类型
    fn infer_function_type(&self, name: &str, arg_types: &[ExprType]) -> ExprType {
        match name {
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan" |
            "sinh" | "cosh" | "tanh" | "asinh" | "acosh" | "atanh" |
            "ln" | "log" | "exp" | "sqrt" | "abs" => {
                if arg_types.len() == 1 {
                    match &arg_types[0] {
                        ExprType::Numeric(NumericType::Complex) => ExprType::Numeric(NumericType::Complex),
                        ExprType::Numeric(_) => ExprType::Numeric(NumericType::Real),
                        _ => ExprType::Symbolic
                    }
                } else {
                    ExprType::Unknown
                }
            }
            "max" | "min" => {
                if arg_types.len() >= 2 {
                    let mut result_type = arg_types[0].clone();
                    for arg_type in &arg_types[1..] {
                        result_type = self.common_type(&result_type, arg_type);
                    }
                    result_type
                } else {
                    ExprType::Unknown
                }
            }
            _ => ExprType::Unknown // 未知函数
        }
    }
    
    /// 获取两个类型的公共类型
    fn common_type(&self, type1: &ExprType, type2: &ExprType) -> ExprType {
        match (type1, type2) {
            (ExprType::Numeric(n1), ExprType::Numeric(n2)) => {
                ExprType::Numeric(n1.common_type(n2))
            }
            (ExprType::Unknown, t) | (t, ExprType::Unknown) => t.clone(),
            (ExprType::Symbolic, _) | (_, ExprType::Symbolic) => ExprType::Symbolic,
            (t1, t2) if t1 == t2 => t1.clone(),
            _ => ExprType::Unknown
        }
    }
    
    /// 验证表达式的类型一致性
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Expression::BinaryOp { op, left, right } => {
                left.validate()?;
                right.validate()?;
                
                let left_type = left.infer_type();
                let right_type = right.infer_type();
                
                match op {
                    BinaryOperator::MatrixMultiply => {
                        match (&left_type, &right_type) {
                            (ExprType::Matrix(_, n, _), ExprType::Matrix(p, _, _)) => {
                                if n != p {
                                    return Err(format!("矩阵乘法维度不匹配：{}×? 和 {}×?", n, p));
                                }
                            }
                            _ => return Err("矩阵乘法只能应用于矩阵".to_string())
                        }
                    }
                    BinaryOperator::DotProduct => {
                        match (&left_type, &right_type) {
                            (ExprType::Vector(m, _), ExprType::Vector(n, _)) => {
                                if m != n {
                                    return Err(format!("向量点积维度不匹配：{} 和 {}", m, n));
                                }
                            }
                            _ => return Err("点积只能应用于向量".to_string())
                        }
                    }
                    BinaryOperator::CrossProduct => {
                        match (&left_type, &right_type) {
                            (ExprType::Vector(3, _), ExprType::Vector(3, _)) => {}
                            _ => return Err("叉积只能应用于3维向量".to_string())
                        }
                    }
                    _ => {} // 其他运算符暂不做特殊验证
                }
                
                Ok(())
            }
            Expression::UnaryOp { op, operand } => {
                operand.validate()?;
                
                let operand_type = operand.infer_type();
                
                match op {
                    UnaryOperator::Determinant | UnaryOperator::Inverse | UnaryOperator::Trace => {
                        match &operand_type {
                            ExprType::Matrix(m, n, _) => {
                                if m != n {
                                    return Err(format!("{}只能应用于方阵", op.name()));
                                }
                            }
                            _ => return Err(format!("{}只能应用于矩阵", op.name()))
                        }
                    }
                    UnaryOperator::Factorial => {
                        match &operand_type {
                            ExprType::Numeric(NumericType::Integer) | ExprType::Symbolic => {}
                            _ => return Err("阶乘只能应用于整数".to_string())
                        }
                    }
                    _ => {} // 其他运算符暂不做特殊验证
                }
                
                Ok(())
            }
            Expression::Function { args, .. } => {
                for arg in args {
                    arg.validate()?;
                }
                Ok(())
            }
            Expression::Matrix(rows) => {
                if rows.is_empty() {
                    return Err("矩阵不能为空".to_string());
                }
                
                let cols = rows[0].len();
                if cols == 0 {
                    return Err("矩阵行不能为空".to_string());
                }
                
                for (i, row) in rows.iter().enumerate() {
                    if row.len() != cols {
                        return Err(format!("矩阵第{}行的列数({})与第一行的列数({})不匹配", i + 1, row.len(), cols));
                    }
                    
                    for elem in row {
                        elem.validate()?;
                    }
                }
                
                Ok(())
            }
            Expression::Vector(elements) => {
                if elements.is_empty() {
                    return Err("向量不能为空".to_string());
                }
                
                for elem in elements {
                    elem.validate()?;
                }
                
                Ok(())
            }
            Expression::Set(elements) => {
                for elem in elements {
                    elem.validate()?;
                }
                Ok(())
            }
            Expression::Interval { start, end, .. } => {
                start.validate()?;
                end.validate()?;
                Ok(())
            }
            _ => Ok(()) // 基本表达式不需要特殊验证
        }
    }
    
    /// 获取表达式的复杂度（用于性能优化）
    pub fn complexity(&self) -> usize {
        match self {
            Expression::Number(_) | Expression::Variable(_) | Expression::Constant(_) => 1,
            Expression::BinaryOp { left, right, .. } => 1 + left.complexity() + right.complexity(),
            Expression::UnaryOp { operand, .. } => 1 + operand.complexity(),
            Expression::Function { args, .. } => {
                1 + args.iter().map(|arg| arg.complexity()).sum::<usize>()
            }
            Expression::Matrix(rows) => {
                1 + rows.iter().map(|row| {
                    row.iter().map(|elem| elem.complexity()).sum::<usize>()
                }).sum::<usize>()
            }
            Expression::Vector(elements) => {
                1 + elements.iter().map(|elem| elem.complexity()).sum::<usize>()
            }
            Expression::Set(elements) => {
                1 + elements.iter().map(|elem| elem.complexity()).sum::<usize>()
            }
            Expression::Interval { start, end, .. } => {
                1 + start.complexity() + end.complexity()
            }
        }
    }
    
    /// 使用变量值替换表达式中的变量
    pub fn substitute(&self, variables: &HashMap<String, Expression>) -> Expression {
        match self {
            Expression::Variable(name) => {
                variables.get(name).cloned().unwrap_or_else(|| self.clone())
            }
            Expression::BinaryOp { op, left, right } => {
                Expression::BinaryOp {
                    op: op.clone(),
                    left: Box::new(left.substitute(variables)),
                    right: Box::new(right.substitute(variables)),
                }
            }
            Expression::UnaryOp { op, operand } => {
                Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(operand.substitute(variables)),
                }
            }
            Expression::Function { name, args } => {
                Expression::Function {
                    name: name.clone(),
                    args: args.iter().map(|arg| arg.substitute(variables)).collect(),
                }
            }
            Expression::Matrix(rows) => {
                Expression::Matrix(
                    rows.iter()
                        .map(|row| row.iter().map(|elem| elem.substitute(variables)).collect())
                        .collect()
                )
            }
            Expression::Vector(elements) => {
                Expression::Vector(
                    elements.iter().map(|elem| elem.substitute(variables)).collect()
                )
            }
            Expression::Set(elements) => {
                Expression::Set(
                    elements.iter().map(|elem| elem.substitute(variables)).collect()
                )
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                Expression::Interval {
                    start: Box::new(start.substitute(variables)),
                    end: Box::new(end.substitute(variables)),
                    start_inclusive: *start_inclusive,
                    end_inclusive: *end_inclusive,
                }
            }
            // 对于数值和常量，直接返回克隆
            _ => self.clone(),
        }
    }
    
    /// 使用数值替换表达式中的变量
    pub fn substitute_numbers(&self, variables: &HashMap<String, Number>) -> Expression {
        let expr_vars: HashMap<String, Expression> = variables
            .iter()
            .map(|(k, v)| (k.clone(), Expression::Number(v.clone())))
            .collect();
        self.substitute(&expr_vars)
    }
    
    /// 求值表达式，返回数值结果
    pub fn evaluate(&self, variables: &HashMap<String, Number>) -> Result<Number, String> {
        // 首先进行变量替换
        let substituted = self.substitute_numbers(variables);
        
        // 然后对替换后的表达式进行求值
        substituted.evaluate_exact()
    }
    
    /// 精确求值表达式（不包含变量）
    pub fn evaluate_exact(&self) -> Result<Number, String> {
        match self {
            Expression::Number(n) => Ok(n.clone()),
            
            Expression::Variable(name) => {
                Err(format!("未定义的变量: {}", name))
            }
            
            Expression::Constant(c) => {
                Ok(self.evaluate_constant(c))
            }
            
            Expression::BinaryOp { op, left, right } => {
                let left_val = left.evaluate_exact()?;
                let right_val = right.evaluate_exact()?;
                self.evaluate_binary_op(op, &left_val, &right_val)
            }
            
            Expression::UnaryOp { op, operand } => {
                let operand_val = operand.evaluate_exact()?;
                self.evaluate_unary_op(op, &operand_val)
            }
            
            Expression::Function { name, args } => {
                // 对于函数调用，我们需要特殊处理，因为有些函数需要检查原始表达式结构
                self.evaluate_function_with_expressions(name, args)
            }
            
            Expression::Matrix(rows) => {
                // 矩阵求值：对每个元素求值
                let evaluated_rows: Result<Vec<Vec<Number>>, String> = rows
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|elem| elem.evaluate_exact())
                            .collect()
                    })
                    .collect();
                
                match evaluated_rows {
                    Ok(values) => {
                        // 将矩阵包装为符号表达式，因为 Number 不直接支持矩阵
                        let matrix_expr = Expression::Matrix(
                            values.into_iter()
                                .map(|row| row.into_iter().map(Expression::Number).collect())
                                .collect()
                        );
                        Ok(Number::Symbolic(Box::new(matrix_expr)))
                    }
                    Err(e) => Err(e),
                }
            }
            
            Expression::Vector(elements) => {
                // 向量求值：对每个元素求值
                let evaluated_elements: Result<Vec<Number>, String> = elements
                    .iter()
                    .map(|elem| elem.evaluate_exact())
                    .collect();
                
                match evaluated_elements {
                    Ok(values) => {
                        // 将向量包装为符号表达式
                        let vector_expr = Expression::Vector(
                            values.into_iter().map(Expression::Number).collect()
                        );
                        Ok(Number::Symbolic(Box::new(vector_expr)))
                    }
                    Err(e) => Err(e),
                }
            }
            
            Expression::Set(elements) => {
                // 集合求值：对每个元素求值
                let evaluated_elements: Result<Vec<Number>, String> = elements
                    .iter()
                    .map(|elem| elem.evaluate_exact())
                    .collect();
                
                match evaluated_elements {
                    Ok(values) => {
                        // 将集合包装为符号表达式
                        let set_expr = Expression::Set(
                            values.into_iter().map(Expression::Number).collect()
                        );
                        Ok(Number::Symbolic(Box::new(set_expr)))
                    }
                    Err(e) => Err(e),
                }
            }
            
            Expression::Interval { start, end, .. } => {
                let start_val = start.evaluate_exact()?;
                let end_val = end.evaluate_exact()?;
                
                // 将区间包装为符号表达式
                let interval_expr = Expression::Interval {
                    start: Box::new(Expression::Number(start_val)),
                    end: Box::new(Expression::Number(end_val)),
                    start_inclusive: true, // 简化处理
                    end_inclusive: true,
                };
                Ok(Number::Symbolic(Box::new(interval_expr)))
            }
        }
    }
    
    /// 求值数学常量
    fn evaluate_constant(&self, constant: &MathConstant) -> Number {
        match constant {
            MathConstant::Pi => {
                // 返回符号表示，保持精确性
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::Pi)))
            }
            MathConstant::E => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::E)))
            }
            MathConstant::I => {
                // 虚数单位可以精确表示
                Number::i()
            }
            MathConstant::EulerGamma => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::EulerGamma)))
            }
            MathConstant::GoldenRatio => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::GoldenRatio)))
            }
            MathConstant::Catalan => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::Catalan)))
            }
            MathConstant::PositiveInfinity => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::PositiveInfinity)))
            }
            MathConstant::NegativeInfinity => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::NegativeInfinity)))
            }
            MathConstant::Undefined => {
                Number::Symbolic(Box::new(Expression::Constant(MathConstant::Undefined)))
            }
        }
    }
    
    /// 求值二元运算
    fn evaluate_binary_op(&self, op: &BinaryOperator, left: &Number, right: &Number) -> Result<Number, String> {
        match op {
            BinaryOperator::Add => Ok(left.clone() + right.clone()),
            BinaryOperator::Subtract => Ok(left.clone() - right.clone()),
            BinaryOperator::Multiply => Ok(left.clone() * right.clone()),
            BinaryOperator::Divide => {
                if right.is_zero() {
                    Err("除零错误".to_string())
                } else {
                    Ok(left.clone() / right.clone())
                }
            }
            BinaryOperator::Power => {
                self.evaluate_power(left, right)
            }
            BinaryOperator::Modulo => {
                self.evaluate_modulo(left, right)
            }
            BinaryOperator::Equal => {
                Ok(if left == right { Number::one() } else { Number::zero() })
            }
            BinaryOperator::NotEqual => {
                Ok(if left != right { Number::one() } else { Number::zero() })
            }
            BinaryOperator::Less => {
                self.evaluate_comparison(left, right, |a, b| a < b)
            }
            BinaryOperator::LessEqual => {
                self.evaluate_comparison(left, right, |a, b| a <= b)
            }
            BinaryOperator::Greater => {
                self.evaluate_comparison(left, right, |a, b| a > b)
            }
            BinaryOperator::GreaterEqual => {
                self.evaluate_comparison(left, right, |a, b| a >= b)
            }
            BinaryOperator::And => {
                Ok(if !left.is_zero() && !right.is_zero() { Number::one() } else { Number::zero() })
            }
            BinaryOperator::Or => {
                Ok(if !left.is_zero() || !right.is_zero() { Number::one() } else { Number::zero() })
            }
            // 对于复杂运算，返回符号表示
            _ => {
                Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
                    op: op.clone(),
                    left: Box::new(Expression::Number(left.clone())),
                    right: Box::new(Expression::Number(right.clone())),
                })))
            }
        }
    }
    
    /// 求值一元运算
    fn evaluate_unary_op(&self, op: &UnaryOperator, operand: &Number) -> Result<Number, String> {
        match op {
            UnaryOperator::Negate => Ok(-operand.clone()),
            UnaryOperator::Plus => Ok(operand.clone()),
            UnaryOperator::Abs => operand.abs().map_err(|e| format!("{}", e)),
            UnaryOperator::Sqrt => {
                self.evaluate_sqrt(operand)
            }
            UnaryOperator::Factorial => {
                self.evaluate_factorial(operand)
            }
            // 对于三角函数和其他复杂函数，返回符号表示以保持精确性
            UnaryOperator::Sin | UnaryOperator::Cos | UnaryOperator::Tan |
            UnaryOperator::Asin | UnaryOperator::Acos | UnaryOperator::Atan |
            UnaryOperator::Sinh | UnaryOperator::Cosh | UnaryOperator::Tanh |
            UnaryOperator::Asinh | UnaryOperator::Acosh | UnaryOperator::Atanh |
            UnaryOperator::Ln | UnaryOperator::Log10 | UnaryOperator::Log2 |
            UnaryOperator::Exp | UnaryOperator::Gamma => {
                Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(Expression::Number(operand.clone())),
                })))
            }
            UnaryOperator::Real => {
                match operand {
                    Number::Complex { real, .. } => Ok(*real.clone()),
                    _ => Ok(operand.clone()),
                }
            }
            UnaryOperator::Imaginary => {
                match operand {
                    Number::Complex { imaginary, .. } => Ok(*imaginary.clone()),
                    _ => Ok(Number::zero()),
                }
            }
            UnaryOperator::Conjugate => {
                match operand {
                    Number::Complex { real, imaginary } => {
                        Ok(Number::Complex {
                            real: real.clone(),
                            imaginary: Box::new(imaginary.clone().neg()),
                        })
                    }
                    _ => Ok(operand.clone()),
                }
            }
            // 其他运算符返回符号表示
            _ => {
                Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(Expression::Number(operand.clone())),
                })))
            }
        }
    }
    
    /// 求值幂运算
    fn evaluate_power(&self, base: &Number, exponent: &Number) -> Result<Number, String> {
        // 处理一些特殊情况
        if exponent.is_zero() {
            return Ok(Number::one());
        }
        if exponent.is_one() {
            return Ok(base.clone());
        }
        if base.is_zero() {
            if exponent.is_positive() {
                return Ok(Number::zero());
            } else {
                return Err("0的负数次幂未定义".to_string());
            }
        }
        if base.is_one() {
            return Ok(Number::one());
        }
        
        // 对于整数底数和小的正整数指数，可以精确计算
        if let (Some(base_int), Some(exp_int)) = (base.to_integer(), exponent.to_integer()) {
            if exp_int >= num_bigint::BigInt::from(0) && exp_int <= num_bigint::BigInt::from(100) {
                let exp_u32 = exp_int.to_u32();
                if let Some(exp_val) = exp_u32 {
                    return Ok(Number::Integer(base_int.pow(exp_val)));
                }
            }
        }
        
        // 其他情况返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(Expression::Number(base.clone())),
            right: Box::new(Expression::Number(exponent.clone())),
        })))
    }
    
    /// 求值取模运算
    fn evaluate_modulo(&self, left: &Number, right: &Number) -> Result<Number, String> {
        if right.is_zero() {
            return Err("模零错误".to_string());
        }
        
        match (left.to_integer(), right.to_integer()) {
            (Some(a), Some(b)) => {
                Ok(Number::Integer(a % b))
            }
            _ => {
                // 非整数的取模运算返回符号表示
                Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
                    op: BinaryOperator::Modulo,
                    left: Box::new(Expression::Number(left.clone())),
                    right: Box::new(Expression::Number(right.clone())),
                })))
            }
        }
    }
    
    /// 求值比较运算
    fn evaluate_comparison<F>(&self, left: &Number, right: &Number, compare: F) -> Result<Number, String>
    where
        F: Fn(f64, f64) -> bool,
    {
        // 对于数值比较，我们使用近似值
        let left_approx = left.approximate();
        let right_approx = right.approximate();
        
        if left_approx.is_nan() || right_approx.is_nan() {
            // 如果无法比较，返回符号表示
            return Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
                op: BinaryOperator::Less, // 这里应该根据实际的比较运算符来设置
                left: Box::new(Expression::Number(left.clone())),
                right: Box::new(Expression::Number(right.clone())),
            })));
        }
        
        Ok(if compare(left_approx, right_approx) { Number::one() } else { Number::zero() })
    }
    
    /// 求值平方根
    fn evaluate_sqrt(&self, operand: &Number) -> Result<Number, String> {
        match operand {
            Number::Integer(i) => {
                // 检查是否为完全平方数
                let sqrt_approx = (i.to_f64().unwrap_or(0.0)).sqrt();
                let sqrt_int = sqrt_approx.round() as i64;
                if (sqrt_int * sqrt_int) as f64 == sqrt_approx * sqrt_approx {
                    Ok(Number::Integer(num_bigint::BigInt::from(sqrt_int)))
                } else {
                    // 不是完全平方数，返回符号表示
                    Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                        op: UnaryOperator::Sqrt,
                        operand: Box::new(Expression::Number(operand.clone())),
                    })))
                }
            }
            Number::Rational(r) => {
                // 有理数的平方根通常是无理数，返回符号表示
                Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                    op: UnaryOperator::Sqrt,
                    operand: Box::new(Expression::Number(operand.clone())),
                })))
            }
            _ => {
                // 其他情况返回符号表示
                Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                    op: UnaryOperator::Sqrt,
                    operand: Box::new(Expression::Number(operand.clone())),
                })))
            }
        }
    }
    
    /// 求值阶乘
    fn evaluate_factorial(&self, operand: &Number) -> Result<Number, String> {
        if let Some(n) = operand.to_integer() {
            if n < num_bigint::BigInt::from(0) {
                return Err("负数的阶乘未定义".to_string());
            }
            
            if n > num_bigint::BigInt::from(1000) {
                // 对于大数，返回符号表示以避免计算过慢
                return Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                    op: UnaryOperator::Factorial,
                    operand: Box::new(Expression::Number(operand.clone())),
                })));
            }
            
            let mut result = num_bigint::BigInt::from(1);
            let mut i = num_bigint::BigInt::from(1);
            while i <= n {
                result *= &i;
                i += 1;
            }
            
            Ok(Number::Integer(result))
        } else {
            Err("阶乘只能应用于非负整数".to_string())
        }
    }
    
    /// 求值函数调用（使用表达式参数）
    fn evaluate_function_with_expressions(&self, name: &str, args: &[Expression]) -> Result<Number, String> {
        match name {
            // 三角函数
            "sin" => {
                if args.len() != 1 {
                    return Err("sin函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function_with_expr("sin", &args[0])
            }
            "cos" => {
                if args.len() != 1 {
                    return Err("cos函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function_with_expr("cos", &args[0])
            }
            "tan" => {
                if args.len() != 1 {
                    return Err("tan函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function_with_expr("tan", &args[0])
            }
            
            // 指数和对数函数
            "exp" => {
                if args.len() != 1 {
                    return Err("exp函数需要一个参数".to_string());
                }
                self.evaluate_exponential_function_with_expr(&args[0])
            }
            "ln" | "log" => {
                if args.len() != 1 {
                    return Err("ln函数需要一个参数".to_string());
                }
                let arg_val = args[0].evaluate_exact()?;
                self.evaluate_logarithm_function(&arg_val)
            }
            
            // 其他函数，先求值参数再调用原有方法
            _ => {
                let arg_values: Result<Vec<Number>, String> = args
                    .iter()
                    .map(|arg| arg.evaluate_exact())
                    .collect();
                let arg_values = arg_values?;
                self.evaluate_function(name, &arg_values)
            }
        }
    }
    
    /// 求值三角函数（使用表达式参数）
    fn evaluate_trigonometric_function_with_expr(&self, name: &str, arg: &Expression) -> Result<Number, String> {
        use super::MathConstant;
        
        // 检查特殊的表达式模式
        match arg {
            // 直接的数学常量
            Expression::Constant(MathConstant::Pi) => {
                match name {
                    "sin" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    "cos" => return Ok(Number::Integer(num_bigint::BigInt::from(-1))),
                    "tan" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            
            // π 的分数
            Expression::BinaryOp { op: BinaryOperator::Divide, left, right } => {
                if let (Expression::Constant(MathConstant::Pi), Expression::Number(Number::Integer(n))) = 
                    (left.as_ref(), right.as_ref()) {
                    if n == &num_bigint::BigInt::from(2) {
                        // π/2 的三角函数值
                        match name {
                            "sin" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                            "cos" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                            "tan" => return Ok(Number::Constant(MathConstant::Undefined)),
                            _ => {}
                        }
                    } else if n == &num_bigint::BigInt::from(4) {
                        // π/4 的三角函数值
                        match name {
                            "sin" | "cos" => {
                                // sin(π/4) = cos(π/4) = √2/2
                                return Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                })));
                            }
                            "tan" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                            _ => {}
                        }
                    }
                }
            }
            
            _ => {}
        }
        
        // 对于其他情况，先求值参数再调用原有方法
        let arg_val = arg.evaluate_exact()?;
        self.evaluate_trigonometric_function(name, &arg_val)
    }
    
    /// 求值指数函数（使用表达式参数）
    fn evaluate_exponential_function_with_expr(&self, arg: &Expression) -> Result<Number, String> {
        use super::MathConstant;
        
        // 检查特殊的表达式模式
        match arg {
            // exp(i*π) = -1 (欧拉公式)
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                // 检查是否为 i*π 或 π*i 的形式
                let is_i_pi = match (left.as_ref(), right.as_ref()) {
                    (Expression::Constant(MathConstant::I), Expression::Constant(MathConstant::Pi)) |
                    (Expression::Constant(MathConstant::Pi), Expression::Constant(MathConstant::I)) => true,
                    _ => false,
                };
                
                if is_i_pi {
                    return Ok(Number::Integer(num_bigint::BigInt::from(-1)));
                }
            }
            
            _ => {}
        }
        
        // 对于其他情况，先求值参数再调用原有方法
        let arg_val = arg.evaluate_exact()?;
        self.evaluate_exponential_function(&arg_val)
    }

    /// 求值函数调用
    fn evaluate_function(&self, name: &str, args: &[Number]) -> Result<Number, String> {
        match name {
            // 三角函数
            "sin" => {
                if args.len() != 1 {
                    return Err("sin函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function("sin", &args[0])
            }
            "cos" => {
                if args.len() != 1 {
                    return Err("cos函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function("cos", &args[0])
            }
            "tan" => {
                if args.len() != 1 {
                    return Err("tan函数需要一个参数".to_string());
                }
                self.evaluate_trigonometric_function("tan", &args[0])
            }
            
            // 反三角函数
            "asin" | "arcsin" => {
                if args.len() != 1 {
                    return Err("asin函数需要一个参数".to_string());
                }
                self.evaluate_inverse_trigonometric_function("asin", &args[0])
            }
            "acos" | "arccos" => {
                if args.len() != 1 {
                    return Err("acos函数需要一个参数".to_string());
                }
                self.evaluate_inverse_trigonometric_function("acos", &args[0])
            }
            "atan" | "arctan" => {
                if args.len() != 1 {
                    return Err("atan函数需要一个参数".to_string());
                }
                self.evaluate_inverse_trigonometric_function("atan", &args[0])
            }
            
            // 指数和对数函数
            "exp" => {
                if args.len() != 1 {
                    return Err("exp函数需要一个参数".to_string());
                }
                self.evaluate_exponential_function(&args[0])
            }
            "ln" | "log" => {
                if args.len() != 1 {
                    return Err("ln函数需要一个参数".to_string());
                }
                self.evaluate_logarithm_function(&args[0])
            }
            "log10" => {
                if args.len() != 1 {
                    return Err("log10函数需要一个参数".to_string());
                }
                self.evaluate_log10_function(&args[0])
            }
            "log2" => {
                if args.len() != 1 {
                    return Err("log2函数需要一个参数".to_string());
                }
                self.evaluate_log2_function(&args[0])
            }
            
            // 幂函数和根函数
            "sqrt" => {
                if args.len() != 1 {
                    return Err("sqrt函数需要一个参数".to_string());
                }
                self.evaluate_sqrt_function(&args[0])
            }
            "pow" => {
                if args.len() != 2 {
                    return Err("pow函数需要两个参数".to_string());
                }
                self.evaluate_power_function(&args[0], &args[1])
            }
            
            // 双曲函数
            "sinh" => {
                if args.len() != 1 {
                    return Err("sinh函数需要一个参数".to_string());
                }
                self.evaluate_hyperbolic_function("sinh", &args[0])
            }
            "cosh" => {
                if args.len() != 1 {
                    return Err("cosh函数需要一个参数".to_string());
                }
                self.evaluate_hyperbolic_function("cosh", &args[0])
            }
            "tanh" => {
                if args.len() != 1 {
                    return Err("tanh函数需要一个参数".to_string());
                }
                self.evaluate_hyperbolic_function("tanh", &args[0])
            }
            
            // 统计函数
            "max" => {
                if args.is_empty() {
                    return Err("max函数需要至少一个参数".to_string());
                }
                let mut max_val = &args[0];
                for arg in &args[1..] {
                    if arg.approximate() > max_val.approximate() {
                        max_val = arg;
                    }
                }
                Ok(max_val.clone())
            }
            "min" => {
                if args.is_empty() {
                    return Err("min函数需要至少一个参数".to_string());
                }
                let mut min_val = &args[0];
                for arg in &args[1..] {
                    if arg.approximate() < min_val.approximate() {
                        min_val = arg;
                    }
                }
                Ok(min_val.clone())
            }
            "abs" => {
                if args.len() != 1 {
                    return Err("abs函数需要一个参数".to_string());
                }
                args[0].abs().map_err(|e| format!("{}", e))
            }
            
            // 对于其他函数，返回符号表示
            _ => {
                Ok(Number::Symbolic(Box::new(Expression::Function {
                    name: name.to_string(),
                    args: args.iter().map(|arg| Expression::Number(arg.clone())).collect(),
                })))
            }
        }
    }
    
    /// 尝试将表达式简化为数值（如果可能）
    pub fn try_to_number(&self) -> Option<Number> {
        match self.evaluate_exact() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }
    
    /// 求值三角函数
    fn evaluate_trigonometric_function(&self, name: &str, arg: &Number) -> Result<Number, String> {
        use super::MathConstant;
        
        // 检查特殊值
        match arg {
            Number::Integer(n) if n.is_zero() => {
                match name {
                    "sin" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    "cos" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                    "tan" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            _ => {}
        }
        
        // 检查是否为数学常量
        if let Number::Constant(c) = arg {
            match (name, c) {
                ("sin", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                ("cos", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(-1))),
                ("tan", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                _ => {}
            }
        }
        
        // 检查是否为符号表达式中的数学常量
        if let Number::Symbolic(expr) = arg {
            if let Expression::Constant(c) = expr.as_ref() {
                match (name, c) {
                    ("sin", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    ("cos", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(-1))),
                    ("tan", MathConstant::Pi) => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            
            // 检查是否为 π 的分数
            if let Expression::BinaryOp { op: BinaryOperator::Divide, left, right } = expr.as_ref() {
                if let (Expression::Constant(MathConstant::Pi), Expression::Number(Number::Integer(n))) = 
                    (left.as_ref(), right.as_ref()) {
                    if n == &num_bigint::BigInt::from(2) {
                        // π/2 的三角函数值
                        match name {
                            "sin" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                            "cos" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                            "tan" => return Ok(Number::Constant(MathConstant::Undefined)),
                            _ => {}
                        }
                    } else if n == &num_bigint::BigInt::from(4) {
                        // π/4 的三角函数值
                        match name {
                            "sin" | "cos" => {
                                // sin(π/4) = cos(π/4) = √2/2
                                return Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                })));
                            }
                            "tan" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                            _ => {}
                        }
                    }
                }
            }
        }
        
        // 检查是否为数学常量的倍数
        if let Some(result) = self.evaluate_trig_constant_multiple(name, arg) {
            return Ok(result);
        }
        
        // 对于一般情况，返回符号表示
        let unary_op = match name {
            "sin" => UnaryOperator::Sin,
            "cos" => UnaryOperator::Cos,
            "tan" => UnaryOperator::Tan,
            _ => return Err(format!("未知的三角函数: {}", name)),
        };
        
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: unary_op,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值反三角函数
    fn evaluate_inverse_trigonometric_function(&self, name: &str, arg: &Number) -> Result<Number, String> {
        // 检查特殊值
        match arg {
            Number::Integer(n) if n.is_zero() => {
                match name {
                    "asin" | "atan" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                match name {
                    "acos" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    "asin" => return Ok(Number::Constant(MathConstant::Pi)),
                    _ => {}
                }
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(-1) => {
                match name {
                    "acos" => return Ok(Number::Constant(MathConstant::Pi)),
                    "asin" => return Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
                        op: UnaryOperator::Negate,
                        operand: Box::new(Expression::BinaryOp {
                            op: BinaryOperator::Divide,
                            left: Box::new(Expression::Constant(MathConstant::Pi)),
                            right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                        }),
                    }))),
                    _ => {}
                }
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        let unary_op = match name {
            "asin" => UnaryOperator::Asin,
            "acos" => UnaryOperator::Acos,
            "atan" => UnaryOperator::Atan,
            _ => return Err(format!("未知的反三角函数: {}", name)),
        };
        
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: unary_op,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值指数函数
    fn evaluate_exponential_function(&self, arg: &Number) -> Result<Number, String> {
        use super::MathConstant;
        
        // 检查特殊值
        match arg {
            Number::Integer(n) if n.is_zero() => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Constant(MathConstant::E));
            }
            // 检查符号表达式
            Number::Symbolic(expr) => {
                if let Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } = expr.as_ref() {
                    // 检查是否为 i*π 或 π*i 的形式
                    let is_i_pi = match (left.as_ref(), right.as_ref()) {
                        (Expression::Constant(MathConstant::I), Expression::Constant(MathConstant::Pi)) |
                        (Expression::Constant(MathConstant::Pi), Expression::Constant(MathConstant::I)) => true,
                        // 检查复数形式的虚数单位
                        (Expression::Number(Number::Complex { real, imaginary }), Expression::Constant(MathConstant::Pi)) 
                            if real.is_zero() && imaginary.is_one() => true,
                        (Expression::Constant(MathConstant::Pi), Expression::Number(Number::Complex { real, imaginary })) 
                            if real.is_zero() && imaginary.is_one() => true,
                        // 检查符号表示的常量
                        (Expression::Number(Number::Symbolic(inner_expr)), Expression::Constant(MathConstant::Pi)) => {
                            matches!(inner_expr.as_ref(), Expression::Constant(MathConstant::I))
                        }
                        (Expression::Constant(MathConstant::Pi), Expression::Number(Number::Symbolic(inner_expr))) => {
                            matches!(inner_expr.as_ref(), Expression::Constant(MathConstant::I))
                        }
                        // 检查符号表示的 π
                        (Expression::Constant(MathConstant::I), Expression::Number(Number::Symbolic(inner_expr))) => {
                            matches!(inner_expr.as_ref(), Expression::Constant(MathConstant::Pi))
                        }
                        (Expression::Number(Number::Symbolic(inner_expr)), Expression::Constant(MathConstant::I)) => {
                            matches!(inner_expr.as_ref(), Expression::Constant(MathConstant::Pi))
                        }
                        _ => false,
                    };
                    
                    if is_i_pi {
                        return Ok(Number::Integer(num_bigint::BigInt::from(-1)));
                    }
                }
            }
            // 检查复数形式的 i*π
            Number::Complex { real, imaginary } => {
                if real.is_zero() && imaginary.is_one() {
                    // 这是虚数单位 i，但我们需要检查是否乘以了 π
                    // 这种情况在函数调用时处理
                }
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: UnaryOperator::Exp,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值对数函数
    fn evaluate_logarithm_function(&self, arg: &Number) -> Result<Number, String> {
        use super::MathConstant;
        
        // 检查特殊值
        match arg {
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(0)));
            }
            Number::Constant(MathConstant::E) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            Number::Integer(n) if n <= &num_bigint::BigInt::from(0) => {
                return Err("对数函数的参数必须为正数".to_string());
            }
            // 检查符号表达式中的常量
            Number::Symbolic(expr) => {
                if let Expression::Constant(MathConstant::E) = expr.as_ref() {
                    return Ok(Number::Integer(num_bigint::BigInt::from(1)));
                }
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: UnaryOperator::Ln,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值常用对数函数
    fn evaluate_log10_function(&self, arg: &Number) -> Result<Number, String> {
        // 检查特殊值
        match arg {
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(0)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(10) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            Number::Integer(n) if n <= &num_bigint::BigInt::from(0) => {
                return Err("对数函数的参数必须为正数".to_string());
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: UnaryOperator::Log10,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值二进制对数函数
    fn evaluate_log2_function(&self, arg: &Number) -> Result<Number, String> {
        // 检查特殊值
        match arg {
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(0)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(2) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            Number::Integer(n) if n <= &num_bigint::BigInt::from(0) => {
                return Err("对数函数的参数必须为正数".to_string());
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: UnaryOperator::Log2,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值平方根函数
    fn evaluate_sqrt_function(&self, arg: &Number) -> Result<Number, String> {
        // 检查特殊值
        match arg {
            Number::Integer(n) if n.is_zero() => {
                return Ok(Number::Integer(num_bigint::BigInt::from(0)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(4) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(2)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(9) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(3)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(16) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(4)));
            }
            Number::Integer(n) if n == &num_bigint::BigInt::from(25) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(5)));
            }
            Number::Integer(n) if n < &num_bigint::BigInt::from(0) => {
                // 负数的平方根是复数
                let abs_n = n.abs();
                return Ok(Number::Complex {
                    real: Box::new(Number::Integer(num_bigint::BigInt::from(0))),
                    imaginary: Box::new(Number::Symbolic(Box::new(Expression::UnaryOp {
                        op: UnaryOperator::Sqrt,
                        operand: Box::new(Expression::Number(Number::Integer(abs_n))),
                    }))),
                });
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: UnaryOperator::Sqrt,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值幂函数
    fn evaluate_power_function(&self, base: &Number, exponent: &Number) -> Result<Number, String> {
        // 检查特殊情况
        match (base, exponent) {
            // 任何数的0次幂都是1
            (_, Number::Integer(n)) if n.is_zero() => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            // 任何数的1次幂都是它本身
            (_, Number::Integer(n)) if n == &num_bigint::BigInt::from(1) => {
                return Ok(base.clone());
            }
            // 0的正数次幂是0
            (Number::Integer(n), _) if n.is_zero() && exponent.is_positive() => {
                return Ok(Number::Integer(num_bigint::BigInt::from(0)));
            }
            // 1的任何次幂都是1
            (Number::Integer(n), _) if n == &num_bigint::BigInt::from(1) => {
                return Ok(Number::Integer(num_bigint::BigInt::from(1)));
            }
            // 整数的整数次幂
            (Number::Integer(base_int), Number::Integer(exp_int)) => {
                if exp_int >= &num_bigint::BigInt::from(0) {
                    if let Some(exp_u32) = exp_int.to_u32() {
                        return Ok(Number::Integer(base_int.pow(exp_u32)));
                    }
                }
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        Ok(Number::Symbolic(Box::new(Expression::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(Expression::Number(base.clone())),
            right: Box::new(Expression::Number(exponent.clone())),
        })))
    }
    
    /// 求值双曲函数
    fn evaluate_hyperbolic_function(&self, name: &str, arg: &Number) -> Result<Number, String> {
        // 检查特殊值
        match arg {
            Number::Integer(n) if n.is_zero() => {
                match name {
                    "sinh" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    "cosh" => return Ok(Number::Integer(num_bigint::BigInt::from(1))),
                    "tanh" => return Ok(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            _ => {}
        }
        
        // 对于一般情况，返回符号表示
        let unary_op = match name {
            "sinh" => UnaryOperator::Sinh,
            "cosh" => UnaryOperator::Cosh,
            "tanh" => UnaryOperator::Tanh,
            _ => return Err(format!("未知的双曲函数: {}", name)),
        };
        
        Ok(Number::Symbolic(Box::new(Expression::UnaryOp {
            op: unary_op,
            operand: Box::new(Expression::Number(arg.clone())),
        })))
    }
    
    /// 求值三角函数的常量倍数
    fn evaluate_trig_constant_multiple(&self, name: &str, arg: &Number) -> Option<Number> {
        use super::MathConstant;
        
        // 检查是否为 π 的倍数
        if let Number::Constant(MathConstant::Pi) = arg {
            match name {
                "sin" => return Some(Number::Integer(num_bigint::BigInt::from(0))),
                "cos" => return Some(Number::Integer(num_bigint::BigInt::from(-1))),
                "tan" => return Some(Number::Integer(num_bigint::BigInt::from(0))),
                _ => {}
            }
        }
        
        // 检查符号表达式中的常量
        if let Number::Symbolic(expr) = arg {
            // 检查是否为常量
            if let Expression::Constant(MathConstant::Pi) = expr.as_ref() {
                match name {
                    "sin" => return Some(Number::Integer(num_bigint::BigInt::from(0))),
                    "cos" => return Some(Number::Integer(num_bigint::BigInt::from(-1))),
                    "tan" => return Some(Number::Integer(num_bigint::BigInt::from(0))),
                    _ => {}
                }
            }
            
            // 检查是否为 π/2 的倍数
            if let Expression::BinaryOp { op: BinaryOperator::Divide, left, right } = expr.as_ref() {
                if let (Expression::Constant(MathConstant::Pi), Expression::Number(Number::Integer(n))) = 
                    (left.as_ref(), right.as_ref()) {
                    if n == &num_bigint::BigInt::from(2) {
                        match name {
                            "sin" => return Some(Number::Integer(num_bigint::BigInt::from(1))),
                            "cos" => return Some(Number::Integer(num_bigint::BigInt::from(0))),
                            // tan(π/2) 是未定义的
                            "tan" => return Some(Number::Constant(MathConstant::Undefined)),
                            _ => {}
                        }
                    } else if n == &num_bigint::BigInt::from(4) {
                        // π/4 的三角函数值
                        match name {
                            "sin" | "cos" => {
                                // sin(π/4) = cos(π/4) = √2/2
                                return Some(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                })));
                            }
                            "tan" => return Some(Number::Integer(num_bigint::BigInt::from(1))),
                            _ => {}
                        }
                    } else if n == &num_bigint::BigInt::from(6) {
                        // π/6 的三角函数值
                        match name {
                            "sin" => return Some(Number::Rational(num_rational::BigRational::new(
                                num_bigint::BigInt::from(1), 
                                num_bigint::BigInt::from(2)
                            ))),
                            "cos" => {
                                // cos(π/6) = √3/2
                                return Some(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(3)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                })));
                            }
                            "tan" => {
                                // tan(π/6) = √3/3
                                return Some(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(3)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(3)))),
                                })));
                            }
                            _ => {}
                        }
                    } else if n == &num_bigint::BigInt::from(3) {
                        // π/3 的三角函数值
                        match name {
                            "sin" => {
                                // sin(π/3) = √3/2
                                return Some(Number::Symbolic(Box::new(Expression::BinaryOp {
                                    op: BinaryOperator::Divide,
                                    left: Box::new(Expression::UnaryOp {
                                        op: UnaryOperator::Sqrt,
                                        operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(3)))),
                                    }),
                                    right: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(2)))),
                                })));
                            }
                            "cos" => return Some(Number::Rational(num_rational::BigRational::new(
                                num_bigint::BigInt::from(1), 
                                num_bigint::BigInt::from(2)
                            ))),
                            "tan" => {
                                // tan(π/3) = √3
                                return Some(Number::Symbolic(Box::new(Expression::UnaryOp {
                                    op: UnaryOperator::Sqrt,
                                    operand: Box::new(Expression::Number(Number::Integer(num_bigint::BigInt::from(3)))),
                                })));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// 检查表达式是否可以求值为数值
    pub fn is_evaluable(&self) -> bool {
        self.evaluate_exact().is_ok()
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

// 手动实现 Eq trait
impl Eq for Expression {}

// 手动实现 Hash trait
impl std::hash::Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expression::Number(n) => {
                0u8.hash(state);
                n.hash(state);
            }
            Expression::Variable(name) => {
                1u8.hash(state);
                name.hash(state);
            }
            Expression::Constant(c) => {
                2u8.hash(state);
                c.hash(state);
            }
            Expression::BinaryOp { op, left, right } => {
                3u8.hash(state);
                op.hash(state);
                left.hash(state);
                right.hash(state);
            }
            Expression::UnaryOp { op, operand } => {
                4u8.hash(state);
                op.hash(state);
                operand.hash(state);
            }
            Expression::Function { name, args } => {
                5u8.hash(state);
                name.hash(state);
                args.hash(state);
            }
            Expression::Matrix(rows) => {
                6u8.hash(state);
                rows.hash(state);
            }
            Expression::Vector(elements) => {
                7u8.hash(state);
                elements.hash(state);
            }
            Expression::Set(elements) => {
                8u8.hash(state);
                elements.hash(state);
            }
            Expression::Interval { start, end, start_inclusive, end_inclusive } => {
                9u8.hash(state);
                start.hash(state);
                end.hash(state);
                start_inclusive.hash(state);
                end_inclusive.hash(state);
            }
        }
    }
}

// 包含测试模块
#[cfg(test)]
#[path = "expression_tests.rs"]
mod expression_tests;

#[cfg(test)]
#[path = "function_tests.rs"]
mod function_tests;