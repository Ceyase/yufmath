//! # Yufmath 主要 API 接口
//!
//! 提供 Yufmath 库的主要入口点和核心功能。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::core::{Expression, Number};
use crate::parser::{Parser, ParseError, syntax::ExpressionParser};
use crate::engine::{ComputeEngine, ComputeError, CachedComputeEngine, EnhancedComputeEngine, RuntimeEnhancedEngine, RuntimeConfig, CacheStats, CacheUsageInfo};
use crate::formatter::{Formatter, FormatOptions, MultiFormatter};
use super::{YufmathError, ComputeConfig, PerformanceMonitor, ComputeProgress};
use super::progress::ProgressCallback;
use super::async_compute::{AsyncComputation, BatchAsyncComputer, AsyncConfig};

/// Yufmath 库的主要入口点
pub struct Yufmath {
    parser: Box<dyn Parser>,
    engine: Box<dyn ComputeEngine>,
    formatter: Arc<Mutex<Box<dyn Formatter>>>,
    monitor: Arc<Mutex<PerformanceMonitor>>,
    config: ComputeConfig,
    cancelled: Arc<AtomicBool>,
    async_computer: Arc<BatchAsyncComputer>,
}

impl Yufmath {
    /// 创建新的 Yufmath 实例
    pub fn new() -> Self {
        let config = ComputeConfig::default();
        let async_computer = Arc::new(BatchAsyncComputer::new(config.parallel.max_parallel_tasks));
        
        Self {
            parser: Box::new(ExpressionParser::new()),
            engine: Box::new(RuntimeEnhancedEngine::new()),
            formatter: Arc::new(Mutex::new(Box::new(MultiFormatter::new()))),
            monitor: Arc::new(Mutex::new(PerformanceMonitor::new())),
            config,
            cancelled: Arc::new(AtomicBool::new(false)),
            async_computer,
        }
    }
    
    /// 创建带配置的 Yufmath 实例
    pub fn with_config(config: ComputeConfig) -> Self {
        let async_computer = Arc::new(BatchAsyncComputer::new(config.parallel.max_parallel_tasks));
        
        Self {
            parser: Box::new(ExpressionParser::new()),
            engine: Box::new(RuntimeEnhancedEngine::new()),
            formatter: Arc::new(Mutex::new(Box::new(MultiFormatter::new()))),
            monitor: Arc::new(Mutex::new(PerformanceMonitor::new())),
            config,
            cancelled: Arc::new(AtomicBool::new(false)),
            async_computer,
        }
    }
    
    /// 解析并计算表达式
    pub fn compute(&self, input: &str) -> Result<String, YufmathError> {
        let expr = self.parser.parse(input)?;
        
        // 使用安全计算（如果是运行时增强引擎）
        let result = if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            // 使用运行时增强引擎的安全计算
            runtime_engine.safe_compute(&expr)?
        } else {
            // 尝试求值（对于纯数值表达式）
            let vars = HashMap::new();
            match self.engine.evaluate(&expr, &vars) {
                Ok(number) => Expression::Number(number),
                Err(_) => {
                    // 如果求值失败，则简化表达式
                    self.engine.simplify(&expr)?
                }
            }
        };
        
        let formatter = self.formatter.lock()
            .map_err(|_| YufmathError::internal("无法获取格式化器锁"))?;
        Ok(formatter.format(&result))
    }
    
    /// 解析表达式
    pub fn parse(&self, input: &str) -> Result<Expression, YufmathError> {
        Ok(self.parser.parse(input)?)
    }
    
    /// 简化表达式
    pub fn simplify(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.simplify(expr)?)
    }
    
    /// 求导
    pub fn diff(&self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        Ok(self.engine.differentiate(expr, var)?)
    }
    
    /// 求导（别名方法）
    pub fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        self.diff(expr, var)
    }
    
    /// 积分
    pub fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        Ok(self.engine.integrate(expr, var)?)
    }
    
    /// 计算极限
    pub fn limit(&self, expr: &Expression, var: &str, point: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.limit(expr, var, point)?)
    }
    
    /// 级数展开
    pub fn series(&self, expr: &Expression, var: &str, point: &Expression, order: usize) -> Result<Expression, YufmathError> {
        Ok(self.engine.series(expr, var, point, order)?)
    }
    
    /// 数值计算
    pub fn numerical_evaluate(&self, expr: &Expression, vars: &std::collections::HashMap<String, f64>) -> Result<f64, YufmathError> {
        Ok(self.engine.numerical_evaluate(expr, vars)?)
    }
    
    /// 计算表达式的数值
    pub fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) -> Result<Number, YufmathError> {
        Ok(self.engine.evaluate(expr, vars)?)
    }
    
    /// 设置格式化选项
    pub fn set_format_options(&mut self, options: FormatOptions) {
        if let Ok(mut formatter) = self.formatter.lock() {
            formatter.set_options(options);
        }
    }
    
    /// 设置是否启用增强化简功能
    pub fn set_enhanced_simplify(&mut self, enabled: bool) {
        if let Some(enhanced_engine) = self.engine.as_any().downcast_ref::<EnhancedComputeEngine>() {
            enhanced_engine.set_auto_simplify(enabled);
        }
    }
    
    /// 检查是否启用了增强化简功能
    pub fn is_enhanced_simplify_enabled(&self) -> bool {
        if let Some(enhanced_engine) = self.engine.as_any().downcast_ref::<EnhancedComputeEngine>() {
            enhanced_engine.is_auto_simplify_enabled()
        } else {
            false
        }
    }
    
    /// 设置进度回调函数
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        if let Ok(mut monitor) = self.monitor.lock() {
            monitor.set_progress_callback(callback);
        }
    }
    
    /// 带进度监控的计算表达式
    pub fn compute_with_progress(&mut self, input: &str) -> Result<String, YufmathError> {
        self.cancelled.store(false, Ordering::Relaxed);
        
        let timer = if let Ok(mut monitor) = self.monitor.lock() {
            monitor.start_computation()
        } else {
            return Err(YufmathError::internal("无法获取性能监控器"));
        };
        
        // 更新进度：开始解析
        self.update_progress(ComputeProgress::new("解析表达式").with_progress(0.1))?;
        
        let expr = self.parser.parse(input)?;
        
        // 更新进度：开始简化
        self.update_progress(ComputeProgress::new("简化表达式").with_progress(0.5))?;
        
        let simplified = self.engine.simplify(&expr)?;
        
        // 更新进度：格式化输出
        self.update_progress(ComputeProgress::new("格式化结果").with_progress(0.9))?;
        
        let result = {
            let formatter = self.formatter.lock()
                .map_err(|_| YufmathError::internal("无法获取格式化器锁"))?;
            formatter.format(&simplified)
        };
        
        // 记录计算完成
        if let Ok(mut monitor) = self.monitor.lock() {
            monitor.record_computation(timer, true, true);
        }
        
        // 更新进度：完成
        self.update_progress(ComputeProgress::new("计算完成").with_progress(1.0))?;
        
        Ok(result)
    }
    
    /// 带进度监控的简化表达式
    pub fn simplify_with_progress(&mut self, expr: &Expression) -> Result<Expression, YufmathError> {
        self.cancelled.store(false, Ordering::Relaxed);
        
        let timer = if let Ok(mut monitor) = self.monitor.lock() {
            monitor.start_computation()
        } else {
            return Err(YufmathError::internal("无法获取性能监控器"));
        };
        
        // 更新进度：开始简化
        self.update_progress(ComputeProgress::new("分析表达式结构").with_progress(0.2))?;
        
        let result = self.engine.simplify(expr)?;
        
        // 记录计算完成
        if let Ok(mut monitor) = self.monitor.lock() {
            monitor.record_computation(timer, true, true);
        }
        
        // 更新进度：完成
        self.update_progress(ComputeProgress::new("简化完成").with_progress(1.0))?;
        
        Ok(result)
    }
    
    /// 带进度监控的积分计算
    pub fn integrate_with_progress(&mut self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        self.cancelled.store(false, Ordering::Relaxed);
        
        let timer = if let Ok(mut monitor) = self.monitor.lock() {
            monitor.start_computation()
        } else {
            return Err(YufmathError::internal("无法获取性能监控器"));
        };
        
        // 更新进度：开始积分
        self.update_progress(ComputeProgress::new("分析被积函数").with_progress(0.1))?;
        
        self.update_progress(ComputeProgress::new("应用积分规则").with_progress(0.5))?;
        
        let result = self.engine.integrate(expr, var)?;
        
        // 记录计算完成
        if let Ok(mut monitor) = self.monitor.lock() {
            monitor.record_computation(timer, true, true);
        }
        
        // 更新进度：完成
        self.update_progress(ComputeProgress::new("积分完成").with_progress(1.0))?;
        
        Ok(result)
    }
    
    /// 取消当前计算
    pub fn cancel_computation(&mut self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
    
    /// 检查是否被取消
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
    
    /// 批量计算多个表达式
    pub fn batch_compute(&self, inputs: &[&str]) -> Vec<Result<String, YufmathError>> {
        inputs.iter().map(|input| self.compute(input)).collect()
    }
    
    /// 批量解析多个表达式
    pub fn batch_parse(&self, inputs: &[&str]) -> Vec<Result<Expression, YufmathError>> {
        inputs.iter().map(|input| self.parse(input)).collect()
    }
    
    /// 批量简化多个表达式
    pub fn batch_simplify(&self, expressions: &[Expression]) -> Vec<Result<Expression, YufmathError>> {
        expressions.iter().map(|expr| self.simplify(expr)).collect()
    }
    
    /// 获取配置信息
    pub fn get_config(&self) -> &ComputeConfig {
        &self.config
    }
    
    /// 更新配置信息
    pub fn update_config(&mut self, config: ComputeConfig) {
        self.config = config;
    }
    
    /// 获取性能统计信息
    pub fn get_performance_stats(&self) -> Option<crate::api::PerformanceStats> {
        self.monitor.lock().ok().map(|monitor| monitor.get_stats().clone())
    }
    
    /// 重置性能统计
    pub fn reset_performance_stats(&mut self) {
        if let Ok(mut monitor) = self.monitor.lock() {
            monitor.reset_stats();
        }
    }
    
    /// 多项式运算：展开
    pub fn expand(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.expand(expr)?)
    }
    
    /// 多项式运算：因式分解
    pub fn factor(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.factor(expr)?)
    }
    
    /// 多项式运算：收集同类项
    pub fn collect(&self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        Ok(self.engine.collect(expr, var)?)
    }
    
    /// 方程求解
    pub fn solve(&self, equation: &Expression, var: &str) -> Result<Vec<Expression>, YufmathError> {
        Ok(self.engine.solve(equation, var)?)
    }
    
    /// 方程组求解
    pub fn solve_system(&self, equations: &[Expression], vars: &[String]) -> Result<Vec<HashMap<String, Expression>>, YufmathError> {
        Ok(self.engine.solve_system(equations, vars)?)
    }
    
    /// 矩阵运算：加法
    pub fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.matrix_add(a, b)?)
    }
    
    /// 矩阵运算：乘法
    pub fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.matrix_multiply(a, b)?)
    }
    
    /// 矩阵运算：行列式
    pub fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.matrix_determinant(matrix)?)
    }
    
    /// 矩阵运算：逆矩阵
    pub fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.matrix_inverse(matrix)?)
    }
    
    /// 数论函数：最大公约数
    pub fn gcd(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.gcd(a, b)?)
    }
    
    /// 数论函数：最小公倍数
    pub fn lcm(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.lcm(a, b)?)
    }
    
    /// 数论函数：素数判断
    pub fn is_prime(&self, n: &Expression) -> Result<bool, YufmathError> {
        Ok(self.engine.is_prime(n)?)
    }
    
    /// 数论函数：质因数分解
    pub fn prime_factors(&self, n: &Expression) -> Result<Vec<Expression>, YufmathError> {
        Ok(self.engine.prime_factors(n)?)
    }
    
    /// 组合数学：二项式系数
    pub fn binomial(&self, n: &Expression, k: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.binomial(n, k)?)
    }
    
    /// 组合数学：排列数
    pub fn permutation(&self, n: &Expression, k: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.permutation(n, k)?)
    }
    
    /// 复数运算：共轭
    pub fn complex_conjugate(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.complex_conjugate(expr)?)
    }
    
    /// 复数运算：模长
    pub fn complex_modulus(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.complex_modulus(expr)?)
    }
    
    /// 复数运算：幅角
    pub fn complex_argument(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.complex_argument(expr)?)
    }
    
    /// 向量运算：点积
    pub fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.vector_dot(a, b)?)
    }
    
    /// 向量运算：叉积
    pub fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.vector_cross(a, b)?)
    }
    
    /// 向量运算：范数
    pub fn vector_norm(&self, v: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.vector_norm(v)?)
    }
    
    /// 集合运算：并集
    pub fn set_union(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.set_union(a, b)?)
    }
    
    /// 集合运算：交集
    pub fn set_intersection(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.set_intersection(a, b)?)
    }
    
    /// 集合运算：差集
    pub fn set_difference(&self, a: &Expression, b: &Expression) -> Result<Expression, YufmathError> {
        Ok(self.engine.set_difference(a, b)?)
    }
    
    /// 统计函数：平均值
    pub fn mean(&self, values: &[Expression]) -> Result<Expression, YufmathError> {
        Ok(self.engine.mean(values)?)
    }
    
    /// 统计函数：方差
    pub fn variance(&self, values: &[Expression]) -> Result<Expression, YufmathError> {
        Ok(self.engine.variance(values)?)
    }
    
    /// 统计函数：标准差
    pub fn standard_deviation(&self, values: &[Expression]) -> Result<Expression, YufmathError> {
        Ok(self.engine.standard_deviation(values)?)
    }
    
    /// 异步计算表达式
    pub fn compute_async(&self, input: &str) -> AsyncComputation<String> {
        let expressions = vec![input.to_string()];
        let mut computations = self.async_computer.submit_batch(expressions);
        computations.pop().unwrap()
    }
    
    /// 异步批量计算
    pub fn batch_compute_async(&self, inputs: &[&str]) -> Vec<AsyncComputation<String>> {
        let expressions: Vec<String> = inputs.iter().map(|s| s.to_string()).collect();
        self.async_computer.submit_batch(expressions)
    }
    
    /// 获取活跃的异步任务数量
    pub fn active_async_tasks(&self) -> usize {
        self.async_computer.active_task_count()
    }
    
    /// 取消所有异步任务
    pub fn cancel_all_async_tasks(&self) {
        self.async_computer.cancel_all();
    }
    
    /// 清理已完成的异步任务
    pub fn cleanup_async_tasks(&self) {
        self.async_computer.cleanup_completed();
    }
    
    /// 获取缓存统计信息
    pub fn get_cache_stats(&self) -> Option<CacheStats> {
        // 尝试将引擎转换为 CachedComputeEngine
        if let Some(cached_engine) = self.engine.as_any().downcast_ref::<CachedComputeEngine>() {
            cached_engine.get_cache_stats().ok()
        } else {
            None
        }
    }
    
    /// 获取缓存使用情况
    pub fn get_cache_usage(&self) -> Option<CacheUsageInfo> {
        if let Some(cached_engine) = self.engine.as_any().downcast_ref::<CachedComputeEngine>() {
            cached_engine.get_cache_usage().ok()
        } else {
            None
        }
    }
    
    /// 清理缓存
    pub fn cleanup_cache(&self) -> Result<(), YufmathError> {
        if let Some(cached_engine) = self.engine.as_any().downcast_ref::<CachedComputeEngine>() {
            cached_engine.cleanup_cache().map_err(YufmathError::from)
        } else {
            Ok(()) // 如果不是缓存引擎，直接返回成功
        }
    }
    
    /// 清空所有缓存
    pub fn clear_cache(&self) -> Result<(), YufmathError> {
        if let Some(cached_engine) = self.engine.as_any().downcast_ref::<CachedComputeEngine>() {
            cached_engine.clear_cache().map_err(YufmathError::from)
        } else {
            Ok(()) // 如果不是缓存引擎，直接返回成功
        }
    }
    
    /// 设置变量值
    pub fn set_variable(&self, name: String, value: Expression) -> Result<(), YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.set_variable(name, value).map_err(YufmathError::from)
        } else {
            Err(YufmathError::internal("当前引擎不支持变量管理"))
        }
    }
    
    /// 设置变量值（从字符串解析）
    pub fn set_variable_from_string(&self, name: String, value_str: &str) -> Result<(), YufmathError> {
        let value_expr = self.parse(value_str)?;
        self.set_variable(name, value_expr)
    }
    
    /// 获取变量值
    pub fn get_variable(&self, name: &str) -> Result<Option<Expression>, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.get_variable(name).map_err(YufmathError::from)
        } else {
            Ok(None)
        }
    }
    
    /// 获取所有变量
    pub fn get_all_variables(&self) -> Result<HashMap<String, Expression>, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.get_all_variables().map_err(YufmathError::from)
        } else {
            Ok(HashMap::new())
        }
    }
    
    /// 清空所有变量
    pub fn clear_variables(&self) -> Result<(), YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.clear_variables().map_err(YufmathError::from)
        } else {
            Ok(())
        }
    }
    
    /// 删除指定变量
    pub fn remove_variable(&self, name: &str) -> Result<bool, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.remove_variable(name).map_err(YufmathError::from)
        } else {
            Ok(false)
        }
    }
    
    /// 更新运行时配置
    pub fn update_runtime_config(&self, config: RuntimeConfig) -> Result<(), YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.update_runtime_config(config).map_err(YufmathError::from)
        } else {
            Err(YufmathError::internal("当前引擎不支持运行时配置"))
        }
    }
    
    /// 获取运行时配置
    pub fn get_runtime_config(&self) -> Result<RuntimeConfig, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.get_runtime_config().map_err(YufmathError::from)
        } else {
            Err(YufmathError::internal("当前引擎不支持运行时配置"))
        }
    }
    
    /// 安全计算表达式（带运行时增强）
    pub fn safe_compute(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.safe_compute(expr).map_err(YufmathError::from)
        } else {
            // 如果不是运行时增强引擎，使用普通简化
            self.simplify(expr)
        }
    }
    
    /// 计算表达式并自动替换变量
    pub fn compute_with_variables(&self, expr: &Expression) -> Result<Expression, YufmathError> {
        if let Some(runtime_engine) = self.engine.as_any().downcast_ref::<RuntimeEnhancedEngine>() {
            runtime_engine.compute_with_variables(expr).map_err(YufmathError::from)
        } else {
            // 如果不是运行时增强引擎，使用普通简化
            self.simplify(expr)
        }
    }
    
    /// 内部方法：更新进度
    fn update_progress(&self, progress: ComputeProgress) -> Result<(), YufmathError> {
        if !self.config.enable_progress {
            return Ok(());
        }
        
        if let Ok(mut monitor) = self.monitor.lock() {
            let should_continue = monitor.update_progress(progress);
            if !should_continue || self.is_cancelled() {
                return Err(YufmathError::internal("计算被用户取消"));
            }
        }
        
        Ok(())
    }
}

impl Default for Yufmath {
    fn default() -> Self {
        Self::new()
    }
}

