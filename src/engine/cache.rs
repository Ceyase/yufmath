//! # 多层缓存系统
//!
//! 实现高效的多层缓存机制，包括快速缓存、精确缓存和符号缓存。
//! 提供缓存管理、清理和性能监控功能。

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
use std::sync::{Arc, RwLock};
use crate::core::{Expression, Number, BinaryOperator, UnaryOperator};
use crate::api::CacheConfig;

/// 缓存项的元数据
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    /// 缓存的值
    pub value: T,
    /// 创建时间
    pub created_at: Instant,
    /// 最后访问时间
    pub last_accessed: Instant,
    /// 访问次数
    pub access_count: u64,
    /// 计算成本（用于优先级排序）
    pub compute_cost: u32,
}

impl<T> CacheEntry<T> {
    /// 创建新的缓存项
    pub fn new(value: T, compute_cost: u32) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            compute_cost,
        }
    }
    
    /// 更新访问信息
    pub fn access(&mut self) {
        self.last_accessed = Instant::now();
        self.access_count += 1;
    }
    
    /// 检查是否过期
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
    
    /// 计算缓存项的优先级（用于LRU清理）
    pub fn priority(&self) -> f64 {
        let age_factor = self.last_accessed.elapsed().as_secs_f64();
        let frequency_factor = self.access_count as f64;
        let cost_factor = self.compute_cost as f64;
        
        // 优先级 = 频率 * 成本 / 年龄
        (frequency_factor * cost_factor) / (age_factor + 1.0)
    }
}

/// 快速缓存键（用于小整数运算）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FastCacheKey {
    /// 二元运算：(操作数1, 操作数2, 运算符)
    BinaryOp(i64, i64, BinaryOperator),
    /// 一元运算：(操作数, 运算符)
    UnaryOp(i64, UnaryOperator),
    /// 函数调用：(函数名, 参数列表)
    Function(String, Vec<i64>),
}

/// 精确缓存键（用于任意精度运算）
#[derive(Debug, Clone)]
pub struct ExactCacheKey {
    /// 操作数1
    pub operand1: Number,
    /// 操作数2（可选）
    pub operand2: Option<Number>,
    /// 运算类型
    pub operation: String,
}

impl PartialEq for ExactCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.operand1 == other.operand1 
            && self.operand2 == other.operand2 
            && self.operation == other.operation
    }
}

impl Eq for ExactCacheKey {}

impl Hash for ExactCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 为 Number 实现简单的哈希
        match &self.operand1 {
            Number::Integer(i) => {
                0u8.hash(state);
                i.to_string().hash(state);
            }
            Number::Rational(r) => {
                1u8.hash(state);
                r.to_string().hash(state);
            }
            Number::Real(r) => {
                2u8.hash(state);
                r.to_string().hash(state);
            }
            Number::Complex { real, imaginary } => {
                3u8.hash(state);
                real.hash(state);
                imaginary.hash(state);
            }
            Number::Symbolic(expr) => {
                4u8.hash(state);
                format!("{:?}", expr).hash(state);
            }
            Number::Float(f) => {
                5u8.hash(state);
                f.to_bits().hash(state);
            }
            Number::Constant(c) => {
                6u8.hash(state);
                format!("{:?}", c).hash(state);
            }
        }
        
        if let Some(ref op2) = self.operand2 {
            match op2 {
                Number::Integer(i) => {
                    0u8.hash(state);
                    i.to_string().hash(state);
                }
                Number::Rational(r) => {
                    1u8.hash(state);
                    r.to_string().hash(state);
                }
                Number::Real(r) => {
                    2u8.hash(state);
                    r.to_string().hash(state);
                }
                Number::Complex { real, imaginary } => {
                    3u8.hash(state);
                    real.hash(state);
                    imaginary.hash(state);
                }
                Number::Symbolic(expr) => {
                    4u8.hash(state);
                    format!("{:?}", expr).hash(state);
                }
                Number::Float(f) => {
                    5u8.hash(state);
                    f.to_bits().hash(state);
                }
                Number::Constant(c) => {
                    6u8.hash(state);
                    format!("{:?}", c).hash(state);
                }
            }
        }
        
        self.operation.hash(state);
    }
}

/// 符号缓存键（用于符号简化结果）
#[derive(Debug, Clone)]
pub struct SymbolicCacheKey {
    /// 表达式
    pub expression: Expression,
    /// 操作类型
    pub operation: String,
    /// 变量（如果适用）
    pub variable: Option<String>,
}

impl PartialEq for SymbolicCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression 
            && self.operation == other.operation 
            && self.variable == other.variable
    }
}

impl Eq for SymbolicCacheKey {}

impl Hash for SymbolicCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:?}", self.expression).hash(state);
        self.operation.hash(state);
        self.variable.hash(state);
    }
}

/// 多层缓存系统
#[derive(Debug)]
pub struct ComputeCache {
    /// 快速缓存：小整数运算结果
    fast_cache: Arc<RwLock<HashMap<FastCacheKey, CacheEntry<i64>>>>,
    /// 精确缓存：任意精度运算结果
    exact_cache: Arc<RwLock<HashMap<ExactCacheKey, CacheEntry<Number>>>>,
    /// 符号缓存：符号简化结果
    symbolic_cache: Arc<RwLock<HashMap<SymbolicCacheKey, CacheEntry<Expression>>>>,
    /// 缓存配置
    config: CacheConfig,
    /// 缓存统计信息
    stats: Arc<RwLock<CacheStats>>,
}

/// 缓存统计信息
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// 快速缓存命中次数
    pub fast_hits: u64,
    /// 快速缓存未命中次数
    pub fast_misses: u64,
    /// 精确缓存命中次数
    pub exact_hits: u64,
    /// 精确缓存未命中次数
    pub exact_misses: u64,
    /// 符号缓存命中次数
    pub symbolic_hits: u64,
    /// 符号缓存未命中次数
    pub symbolic_misses: u64,
    /// 缓存清理次数
    pub cleanup_count: u64,
    /// 总节省的计算时间（估算）
    pub total_time_saved: Duration,
}

impl CacheStats {
    /// 计算总命中率
    pub fn total_hit_rate(&self) -> f64 {
        let total_hits = self.fast_hits + self.exact_hits + self.symbolic_hits;
        let total_requests = total_hits + self.fast_misses + self.exact_misses + self.symbolic_misses;
        
        if total_requests == 0 {
            0.0
        } else {
            total_hits as f64 / total_requests as f64
        }
    }
    
    /// 计算快速缓存命中率
    pub fn fast_hit_rate(&self) -> f64 {
        let total = self.fast_hits + self.fast_misses;
        if total == 0 {
            0.0
        } else {
            self.fast_hits as f64 / total as f64
        }
    }
    
    /// 计算精确缓存命中率
    pub fn exact_hit_rate(&self) -> f64 {
        let total = self.exact_hits + self.exact_misses;
        if total == 0 {
            0.0
        } else {
            self.exact_hits as f64 / total as f64
        }
    }
    
    /// 计算符号缓存命中率
    pub fn symbolic_hit_rate(&self) -> f64 {
        let total = self.symbolic_hits + self.symbolic_misses;
        if total == 0 {
            0.0
        } else {
            self.symbolic_hits as f64 / total as f64
        }
    }
}

impl ComputeCache {
    /// 创建新的缓存系统
    pub fn new(config: CacheConfig) -> Self {
        Self {
            fast_cache: Arc::new(RwLock::new(HashMap::new())),
            exact_cache: Arc::new(RwLock::new(HashMap::new())),
            symbolic_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }
    
    /// 获取快速缓存中的值
    pub fn get_fast(&self, key: &FastCacheKey) -> Option<i64> {
        if !self.config.enabled {
            return None;
        }
        
        let mut cache = self.fast_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            // 检查是否过期
            if let Some(ttl) = self.config.cache_ttl {
                if entry.is_expired(ttl) {
                    cache.remove(key);
                    self.record_fast_miss();
                    return None;
                }
            }
            
            entry.access();
            self.record_fast_hit();
            Some(entry.value)
        } else {
            self.record_fast_miss();
            None
        }
    }
    
    /// 在快速缓存中存储值
    pub fn put_fast(&self, key: FastCacheKey, value: i64, compute_cost: u32) {
        if !self.config.enabled {
            return;
        }
        
        let mut cache = self.fast_cache.write().unwrap();
        
        // 检查缓存大小限制
        if cache.len() >= self.config.fast_cache_size {
            self.cleanup_fast_cache(&mut cache);
        }
        
        cache.insert(key, CacheEntry::new(value, compute_cost));
    }
    
    /// 获取精确缓存中的值
    pub fn get_exact(&self, key: &ExactCacheKey) -> Option<Number> {
        if !self.config.enabled {
            return None;
        }
        
        let mut cache = self.exact_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            // 检查是否过期
            if let Some(ttl) = self.config.cache_ttl {
                if entry.is_expired(ttl) {
                    cache.remove(key);
                    self.record_exact_miss();
                    return None;
                }
            }
            
            entry.access();
            self.record_exact_hit();
            Some(entry.value.clone())
        } else {
            self.record_exact_miss();
            None
        }
    }
    
    /// 在精确缓存中存储值
    pub fn put_exact(&self, key: ExactCacheKey, value: Number, compute_cost: u32) {
        if !self.config.enabled {
            return;
        }
        
        let mut cache = self.exact_cache.write().unwrap();
        
        // 检查缓存大小限制
        if cache.len() >= self.config.exact_cache_size {
            self.cleanup_exact_cache(&mut cache);
        }
        
        cache.insert(key, CacheEntry::new(value, compute_cost));
    }
    
    /// 获取符号缓存中的值
    pub fn get_symbolic(&self, key: &SymbolicCacheKey) -> Option<Expression> {
        if !self.config.enabled {
            return None;
        }
        
        let mut cache = self.symbolic_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            // 检查是否过期
            if let Some(ttl) = self.config.cache_ttl {
                if entry.is_expired(ttl) {
                    cache.remove(key);
                    self.record_symbolic_miss();
                    return None;
                }
            }
            
            entry.access();
            self.record_symbolic_hit();
            Some(entry.value.clone())
        } else {
            self.record_symbolic_miss();
            None
        }
    }
    
    /// 在符号缓存中存储值
    pub fn put_symbolic(&self, key: SymbolicCacheKey, value: Expression, compute_cost: u32) {
        if !self.config.enabled {
            return;
        }
        
        let mut cache = self.symbolic_cache.write().unwrap();
        
        // 检查缓存大小限制
        if cache.len() >= self.config.symbolic_cache_size {
            self.cleanup_symbolic_cache(&mut cache);
        }
        
        cache.insert(key, CacheEntry::new(value, compute_cost));
    }
    
    /// 清理快速缓存
    fn cleanup_fast_cache(&self, cache: &mut HashMap<FastCacheKey, CacheEntry<i64>>) {
        let target_size = self.config.fast_cache_size * 3 / 4; // 清理到75%
        
        if cache.len() <= target_size {
            return;
        }
        
        // 收集所有项目及其优先级
        let mut items: Vec<_> = cache.iter()
            .map(|(key, entry)| (key.clone(), entry.priority()))
            .collect();
        
        // 按优先级排序（低优先级先删除）
        items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // 删除低优先级项目
        let to_remove = cache.len() - target_size;
        for (key, _) in items.iter().take(to_remove) {
            cache.remove(key);
        }
        
        self.record_cleanup();
    }
    
    /// 清理精确缓存
    fn cleanup_exact_cache(&self, cache: &mut HashMap<ExactCacheKey, CacheEntry<Number>>) {
        let target_size = self.config.exact_cache_size * 3 / 4; // 清理到75%
        
        if cache.len() <= target_size {
            return;
        }
        
        // 收集所有项目及其优先级
        let mut items: Vec<_> = cache.iter()
            .map(|(key, entry)| (key.clone(), entry.priority()))
            .collect();
        
        // 按优先级排序（低优先级先删除）
        items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // 删除低优先级项目
        let to_remove = cache.len() - target_size;
        for (key, _) in items.iter().take(to_remove) {
            cache.remove(key);
        }
        
        self.record_cleanup();
    }
    
    /// 清理符号缓存
    fn cleanup_symbolic_cache(&self, cache: &mut HashMap<SymbolicCacheKey, CacheEntry<Expression>>) {
        let target_size = self.config.symbolic_cache_size * 3 / 4; // 清理到75%
        
        if cache.len() <= target_size {
            return;
        }
        
        // 收集所有项目及其优先级
        let mut items: Vec<_> = cache.iter()
            .map(|(key, entry)| (key.clone(), entry.priority()))
            .collect();
        
        // 按优先级排序（低优先级先删除）
        items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // 删除低优先级项目
        let to_remove = cache.len() - target_size;
        for (key, _) in items.iter().take(to_remove) {
            cache.remove(key);
        }
        
        self.record_cleanup();
    }
    
    /// 清理所有过期项目
    pub fn cleanup_expired(&self) {
        if let Some(ttl) = self.config.cache_ttl {
            // 清理快速缓存中的过期项目
            {
                let mut cache = self.fast_cache.write().unwrap();
                cache.retain(|_, entry| !entry.is_expired(ttl));
            }
            
            // 清理精确缓存中的过期项目
            {
                let mut cache = self.exact_cache.write().unwrap();
                cache.retain(|_, entry| !entry.is_expired(ttl));
            }
            
            // 清理符号缓存中的过期项目
            {
                let mut cache = self.symbolic_cache.write().unwrap();
                cache.retain(|_, entry| !entry.is_expired(ttl));
            }
            
            self.record_cleanup();
        }
    }
    
    /// 清空所有缓存
    pub fn clear_all(&self) {
        self.fast_cache.write().unwrap().clear();
        self.exact_cache.write().unwrap().clear();
        self.symbolic_cache.write().unwrap().clear();
        self.record_cleanup();
    }
    
    /// 获取缓存统计信息
    pub fn get_stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }
    
    /// 获取缓存使用情况
    pub fn get_usage_info(&self) -> CacheUsageInfo {
        let fast_size = self.fast_cache.read().unwrap().len();
        let exact_size = self.exact_cache.read().unwrap().len();
        let symbolic_size = self.symbolic_cache.read().unwrap().len();
        
        CacheUsageInfo {
            fast_cache_usage: fast_size,
            fast_cache_capacity: self.config.fast_cache_size,
            exact_cache_usage: exact_size,
            exact_cache_capacity: self.config.exact_cache_size,
            symbolic_cache_usage: symbolic_size,
            symbolic_cache_capacity: self.config.symbolic_cache_size,
        }
    }
    
    // 统计记录方法
    
    fn record_fast_hit(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.fast_hits += 1;
            // 估算节省的时间（快速运算约节省1微秒）
            stats.total_time_saved += Duration::from_micros(1);
        }
    }
    
    fn record_fast_miss(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.fast_misses += 1;
        }
    }
    
    fn record_exact_hit(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.exact_hits += 1;
            // 估算节省的时间（精确运算约节省100微秒）
            stats.total_time_saved += Duration::from_micros(100);
        }
    }
    
    fn record_exact_miss(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.exact_misses += 1;
        }
    }
    
    fn record_symbolic_hit(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.symbolic_hits += 1;
            // 估算节省的时间（符号运算约节省1毫秒）
            stats.total_time_saved += Duration::from_millis(1);
        }
    }
    
    fn record_symbolic_miss(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.symbolic_misses += 1;
        }
    }
    
    fn record_cleanup(&self) {
        if let Ok(mut stats) = self.stats.write() {
            stats.cleanup_count += 1;
        }
    }
}

/// 缓存使用情况信息
#[derive(Debug, Clone)]
pub struct CacheUsageInfo {
    /// 快速缓存使用量
    pub fast_cache_usage: usize,
    /// 快速缓存容量
    pub fast_cache_capacity: usize,
    /// 精确缓存使用量
    pub exact_cache_usage: usize,
    /// 精确缓存容量
    pub exact_cache_capacity: usize,
    /// 符号缓存使用量
    pub symbolic_cache_usage: usize,
    /// 符号缓存容量
    pub symbolic_cache_capacity: usize,
}

impl CacheUsageInfo {
    /// 计算快速缓存使用率
    pub fn fast_cache_usage_rate(&self) -> f64 {
        if self.fast_cache_capacity == 0 {
            0.0
        } else {
            self.fast_cache_usage as f64 / self.fast_cache_capacity as f64
        }
    }
    
    /// 计算精确缓存使用率
    pub fn exact_cache_usage_rate(&self) -> f64 {
        if self.exact_cache_capacity == 0 {
            0.0
        } else {
            self.exact_cache_usage as f64 / self.exact_cache_capacity as f64
        }
    }
    
    /// 计算符号缓存使用率
    pub fn symbolic_cache_usage_rate(&self) -> f64 {
        if self.symbolic_cache_capacity == 0 {
            0.0
        } else {
            self.symbolic_cache_usage as f64 / self.symbolic_cache_capacity as f64
        }
    }
    
    /// 计算总体缓存使用率
    pub fn total_usage_rate(&self) -> f64 {
        let total_usage = self.fast_cache_usage + self.exact_cache_usage + self.symbolic_cache_usage;
        let total_capacity = self.fast_cache_capacity + self.exact_cache_capacity + self.symbolic_cache_capacity;
        
        if total_capacity == 0 {
            0.0
        } else {
            total_usage as f64 / total_capacity as f64
        }
    }
}

/// 缓存管理器
pub struct CacheManager {
    cache: ComputeCache,
    last_cleanup: Instant,
    cleanup_interval: Duration,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: ComputeCache::new(config),
            last_cleanup: Instant::now(),
            cleanup_interval: Duration::from_secs(300), // 5分钟清理一次
        }
    }
    
    /// 获取缓存引用
    pub fn cache(&self) -> &ComputeCache {
        &self.cache
    }
    
    /// 定期清理缓存
    pub fn periodic_cleanup(&mut self) {
        if self.last_cleanup.elapsed() >= self.cleanup_interval {
            self.cache.cleanup_expired();
            self.last_cleanup = Instant::now();
        }
    }
    
    /// 强制清理缓存
    pub fn force_cleanup(&mut self) {
        self.cache.cleanup_expired();
        self.last_cleanup = Instant::now();
    }
    
    /// 设置清理间隔
    pub fn set_cleanup_interval(&mut self, interval: Duration) {
        self.cleanup_interval = interval;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Number;
    use num_bigint::BigInt;
    
    #[test]
    fn test_cache_entry_creation() {
        let entry = CacheEntry::new(42i64, 10);
        assert_eq!(entry.value, 42);
        assert_eq!(entry.access_count, 1);
        assert_eq!(entry.compute_cost, 10);
    }
    
    #[test]
    fn test_cache_entry_access() {
        let mut entry = CacheEntry::new(42i64, 10);
        let initial_time = entry.last_accessed;
        
        std::thread::sleep(Duration::from_millis(1));
        entry.access();
        
        assert_eq!(entry.access_count, 2);
        assert!(entry.last_accessed > initial_time);
    }
    
    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new(42i64, 10);
        
        // 不应该立即过期
        assert!(!entry.is_expired(Duration::from_secs(1)));
        
        // 使用很短的TTL应该过期
        assert!(entry.is_expired(Duration::from_nanos(1)));
    }
    
    #[test]
    fn test_fast_cache_operations() {
        let config = CacheConfig::default();
        let cache = ComputeCache::new(config);
        
        let key = FastCacheKey::BinaryOp(2, 3, BinaryOperator::Add);
        
        // 初始时应该没有值
        assert!(cache.get_fast(&key).is_none());
        
        // 存储值
        cache.put_fast(key.clone(), 5, 1);
        
        // 应该能够获取值
        assert_eq!(cache.get_fast(&key), Some(5));
        
        // 统计信息应该正确
        let stats = cache.get_stats();
        assert_eq!(stats.fast_hits, 1);
        assert_eq!(stats.fast_misses, 1);
    }
    
    #[test]
    fn test_exact_cache_operations() {
        let config = CacheConfig::default();
        let cache = ComputeCache::new(config);
        
        let key = ExactCacheKey {
            operand1: Number::Integer(BigInt::from(123)),
            operand2: Some(Number::Integer(BigInt::from(456))),
            operation: "add".to_string(),
        };
        
        let value = Number::Integer(BigInt::from(579));
        
        // 初始时应该没有值
        assert!(cache.get_exact(&key).is_none());
        
        // 存储值
        cache.put_exact(key.clone(), value.clone(), 5);
        
        // 应该能够获取值
        assert_eq!(cache.get_exact(&key), Some(value));
        
        // 统计信息应该正确
        let stats = cache.get_stats();
        assert_eq!(stats.exact_hits, 1);
        assert_eq!(stats.exact_misses, 1);
    }
    
    #[test]
    fn test_symbolic_cache_operations() {
        let config = CacheConfig::default();
        let cache = ComputeCache::new(config);
        
        let key = SymbolicCacheKey {
            expression: Expression::variable("x"),
            operation: "simplify".to_string(),
            variable: None,
        };
        
        let value = Expression::variable("x");
        
        // 初始时应该没有值
        assert!(cache.get_symbolic(&key).is_none());
        
        // 存储值
        cache.put_symbolic(key.clone(), value.clone(), 10);
        
        // 应该能够获取值
        assert_eq!(cache.get_symbolic(&key), Some(value));
        
        // 统计信息应该正确
        let stats = cache.get_stats();
        assert_eq!(stats.symbolic_hits, 1);
        assert_eq!(stats.symbolic_misses, 1);
    }
    
    #[test]
    fn test_cache_size_limits() {
        let config = CacheConfig {
            enabled: true,
            fast_cache_size: 2,
            exact_cache_size: 2,
            symbolic_cache_size: 2,
            cache_ttl: None,
        };
        
        let cache = ComputeCache::new(config);
        
        // 填满快速缓存
        cache.put_fast(FastCacheKey::BinaryOp(1, 2, BinaryOperator::Add), 3, 1);
        cache.put_fast(FastCacheKey::BinaryOp(2, 3, BinaryOperator::Add), 5, 1);
        
        // 添加第三个项目应该触发清理
        cache.put_fast(FastCacheKey::BinaryOp(3, 4, BinaryOperator::Add), 7, 1);
        
        let usage = cache.get_usage_info();
        assert!(usage.fast_cache_usage <= 2);
    }
    
    #[test]
    fn test_cache_stats() {
        let config = CacheConfig::default();
        let cache = ComputeCache::new(config);
        
        let key = FastCacheKey::BinaryOp(1, 1, BinaryOperator::Add);
        
        // 未命中
        cache.get_fast(&key);
        
        // 存储
        cache.put_fast(key.clone(), 2, 1);
        
        // 命中
        cache.get_fast(&key);
        cache.get_fast(&key);
        
        let stats = cache.get_stats();
        assert_eq!(stats.fast_hits, 2);
        assert_eq!(stats.fast_misses, 1);
        assert_eq!(stats.fast_hit_rate(), 2.0 / 3.0);
    }
    
    #[test]
    fn test_cache_manager() {
        let config = CacheConfig::default();
        let mut manager = CacheManager::new(config);
        
        // 测试定期清理
        manager.set_cleanup_interval(Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(2));
        manager.periodic_cleanup();
        
        // 测试强制清理
        manager.force_cleanup();
    }
}