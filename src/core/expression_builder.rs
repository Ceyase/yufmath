//! # 优化的表达式构建器
//!
//! 提供内存优化的表达式构建功能，自动使用共享和写时复制策略。

use super::{Expression, Number, MathConstant, BinaryOperator, UnaryOperator};
use super::memory::{SharedExpression, MemoryManager};
use std::collections::HashMap;
use num_traits::{ToPrimitive, Zero};

/// 优化的表达式构建器
pub struct ExpressionBuilder {
    /// 内存管理器
    memory_manager: MemoryManager,
    /// 常用表达式缓存
    common_expressions: HashMap<String, SharedExpression>,
}

impl ExpressionBuilder {
    /// 创建新的表达式构建器
    pub fn new() -> Self {
        let mut builder = Self {
            memory_manager: MemoryManager::new(),
            common_expressions: HashMap::new(),
        };
        
        // 预创建常用表达式
        builder.preload_common_expressions();
        builder
    }
    
    /// 预加载常用表达式
    fn preload_common_expressions(&mut self) {
        // 常用数值
        let common_numbers = [0, 1, -1, 2, -2, 10];
        for &n in &common_numbers {
            let expr = Expression::Number(Number::integer(n));
            let shared = self.memory_manager.create_shared(expr);
            self.common_expressions.insert(n.to_string(), shared);
        }
        
        // 常用变量
        let common_vars = ["x", "y", "z", "t", "n"];
        for &var in &common_vars {
            let expr = Expression::Variable(var.to_string());
            let shared = self.memory_manager.create_shared(expr);
            self.common_expressions.insert(format!("var_{}", var), shared);
        }
        
        // 常用常量
        let constants = [
            ("pi", MathConstant::Pi),
            ("e", MathConstant::E),
            ("i", MathConstant::I),
        ];
        for (name, constant) in constants {
            let expr = Expression::Constant(constant);
            let shared = self.memory_manager.create_shared(expr);
            self.common_expressions.insert(format!("const_{}", name), shared);
        }
    }
    
    /// 创建数值表达式
    pub fn number(&mut self, n: Number) -> SharedExpression {
        // 检查是否为常用数值
        if let Number::Integer(ref big_int) = n {
            if let Some(small_int) = big_int.to_i32() {
                if let Some(cached) = self.common_expressions.get(&small_int.to_string()) {
                    return cached.clone_shared();
                }
            }
        }
        
        let expr = Expression::Number(n);
        self.memory_manager.create_shared(expr)
    }
    
    /// 创建变量表达式
    pub fn variable(&mut self, name: &str) -> SharedExpression {
        let cache_key = format!("var_{}", name);
        if let Some(cached) = self.common_expressions.get(&cache_key) {
            return cached.clone_shared();
        }
        
        let expr = Expression::Variable(name.to_string());
        let shared = self.memory_manager.create_shared(expr);
        
        // 缓存新变量
        if self.common_expressions.len() < 1000 {
            self.common_expressions.insert(cache_key, shared.clone_shared());
        }
        
        shared
    }
    
    /// 创建常量表达式
    pub fn constant(&mut self, c: MathConstant) -> SharedExpression {
        let cache_key = format!("const_{:?}", c);
        if let Some(cached) = self.common_expressions.get(&cache_key) {
            return cached.clone_shared();
        }
        
        let expr = Expression::Constant(c);
        let shared = self.memory_manager.create_shared(expr);
        
        // 缓存新常量
        self.common_expressions.insert(cache_key, shared.clone_shared());
        shared
    }
    
    /// 创建二元运算表达式
    pub fn binary_op(&mut self, op: BinaryOperator, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        // 应用简化规则
        if let Some(simplified) = self.try_simplify_binary_op(&op, &left, &right) {
            return simplified;
        }
        
        let expr = Expression::BinaryOp {
            op,
            left: Box::new(left.as_ref().clone()),
            right: Box::new(right.as_ref().clone()),
        };
        
        self.memory_manager.create_shared(expr)
    }
    
    /// 创建一元运算表达式
    pub fn unary_op(&mut self, op: UnaryOperator, operand: SharedExpression) -> SharedExpression {
        // 应用简化规则
        if let Some(simplified) = self.try_simplify_unary_op(&op, &operand) {
            return simplified;
        }
        
        let expr = Expression::UnaryOp {
            op,
            operand: Box::new(operand.as_ref().clone()),
        };
        
        self.memory_manager.create_shared(expr)
    }
    
    /// 创建函数调用表达式
    pub fn function(&mut self, name: &str, args: Vec<SharedExpression>) -> SharedExpression {
        let expr_args: Vec<Expression> = args.iter()
            .map(|arg| arg.as_ref().clone())
            .collect();
        
        let expr = Expression::Function {
            name: name.to_string(),
            args: expr_args,
        };
        
        self.memory_manager.create_shared(expr)
    }
    
    /// 尝试简化二元运算
    fn try_simplify_binary_op(&mut self, op: &BinaryOperator, left: &SharedExpression, right: &SharedExpression) -> Option<SharedExpression> {
        match op {
            BinaryOperator::Add => {
                // 0 + x = x
                if self.is_zero(left) {
                    return Some(right.clone_shared());
                }
                // x + 0 = x
                if self.is_zero(right) {
                    return Some(left.clone_shared());
                }
                // x + x = 2*x
                if left == right {
                    let two = self.number(Number::integer(2));
                    return Some(self.binary_op(BinaryOperator::Multiply, two, left.clone_shared()));
                }
            }
            BinaryOperator::Subtract => {
                // x - 0 = x
                if self.is_zero(right) {
                    return Some(left.clone_shared());
                }
                // x - x = 0
                if left == right {
                    return Some(self.number(Number::integer(0)));
                }
            }
            BinaryOperator::Multiply => {
                // 0 * x = 0
                if self.is_zero(left) || self.is_zero(right) {
                    return Some(self.number(Number::integer(0)));
                }
                // 1 * x = x
                if self.is_one(left) {
                    return Some(right.clone_shared());
                }
                // x * 1 = x
                if self.is_one(right) {
                    return Some(left.clone_shared());
                }
                // (-1) * x = -x
                if self.is_negative_one(left) {
                    return Some(self.unary_op(UnaryOperator::Negate, right.clone_shared()));
                }
                // x * (-1) = -x
                if self.is_negative_one(right) {
                    return Some(self.unary_op(UnaryOperator::Negate, left.clone_shared()));
                }
            }
            BinaryOperator::Divide => {
                // x / 1 = x
                if self.is_one(right) {
                    return Some(left.clone_shared());
                }
                // x / x = 1 (假设 x != 0)
                if left == right {
                    return Some(self.number(Number::integer(1)));
                }
            }
            BinaryOperator::Power => {
                // x^0 = 1
                if self.is_zero(right) {
                    return Some(self.number(Number::integer(1)));
                }
                // x^1 = x
                if self.is_one(right) {
                    return Some(left.clone_shared());
                }
                // 1^x = 1
                if self.is_one(left) {
                    return Some(self.number(Number::integer(1)));
                }
            }
            _ => {}
        }
        
        None
    }
    
    /// 尝试简化一元运算
    fn try_simplify_unary_op(&mut self, op: &UnaryOperator, operand: &SharedExpression) -> Option<SharedExpression> {
        match op {
            UnaryOperator::Negate => {
                // -(-x) = x
                if let Expression::UnaryOp { op: UnaryOperator::Negate, operand: inner } = operand.as_ref() {
                    let inner_expr = Expression::clone(inner);
                    return Some(self.memory_manager.create_shared(inner_expr));
                }
                // -0 = 0
                if self.is_zero(operand) {
                    return Some(operand.clone_shared());
                }
            }
            UnaryOperator::Plus => {
                // +x = x
                return Some(operand.clone_shared());
            }
            UnaryOperator::Abs => {
                // |0| = 0
                if self.is_zero(operand) {
                    return Some(operand.clone_shared());
                }
            }
            _ => {}
        }
        
        None
    }
    
    /// 检查表达式是否为零
    fn is_zero(&self, expr: &SharedExpression) -> bool {
        match expr.as_ref() {
            Expression::Number(Number::Integer(n)) => n.is_zero(),
            Expression::Number(Number::Rational(r)) => r.is_zero(),
            Expression::Number(Number::Real(r)) => r.is_zero(),
            Expression::Number(Number::Float(f)) => *f == 0.0,
            _ => false,
        }
    }
    
    /// 检查表达式是否为一
    fn is_one(&self, expr: &SharedExpression) -> bool {
        match expr.as_ref() {
            Expression::Number(Number::Integer(n)) => n == &num_bigint::BigInt::from(1),
            Expression::Number(Number::Rational(r)) => r == &num_rational::BigRational::from_integer(num_bigint::BigInt::from(1)),
            Expression::Number(Number::Float(f)) => *f == 1.0,
            _ => false,
        }
    }
    
    /// 检查表达式是否为负一
    fn is_negative_one(&self, expr: &SharedExpression) -> bool {
        match expr.as_ref() {
            Expression::Number(Number::Integer(n)) => n == &num_bigint::BigInt::from(-1),
            Expression::Number(Number::Rational(r)) => r == &num_rational::BigRational::from_integer(num_bigint::BigInt::from(-1)),
            Expression::Number(Number::Float(f)) => *f == -1.0,
            _ => false,
        }
    }
    
    /// 创建加法表达式
    pub fn add(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.binary_op(BinaryOperator::Add, left, right)
    }
    
    /// 创建减法表达式
    pub fn subtract(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.binary_op(BinaryOperator::Subtract, left, right)
    }
    
    /// 创建乘法表达式
    pub fn multiply(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.binary_op(BinaryOperator::Multiply, left, right)
    }
    
    /// 创建除法表达式
    pub fn divide(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.binary_op(BinaryOperator::Divide, left, right)
    }
    
    /// 创建幂运算表达式
    pub fn power(&mut self, base: SharedExpression, exponent: SharedExpression) -> SharedExpression {
        self.binary_op(BinaryOperator::Power, base, exponent)
    }
    
    /// 创建负号表达式
    pub fn negate(&mut self, operand: SharedExpression) -> SharedExpression {
        self.unary_op(UnaryOperator::Negate, operand)
    }
    
    /// 获取内存统计信息
    pub fn memory_stats(&mut self) -> &super::memory::MemoryStats {
        self.memory_manager.get_stats()
    }
    
    /// 执行内存清理
    pub fn cleanup(&mut self) {
        self.memory_manager.cleanup();
        
        // 清理常用表达式缓存中的孤立引用
        self.common_expressions.retain(|_, shared_expr| {
            shared_expr.ref_count() > 1
        });
    }
    
    /// 获取内存管理器的引用
    pub fn memory_manager(&mut self) -> &mut MemoryManager {
        &mut self.memory_manager
    }
}

impl Default for ExpressionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 表达式工厂，提供便捷的表达式创建方法
pub struct ExpressionFactory {
    builder: ExpressionBuilder,
}

impl ExpressionFactory {
    /// 创建新的表达式工厂
    pub fn new() -> Self {
        Self {
            builder: ExpressionBuilder::new(),
        }
    }
    
    /// 创建整数表达式
    pub fn int(&mut self, value: i64) -> SharedExpression {
        self.builder.number(Number::integer(value))
    }
    
    /// 创建有理数表达式
    pub fn rational(&mut self, numerator: i64, denominator: i64) -> SharedExpression {
        self.builder.number(Number::rational(numerator, denominator))
    }
    
    /// 创建浮点数表达式
    pub fn float(&mut self, value: f64) -> SharedExpression {
        self.builder.number(Number::Float(value))
    }
    
    /// 创建变量表达式
    pub fn var(&mut self, name: &str) -> SharedExpression {
        self.builder.variable(name)
    }
    
    /// 创建 π 常量
    pub fn pi(&mut self) -> SharedExpression {
        self.builder.constant(MathConstant::Pi)
    }
    
    /// 创建 e 常量
    pub fn e(&mut self) -> SharedExpression {
        self.builder.constant(MathConstant::E)
    }
    
    /// 创建虚数单位 i
    pub fn i(&mut self) -> SharedExpression {
        self.builder.constant(MathConstant::I)
    }
    
    /// 创建加法表达式
    pub fn add(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.builder.add(left, right)
    }
    
    /// 创建减法表达式
    pub fn sub(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.builder.subtract(left, right)
    }
    
    /// 创建乘法表达式
    pub fn mul(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.builder.multiply(left, right)
    }
    
    /// 创建除法表达式
    pub fn div(&mut self, left: SharedExpression, right: SharedExpression) -> SharedExpression {
        self.builder.divide(left, right)
    }
    
    /// 创建幂运算表达式
    pub fn pow(&mut self, base: SharedExpression, exponent: SharedExpression) -> SharedExpression {
        self.builder.power(base, exponent)
    }
    
    /// 创建正弦函数表达式
    pub fn sin(&mut self, operand: SharedExpression) -> SharedExpression {
        self.builder.unary_op(UnaryOperator::Sin, operand)
    }
    
    /// 创建余弦函数表达式
    pub fn cos(&mut self, operand: SharedExpression) -> SharedExpression {
        self.builder.unary_op(UnaryOperator::Cos, operand)
    }
    
    /// 创建自然对数表达式
    pub fn ln(&mut self, operand: SharedExpression) -> SharedExpression {
        self.builder.unary_op(UnaryOperator::Ln, operand)
    }
    
    /// 创建指数函数表达式
    pub fn exp(&mut self, operand: SharedExpression) -> SharedExpression {
        self.builder.unary_op(UnaryOperator::Exp, operand)
    }
    
    /// 获取内存统计信息
    pub fn memory_stats(&mut self) -> &super::memory::MemoryStats {
        self.builder.memory_stats()
    }
    
    /// 执行内存清理
    pub fn cleanup(&mut self) {
        self.builder.cleanup();
    }
}

impl Default for ExpressionFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expression_builder_basic() {
        let mut builder = ExpressionBuilder::new();
        
        let x = builder.variable("x");
        let one = builder.number(Number::integer(1));
        let zero = builder.number(Number::integer(0));
        
        // 测试基本简化
        let x_plus_zero = builder.add(x.clone_shared(), zero);
        assert_eq!(x_plus_zero.as_ref(), x.as_ref());
        
        let one_times_x = builder.multiply(one, x.clone_shared());
        assert_eq!(one_times_x.as_ref(), x.as_ref());
    }
    
    #[test]
    fn test_expression_factory() {
        let mut factory = ExpressionFactory::new();
        
        let x = factory.var("x");
        let two = factory.int(2);
        let pi = factory.pi();
        
        let sum = factory.add(x, pi);
        let expr = factory.mul(two, sum);
        
        // 验证表达式结构
        match expr.as_ref() {
            Expression::BinaryOp { op: BinaryOperator::Multiply, left, right } => {
                assert!(matches!(left.as_ref(), Expression::Number(Number::Integer(_))));
                assert!(matches!(right.as_ref(), Expression::BinaryOp { op: BinaryOperator::Add, .. }));
            }
            _ => panic!("期望乘法表达式"),
        }
    }
    
    #[test]
    fn test_common_expression_caching() {
        let mut builder = ExpressionBuilder::new();
        
        let x1 = builder.variable("x");
        let x2 = builder.variable("x");
        
        // 相同的变量应该共享内存
        assert_eq!(x1, x2);
        
        let one1 = builder.number(Number::integer(1));
        let one2 = builder.number(Number::integer(1));
        
        // 相同的数值应该共享内存
        assert_eq!(one1, one2);
    }
    
    #[test]
    fn test_algebraic_simplification() {
        let mut builder = ExpressionBuilder::new();
        
        let x = builder.variable("x");
        let zero = builder.number(Number::integer(0));
        let one = builder.number(Number::integer(1));
        
        // x + 0 = x
        let simplified = builder.add(x.clone_shared(), zero.clone_shared());
        assert_eq!(simplified.as_ref(), x.as_ref());
        
        // x * 1 = x
        let simplified = builder.multiply(x.clone_shared(), one.clone_shared());
        assert_eq!(simplified.as_ref(), x.as_ref());
        
        // x - x = 0
        let simplified = builder.subtract(x.clone_shared(), x.clone_shared());
        assert_eq!(simplified.as_ref(), zero.as_ref());
    }
    
    #[test]
    fn test_memory_management() {
        let mut builder = ExpressionBuilder::new();
        
        // 创建大量表达式
        for i in 0..1000 {
            let var = builder.variable(&format!("x{}", i));
            let num = builder.number(Number::integer(i));
            let _expr = builder.add(var, num);
        }
        
        let stats_before = builder.memory_stats().clone();
        builder.cleanup();
        let stats_after = builder.memory_stats().clone();
        
        println!("清理前: 活跃表达式 {}, 共享表达式 {}", 
                stats_before.active_expressions, stats_before.shared_expressions);
        println!("清理后: 活跃表达式 {}, 共享表达式 {}", 
                stats_after.active_expressions, stats_after.shared_expressions);
    }
}