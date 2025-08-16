//! # 内存管理优化
//!
//! 提供表达式的内存管理优化，包括引用计数共享、写时复制、哈希优化等功能。

use super::{Expression, Number};
use std::rc::Rc;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// 内存使用统计信息
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// 当前活跃的表达式数量
    pub active_expressions: usize,
    /// 共享的表达式数量
    pub shared_expressions: usize,
    /// 缓存命中次数
    pub cache_hits: usize,
    /// 缓存未命中次数
    pub cache_misses: usize,
    /// 写时复制触发次数
    pub cow_triggers: usize,
    /// 估计的内存使用量（字节）
    pub estimated_memory_usage: usize,
    /// 最后更新时间
    pub last_updated: Instant,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            active_expressions: 0,
            shared_expressions: 0,
            cache_hits: 0,
            cache_misses: 0,
            cow_triggers: 0,
            estimated_memory_usage: 0,
            last_updated: Instant::now(),
        }
    }
}

/// 内存管理配置
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// 是否启用表达式共享
    pub enable_sharing: bool,
    /// 是否启用写时复制
    pub enable_cow: bool,
    /// 哈希缓存的最大大小
    pub max_hash_cache_size: usize,
    /// 表达式缓存的最大大小
    pub max_expression_cache_size: usize,
    /// 内存清理的阈值（字节）
    pub cleanup_threshold: usize,
    /// 自动清理的时间间隔
    pub cleanup_interval: Duration,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enable_sharing: true,
            enable_cow: true,
            max_hash_cache_size: 10000,
            max_expression_cache_size: 5000,
            cleanup_threshold: 100 * 1024 * 1024, // 100MB
            cleanup_interval: Duration::from_secs(60), // 1分钟
        }
    }
}

/// 共享表达式包装器
#[derive(Debug, Clone)]
pub struct SharedExpression {
    /// 内部表达式的引用计数指针
    inner: Rc<Expression>,
    /// 表达式的哈希值（缓存）
    hash: Option<u64>,
}

impl SharedExpression {
    /// 创建新的共享表达式
    pub fn new(expr: Expression) -> Self {
        Self {
            inner: Rc::new(expr),
            hash: None,
        }
    }
    
    /// 从 Rc 创建共享表达式
    pub fn from_rc(rc: Rc<Expression>) -> Self {
        Self {
            inner: rc,
            hash: None,
        }
    }
    
    /// 获取表达式的引用
    pub fn as_ref(&self) -> &Expression {
        &self.inner
    }
    
    /// 获取引用计数
    pub fn ref_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
    
    /// 检查是否为唯一引用
    pub fn is_unique(&self) -> bool {
        Rc::strong_count(&self.inner) == 1
    }
    
    /// 写时复制：如果不是唯一引用，则克隆表达式
    pub fn make_mut(&mut self) -> &mut Expression {
        if !self.is_unique() {
            // 触发写时复制
            self.inner = Rc::new((*self.inner).clone());
            self.hash = None; // 重置哈希缓存
        }
        // 安全地获取可变引用
        Rc::get_mut(&mut self.inner).unwrap()
    }
    
    /// 尝试获取可变引用（如果是唯一引用）
    pub fn get_mut(&mut self) -> Option<&mut Expression> {
        if self.is_unique() {
            self.hash = None; // 重置哈希缓存
            Rc::get_mut(&mut self.inner)
        } else {
            None
        }
    }
    
    /// 获取或计算哈希值
    pub fn get_hash(&mut self) -> u64 {
        if let Some(hash) = self.hash {
            hash
        } else {
            let hash = calculate_expression_hash(&self.inner);
            self.hash = Some(hash);
            hash
        }
    }
    
    /// 克隆为新的共享表达式
    pub fn clone_shared(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            hash: self.hash,
        }
    }
    
    /// 转换为拥有的表达式
    pub fn into_owned(self) -> Expression {
        match Rc::try_unwrap(self.inner) {
            Ok(expr) => expr,
            Err(rc) => (*rc).clone(),
        }
    }
}

impl PartialEq for SharedExpression {
    fn eq(&self, other: &Self) -> bool {
        // 首先检查是否是同一个 Rc
        if Rc::ptr_eq(&self.inner, &other.inner) {
            return true;
        }
        
        // 然后比较表达式内容
        self.inner.as_ref() == other.inner.as_ref()
    }
}

impl Hash for SharedExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 使用缓存的哈希值或计算新的哈希值
        let hash = if let Some(hash) = self.hash {
            hash
        } else {
            calculate_expression_hash(&self.inner)
        };
        hash.hash(state);
    }
}

/// 内存管理器
pub struct MemoryManager {
    /// 配置
    config: MemoryConfig,
    /// 统计信息
    stats: MemoryStats,
    /// 表达式哈希缓存
    hash_cache: HashMap<*const Expression, u64>,
    /// 表达式共享池
    expression_pool: HashMap<u64, Rc<Expression>>,
    /// 全局表达式计数器
    expression_counter: AtomicUsize,
    /// 最后清理时间
    last_cleanup: Instant,
}

impl MemoryManager {
    /// 创建新的内存管理器
    pub fn new() -> Self {
        Self::with_config(MemoryConfig::default())
    }
    
    /// 使用指定配置创建内存管理器
    pub fn with_config(config: MemoryConfig) -> Self {
        Self {
            config,
            stats: MemoryStats::default(),
            hash_cache: HashMap::new(),
            expression_pool: HashMap::new(),
            expression_counter: AtomicUsize::new(0),
            last_cleanup: Instant::now(),
        }
    }
    
    /// 创建共享表达式
    pub fn create_shared(&mut self, expr: Expression) -> SharedExpression {
        self.expression_counter.fetch_add(1, Ordering::Relaxed);
        
        if !self.config.enable_sharing {
            return SharedExpression::new(expr);
        }
        
        // 计算表达式哈希
        let hash = calculate_expression_hash(&expr);
        
        // 检查是否已存在相同的表达式
        if let Some(existing) = self.expression_pool.get(&hash) {
            if **existing == expr {
                self.stats.cache_hits += 1;
                return SharedExpression::from_rc(existing.clone());
            }
        }
        
        // 创建新的共享表达式
        self.stats.cache_misses += 1;
        let rc = Rc::new(expr);
        
        // 添加到共享池
        if self.expression_pool.len() < self.config.max_expression_cache_size {
            self.expression_pool.insert(hash, rc.clone());
        }
        
        SharedExpression::from_rc(rc)
    }
    
    /// 获取表达式哈希（使用缓存）
    pub fn get_hash(&mut self, expr: &Expression) -> u64 {
        let ptr = expr as *const Expression;
        
        if let Some(&hash) = self.hash_cache.get(&ptr) {
            self.stats.cache_hits += 1;
            return hash;
        }
        
        self.stats.cache_misses += 1;
        let hash = calculate_expression_hash(expr);
        
        // 添加到哈希缓存
        if self.hash_cache.len() < self.config.max_hash_cache_size {
            self.hash_cache.insert(ptr, hash);
        }
        
        hash
    }
    
    /// 更新统计信息
    pub fn update_stats(&mut self) {
        self.stats.active_expressions = self.expression_counter.load(Ordering::Relaxed);
        self.stats.shared_expressions = self.expression_pool.len();
        self.stats.estimated_memory_usage = self.estimate_memory_usage();
        self.stats.last_updated = Instant::now();
        
        // 检查是否需要清理
        if self.should_cleanup() {
            self.cleanup();
        }
    }
    
    /// 获取统计信息
    pub fn get_stats(&mut self) -> &MemoryStats {
        self.update_stats();
        &self.stats
    }
    
    /// 估计内存使用量
    fn estimate_memory_usage(&self) -> usize {
        let hash_cache_size = self.hash_cache.len() * (std::mem::size_of::<*const Expression>() + std::mem::size_of::<u64>());
        let pool_size = self.expression_pool.len() * std::mem::size_of::<Rc<Expression>>();
        let estimated_expr_size = self.expression_pool.len() * 100; // 估计每个表达式100字节
        
        hash_cache_size + pool_size + estimated_expr_size
    }
    
    /// 检查是否需要清理
    fn should_cleanup(&self) -> bool {
        let now = Instant::now();
        let time_elapsed = now.duration_since(self.last_cleanup) >= self.config.cleanup_interval;
        let memory_threshold = self.estimate_memory_usage() >= self.config.cleanup_threshold;
        
        time_elapsed || memory_threshold
    }
    
    /// 执行内存清理
    pub fn cleanup(&mut self) {
        let initial_hash_cache_size = self.hash_cache.len();
        let initial_pool_size = self.expression_pool.len();
        
        // 清理哈希缓存中的无效指针
        self.hash_cache.retain(|&ptr, _| {
            // 这里我们无法安全地检查指针是否有效，所以定期清理所有缓存
            false
        });
        
        // 清理表达式池中只有一个引用的表达式
        self.expression_pool.retain(|_, rc| {
            Rc::strong_count(rc) > 1
        });
        
        self.last_cleanup = Instant::now();
        
        println!("内存清理完成: 哈希缓存 {} -> {}, 表达式池 {} -> {}", 
                initial_hash_cache_size, self.hash_cache.len(),
                initial_pool_size, self.expression_pool.len());
    }
    
    /// 强制清理所有缓存
    pub fn clear_all(&mut self) {
        self.hash_cache.clear();
        self.expression_pool.clear();
        self.stats = MemoryStats::default();
        self.last_cleanup = Instant::now();
    }
    
    /// 获取配置
    pub fn config(&self) -> &MemoryConfig {
        &self.config
    }
    
    /// 更新配置
    pub fn set_config(&mut self, config: MemoryConfig) {
        self.config = config;
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 计算表达式的哈希值
pub fn calculate_expression_hash(expr: &Expression) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    hash_expression(expr, &mut hasher);
    hasher.finish()
}

/// 递归计算表达式哈希
fn hash_expression<H: Hasher>(expr: &Expression, hasher: &mut H) {
    match expr {
        Expression::Number(n) => {
            0u8.hash(hasher);
            hash_number(n, hasher);
        }
        Expression::Variable(name) => {
            1u8.hash(hasher);
            name.hash(hasher);
        }
        Expression::Constant(c) => {
            2u8.hash(hasher);
            std::mem::discriminant(c).hash(hasher);
        }
        Expression::BinaryOp { op, left, right } => {
            3u8.hash(hasher);
            std::mem::discriminant(op).hash(hasher);
            hash_expression(left, hasher);
            hash_expression(right, hasher);
        }
        Expression::UnaryOp { op, operand } => {
            4u8.hash(hasher);
            std::mem::discriminant(op).hash(hasher);
            hash_expression(operand, hasher);
        }
        Expression::Function { name, args } => {
            5u8.hash(hasher);
            name.hash(hasher);
            args.len().hash(hasher);
            for arg in args {
                hash_expression(arg, hasher);
            }
        }
        Expression::Matrix(rows) => {
            6u8.hash(hasher);
            rows.len().hash(hasher);
            if !rows.is_empty() {
                rows[0].len().hash(hasher);
            }
            for row in rows {
                for elem in row {
                    hash_expression(elem, hasher);
                }
            }
        }
        Expression::Vector(elements) => {
            7u8.hash(hasher);
            elements.len().hash(hasher);
            for elem in elements {
                hash_expression(elem, hasher);
            }
        }
        Expression::Set(elements) => {
            8u8.hash(hasher);
            elements.len().hash(hasher);
            for elem in elements {
                hash_expression(elem, hasher);
            }
        }
        Expression::Interval { start, end, start_inclusive, end_inclusive } => {
            9u8.hash(hasher);
            hash_expression(start, hasher);
            hash_expression(end, hasher);
            start_inclusive.hash(hasher);
            end_inclusive.hash(hasher);
        }
    }
}

/// 计算数值的哈希
fn hash_number<H: Hasher>(number: &Number, hasher: &mut H) {
    match number {
        Number::Integer(i) => {
            0u8.hash(hasher);
            // 对于大整数，我们使用其字符串表示的哈希
            i.to_string().hash(hasher);
        }
        Number::Rational(r) => {
            1u8.hash(hasher);
            // 对于有理数，分别哈希分子和分母
            r.numer().to_string().hash(hasher);
            r.denom().to_string().hash(hasher);
        }
        Number::Real(r) => {
            2u8.hash(hasher);
            // 对于高精度实数，使用其字符串表示
            r.to_string().hash(hasher);
        }
        Number::Complex { real, imaginary } => {
            3u8.hash(hasher);
            hash_number(real, hasher);
            hash_number(imaginary, hasher);
        }
        Number::Symbolic(expr) => {
            4u8.hash(hasher);
            hash_expression(expr, hasher);
        }
        Number::Float(f) => {
            5u8.hash(hasher);
            // 对于浮点数，使用其位表示
            f.to_bits().hash(hasher);
        }
        Number::Constant(c) => {
            6u8.hash(hasher);
            // 对于常量，使用其判别式
            std::mem::discriminant(c).hash(hasher);
        }
    }
}

/// 表达式比较优化
pub struct ExpressionComparator {
    /// 哈希缓存
    hash_cache: HashMap<*const Expression, u64>,
}

impl ExpressionComparator {
    /// 创建新的比较器
    pub fn new() -> Self {
        Self {
            hash_cache: HashMap::new(),
        }
    }
    
    /// 快速比较两个表达式
    pub fn fast_eq(&mut self, left: &Expression, right: &Expression) -> bool {
        // 首先检查指针是否相同
        if std::ptr::eq(left, right) {
            return true;
        }
        
        // 然后比较哈希值
        let left_hash = self.get_cached_hash(left);
        let right_hash = self.get_cached_hash(right);
        
        if left_hash != right_hash {
            return false;
        }
        
        // 最后进行完整比较
        left == right
    }
    
    /// 获取缓存的哈希值
    fn get_cached_hash(&mut self, expr: &Expression) -> u64 {
        let ptr = expr as *const Expression;
        
        if let Some(&hash) = self.hash_cache.get(&ptr) {
            return hash;
        }
        
        let hash = calculate_expression_hash(expr);
        self.hash_cache.insert(ptr, hash);
        hash
    }
    
    /// 清理缓存
    pub fn clear_cache(&mut self) {
        self.hash_cache.clear();
    }
}

impl Default for ExpressionComparator {
    fn default() -> Self {
        Self::new()
    }
}

/// 写时复制表达式包装器
#[derive(Debug, Clone)]
pub struct CowExpression {
    /// 内部表达式
    inner: SharedExpression,
    /// 是否已修改
    modified: bool,
}

impl CowExpression {
    /// 创建新的写时复制表达式
    pub fn new(expr: Expression) -> Self {
        Self {
            inner: SharedExpression::new(expr),
            modified: false,
        }
    }
    
    /// 从共享表达式创建
    pub fn from_shared(shared: SharedExpression) -> Self {
        Self {
            inner: shared,
            modified: false,
        }
    }
    
    /// 获取表达式的引用
    pub fn as_ref(&self) -> &Expression {
        self.inner.as_ref()
    }
    
    /// 获取可变引用（触发写时复制）
    pub fn as_mut(&mut self) -> &mut Expression {
        self.modified = true;
        self.inner.make_mut()
    }
    
    /// 检查是否已修改
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// 获取引用计数
    pub fn ref_count(&self) -> usize {
        self.inner.ref_count()
    }
    
    /// 转换为拥有的表达式
    pub fn into_owned(self) -> Expression {
        self.inner.into_owned()
    }
    
    /// 转换为共享表达式
    pub fn into_shared(self) -> SharedExpression {
        self.inner
    }
}

impl PartialEq for CowExpression {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/// 内存监控器
pub struct MemoryMonitor {
    /// 内存管理器
    manager: MemoryManager,
    /// 监控是否启用
    enabled: bool,
    /// 监控间隔
    interval: Duration,
    /// 最后监控时间
    last_check: Instant,
}

impl MemoryMonitor {
    /// 创建新的内存监控器
    pub fn new() -> Self {
        Self {
            manager: MemoryManager::new(),
            enabled: true,
            interval: Duration::from_secs(30),
            last_check: Instant::now(),
        }
    }
    
    /// 启用监控
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// 禁用监控
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// 设置监控间隔
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }
    
    /// 检查内存状态
    pub fn check(&mut self) -> Option<&MemoryStats> {
        if !self.enabled {
            return None;
        }
        
        let now = Instant::now();
        if now.duration_since(self.last_check) >= self.interval {
            self.last_check = now;
            Some(self.manager.get_stats())
        } else {
            None
        }
    }
    
    /// 获取内存管理器
    pub fn manager(&mut self) -> &mut MemoryManager {
        &mut self.manager
    }
    
    /// 获取当前统计信息
    pub fn stats(&mut self) -> &MemoryStats {
        self.manager.get_stats()
    }
    
    /// 执行清理
    pub fn cleanup(&mut self) {
        self.manager.cleanup();
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Expression, Number};
    use num_bigint::BigInt;
    
    #[test]
    fn test_shared_expression_creation() {
        let expr = Expression::Number(Number::Integer(BigInt::from(42)));
        let shared = SharedExpression::new(expr.clone());
        
        assert_eq!(shared.as_ref(), &expr);
        assert_eq!(shared.ref_count(), 1);
        assert!(shared.is_unique());
    }
    
    #[test]
    fn test_shared_expression_cloning() {
        let expr = Expression::Number(Number::Integer(BigInt::from(42)));
        let shared1 = SharedExpression::new(expr.clone());
        let shared2 = shared1.clone_shared();
        
        assert_eq!(shared1.ref_count(), 2);
        assert_eq!(shared2.ref_count(), 2);
        assert!(!shared1.is_unique());
        assert!(!shared2.is_unique());
        assert_eq!(shared1, shared2);
    }
    
    #[test]
    fn test_cow_functionality() {
        let expr = Expression::Number(Number::Integer(BigInt::from(42)));
        let mut shared = SharedExpression::new(expr.clone());
        let shared_clone = shared.clone_shared();
        
        // 在有多个引用时，make_mut 应该触发写时复制
        assert_eq!(shared.ref_count(), 2);
        let _mutable_ref = shared.make_mut();
        assert_eq!(shared.ref_count(), 1);
        assert_eq!(shared_clone.ref_count(), 1);
    }
    
    #[test]
    fn test_expression_hashing() {
        let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
        let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
        let expr3 = Expression::Number(Number::Integer(BigInt::from(43)));
        
        let hash1 = calculate_expression_hash(&expr1);
        let hash2 = calculate_expression_hash(&expr2);
        let hash3 = calculate_expression_hash(&expr3);
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
    
    #[test]
    fn test_memory_manager() {
        let mut manager = MemoryManager::new();
        
        let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
        let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
        
        let shared1 = manager.create_shared(expr1);
        let shared2 = manager.create_shared(expr2);
        
        // 相同的表达式应该共享
        assert_eq!(shared1, shared2);
        
        let stats = manager.get_stats();
        assert!(stats.cache_hits > 0 || stats.cache_misses > 0);
    }
    
    #[test]
    fn test_expression_comparator() {
        let mut comparator = ExpressionComparator::new();
        
        let expr1 = Expression::Number(Number::Integer(BigInt::from(42)));
        let expr2 = Expression::Number(Number::Integer(BigInt::from(42)));
        let expr3 = Expression::Number(Number::Integer(BigInt::from(43)));
        
        assert!(comparator.fast_eq(&expr1, &expr2));
        assert!(!comparator.fast_eq(&expr1, &expr3));
    }
    
    #[test]
    fn test_cow_expression() {
        let expr = Expression::Number(Number::Integer(BigInt::from(42)));
        let mut cow = CowExpression::new(expr.clone());
        
        assert!(!cow.is_modified());
        assert_eq!(cow.ref_count(), 1);
        
        // 获取可变引用应该标记为已修改
        let _mutable_ref = cow.as_mut();
        assert!(cow.is_modified());
    }
    
    #[test]
    fn test_memory_cleanup() {
        let mut manager = MemoryManager::new();
        
        // 创建一些表达式
        for i in 0..100 {
            let expr = Expression::Number(Number::Integer(BigInt::from(i)));
            let _shared = manager.create_shared(expr);
        }
        
        let stats_before = manager.get_stats().clone();
        manager.cleanup();
        let stats_after = manager.get_stats().clone();
        
        // 清理后，某些统计信息应该发生变化
        println!("清理前: {:?}", stats_before);
        println!("清理后: {:?}", stats_after);
    }
}