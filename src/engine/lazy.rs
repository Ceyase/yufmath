//! # 惰性求值系统
//!
//! 实现延迟计算和依赖跟踪，提高计算效率。

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use crate::core::Expression;
use crate::engine::{ComputeEngine, ComputeError};

/// 惰性表达式状态
#[derive(Debug, Clone, PartialEq)]
pub enum LazyState {
    /// 未计算状态
    Pending,
    /// 正在计算状态
    Computing,
    /// 已计算完成
    Computed(Expression),
    /// 计算失败
    Failed(ComputeError),
}

/// 惰性表达式
#[derive(Clone)]
pub struct LazyExpression {
    /// 表达式 ID
    id: usize,
    /// 原始表达式
    original: Expression,
    /// 计算状态
    state: Arc<RwLock<LazyState>>,
    /// 依赖的其他惰性表达式
    dependencies: Vec<Arc<LazyExpression>>,
    /// 计算函数
    compute_fn: Option<Arc<dyn Fn(&Expression, &dyn ComputeEngine) -> Result<Expression, ComputeError> + Send + Sync>>,
    /// 优先级（用于调度）
    priority: i32,
}

impl std::fmt::Debug for LazyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyExpression")
            .field("id", &self.id)
            .field("original", &self.original)
            .field("state", &self.state)
            .field("dependencies", &self.dependencies)
            .field("compute_fn", &self.compute_fn.is_some())
            .field("priority", &self.priority)
            .finish()
    }
}

impl LazyExpression {
    /// 创建新的惰性表达式
    pub fn new(id: usize, expr: Expression) -> Self {
        Self {
            id,
            original: expr,
            state: Arc::new(RwLock::new(LazyState::Pending)),
            dependencies: Vec::new(),
            compute_fn: None,
            priority: 0,
        }
    }
    
    /// 创建带计算函数的惰性表达式
    pub fn with_compute_fn<F>(id: usize, expr: Expression, compute_fn: F) -> Self 
    where
        F: Fn(&Expression, &dyn ComputeEngine) -> Result<Expression, ComputeError> + Send + Sync + 'static,
    {
        Self {
            id,
            original: expr,
            state: Arc::new(RwLock::new(LazyState::Pending)),
            dependencies: Vec::new(),
            compute_fn: Some(Arc::new(compute_fn)),
            priority: 0,
        }
    }
    
    /// 添加依赖
    pub fn add_dependency(&mut self, dep: Arc<LazyExpression>) {
        self.dependencies.push(dep);
    }
    
    /// 设置优先级
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
    /// 获取表达式 ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// 获取原始表达式
    pub fn original(&self) -> &Expression {
        &self.original
    }
    
    /// 获取当前状态
    pub fn state(&self) -> LazyState {
        self.state.read().unwrap().clone()
    }
    
    /// 获取依赖列表
    pub fn dependencies(&self) -> &[Arc<LazyExpression>] {
        &self.dependencies
    }
    
    /// 获取优先级
    pub fn priority(&self) -> i32 {
        self.priority
    }
    
    /// 检查是否已计算完成
    pub fn is_computed(&self) -> bool {
        matches!(self.state(), LazyState::Computed(_))
    }
    
    /// 检查是否计算失败
    pub fn is_failed(&self) -> bool {
        matches!(self.state(), LazyState::Failed(_))
    }
    
    /// 检查是否正在计算
    pub fn is_computing(&self) -> bool {
        matches!(self.state(), LazyState::Computing)
    }
    
    /// 检查是否可以开始计算（所有依赖都已完成）
    pub fn can_compute(&self) -> bool {
        if !matches!(self.state(), LazyState::Pending) {
            return false;
        }
        
        self.dependencies.iter().all(|dep| dep.is_computed())
    }
    
    /// 获取计算结果（如果已计算完成）
    pub fn get_result(&self) -> Option<Expression> {
        match self.state() {
            LazyState::Computed(expr) => Some(expr),
            _ => None,
        }
    }
    
    /// 获取计算错误（如果计算失败）
    pub fn get_error(&self) -> Option<ComputeError> {
        match self.state() {
            LazyState::Failed(err) => Some(err),
            _ => None,
        }
    }
    
    /// 强制计算表达式
    pub fn force_compute(&self, engine: &dyn ComputeEngine) -> Result<Expression, ComputeError> {
        // 检查当前状态
        match self.state() {
            LazyState::Computed(expr) => return Ok(expr),
            LazyState::Failed(err) => return Err(err),
            LazyState::Computing => {
                return Err(ComputeError::UnsupportedOperation { 
                    operation: "循环依赖或重复计算".to_string() 
                });
            }
            LazyState::Pending => {}
        }
        
        // 设置为计算中状态
        {
            let mut state = self.state.write().unwrap();
            *state = LazyState::Computing;
        }
        
        // 首先计算所有依赖
        for dep in &self.dependencies {
            if let Err(err) = dep.force_compute(engine) {
                let mut state = self.state.write().unwrap();
                *state = LazyState::Failed(err.clone());
                return Err(err);
            }
        }
        
        // 执行计算
        let result = if let Some(ref compute_fn) = self.compute_fn {
            compute_fn(&self.original, engine)
        } else {
            // 默认计算：简化表达式
            engine.simplify(&self.original)
        };
        
        // 更新状态
        match result {
            Ok(expr) => {
                let mut state = self.state.write().unwrap();
                *state = LazyState::Computed(expr.clone());
                Ok(expr)
            }
            Err(err) => {
                let mut state = self.state.write().unwrap();
                *state = LazyState::Failed(err.clone());
                Err(err)
            }
        }
    }
    
    /// 重置计算状态
    pub fn reset(&self) {
        let mut state = self.state.write().unwrap();
        *state = LazyState::Pending;
    }
}

/// 依赖图
#[derive(Debug)]
pub struct DependencyGraph {
    /// 所有惰性表达式
    expressions: HashMap<usize, Arc<LazyExpression>>,
    /// 依赖关系图（表达式ID -> 依赖的表达式ID列表）
    dependencies: HashMap<usize, HashSet<usize>>,
    /// 反向依赖关系图（表达式ID -> 依赖它的表达式ID列表）
    dependents: HashMap<usize, HashSet<usize>>,
    /// 下一个可用的表达式ID
    next_id: usize,
}

impl DependencyGraph {
    /// 创建新的依赖图
    pub fn new() -> Self {
        Self {
            expressions: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// 添加惰性表达式
    pub fn add_expression(&mut self, expr: Expression) -> Arc<LazyExpression> {
        let id = self.next_id;
        self.next_id += 1;
        
        let lazy_expr = Arc::new(LazyExpression::new(id, expr));
        self.expressions.insert(id, lazy_expr.clone());
        self.dependencies.insert(id, HashSet::new());
        self.dependents.insert(id, HashSet::new());
        
        lazy_expr
    }
    
    /// 添加带计算函数的惰性表达式
    pub fn add_expression_with_fn<F>(&mut self, expr: Expression, compute_fn: F) -> Arc<LazyExpression>
    where
        F: Fn(&Expression, &dyn ComputeEngine) -> Result<Expression, ComputeError> + Send + Sync + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        
        let lazy_expr = Arc::new(LazyExpression::with_compute_fn(id, expr, compute_fn));
        self.expressions.insert(id, lazy_expr.clone());
        self.dependencies.insert(id, HashSet::new());
        self.dependents.insert(id, HashSet::new());
        
        lazy_expr
    }
    
    /// 添加依赖关系
    pub fn add_dependency(&mut self, expr_id: usize, dep_id: usize) -> Result<(), ComputeError> {
        // 检查是否会形成循环依赖
        if self.would_create_cycle(expr_id, dep_id) {
            return Err(ComputeError::UnsupportedOperation {
                operation: "添加依赖会形成循环".to_string(),
            });
        }
        
        // 添加依赖关系
        self.dependencies.entry(expr_id).or_default().insert(dep_id);
        self.dependents.entry(dep_id).or_default().insert(expr_id);
        
        // 更新惰性表达式的依赖列表
        if let Some(_expr) = self.expressions.get(&expr_id) {
            if let Some(_dep_expr) = self.expressions.get(&dep_id) {
                // 注意：这里需要修改 LazyExpression 的实现以支持动态添加依赖
                // 暂时跳过这个更新，在实际使用时需要重新设计
            }
        }
        
        Ok(())
    }
    
    /// 检查添加依赖是否会形成循环
    fn would_create_cycle(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        
        let mut visited = HashSet::new();
        let mut stack = vec![to];
        
        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            
            if current == from {
                return true;
            }
            
            if let Some(deps) = self.dependencies.get(&current) {
                for &dep in deps {
                    if !visited.contains(&dep) {
                        stack.push(dep);
                    }
                }
            }
        }
        
        false
    }
    
    /// 获取拓扑排序的计算顺序
    pub fn topological_sort(&self) -> Result<Vec<usize>, ComputeError> {
        let mut in_degree = HashMap::new();
        let mut result = Vec::new();
        let mut queue = Vec::new();
        
        // 计算每个节点的入度
        for &id in self.expressions.keys() {
            in_degree.insert(id, 0);
        }
        
        // 对于每个表达式的依赖，增加该表达式的入度
        for (&expr_id, deps) in &self.dependencies {
            let current_degree = in_degree.get(&expr_id).unwrap_or(&0);
            in_degree.insert(expr_id, current_degree + deps.len());
        }
        
        // 找到所有入度为0的节点
        for (&id, &degree) in &in_degree {
            if degree == 0 {
                queue.push(id);
            }
        }
        
        // 拓扑排序
        while let Some(current) = queue.pop() {
            result.push(current);
            
            // 对于依赖当前节点的所有节点，减少它们的入度
            if let Some(dependents) = self.dependents.get(&current) {
                for &dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(&dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dependent);
                        }
                    }
                }
            }
        }
        
        // 检查是否存在循环依赖
        if result.len() != self.expressions.len() {
            return Err(ComputeError::UnsupportedOperation {
                operation: "检测到循环依赖".to_string(),
            });
        }
        
        Ok(result)
    }
    
    /// 获取可以并行计算的表达式组
    pub fn get_parallel_groups(&self) -> Result<Vec<Vec<usize>>, ComputeError> {
        let sorted = self.topological_sort()?;
        let mut groups = Vec::new();
        let mut computed = HashSet::new();
        
        let mut remaining: HashSet<usize> = sorted.iter().cloned().collect();
        
        while !remaining.is_empty() {
            let mut current_group = Vec::new();
            
            // 收集当前可以并行计算的表达式
            let ready_to_compute: Vec<usize> = remaining.iter()
                .filter(|&&expr_id| {
                    // 检查所有依赖是否都已计算完成
                    if let Some(deps) = self.dependencies.get(&expr_id) {
                        deps.iter().all(|dep| computed.contains(dep))
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
            
            for expr_id in ready_to_compute {
                current_group.push(expr_id);
                computed.insert(expr_id);
                remaining.remove(&expr_id);
            }
            
            if !current_group.is_empty() {
                groups.push(current_group);
            } else {
                // 如果没有可以计算的表达式，但还有剩余的，说明有循环依赖
                return Err(ComputeError::UnsupportedOperation {
                    operation: "检测到循环依赖或无法解析的依赖关系".to_string(),
                });
            }
        }
        
        Ok(groups)
    }
    
    /// 获取表达式
    pub fn get_expression(&self, id: usize) -> Option<Arc<LazyExpression>> {
        self.expressions.get(&id).cloned()
    }
    
    /// 获取所有表达式
    pub fn get_all_expressions(&self) -> Vec<Arc<LazyExpression>> {
        self.expressions.values().cloned().collect()
    }
    
    /// 清理已完成的表达式
    pub fn cleanup_completed(&mut self) {
        let completed_ids: Vec<usize> = self.expressions
            .iter()
            .filter(|(_, expr)| expr.is_computed() || expr.is_failed())
            .map(|(&id, _)| id)
            .collect();
        
        for id in completed_ids {
            self.expressions.remove(&id);
            self.dependencies.remove(&id);
            self.dependents.remove(&id);
            
            // 从其他表达式的依赖中移除
            for deps in self.dependencies.values_mut() {
                deps.remove(&id);
            }
            for dependents in self.dependents.values_mut() {
                dependents.remove(&id);
            }
        }
    }
    
    /// 重置所有表达式状态
    pub fn reset_all(&self) {
        for expr in self.expressions.values() {
            expr.reset();
        }
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> DependencyGraphStats {
        let total = self.expressions.len();
        let mut pending = 0;
        let mut computing = 0;
        let mut computed = 0;
        let mut failed = 0;
        
        for expr in self.expressions.values() {
            match expr.state() {
                LazyState::Pending => pending += 1,
                LazyState::Computing => computing += 1,
                LazyState::Computed(_) => computed += 1,
                LazyState::Failed(_) => failed += 1,
            }
        }
        
        DependencyGraphStats {
            total_expressions: total,
            pending_expressions: pending,
            computing_expressions: computing,
            computed_expressions: computed,
            failed_expressions: failed,
            total_dependencies: self.dependencies.values().map(|deps| deps.len()).sum(),
        }
    }
}

/// 依赖图统计信息
#[derive(Debug, Clone)]
pub struct DependencyGraphStats {
    /// 总表达式数
    pub total_expressions: usize,
    /// 待计算表达式数
    pub pending_expressions: usize,
    /// 正在计算表达式数
    pub computing_expressions: usize,
    /// 已计算表达式数
    pub computed_expressions: usize,
    /// 计算失败表达式数
    pub failed_expressions: usize,
    /// 总依赖关系数
    pub total_dependencies: usize,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use crate::engine::compute::BasicComputeEngine;
    
    #[test]
    fn test_lazy_expression_creation() {
        let expr = Expression::variable("x");
        let lazy_expr = LazyExpression::new(1, expr.clone());
        
        assert_eq!(lazy_expr.id(), 1);
        assert_eq!(lazy_expr.original(), &expr);
        assert!(matches!(lazy_expr.state(), LazyState::Pending));
        assert!(!lazy_expr.is_computed());
        assert!(!lazy_expr.is_failed());
        assert!(!lazy_expr.is_computing());
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        // 添加表达式
        let expr1 = graph.add_expression(Expression::variable("x"));
        let expr2 = graph.add_expression(Expression::variable("y"));
        let expr3 = graph.add_expression(Expression::add(
            Expression::variable("x"),
            Expression::variable("y")
        ));
        
        // 添加依赖关系：expr3 依赖 expr1 和 expr2
        graph.add_dependency(expr3.id(), expr1.id()).unwrap();
        graph.add_dependency(expr3.id(), expr2.id()).unwrap();
        
        // 测试拓扑排序
        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted.len(), 3);
        
        // expr1 和 expr2 应该在 expr3 之前
        let pos1 = sorted.iter().position(|&id| id == expr1.id()).unwrap();
        let pos2 = sorted.iter().position(|&id| id == expr2.id()).unwrap();
        let pos3 = sorted.iter().position(|&id| id == expr3.id()).unwrap();
        
        assert!(pos1 < pos3);
        assert!(pos2 < pos3);
    }
    
    #[test]
    fn test_parallel_groups() {
        let mut graph = DependencyGraph::new();
        
        // 创建表达式：x, y, x+y, z, (x+y)*z
        let expr_x = graph.add_expression(Expression::variable("x"));
        let expr_y = graph.add_expression(Expression::variable("y"));
        let expr_z = graph.add_expression(Expression::variable("z"));
        let expr_sum = graph.add_expression(Expression::add(
            Expression::variable("x"),
            Expression::variable("y")
        ));
        let expr_product = graph.add_expression(Expression::multiply(
            Expression::add(Expression::variable("x"), Expression::variable("y")),
            Expression::variable("z")
        ));
        
        // 添加依赖关系
        graph.add_dependency(expr_sum.id(), expr_x.id()).unwrap();
        graph.add_dependency(expr_sum.id(), expr_y.id()).unwrap();
        graph.add_dependency(expr_product.id(), expr_sum.id()).unwrap();
        graph.add_dependency(expr_product.id(), expr_z.id()).unwrap();
        
        // 获取并行组
        let groups = graph.get_parallel_groups().unwrap();
        
        // 第一组应该包含 x, y, z（可以并行计算）
        // 第二组应该包含 x+y
        // 第三组应该包含 (x+y)*z
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].len(), 3); // x, y, z
        assert_eq!(groups[1].len(), 1); // x+y
        assert_eq!(groups[2].len(), 1); // (x+y)*z
    }
    
    #[test]
    fn test_cycle_detection() {
        let mut graph = DependencyGraph::new();
        
        let expr1 = graph.add_expression(Expression::variable("x"));
        let expr2 = graph.add_expression(Expression::variable("y"));
        
        // 添加正常依赖
        graph.add_dependency(expr2.id(), expr1.id()).unwrap();
        
        // 尝试添加循环依赖
        let result = graph.add_dependency(expr1.id(), expr2.id());
        assert!(result.is_err());
    }
    
    #[test]
    fn test_force_compute() {
        let engine = BasicComputeEngine::new();
        
        // 创建简单表达式
        let expr = Expression::add(
            Expression::number(2.into()),
            Expression::number(3.into())
        );
        
        let lazy_expr = LazyExpression::new(1, expr);
        
        // 强制计算
        let result = lazy_expr.force_compute(&engine).unwrap();
        
        // 验证结果
        assert!(lazy_expr.is_computed());
        assert_eq!(lazy_expr.get_result(), Some(result));
    }
}