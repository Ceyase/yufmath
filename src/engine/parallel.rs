//! # 并行计算系统
//!
//! 实现表达式的并行计算和任务调度。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use rayon::prelude::*;
use crate::core::{Expression, Number};
use crate::engine::{ComputeEngine, ComputeError};
use crate::engine::lazy::{LazyExpression, DependencyGraph};
use crate::api::config::ParallelConfig;

/// 并行计算任务
#[derive(Debug, Clone)]
pub struct ComputeTask {
    /// 任务 ID
    pub id: usize,
    /// 惰性表达式
    pub lazy_expr: Arc<LazyExpression>,
    /// 优先级
    pub priority: i32,
    /// 创建时间
    pub created_at: Instant,
    /// 预估计算时间
    pub estimated_duration: Option<Duration>,
}

impl ComputeTask {
    /// 创建新任务
    pub fn new(id: usize, lazy_expr: Arc<LazyExpression>) -> Self {
        let priority = lazy_expr.priority();
        Self {
            id,
            lazy_expr,
            priority,
            created_at: Instant::now(),
            estimated_duration: None,
        }
    }
    
    /// 设置预估计算时间
    pub fn with_estimated_duration(mut self, duration: Duration) -> Self {
        self.estimated_duration = Some(duration);
        self
    }
    
    /// 获取任务权重（用于调度）
    pub fn weight(&self) -> f64 {
        let base_weight = self.priority as f64;
        let age_bonus = self.created_at.elapsed().as_secs_f64() * 0.1;
        let complexity_penalty = self.lazy_expr.original().complexity() as f64 * 0.01;
        
        (base_weight + age_bonus - complexity_penalty).max(0.0)
    }
}

/// 任务调度器
#[derive(Debug)]
pub struct TaskScheduler {
    /// 待执行任务队列
    pending_tasks: Arc<Mutex<Vec<ComputeTask>>>,
    /// 正在执行的任务
    running_tasks: Arc<RwLock<HashMap<usize, ComputeTask>>>,
    /// 已完成的任务
    completed_tasks: Arc<RwLock<HashMap<usize, Result<Expression, ComputeError>>>>,
    /// 下一个任务 ID
    next_task_id: Arc<Mutex<usize>>,
    /// 配置
    config: ParallelConfig,
}

impl TaskScheduler {
    /// 创建新的任务调度器
    pub fn new(config: ParallelConfig) -> Self {
        Self {
            pending_tasks: Arc::new(Mutex::new(Vec::new())),
            running_tasks: Arc::new(RwLock::new(HashMap::new())),
            completed_tasks: Arc::new(RwLock::new(HashMap::new())),
            next_task_id: Arc::new(Mutex::new(1)),
            config,
        }
    }
    
    /// 添加任务
    pub fn add_task(&self, lazy_expr: Arc<LazyExpression>) -> usize {
        let task_id = {
            let mut next_id = self.next_task_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };
        
        let task = ComputeTask::new(task_id, lazy_expr);
        
        {
            let mut pending = self.pending_tasks.lock().unwrap();
            pending.push(task);
            // 按权重排序（高权重优先）
            pending.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap_or(std::cmp::Ordering::Equal));
        }
        
        task_id
    }
    
    /// 获取下一个可执行的任务
    pub fn get_next_task(&self) -> Option<ComputeTask> {
        let mut pending = self.pending_tasks.lock().unwrap();
        
        // 找到第一个可以执行的任务（依赖都已完成）
        for i in 0..pending.len() {
            if pending[i].lazy_expr.can_compute() {
                let task = pending.remove(i);
                
                // 添加到正在执行的任务列表
                {
                    let mut running = self.running_tasks.write().unwrap();
                    running.insert(task.id, task.clone());
                }
                
                return Some(task);
            }
        }
        
        None
    }
    
    /// 标记任务完成
    pub fn complete_task(&self, task_id: usize, result: Result<Expression, ComputeError>) {
        // 从正在执行的任务中移除
        {
            let mut running = self.running_tasks.write().unwrap();
            running.remove(&task_id);
        }
        
        // 添加到已完成的任务
        {
            let mut completed = self.completed_tasks.write().unwrap();
            completed.insert(task_id, result);
        }
    }
    
    /// 获取任务结果
    pub fn get_task_result(&self, task_id: usize) -> Option<Result<Expression, ComputeError>> {
        let completed = self.completed_tasks.read().unwrap();
        completed.get(&task_id).cloned()
    }
    
    /// 获取待执行任务数量
    pub fn pending_count(&self) -> usize {
        self.pending_tasks.lock().unwrap().len()
    }
    
    /// 获取正在执行任务数量
    pub fn running_count(&self) -> usize {
        self.running_tasks.read().unwrap().len()
    }
    
    /// 获取已完成任务数量
    pub fn completed_count(&self) -> usize {
        self.completed_tasks.read().unwrap().len()
    }
    
    /// 清理已完成的任务
    pub fn cleanup_completed(&self) {
        let mut completed = self.completed_tasks.write().unwrap();
        completed.clear();
    }
}

/// 并行计算引擎
pub struct ParallelComputeEngine {
    /// 基础计算引擎
    base_engine: Arc<dyn ComputeEngine>,
    /// 任务调度器
    scheduler: Arc<TaskScheduler>,
    /// 工作线程池
    thread_pool: Option<rayon::ThreadPool>,
    /// 配置
    config: ParallelConfig,
    /// 是否正在运行
    is_running: Arc<RwLock<bool>>,
}

impl ParallelComputeEngine {
    /// 创建新的并行计算引擎
    pub fn new(base_engine: Arc<dyn ComputeEngine>, config: ParallelConfig) -> Result<Self, ComputeError> {
        let scheduler = Arc::new(TaskScheduler::new(config.clone()));
        
        // 创建线程池
        let thread_pool = if config.enabled {
            let pool_builder = rayon::ThreadPoolBuilder::new();
            let pool_builder = if let Some(thread_count) = config.thread_count {
                pool_builder.num_threads(thread_count)
            } else {
                pool_builder
            };
            
            Some(pool_builder.build().map_err(|e| ComputeError::UnsupportedOperation {
                operation: format!("创建线程池失败: {}", e),
            })?)
        } else {
            None
        };
        
        Ok(Self {
            base_engine,
            scheduler,
            thread_pool,
            config,
            is_running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// 并行计算表达式列表
    pub fn compute_parallel(&self, expressions: Vec<Expression>) -> Vec<Result<Expression, ComputeError>> {
        if !self.config.enabled || expressions.len() < 2 {
            // 如果并行计算被禁用或表达式数量太少，使用串行计算
            return expressions.into_iter()
                .map(|expr| self.base_engine.simplify(&expr))
                .collect();
        }
        
        // 使用 rayon 进行并行计算
        if let Some(ref pool) = self.thread_pool {
            pool.install(|| {
                expressions.into_par_iter()
                    .map(|expr| {
                        // 检查表达式复杂度
                        if expr.complexity() >= self.config.complexity_threshold {
                            self.base_engine.simplify(&expr)
                        } else {
                            // 对于简单表达式，直接串行计算可能更快
                            self.base_engine.simplify(&expr)
                        }
                    })
                    .collect()
            })
        } else {
            // 回退到串行计算
            expressions.into_iter()
                .map(|expr| self.base_engine.simplify(&expr))
                .collect()
        }
    }
    
    /// 使用依赖图进行并行计算
    pub fn compute_with_dependencies(&self, graph: &mut DependencyGraph) -> Result<(), ComputeError> {
        if !self.config.enabled {
            // 串行计算
            let sorted = graph.topological_sort()?;
            for expr_id in sorted {
                if let Some(expr) = graph.get_expression(expr_id) {
                    expr.force_compute(self.base_engine.as_ref())?;
                }
            }
            return Ok(());
        }
        
        // 获取并行组
        let parallel_groups = graph.get_parallel_groups()?;
        
        for group in parallel_groups {
            if group.len() == 1 {
                // 单个表达式，直接计算
                if let Some(expr) = graph.get_expression(group[0]) {
                    expr.force_compute(self.base_engine.as_ref())?;
                }
            } else if group.len() <= self.config.max_parallel_tasks {
                // 并行计算组内的表达式
                self.compute_group_parallel(&graph, &group)?;
            } else {
                // 分批并行计算
                for chunk in group.chunks(self.config.max_parallel_tasks) {
                    self.compute_group_parallel(&graph, chunk)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 并行计算表达式组
    fn compute_group_parallel(&self, graph: &DependencyGraph, group: &[usize]) -> Result<(), ComputeError> {
        let expressions: Vec<_> = group.iter()
            .filter_map(|&id| graph.get_expression(id))
            .collect();
        
        if let Some(ref pool) = self.thread_pool {
            pool.install(|| {
                expressions.into_par_iter()
                    .map(|expr| expr.force_compute(self.base_engine.as_ref()))
                    .collect::<Result<Vec<_>, _>>()
            })?;
        } else {
            // 回退到串行计算
            for expr in expressions {
                expr.force_compute(self.base_engine.as_ref())?;
            }
        }
        
        Ok(())
    }
    
    /// 启动后台计算任务
    pub fn start_background_computation(&self) -> Result<(), ComputeError> {
        let mut is_running = self.is_running.write().unwrap();
        if *is_running {
            return Ok(()); // 已经在运行
        }
        *is_running = true;
        
        // 启动工作线程
        if let Some(ref pool) = self.thread_pool {
            let scheduler = self.scheduler.clone();
            let engine = self.base_engine.clone();
            let is_running_flag = self.is_running.clone();
            
            pool.spawn(move || {
                while *is_running_flag.read().unwrap() {
                    if let Some(task) = scheduler.get_next_task() {
                        let result = task.lazy_expr.force_compute(engine.as_ref());
                        scheduler.complete_task(task.id, result);
                    } else {
                        // 没有可执行的任务，短暂休眠
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            });
        }
        
        Ok(())
    }
    
    /// 停止后台计算任务
    pub fn stop_background_computation(&self) {
        let mut is_running = self.is_running.write().unwrap();
        *is_running = false;
    }
    
    /// 添加计算任务
    pub fn add_task(&self, lazy_expr: Arc<LazyExpression>) -> usize {
        self.scheduler.add_task(lazy_expr)
    }
    
    /// 等待任务完成
    pub fn wait_for_task(&self, task_id: usize, timeout: Option<Duration>) -> Result<Expression, ComputeError> {
        let start_time = Instant::now();
        
        loop {
            if let Some(result) = self.scheduler.get_task_result(task_id) {
                return result;
            }
            
            if let Some(timeout) = timeout {
                if start_time.elapsed() > timeout {
                    return Err(ComputeError::UnsupportedOperation {
                        operation: "任务执行超时".to_string(),
                    });
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// 获取调度器统计信息
    pub fn get_scheduler_stats(&self) -> SchedulerStats {
        SchedulerStats {
            pending_tasks: self.scheduler.pending_count(),
            running_tasks: self.scheduler.running_count(),
            completed_tasks: self.scheduler.completed_count(),
            is_running: *self.is_running.read().unwrap(),
        }
    }
}

/// 调度器统计信息
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    /// 待执行任务数
    pub pending_tasks: usize,
    /// 正在执行任务数
    pub running_tasks: usize,
    /// 已完成任务数
    pub completed_tasks: usize,
    /// 是否正在运行
    pub is_running: bool,
}

/// 表达式预处理器
pub struct ExpressionPreprocessor {
    /// 配置
    config: ParallelConfig,
}

impl ExpressionPreprocessor {
    /// 创建新的预处理器
    pub fn new(config: ParallelConfig) -> Self {
        Self { config }
    }
    
    /// 预处理表达式，进行常量折叠和简单优化
    pub fn preprocess(&self, expr: &Expression) -> Expression {
        self.fold_constants(expr)
    }
    
    /// 常量折叠
    fn fold_constants(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                let left_folded = self.fold_constants(left);
                let right_folded = self.fold_constants(right);
                
                // 尝试计算常量表达式
                if let (Expression::Number(a), Expression::Number(b)) = (&left_folded, &right_folded) {
                    if let Ok(result) = self.compute_constant_operation(op, a, b) {
                        return Expression::Number(result);
                    }
                }
                
                // 应用代数简化规则
                self.apply_algebraic_rules(op, &left_folded, &right_folded)
            }
            Expression::UnaryOp { op, operand } => {
                let operand_folded = self.fold_constants(operand);
                
                // 尝试计算常量表达式
                if let Expression::Number(n) = &operand_folded {
                    if let Ok(result) = self.compute_constant_unary_operation(op, n) {
                        return Expression::Number(result);
                    }
                }
                
                Expression::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(operand_folded),
                }
            }
            Expression::Function { name, args } => {
                let args_folded: Vec<_> = args.iter().map(|arg| self.fold_constants(arg)).collect();
                Expression::Function {
                    name: name.clone(),
                    args: args_folded,
                }
            }
            _ => expr.clone(),
        }
    }
    
    /// 计算常量二元运算
    fn compute_constant_operation(&self, op: &crate::core::BinaryOperator, a: &Number, b: &Number) -> Result<Number, ComputeError> {
        use crate::core::BinaryOperator;
        
        match op {
            BinaryOperator::Add => a.add(b),
            BinaryOperator::Subtract => a.subtract(b),
            BinaryOperator::Multiply => a.multiply(b),
            BinaryOperator::Divide => a.divide(b),
            BinaryOperator::Power => a.power(b),
            _ => Err(ComputeError::UnsupportedOperation {
                operation: format!("常量运算不支持操作符: {:?}", op),
            }),
        }
    }
    
    /// 计算常量一元运算
    fn compute_constant_unary_operation(&self, op: &crate::core::UnaryOperator, n: &Number) -> Result<Number, ComputeError> {
        use crate::core::UnaryOperator;
        
        match op {
            UnaryOperator::Negate => n.negate(),
            UnaryOperator::Plus => Ok(n.clone()),
            UnaryOperator::Abs => n.abs(),
            _ => Err(ComputeError::UnsupportedOperation {
                operation: format!("常量运算不支持操作符: {:?}", op),
            }),
        }
    }
    
    /// 应用代数简化规则
    fn apply_algebraic_rules(&self, op: &crate::core::BinaryOperator, left: &Expression, right: &Expression) -> Expression {
        use crate::core::BinaryOperator;
        
        match op {
            BinaryOperator::Add => {
                // 0 + x = x
                if let Expression::Number(n) = left {
                    if n.is_zero() {
                        return right.clone();
                    }
                }
                // x + 0 = x
                if let Expression::Number(n) = right {
                    if n.is_zero() {
                        return left.clone();
                    }
                }
            }
            BinaryOperator::Multiply => {
                // 0 * x = 0
                if let Expression::Number(n) = left {
                    if n.is_zero() {
                        return Expression::Number(Number::from(0));
                    }
                    if n.is_one() {
                        return right.clone();
                    }
                }
                // x * 0 = 0, x * 1 = x
                if let Expression::Number(n) = right {
                    if n.is_zero() {
                        return Expression::Number(Number::from(0));
                    }
                    if n.is_one() {
                        return left.clone();
                    }
                }
            }
            BinaryOperator::Power => {
                // x^0 = 1
                if let Expression::Number(n) = right {
                    if n.is_zero() {
                        return Expression::Number(Number::from(1));
                    }
                    if n.is_one() {
                        return left.clone();
                    }
                }
                // 0^x = 0 (x != 0)
                if let Expression::Number(n) = left {
                    if n.is_zero() {
                        return Expression::Number(Number::from(0));
                    }
                    if n.is_one() {
                        return Expression::Number(Number::from(1));
                    }
                }
            }
            _ => {}
        }
        
        // 默认情况：返回原始表达式
        Expression::BinaryOp {
            op: op.clone(),
            left: Box::new(left.clone()),
            right: Box::new(right.clone()),
        }
    }
    
    /// 分析表达式的并行化潜力
    pub fn analyze_parallelization_potential(&self, expr: &Expression) -> ParallelizationAnalysis {
        let complexity = expr.complexity();
        let subexpr_count = self.count_subexpressions(expr);
        let independent_parts = self.find_independent_parts(expr);
        
        ParallelizationAnalysis {
            complexity,
            subexpression_count: subexpr_count,
            independent_parts_count: independent_parts.len(),
            recommended_parallel: complexity >= self.config.complexity_threshold && independent_parts.len() > 1,
            estimated_speedup: if independent_parts.len() > 1 {
                (independent_parts.len() as f64).min(self.config.max_parallel_tasks as f64)
            } else {
                1.0
            },
        }
    }
    
    /// 计算子表达式数量
    fn count_subexpressions(&self, expr: &Expression) -> usize {
        match expr {
            Expression::BinaryOp { left, right, .. } => {
                1 + self.count_subexpressions(left) + self.count_subexpressions(right)
            }
            Expression::UnaryOp { operand, .. } => {
                1 + self.count_subexpressions(operand)
            }
            Expression::Function { args, .. } => {
                1 + args.iter().map(|arg| self.count_subexpressions(arg)).sum::<usize>()
            }
            _ => 1,
        }
    }
    
    /// 找到独立的子表达式部分
    fn find_independent_parts(&self, expr: &Expression) -> Vec<Expression> {
        match expr {
            Expression::BinaryOp { op, left, right } => {
                use crate::core::BinaryOperator;
                match op {
                    BinaryOperator::Add | BinaryOperator::Multiply => {
                        // 加法和乘法的操作数可以独立计算
                        let mut parts = vec![*left.clone(), *right.clone()];
                        parts.extend(self.find_independent_parts(left));
                        parts.extend(self.find_independent_parts(right));
                        parts
                    }
                    _ => {
                        // 其他运算符的操作数可能有依赖关系
                        let mut parts = self.find_independent_parts(left);
                        parts.extend(self.find_independent_parts(right));
                        parts
                    }
                }
            }
            Expression::Function { args, .. } => {
                // 函数参数通常可以独立计算
                let mut parts = args.clone();
                for arg in args {
                    parts.extend(self.find_independent_parts(arg));
                }
                parts
            }
            _ => vec![],
        }
    }
}

/// 并行化分析结果
#[derive(Debug, Clone)]
pub struct ParallelizationAnalysis {
    /// 表达式复杂度
    pub complexity: usize,
    /// 子表达式数量
    pub subexpression_count: usize,
    /// 独立部分数量
    pub independent_parts_count: usize,
    /// 是否推荐并行计算
    pub recommended_parallel: bool,
    /// 预估加速比
    pub estimated_speedup: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use crate::engine::compute::BasicComputeEngine;
    use crate::api::config::ParallelConfig;
    
    #[test]
    fn test_task_scheduler() {
        let config = ParallelConfig::default();
        let scheduler = TaskScheduler::new(config);
        
        // 创建测试表达式
        let expr = Expression::add(
            Expression::number(2.into()),
            Expression::number(3.into())
        );
        let lazy_expr = Arc::new(LazyExpression::new(1, expr));
        
        // 添加任务
        let task_id = scheduler.add_task(lazy_expr.clone());
        assert_eq!(scheduler.pending_count(), 1);
        
        // 获取任务
        let task = scheduler.get_next_task().unwrap();
        assert_eq!(task.id, task_id);
        assert_eq!(scheduler.pending_count(), 0);
        assert_eq!(scheduler.running_count(), 1);
        
        // 完成任务
        let result = Expression::number(5.into());
        scheduler.complete_task(task_id, Ok(result.clone()));
        assert_eq!(scheduler.running_count(), 0);
        assert_eq!(scheduler.completed_count(), 1);
        
        // 获取结果
        let task_result = scheduler.get_task_result(task_id).unwrap();
        assert!(task_result.is_ok());
    }
    
    #[test]
    fn test_parallel_compute_engine() {
        let base_engine = Arc::new(BasicComputeEngine::new());
        let config = ParallelConfig::default();
        let parallel_engine = ParallelComputeEngine::new(base_engine, config).unwrap();
        
        // 测试并行计算
        let expressions = vec![
            Expression::add(Expression::number(1.into()), Expression::number(2.into())),
            Expression::multiply(Expression::number(3.into()), Expression::number(4.into())),
            Expression::subtract(Expression::number(10.into()), Expression::number(5.into())),
        ];
        
        let results = parallel_engine.compute_parallel(expressions);
        assert_eq!(results.len(), 3);
        
        for result in results {
            assert!(result.is_ok());
        }
    }
    
    #[test]
    fn test_expression_preprocessor() {
        let config = ParallelConfig::default();
        let preprocessor = ExpressionPreprocessor::new(config);
        
        // 测试常量折叠
        let expr = Expression::add(
            Expression::number(2.into()),
            Expression::number(3.into())
        );
        
        let folded = preprocessor.preprocess(&expr);
        
        // 应该被折叠为常量 5
        if let Expression::Number(n) = folded {
            assert_eq!(n, Number::from(5));
        } else {
            panic!("常量折叠失败");
        }
    }
    
    #[test]
    fn test_parallelization_analysis() {
        let config = ParallelConfig::default();
        let preprocessor = ExpressionPreprocessor::new(config);
        
        // 创建复杂表达式
        let expr = Expression::add(
            Expression::multiply(
                Expression::variable("x"),
                Expression::variable("y")
            ),
            Expression::multiply(
                Expression::variable("z"),
                Expression::variable("w")
            )
        );
        
        let analysis = preprocessor.analyze_parallelization_potential(&expr);
        
        assert!(analysis.complexity > 0);
        assert!(analysis.subexpression_count > 1);
        assert!(analysis.independent_parts_count > 0);
    }
}