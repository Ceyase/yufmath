//! # 单元格执行引擎
//!
//! 提供单元格的执行、调度和依赖管理功能。
//! 
//! ## 主要功能
//! 
//! - **执行队列和调度器**：管理单元格的执行顺序和依赖关系
//! - **增量执行**：只执行修改过的单元格及其依赖
//! - **异步执行**：支持后台执行和进度显示
//! - **结果缓存**：缓存执行结果以提高性能
//! - **错误处理**：完善的错误处理和恢复机制

use super::{NotebookCell, CellId, ScopeManager, NotebookError, NotebookResult};
use crate::api::{Yufmath, ComputeProgress, ProgressCallback};
use crate::core::Expression;
use crate::engine::ComputeError;
use crate::formatter::{FormatType};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionResult {
    /// 成功执行
    Success {
        value: String,
        format: FormatType,
        execution_time: Duration,
    },
    /// 执行错误
    Error {
        error: String,
        error_type: String,
        execution_time: Duration,
    },
    /// 跳过执行（非代码单元格）
    Skipped,
    /// 取消执行
    Cancelled,
}

impl ExecutionResult {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, ExecutionResult::Success { .. })
    }
    
    /// 检查是否出错
    pub fn is_error(&self) -> bool {
        matches!(self, ExecutionResult::Error { .. })
    }
    
    /// 获取执行时间
    pub fn execution_time(&self) -> Option<Duration> {
        match self {
            ExecutionResult::Success { execution_time, .. } => Some(*execution_time),
            ExecutionResult::Error { execution_time, .. } => Some(*execution_time),
            _ => None,
        }
    }
    
    /// 获取结果值
    pub fn value(&self) -> Option<&str> {
        match self {
            ExecutionResult::Success { value, .. } => Some(value),
            ExecutionResult::Error { error, .. } => Some(error),
            _ => None,
        }
    }
}

/// 执行上下文
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 当前单元格 ID
    pub cell_id: CellId,
    /// 执行开始时间
    pub start_time: Instant,
    /// 是否允许取消
    pub allow_cancellation: bool,
    /// 输出格式
    pub output_format: FormatType,
    /// 是否显示执行时间
    pub show_timing: bool,
    /// 最大执行时间
    pub max_execution_time: Option<Duration>,
}

impl ExecutionContext {
    /// 创建新的执行上下文
    pub fn new(cell_id: CellId) -> Self {
        Self {
            cell_id,
            start_time: Instant::now(),
            allow_cancellation: true,
            output_format: FormatType::Standard,
            show_timing: true,
            max_execution_time: Some(Duration::from_secs(60)), // 默认60秒超时
        }
    }
    
    /// 获取已执行时间
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// 检查是否超时
    pub fn is_timeout(&self) -> bool {
        if let Some(max_time) = self.max_execution_time {
            self.elapsed_time() > max_time
        } else {
            false
        }
    }
}

/// 执行队列项
#[derive(Debug, Clone)]
pub struct ExecutionQueueItem {
    /// 单元格 ID
    pub cell_id: CellId,
    /// 优先级（数字越小优先级越高）
    pub priority: u32,
    /// 依赖的单元格 ID 列表
    pub dependencies: Vec<CellId>,
    /// 添加到队列的时间
    pub queued_at: SystemTime,
    /// 是否为增量执行（只有修改过的单元格才需要执行）
    pub is_incremental: bool,
    /// 预估执行时间
    pub estimated_duration: Option<Duration>,
}

/// 执行任务状态
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    /// 等待执行
    Pending,
    /// 正在执行
    Running,
    /// 执行完成
    Completed,
    /// 执行失败
    Failed,
    /// 被取消
    Cancelled,
    /// 跳过执行
    Skipped,
}

/// 执行任务
#[derive(Debug, Clone)]
pub struct ExecutionTask {
    /// 任务 ID
    pub id: Uuid,
    /// 单元格 ID
    pub cell_id: CellId,
    /// 任务状态
    pub status: TaskStatus,
    /// 开始时间
    pub started_at: Option<Instant>,
    /// 完成时间
    pub completed_at: Option<Instant>,
    /// 执行结果
    pub result: Option<ExecutionResult>,
    /// 错误信息
    pub error: Option<String>,
    /// 重试次数
    pub retry_count: u32,
    /// 最大重试次数
    pub max_retries: u32,
}

impl ExecutionTask {
    /// 创建新的执行任务
    pub fn new(cell_id: CellId) -> Self {
        Self {
            id: Uuid::new_v4(),
            cell_id,
            status: TaskStatus::Pending,
            started_at: None,
            completed_at: None,
            result: None,
            error: None,
            retry_count: 0,
            max_retries: 3,
        }
    }
    
    /// 开始执行任务
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(Instant::now());
    }
    
    /// 完成任务
    pub fn complete(&mut self, result: ExecutionResult) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Instant::now());
        self.result = Some(result);
    }
    
    /// 任务失败
    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Instant::now());
        self.error = Some(error);
    }
    
    /// 取消任务
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(Instant::now());
    }
    
    /// 跳过任务
    pub fn skip(&mut self) {
        self.status = TaskStatus::Skipped;
        self.completed_at = Some(Instant::now());
    }
    
    /// 检查是否可以重试
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries && matches!(self.status, TaskStatus::Failed)
    }
    
    /// 重试任务
    pub fn retry(&mut self) {
        if self.can_retry() {
            self.retry_count += 1;
            self.status = TaskStatus::Pending;
            self.started_at = None;
            self.completed_at = None;
            self.result = None;
            self.error = None;
        }
    }
    
    /// 获取执行时间
    pub fn execution_time(&self) -> Option<Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }
}

/// 依赖图节点
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// 单元格 ID
    pub cell_id: CellId,
    /// 依赖的单元格
    pub dependencies: HashSet<CellId>,
    /// 被依赖的单元格
    pub dependents: HashSet<CellId>,
    /// 最后修改时间
    pub last_modified: SystemTime,
    /// 是否需要重新执行
    pub needs_execution: bool,
}

impl DependencyNode {
    /// 创建新的依赖节点
    pub fn new(cell_id: CellId) -> Self {
        Self {
            cell_id,
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
            last_modified: SystemTime::now(),
            needs_execution: true,
        }
    }
    
    /// 添加依赖
    pub fn add_dependency(&mut self, dep_id: CellId) {
        self.dependencies.insert(dep_id);
    }
    
    /// 移除依赖
    pub fn remove_dependency(&mut self, dep_id: &CellId) {
        self.dependencies.remove(dep_id);
    }
    
    /// 添加被依赖者
    pub fn add_dependent(&mut self, dep_id: CellId) {
        self.dependents.insert(dep_id);
    }
    
    /// 移除被依赖者
    pub fn remove_dependent(&mut self, dep_id: &CellId) {
        self.dependents.remove(dep_id);
    }
    
    /// 标记为已修改
    pub fn mark_modified(&mut self) {
        self.last_modified = SystemTime::now();
        self.needs_execution = true;
    }
    
    /// 标记为已执行
    pub fn mark_executed(&mut self) {
        self.needs_execution = false;
    }
}

/// 依赖图
pub struct DependencyGraph {
    /// 节点映射
    nodes: HashMap<CellId, DependencyNode>,
    /// 拓扑排序缓存
    topo_cache: Option<Vec<CellId>>,
    /// 缓存是否有效
    cache_valid: bool,
}

impl DependencyGraph {
    /// 创建新的依赖图
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            topo_cache: None,
            cache_valid: false,
        }
    }
    
    /// 添加节点
    pub fn add_node(&mut self, cell_id: CellId) {
        if !self.nodes.contains_key(&cell_id) {
            self.nodes.insert(cell_id, DependencyNode::new(cell_id));
            self.invalidate_cache();
        }
    }
    
    /// 移除节点
    pub fn remove_node(&mut self, cell_id: &CellId) {
        if let Some(node) = self.nodes.remove(cell_id) {
            // 移除所有相关的依赖关系
            for dep_id in &node.dependencies {
                if let Some(dep_node) = self.nodes.get_mut(dep_id) {
                    dep_node.remove_dependent(cell_id);
                }
            }
            
            for dep_id in &node.dependents {
                if let Some(dep_node) = self.nodes.get_mut(dep_id) {
                    dep_node.remove_dependency(cell_id);
                }
            }
            
            self.invalidate_cache();
        }
    }
    
    /// 添加依赖关系
    pub fn add_dependency(&mut self, cell_id: CellId, dep_id: CellId) {
        // 确保两个节点都存在
        self.add_node(cell_id);
        self.add_node(dep_id);
        
        // 检查是否会形成循环依赖
        if self.would_create_cycle(cell_id, dep_id) {
            return; // 忽略会形成循环的依赖
        }
        
        // 添加依赖关系
        if let Some(node) = self.nodes.get_mut(&cell_id) {
            node.add_dependency(dep_id);
        }
        
        if let Some(dep_node) = self.nodes.get_mut(&dep_id) {
            dep_node.add_dependent(cell_id);
        }
        
        self.invalidate_cache();
    }
    
    /// 移除依赖关系
    pub fn remove_dependency(&mut self, cell_id: &CellId, dep_id: &CellId) {
        if let Some(node) = self.nodes.get_mut(cell_id) {
            node.remove_dependency(dep_id);
        }
        
        if let Some(dep_node) = self.nodes.get_mut(dep_id) {
            dep_node.remove_dependent(cell_id);
        }
        
        self.invalidate_cache();
    }
    
    /// 检查是否会形成循环依赖
    fn would_create_cycle(&self, from: CellId, to: CellId) -> bool {
        if from == to {
            return true;
        }
        
        let mut visited = HashSet::new();
        let mut stack = vec![to];
        
        while let Some(current) = stack.pop() {
            if current == from {
                return true;
            }
            
            if visited.contains(&current) {
                continue;
            }
            
            visited.insert(current);
            
            if let Some(node) = self.nodes.get(&current) {
                for dep in &node.dependencies {
                    stack.push(*dep);
                }
            }
        }
        
        false
    }
    
    /// 获取拓扑排序
    pub fn topological_sort(&mut self) -> Vec<CellId> {
        if self.cache_valid && self.topo_cache.is_some() {
            return self.topo_cache.as_ref().unwrap().clone();
        }
        
        // 使用 Kahn 算法进行拓扑排序
        let mut result = Vec::new();
        let mut in_degree: HashMap<CellId, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        
        // 计算每个节点的入度
        for &cell_id in self.nodes.keys() {
            in_degree.insert(cell_id, 0);
        }
        
        for node in self.nodes.values() {
            for &dep_id in &node.dependencies {
                *in_degree.entry(node.cell_id).or_insert(0) += 1;
            }
        }
        
        // 将入度为0的节点加入队列
        for (&cell_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(cell_id);
            }
        }
        
        // 处理队列
        while let Some(cell_id) = queue.pop_front() {
            result.push(cell_id);
            
            // 对于当前节点的所有被依赖者，减少其入度
            if let Some(node) = self.nodes.get(&cell_id) {
                for &dependent_id in &node.dependents {
                    if let Some(degree) = in_degree.get_mut(&dependent_id) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent_id);
                        }
                    }
                }
            }
        }
        
        self.topo_cache = Some(result.clone());
        self.cache_valid = true;
        
        result
    }
    
    /// 深度优先搜索拓扑排序
    fn dfs_topo_sort(
        &self,
        cell_id: CellId,
        visited: &mut HashSet<CellId>,
        temp_visited: &mut HashSet<CellId>,
        result: &mut Vec<CellId>,
    ) {
        if temp_visited.contains(&cell_id) {
            // 检测到循环依赖，跳过
            return;
        }
        
        if visited.contains(&cell_id) {
            return;
        }
        
        temp_visited.insert(cell_id);
        
        if let Some(node) = self.nodes.get(&cell_id) {
            // 先访问所有依赖的节点
            for &dep_id in &node.dependencies {
                self.dfs_topo_sort(dep_id, visited, temp_visited, result);
            }
        }
        
        temp_visited.remove(&cell_id);
        visited.insert(cell_id);
        // 在后序位置添加当前节点，这样依赖的节点会先被添加
        result.push(cell_id);
    }
    
    /// 获取需要重新执行的单元格（增量执行）
    pub fn get_cells_to_execute(&mut self, modified_cells: &HashSet<CellId>) -> Vec<CellId> {
        let mut to_execute = HashSet::new();
        
        // 标记修改过的单元格需要执行
        for &cell_id in modified_cells {
            if let Some(node) = self.nodes.get_mut(&cell_id) {
                node.mark_modified();
                to_execute.insert(cell_id);
                
                // 递归标记所有依赖此单元格的单元格
                self.mark_dependents_for_execution(cell_id, &mut to_execute);
            }
        }
        
        // 按拓扑顺序返回
        let topo_order = self.topological_sort();
        topo_order.into_iter()
            .filter(|id| to_execute.contains(id))
            .collect()
    }
    
    /// 递归标记依赖者需要执行
    fn mark_dependents_for_execution(&mut self, cell_id: CellId, to_execute: &mut HashSet<CellId>) {
        if let Some(node) = self.nodes.get(&cell_id) {
            let dependents = node.dependents.clone();
            for dependent_id in dependents {
                if !to_execute.contains(&dependent_id) {
                    to_execute.insert(dependent_id);
                    if let Some(dep_node) = self.nodes.get_mut(&dependent_id) {
                        dep_node.needs_execution = true;
                    }
                    self.mark_dependents_for_execution(dependent_id, to_execute);
                }
            }
        }
    }
    
    /// 标记单元格为已执行
    pub fn mark_executed(&mut self, cell_id: &CellId) {
        if let Some(node) = self.nodes.get_mut(cell_id) {
            node.mark_executed();
        }
    }
    
    /// 获取直接依赖
    pub fn get_dependencies(&self, cell_id: &CellId) -> Vec<CellId> {
        self.nodes.get(cell_id)
            .map(|node| node.dependencies.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// 获取直接被依赖者
    pub fn get_dependents(&self, cell_id: &CellId) -> Vec<CellId> {
        self.nodes.get(cell_id)
            .map(|node| node.dependents.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// 清空图
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.invalidate_cache();
    }
    
    /// 使缓存无效
    fn invalidate_cache(&mut self) {
        self.cache_valid = false;
        self.topo_cache = None;
    }
    
    /// 获取统计信息
    pub fn statistics(&self) -> DependencyGraphStats {
        let total_nodes = self.nodes.len();
        let total_edges: usize = self.nodes.values()
            .map(|node| node.dependencies.len())
            .sum();
        let nodes_needing_execution = self.nodes.values()
            .filter(|node| node.needs_execution)
            .count();
        
        DependencyGraphStats {
            total_nodes,
            total_edges,
            nodes_needing_execution,
            has_cycles: self.has_cycles(),
        }
    }
    
    /// 检查是否有循环依赖
    fn has_cycles(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for &cell_id in self.nodes.keys() {
            if !visited.contains(&cell_id) {
                if self.has_cycle_util(cell_id, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// 循环检测辅助函数
    fn has_cycle_util(
        &self,
        cell_id: CellId,
        visited: &mut HashSet<CellId>,
        rec_stack: &mut HashSet<CellId>,
    ) -> bool {
        visited.insert(cell_id);
        rec_stack.insert(cell_id);
        
        if let Some(node) = self.nodes.get(&cell_id) {
            for &dep_id in &node.dependencies {
                if !visited.contains(&dep_id) {
                    if self.has_cycle_util(dep_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&dep_id) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(&cell_id);
        false
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// 依赖图统计信息
#[derive(Debug, Clone)]
pub struct DependencyGraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub nodes_needing_execution: usize,
    pub has_cycles: bool,
}

/// 执行队列和调度器
pub struct ExecutionQueue {
    /// 待执行队列
    queue: VecDeque<ExecutionQueueItem>,
    /// 正在执行的任务
    executing: HashMap<CellId, ExecutionTask>,
    /// 已完成的单元格
    completed: HashSet<CellId>,
    /// 失败的单元格
    failed: HashSet<CellId>,
    /// 依赖图
    dependency_graph: DependencyGraph,
    /// 最大并发执行数
    max_concurrent: usize,
}

impl ExecutionQueue {
    /// 创建新的执行队列
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            executing: HashMap::new(),
            completed: HashSet::new(),
            failed: HashSet::new(),
            dependency_graph: DependencyGraph::new(),
            max_concurrent: 4, // 默认最多同时执行4个任务
        }
    }
    
    /// 设置最大并发数
    pub fn set_max_concurrent(&mut self, max: usize) {
        self.max_concurrent = max.max(1); // 至少为1
    }
    
    /// 添加单元格到队列
    pub fn enqueue(&mut self, item: ExecutionQueueItem) {
        // 添加到依赖图
        self.dependency_graph.add_node(item.cell_id);
        for dep_id in &item.dependencies {
            self.dependency_graph.add_dependency(item.cell_id, *dep_id);
        }
        
        // 按优先级插入队列
        let mut insert_index = None;
        for (i, existing) in self.queue.iter().enumerate() {
            if item.priority < existing.priority {
                insert_index = Some(i);
                break;
            }
        }
        
        if let Some(index) = insert_index {
            self.queue.insert(index, item);
        } else {
            self.queue.push_back(item);
        }
    }
    
    /// 批量添加单元格（用于增量执行）
    pub fn enqueue_incremental(&mut self, modified_cells: &HashSet<CellId>) {
        let cells_to_execute = self.dependency_graph.get_cells_to_execute(modified_cells);
        
        for cell_id in cells_to_execute {
            let dependencies = self.dependency_graph.get_dependencies(&cell_id);
            let item = ExecutionQueueItem {
                cell_id,
                priority: 0,
                dependencies,
                queued_at: SystemTime::now(),
                is_incremental: true,
                estimated_duration: None,
            };
            
            // 直接添加到队列，不重复添加到依赖图
            self.queue.push_back(item);
        }
        
        // 按拓扑顺序重新排序队列
        self.reorder_by_dependencies();
    }
    
    /// 按依赖关系重新排序队列
    fn reorder_by_dependencies(&mut self) {
        let topo_order = self.dependency_graph.topological_sort();
        let mut ordered_queue = VecDeque::new();
        
        // 按拓扑顺序重新排列队列项
        for cell_id in topo_order {
            if let Some(pos) = self.queue.iter().position(|item| item.cell_id == cell_id) {
                let item = self.queue.remove(pos).unwrap();
                ordered_queue.push_back(item);
            }
        }
        
        // 添加剩余的项目（如果有的话）
        while let Some(item) = self.queue.pop_front() {
            ordered_queue.push_back(item);
        }
        
        self.queue = ordered_queue;
    }
    
    /// 获取下一个可执行的单元格
    pub fn dequeue(&mut self) -> Option<ExecutionQueueItem> {
        // 检查是否已达到最大并发数
        if self.executing.len() >= self.max_concurrent {
            return None;
        }
        
        // 找到依赖都已满足且未在执行中的单元格
        for i in 0..self.queue.len() {
            let item = &self.queue[i];
            
            // 检查是否已在执行中
            if self.executing.contains_key(&item.cell_id) {
                continue;
            }
            
            // 检查依赖是否都已完成
            let dependencies_satisfied = item.dependencies.iter()
                .all(|dep| self.completed.contains(dep) || self.failed.contains(dep));
            
            if dependencies_satisfied {
                let item = self.queue.remove(i).unwrap();
                let mut task = ExecutionTask::new(item.cell_id);
                task.start();
                self.executing.insert(item.cell_id, task);
                return Some(item);
            }
        }
        
        None
    }
    
    /// 获取多个可执行的单元格（用于并行执行）
    pub fn dequeue_batch(&mut self, max_count: usize) -> Vec<ExecutionQueueItem> {
        let mut items = Vec::new();
        let available_slots = self.max_concurrent.saturating_sub(self.executing.len());
        let count = max_count.min(available_slots);
        
        for _ in 0..count {
            if let Some(item) = self.dequeue() {
                items.push(item);
            } else {
                break;
            }
        }
        
        items
    }
    
    /// 标记单元格执行完成
    pub fn mark_completed(&mut self, cell_id: CellId, success: bool, result: Option<ExecutionResult>) {
        if let Some(mut task) = self.executing.remove(&cell_id) {
            if success {
                if let Some(result) = result {
                    task.complete(result);
                } else {
                    task.complete(ExecutionResult::Skipped);
                }
                self.completed.insert(cell_id);
                self.dependency_graph.mark_executed(&cell_id);
            } else {
                let error = task.error.clone().unwrap_or_else(|| "未知错误".to_string());
                task.fail(error);
                self.failed.insert(cell_id);
            }
        }
    }
    
    /// 标记单元格执行失败并尝试重试
    pub fn mark_failed_with_retry(&mut self, cell_id: CellId, error: String) -> bool {
        if let Some(mut task) = self.executing.remove(&cell_id) {
            task.fail(error);
            
            if task.can_retry() {
                task.retry();
                // 重新添加到队列
                let dependencies = self.dependency_graph.get_dependencies(&cell_id);
                let item = ExecutionQueueItem {
                    cell_id,
                    priority: 1000, // 重试任务优先级较低
                    dependencies,
                    queued_at: SystemTime::now(),
                    is_incremental: false,
                    estimated_duration: task.execution_time(),
                };
                self.queue.push_back(item);
                return true; // 将会重试
            } else {
                self.failed.insert(cell_id);
                return false; // 不会重试
            }
        }
        false
    }
    
    /// 取消单元格执行
    pub fn cancel_cell(&mut self, cell_id: &CellId) {
        if let Some(mut task) = self.executing.remove(cell_id) {
            task.cancel();
        }
        
        // 从队列中移除
        self.queue.retain(|item| item.cell_id != *cell_id);
    }
    
    /// 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty() && self.executing.is_empty()
    }
    
    /// 获取队列长度
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    /// 获取正在执行的单元格数量
    pub fn executing_count(&self) -> usize {
        self.executing.len()
    }
    
    /// 获取正在执行的任务
    pub fn get_executing_tasks(&self) -> Vec<&ExecutionTask> {
        self.executing.values().collect()
    }
    
    /// 获取指定单元格的执行任务
    pub fn get_task(&self, cell_id: &CellId) -> Option<&ExecutionTask> {
        self.executing.get(cell_id)
    }
    
    /// 清空队列
    pub fn clear(&mut self) {
        self.queue.clear();
        self.executing.clear();
        self.completed.clear();
        self.failed.clear();
        self.dependency_graph.clear();
    }
    
    /// 获取统计信息
    pub fn statistics(&self) -> QueueStatistics {
        QueueStatistics {
            queued: self.queue.len(),
            executing: self.executing.len(),
            completed: self.completed.len(),
            failed: self.failed.len(),
            max_concurrent: self.max_concurrent,
            dependency_stats: self.dependency_graph.statistics(),
        }
    }
    
    /// 获取依赖图
    pub fn get_dependency_graph(&mut self) -> &mut DependencyGraph {
        &mut self.dependency_graph
    }
    
    /// 估算剩余执行时间
    pub fn estimate_remaining_time(&self) -> Option<Duration> {
        if self.queue.is_empty() {
            return None;
        }
        
        // 基于历史执行时间估算
        let avg_time = self.executing.values()
            .filter_map(|task| task.execution_time())
            .fold(Duration::ZERO, |acc, time| acc + time)
            .checked_div(self.executing.len() as u32)
            .unwrap_or(Duration::from_secs(10)); // 默认10秒
        
        Some(avg_time * self.queue.len() as u32)
    }
}

impl Default for ExecutionQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// 队列统计信息
#[derive(Debug, Clone)]
pub struct QueueStatistics {
    pub queued: usize,
    pub executing: usize,
    pub completed: usize,
    pub failed: usize,
    pub max_concurrent: usize,
    pub dependency_stats: DependencyGraphStats,
}

/// 执行结果缓存
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCache {
    /// 缓存的结果
    results: HashMap<CellId, CachedResult>,
    /// 缓存文件路径
    cache_file: Option<String>,
    /// 最大缓存大小
    max_size: usize,
}

/// 缓存的执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResult {
    /// 单元格内容的哈希值
    content_hash: u64,
    /// 执行结果
    result: ExecutionResult,
    /// 缓存时间
    cached_at: SystemTime,
    /// 访问次数
    access_count: u64,
    /// 最后访问时间
    last_accessed: SystemTime,
}

impl CachedResult {
    /// 创建新的缓存结果
    pub fn new(content_hash: u64, result: ExecutionResult) -> Self {
        let now = SystemTime::now();
        Self {
            content_hash,
            result,
            cached_at: now,
            access_count: 0,
            last_accessed: now,
        }
    }
    
    /// 标记为已访问
    pub fn mark_accessed(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now();
    }
    
    /// 检查是否过期
    pub fn is_expired(&self, max_age: Duration) -> bool {
        self.cached_at.elapsed().unwrap_or(Duration::ZERO) > max_age
    }
}

impl ExecutionCache {
    /// 创建新的执行缓存
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            cache_file: None,
            max_size: 1000, // 默认最多缓存1000个结果
        }
    }
    
    /// 创建带持久化的缓存
    pub fn with_persistence<P: AsRef<Path>>(cache_file: P) -> Self {
        let mut cache = Self::new();
        cache.cache_file = Some(cache_file.as_ref().to_string_lossy().to_string());
        cache.load_from_disk();
        cache
    }
    
    /// 设置最大缓存大小
    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
        self.evict_if_needed();
    }
    
    /// 计算内容哈希
    fn compute_hash(content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
    
    /// 获取缓存的结果
    pub fn get(&mut self, cell_id: &CellId, content: &str) -> Option<ExecutionResult> {
        let content_hash = Self::compute_hash(content);
        
        if let Some(cached) = self.results.get_mut(cell_id) {
            if cached.content_hash == content_hash {
                cached.mark_accessed();
                return Some(cached.result.clone());
            } else {
                // 内容已更改，移除旧缓存
                self.results.remove(cell_id);
            }
        }
        
        None
    }
    
    /// 缓存执行结果
    pub fn put(&mut self, cell_id: CellId, content: &str, result: ExecutionResult) {
        let content_hash = Self::compute_hash(content);
        let cached_result = CachedResult::new(content_hash, result);
        
        self.results.insert(cell_id, cached_result);
        self.evict_if_needed();
        
        // 异步保存到磁盘
        if self.cache_file.is_some() {
            self.save_to_disk();
        }
    }
    
    /// 移除缓存
    pub fn remove(&mut self, cell_id: &CellId) {
        self.results.remove(cell_id);
    }
    
    /// 清空缓存
    pub fn clear(&mut self) {
        self.results.clear();
        if self.cache_file.is_some() {
            self.save_to_disk();
        }
    }
    
    /// 清理过期缓存
    pub fn cleanup_expired(&mut self, max_age: Duration) {
        self.results.retain(|_, cached| !cached.is_expired(max_age));
    }
    
    /// 如果需要则驱逐缓存项
    fn evict_if_needed(&mut self) {
        if self.results.len() <= self.max_size {
            return;
        }
        
        // 使用 LRU 策略驱逐
        let mut items: Vec<_> = self.results.iter().map(|(k, v)| (*k, v.last_accessed)).collect();
        items.sort_by_key(|(_, last_accessed)| *last_accessed);
        
        let to_remove = items.len() - self.max_size;
        let keys_to_remove: Vec<_> = items.into_iter().take(to_remove).map(|(k, _)| k).collect();
        for cell_id in keys_to_remove {
            self.results.remove(&cell_id);
        }
    }
    
    /// 从磁盘加载缓存
    fn load_from_disk(&mut self) {
        if let Some(ref cache_file) = self.cache_file {
            if let Ok(data) = fs::read_to_string(cache_file) {
                if let Ok(results) = serde_json::from_str(&data) {
                    self.results = results;
                }
            }
        }
    }
    
    /// 保存缓存到磁盘
    fn save_to_disk(&self) {
        if let Some(ref cache_file) = self.cache_file {
            if let Ok(data) = serde_json::to_string(&self.results) {
                let _ = fs::write(cache_file, data);
            }
        }
    }
    
    /// 获取缓存统计信息
    pub fn statistics(&self) -> CacheStatistics {
        let total_access_count: u64 = self.results.values()
            .map(|cached| cached.access_count)
            .sum();
        
        let avg_access_count = if self.results.is_empty() {
            0.0
        } else {
            total_access_count as f64 / self.results.len() as f64
        };
        
        CacheStatistics {
            total_entries: self.results.len(),
            max_size: self.max_size,
            total_access_count,
            avg_access_count,
            hit_rate: 0.0, // 需要在使用时统计
        }
    }
}

impl Default for ExecutionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_entries: usize,
    pub max_size: usize,
    pub total_access_count: u64,
    pub avg_access_count: f64,
    pub hit_rate: f64,
}

/// 执行引擎状态
#[derive(Debug, Clone)]
pub struct ExecutionEngineStatus {
    pub is_running: bool,
    pub is_cancelled: bool,
    pub queue_statistics: QueueStatistics,
    pub cache_statistics: CacheStatistics,
    pub execution_statistics: ExecutionStatistics,
    pub cache_hit_rate: f64,
}

/// 执行引擎状态（用于持久化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEngineState {
    pub statistics: ExecutionStatistics,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub config: ExecutionEngineConfig,
}

/// 执行引擎配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEngineConfig {
    /// 最大并发执行数
    pub max_concurrent: usize,
    /// 是否启用缓存
    pub enable_cache: bool,
    /// 缓存文件路径
    pub cache_file: Option<String>,
    /// 缓存最大大小
    pub cache_max_size: usize,
    /// 缓存过期时间
    pub cache_max_age: Duration,
    /// 是否启用进度报告
    pub enable_progress: bool,
    /// 进度更新间隔
    pub progress_interval: Duration,
    /// 执行超时时间
    pub execution_timeout: Option<Duration>,
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for ExecutionEngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 4,
            enable_cache: true,
            cache_file: None,
            cache_max_size: 1000,
            cache_max_age: Duration::from_secs(3600), // 1小时
            enable_progress: true,
            progress_interval: Duration::from_millis(100),
            execution_timeout: Some(Duration::from_secs(300)), // 5分钟
            max_retries: 3,
        }
    }
}

/// 执行引擎
pub struct ExecutionEngine {
    /// Yufmath 计算引擎
    yufmath: Yufmath,
    /// 作用域管理器
    scope_manager: ScopeManager,
    /// 执行队列
    execution_queue: ExecutionQueue,
    /// 执行缓存
    cache: ExecutionCache,
    /// 配置
    config: ExecutionEngineConfig,
    /// 是否正在执行
    is_running: Arc<RwLock<bool>>,
    /// 取消标志
    cancel_flag: Arc<Mutex<bool>>,
    /// 进度回调
    progress_callback: Option<ProgressCallback>,
    /// 执行统计
    statistics: ExecutionStatistics,
    /// 缓存统计
    cache_hits: u64,
    cache_misses: u64,
}

impl ExecutionEngine {
    /// 创建新的执行引擎
    pub fn new() -> Self {
        Self::with_config(ExecutionEngineConfig::default())
    }
    
    /// 使用配置创建执行引擎
    pub fn with_config(config: ExecutionEngineConfig) -> Self {
        let mut execution_queue = ExecutionQueue::new();
        execution_queue.set_max_concurrent(config.max_concurrent);
        
        let cache = if config.enable_cache {
            if let Some(ref cache_file) = config.cache_file {
                ExecutionCache::with_persistence(cache_file)
            } else {
                ExecutionCache::new()
            }
        } else {
            ExecutionCache::new()
        };
        
        Self {
            yufmath: Yufmath::new(),
            scope_manager: ScopeManager::new(),
            execution_queue,
            cache,
            config,
            is_running: Arc::new(RwLock::new(false)),
            cancel_flag: Arc::new(Mutex::new(false)),
            progress_callback: None,
            statistics: ExecutionStatistics::default(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
    
    /// 设置进度回调
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }
    
    /// 更新配置
    pub fn update_config(&mut self, config: ExecutionEngineConfig) {
        self.execution_queue.set_max_concurrent(config.max_concurrent);
        self.cache.set_max_size(config.cache_max_size);
        self.config = config;
    }
    
    /// 执行单个单元格
    pub fn execute_cell(&mut self, cell: &mut NotebookCell) -> NotebookResult<ExecutionResult> {
        let context = ExecutionContext::new(cell.id);
        self.execute_cell_with_context(cell, context)
    }
    
    /// 异步执行单个单元格
    pub async fn execute_cell_async(&mut self, cell: &mut NotebookCell) -> NotebookResult<ExecutionResult> {
        let context = ExecutionContext::new(cell.id);
        self.execute_cell_with_context_async(cell, context).await
    }
    
    /// 使用指定上下文执行单元格
    pub fn execute_cell_with_context(
        &mut self, 
        cell: &mut NotebookCell, 
        context: ExecutionContext
    ) -> NotebookResult<ExecutionResult> {
        let start_time = Instant::now();
        
        // 检查是否可执行
        if !cell.is_executable() {
            return Ok(ExecutionResult::Skipped);
        }
        
        // 检查是否被取消
        if *self.cancel_flag.lock().unwrap() {
            return Ok(ExecutionResult::Cancelled);
        }
        
        // 检查缓存
        if self.config.enable_cache {
            let content = cell.get_text();
            if let Some(cached_result) = self.cache.get(&cell.id, &content) {
                self.cache_hits += 1;
                return Ok(cached_result);
            } else {
                self.cache_misses += 1;
            }
        }
        
        // 设置当前作用域
        self.scope_manager.set_current_scope(Some(cell.id));
        
        // 报告进度
        if let Some(ref callback) = self.progress_callback {
            let progress = ComputeProgress {
                current_step: "解析表达式".to_string(),
                progress: 0.1,
                estimated_remaining: None,
                expression_size: cell.get_text().len(),
                completed_subtasks: 0,
                total_subtasks: 3,
                phase: crate::api::ComputePhase::Parsing,
                details: None,
                memory_usage: 0,
                cache_hit_rate: 0.0,
            };
            
            if !callback(&progress) {
                return Ok(ExecutionResult::Cancelled);
            }
        }
        
        // 解析表达式
        let expression = match self.parse_cell_content(cell) {
            Ok(expr) => expr,
            Err(e) => {
                let execution_time = start_time.elapsed();
                self.statistics.record_execution(false, execution_time);
                
                let result = ExecutionResult::Error {
                    error: format!("解析错误: {}", e),
                    error_type: "ParseError".to_string(),
                    execution_time,
                };
                
                // 缓存错误结果
                if self.config.enable_cache {
                    self.cache.put(cell.id, &cell.get_text(), result.clone());
                }
                
                return Ok(result);
            }
        };
        
        // 报告进度
        if let Some(ref callback) = self.progress_callback {
            let progress = ComputeProgress {
                current_step: "执行计算".to_string(),
                progress: 0.5,
                estimated_remaining: Some(Duration::from_secs(5)),
                expression_size: cell.get_text().len(),
                completed_subtasks: 1,
                total_subtasks: 3,
                phase: crate::api::ComputePhase::Computation,
                details: None,
                memory_usage: 0,
                cache_hit_rate: 0.0,
            };
            
            if !callback(&progress) {
                return Ok(ExecutionResult::Cancelled);
            }
        }
        
        // 执行计算
        let result = match self.compute_expression(&expression, &context) {
            Ok(value) => {
                let execution_time = start_time.elapsed();
                self.statistics.record_execution(true, execution_time);
                
                // 创建输出单元格
                let output_cell = NotebookCell::new_output(
                    value.clone(),
                    context.output_format.clone(),
                    Some(execution_time)
                );
                
                cell.set_output(output_cell);
                
                ExecutionResult::Success {
                    value,
                    format: context.output_format,
                    execution_time,
                }
            }
            Err(e) => {
                let execution_time = start_time.elapsed();
                self.statistics.record_execution(false, execution_time);
                
                ExecutionResult::Error {
                    error: format!("计算错误: {}", e),
                    error_type: "ComputeError".to_string(),
                    execution_time,
                }
            }
        };
        
        // 缓存结果
        if self.config.enable_cache {
            self.cache.put(cell.id, &cell.get_text(), result.clone());
        }
        
        // 报告完成
        if let Some(ref callback) = self.progress_callback {
            let progress = ComputeProgress {
                current_step: "执行完成".to_string(),
                progress: 1.0,
                estimated_remaining: Some(Duration::ZERO),
                expression_size: cell.get_text().len(),
                completed_subtasks: 3,
                total_subtasks: 3,
                phase: crate::api::ComputePhase::Completed,
                details: None,
                memory_usage: 0,
                cache_hit_rate: 0.0,
            };
            
            callback(&progress);
        }
        
        Ok(result)
    }
    
    /// 异步执行单元格
    pub async fn execute_cell_with_context_async(
        &mut self, 
        cell: &mut NotebookCell, 
        context: ExecutionContext
    ) -> NotebookResult<ExecutionResult> {
        // 在异步环境中执行同步代码
        let result = self.execute_cell_with_context(cell, context);
        
        // 模拟异步延迟（实际实现中可能需要真正的异步计算）
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        result
    }
    
    /// 解析单元格内容
    fn parse_cell_content(&mut self, cell: &NotebookCell) -> NotebookResult<Expression> {
        let content = cell.get_text();
        
        match self.yufmath.parse(&content) {
            Ok(expr) => Ok(expr),
            Err(e) => Err(NotebookError::Execution(ComputeError::UnsupportedOperation { 
                operation: format!("解析失败: {}", e) 
            })),
        }
    }
    
    /// 计算表达式
    fn compute_expression(&mut self, expr: &Expression, context: &ExecutionContext) -> NotebookResult<String> {
        // 检查超时
        if context.is_timeout() {
            return Err(NotebookError::Execution(ComputeError::UnsupportedOperation { 
                operation: "执行超时".to_string() 
            }));
        }
        
        // 检查取消标志
        if *self.cancel_flag.lock().unwrap() {
            return Err(NotebookError::Execution(ComputeError::UnsupportedOperation { 
                operation: "执行被取消".to_string() 
            }));
        }
        
        // 导出变量用于计算
        let variables = self.scope_manager.export_for_computation();
        
        // 执行计算
        match self.yufmath.evaluate(expr, &variables) {
            Ok(result) => {
                // 格式化结果
                let formatted = format!("{:?}", result); // 简化实现，实际应该使用格式化器
                
                Ok(formatted)
            }
            Err(e) => Err(NotebookError::Execution(ComputeError::UnsupportedOperation { 
                operation: format!("计算失败: {}", e) 
            })),
        }
    }
    
    /// 批量执行单元格
    pub fn execute_cells(&mut self, cells: &mut [NotebookCell]) -> Vec<NotebookResult<ExecutionResult>> {
        let mut results = Vec::new();
        
        for cell in cells.iter_mut() {
            let result = self.execute_cell(cell);
            results.push(result);
            
            // 检查是否被取消
            if *self.cancel_flag.lock().unwrap() {
                break;
            }
        }
        
        results
    }
    
    /// 增量执行单元格（只执行修改过的单元格及其依赖）
    pub fn execute_cells_incremental(
        &mut self, 
        cells: &mut HashMap<CellId, NotebookCell>
    ) -> HashMap<CellId, NotebookResult<ExecutionResult>> {
        let mut results = HashMap::new();
        
        // 找出需要重新执行的单元格
        let modified_cells: HashSet<CellId> = cells.values()
            .filter(|cell| cell.needs_execution())
            .map(|cell| cell.id)
            .collect();
        
        if modified_cells.is_empty() {
            return results;
        }
        
        // 分析依赖关系
        let dependencies = self.analyze_dependencies_from_cells(cells.values().collect());
        
        // 更新依赖图
        for (cell_id, deps) in &dependencies {
            self.execution_queue.get_dependency_graph().add_node(*cell_id);
            for dep_id in deps {
                self.execution_queue.get_dependency_graph().add_dependency(*cell_id, *dep_id);
            }
        }
        
        // 获取需要执行的单元格（按拓扑顺序）
        let cells_to_execute = self.execution_queue.get_dependency_graph()
            .get_cells_to_execute(&modified_cells);
        
        // 执行单元格
        for cell_id in cells_to_execute {
            if let Some(cell) = cells.get_mut(&cell_id) {
                let result = self.execute_cell(cell);
                results.insert(cell_id, result);
                
                // 检查是否被取消
                if *self.cancel_flag.lock().unwrap() {
                    break;
                }
            }
        }
        
        results
    }
    
    /// 异步批量执行单元格
    pub async fn execute_cells_async(
        &mut self, 
        cells: &mut HashMap<CellId, NotebookCell>
    ) -> HashMap<CellId, NotebookResult<ExecutionResult>> {
        let mut results = HashMap::new();
        *self.is_running.write().unwrap() = true;
        
        // 分析依赖关系并构建执行队列
        let dependencies = self.analyze_dependencies_from_cells(cells.values().collect());
        
        // 清空队列并重新构建
        self.execution_queue.clear();
        
        for (cell_id, deps) in dependencies {
            let item = ExecutionQueueItem {
                cell_id,
                priority: 0,
                dependencies: deps,
                queued_at: SystemTime::now(),
                is_incremental: false,
                estimated_duration: None,
            };
            self.execution_queue.enqueue(item);
        }
        
        // 异步执行队列
        while !self.execution_queue.is_empty() {
            // 检查取消标志
            if *self.cancel_flag.lock().unwrap() {
                break;
            }
            
            // 获取可执行的任务批次
            let batch = self.execution_queue.dequeue_batch(self.config.max_concurrent);
            
            if batch.is_empty() {
                // 没有可执行的任务，等待一段时间
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
            
            // 并行执行批次中的任务
            let mut handles = Vec::new();
            
            for item in batch {
                if let Some(cell) = cells.get_mut(&item.cell_id) {
                    let cell_clone = cell.clone();
                    let context = ExecutionContext::new(item.cell_id);
                    
                    // 创建异步任务
                    let handle = tokio::spawn(async move {
                        // 这里应该是真正的异步执行，暂时使用同步版本
                        (item.cell_id, Ok(ExecutionResult::Skipped))
                    });
                    
                    handles.push(handle);
                }
            }
            
            // 等待所有任务完成
            for handle in handles {
                if let Ok((cell_id, result)) = handle.await {
                    let success = result.as_ref().map(|r| r.is_success()).unwrap_or(false);
                    let result_for_queue = result.as_ref().ok().cloned();
                    self.execution_queue.mark_completed(cell_id, success, result_for_queue);
                    results.insert(cell_id, result);
                }
            }
            
            // 短暂延迟以避免忙等待
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        *self.is_running.write().unwrap() = false;
        results
    }
    
    /// 异步执行单元格队列
    pub fn execute_queue(&mut self) -> NotebookResult<()> {
        *self.is_running.write().unwrap() = true;
        *self.cancel_flag.lock().unwrap() = false;
        
        while !self.execution_queue.is_empty() {
            // 检查取消标志
            if *self.cancel_flag.lock().unwrap() {
                break;
            }
            
            // 获取下一个可执行的单元格
            if let Some(queue_item) = self.execution_queue.dequeue() {
                // 这里需要实际的单元格数据，在实际实现中需要从笔记本获取
                // 暂时跳过具体执行，只标记完成
                self.execution_queue.mark_completed(queue_item.cell_id, true, None);
            } else {
                // 没有可执行的单元格，等待一段时间
                thread::sleep(Duration::from_millis(100));
            }
        }
        
        *self.is_running.write().unwrap() = false;
        Ok(())
    }
    
    /// 添加单元格到执行队列
    pub fn queue_cell(&mut self, cell_id: CellId, dependencies: Vec<CellId>) {
        let item = ExecutionQueueItem {
            cell_id,
            priority: 0,
            dependencies,
            queued_at: SystemTime::now(),
            is_incremental: false,
            estimated_duration: None,
        };
        
        self.execution_queue.enqueue(item);
    }
    
    /// 取消执行
    pub fn cancel_execution(&mut self) {
        *self.cancel_flag.lock().unwrap() = true;
    }
    
    /// 检查是否正在执行
    pub fn is_running(&self) -> bool {
        *self.is_running.read().unwrap()
    }
    
    /// 清空执行队列
    pub fn clear_queue(&mut self) {
        self.execution_queue.clear();
    }
    
    /// 获取作用域管理器
    pub fn get_scope_manager(&mut self) -> &mut ScopeManager {
        &mut self.scope_manager
    }
    
    /// 获取执行统计信息
    pub fn get_statistics(&self) -> &ExecutionStatistics {
        &self.statistics
    }
    
    /// 重置统计信息
    pub fn reset_statistics(&mut self) {
        self.statistics = ExecutionStatistics::default();
    }
    
    /// 分析单元格依赖关系
    pub fn analyze_dependencies(&self, cells: &[NotebookCell]) -> HashMap<CellId, Vec<CellId>> {
        self.analyze_dependencies_from_cells(cells.iter().collect())
    }
    
    /// 从单元格向量分析依赖关系
    fn analyze_dependencies_from_cells(&self, cells: Vec<&NotebookCell>) -> HashMap<CellId, Vec<CellId>> {
        let mut dependencies = HashMap::new();
        
        // 收集所有变量定义
        let mut variable_definitions: HashMap<String, CellId> = HashMap::new();
        
        for cell in &cells {
            if cell.is_executable() {
                let content = cell.get_text();
                
                // 简单的变量定义检测（实际实现需要更复杂的解析）
                if content.contains('=') && !content.contains("==") {
                    if let Some(var_name) = content.split('=').next() {
                        let var_name = var_name.trim().to_string();
                        variable_definitions.insert(var_name, cell.id);
                    }
                }
            }
        }
        
        // 分析每个单元格的依赖
        for cell in &cells {
            if cell.is_executable() {
                let content = cell.get_text();
                let mut cell_deps = Vec::new();
                
                // 检查使用了哪些变量
                for (var_name, def_cell_id) in &variable_definitions {
                    if content.contains(var_name) && *def_cell_id != cell.id {
                        cell_deps.push(*def_cell_id);
                    }
                }
                
                dependencies.insert(cell.id, cell_deps);
            }
        }
        
        dependencies
    }
    
    /// 处理执行错误并尝试恢复
    pub fn handle_execution_error(
        &mut self, 
        cell_id: CellId, 
        error: &NotebookError
    ) -> NotebookResult<bool> {
        match error {
            NotebookError::Execution(compute_error) => {
                match compute_error {
                    ComputeError::DivisionByZero => {
                        // 对于除零错误，可以尝试符号计算
                        self.try_symbolic_recovery(cell_id)
                    }
                    ComputeError::UndefinedVariable { name } => {
                        // 对于未定义变量，尝试从作用域中查找
                        self.try_variable_recovery(cell_id, name)
                    }
                    ComputeError::Overflow => {
                        // 对于溢出错误，尝试使用更高精度
                        self.try_precision_recovery(cell_id)
                    }
                    _ => Ok(false), // 其他错误暂不处理
                }
            }
            _ => Ok(false), // 非计算错误暂不处理
        }
    }
    
    /// 尝试符号计算恢复
    fn try_symbolic_recovery(&mut self, _cell_id: CellId) -> NotebookResult<bool> {
        // 实现符号计算恢复逻辑
        // 这里是简化实现
        Ok(false)
    }
    
    /// 尝试变量恢复
    fn try_variable_recovery(&mut self, _cell_id: CellId, var_name: &str) -> NotebookResult<bool> {
        // 检查是否在作用域中存在该变量
        if self.scope_manager.has_variable(var_name) {
            // 变量存在，可能是作用域问题
            return Ok(true);
        }
        
        // 尝试从全局作用域查找
        Ok(false)
    }
    
    /// 尝试精度恢复
    fn try_precision_recovery(&mut self, _cell_id: CellId) -> NotebookResult<bool> {
        // 实现高精度计算恢复逻辑
        Ok(false)
    }
    
    /// 清理执行环境
    pub fn cleanup_execution_environment(&mut self) {
        // 清理缓存中的过期项
        self.cache.cleanup_expired(self.config.cache_max_age);
        
        // 重置取消标志
        *self.cancel_flag.lock().unwrap() = false;
        
        // 清理执行队列中的失败任务
        self.execution_queue.clear();
        
        // 重置运行状态
        *self.is_running.write().unwrap() = false;
    }
    
    /// 获取执行引擎状态
    pub fn get_engine_status(&self) -> ExecutionEngineStatus {
        let queue_stats = self.execution_queue.statistics();
        let cache_stats = self.cache.statistics();
        let is_running = *self.is_running.read().unwrap();
        let is_cancelled = *self.cancel_flag.lock().unwrap();
        
        ExecutionEngineStatus {
            is_running,
            is_cancelled,
            queue_statistics: queue_stats,
            cache_statistics: cache_stats,
            execution_statistics: self.statistics.clone(),
            cache_hit_rate: if self.cache_hits + self.cache_misses > 0 {
                self.cache_hits as f64 / (self.cache_hits + self.cache_misses) as f64
            } else {
                0.0
            },
        }
    }
    
    /// 保存执行状态到磁盘
    pub fn save_state<P: AsRef<Path>>(&self, path: P) -> NotebookResult<()> {
        let state = ExecutionEngineState {
            statistics: self.statistics.clone(),
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            config: self.config.clone(),
        };
        
        let data = serde_json::to_string_pretty(&state)
            .map_err(|e| NotebookError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("序列化失败: {}", e)
            )))?;
        
        fs::write(path, data)
            .map_err(|e| NotebookError::Io(e))?;
        
        Ok(())
    }
    
    /// 从磁盘加载执行状态
    pub fn load_state<P: AsRef<Path>>(&mut self, path: P) -> NotebookResult<()> {
        let data = fs::read_to_string(path)
            .map_err(|e| NotebookError::Io(e))?;
        
        let state: ExecutionEngineState = serde_json::from_str(&data)
            .map_err(|e| NotebookError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("反序列化失败: {}", e)
            )))?;
        
        self.statistics = state.statistics;
        self.cache_hits = state.cache_hits;
        self.cache_misses = state.cache_misses;
        self.update_config(state.config);
        
        Ok(())
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 执行统计信息
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExecutionStatistics {
    /// 总执行次数
    pub total_executions: u64,
    /// 成功执行次数
    pub successful_executions: u64,
    /// 失败执行次数
    pub failed_executions: u64,
    /// 总执行时间
    pub total_execution_time: Duration,
    /// 平均执行时间
    pub average_execution_time: Duration,
    /// 最长执行时间
    pub max_execution_time: Duration,
    /// 最短执行时间
    pub min_execution_time: Duration,
}

impl ExecutionStatistics {
    /// 记录执行结果
    pub fn record_execution(&mut self, success: bool, execution_time: Duration) {
        self.total_executions += 1;
        
        if success {
            self.successful_executions += 1;
        } else {
            self.failed_executions += 1;
        }
        
        self.total_execution_time += execution_time;
        self.average_execution_time = self.total_execution_time / self.total_executions as u32;
        
        if execution_time > self.max_execution_time {
            self.max_execution_time = execution_time;
        }
        
        if self.min_execution_time == Duration::ZERO || execution_time < self.min_execution_time {
            self.min_execution_time = execution_time;
        }
    }
    
    /// 获取成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64
        }
    }
    
    /// 获取失败率
    pub fn failure_rate(&self) -> f64 {
        1.0 - self.success_rate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notebook::NotebookCell;
    use std::collections::HashSet;
    
    #[test]
    fn test_execution_result() {
        let success = ExecutionResult::Success {
            value: "42".to_string(),
            format: FormatType::Standard,
            execution_time: Duration::from_millis(100),
        };
        
        assert!(success.is_success());
        assert!(!success.is_error());
        assert_eq!(success.value(), Some("42"));
        assert_eq!(success.execution_time(), Some(Duration::from_millis(100)));
        
        let error = ExecutionResult::Error {
            error: "除零错误".to_string(),
            error_type: "DivisionByZero".to_string(),
            execution_time: Duration::from_millis(50),
        };
        
        assert!(!error.is_success());
        assert!(error.is_error());
        assert_eq!(error.value(), Some("除零错误"));
    }
    
    #[test]
    fn test_execution_context() {
        let cell_id = uuid::Uuid::new_v4();
        let context = ExecutionContext::new(cell_id);
        
        assert_eq!(context.cell_id, cell_id);
        assert!(context.allow_cancellation);
        assert!(context.show_timing);
        
        // 测试超时检查
        let mut timeout_context = context.clone();
        timeout_context.max_execution_time = Some(Duration::from_nanos(1));
        thread::sleep(Duration::from_millis(1));
        assert!(timeout_context.is_timeout());
    }
    
    #[test]
    fn test_execution_task() {
        let cell_id = uuid::Uuid::new_v4();
        let mut task = ExecutionTask::new(cell_id);
        
        assert_eq!(task.cell_id, cell_id);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.retry_count, 0);
        
        // 开始任务
        task.start();
        assert_eq!(task.status, TaskStatus::Running);
        assert!(task.started_at.is_some());
        
        // 完成任务
        let result = ExecutionResult::Success {
            value: "42".to_string(),
            format: FormatType::Standard,
            execution_time: Duration::from_millis(100),
        };
        task.complete(result);
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.completed_at.is_some());
        
        // 测试重试
        let mut failed_task = ExecutionTask::new(cell_id);
        failed_task.fail("测试错误".to_string());
        assert!(failed_task.can_retry());
        
        failed_task.retry();
        assert_eq!(failed_task.status, TaskStatus::Pending);
        assert_eq!(failed_task.retry_count, 1);
    }
    
    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        let cell1 = uuid::Uuid::new_v4();
        let cell2 = uuid::Uuid::new_v4();
        let cell3 = uuid::Uuid::new_v4();
        
        // 添加节点和依赖关系
        graph.add_node(cell1);
        graph.add_node(cell2);
        graph.add_node(cell3);
        
        graph.add_dependency(cell2, cell1); // cell2 依赖 cell1
        graph.add_dependency(cell3, cell2); // cell3 依赖 cell2
        
        // 测试依赖关系
        let deps_cell2 = graph.get_dependencies(&cell2);
        let deps_cell3 = graph.get_dependencies(&cell3);
        println!("cell2 的依赖: {:?}", deps_cell2);
        println!("cell3 的依赖: {:?}", deps_cell3);
        
        assert!(deps_cell2.contains(&cell1), "cell2 应该依赖 cell1");
        assert!(deps_cell3.contains(&cell2), "cell3 应该依赖 cell2");
        
        // 测试拓扑排序
        let topo_order = graph.topological_sort();
        println!("拓扑排序结果: {:?}", topo_order);
        
        // 验证依赖关系是否正确：依赖的节点应该在被依赖的节点之前
        let cell1_pos = topo_order.iter().position(|&id| id == cell1).unwrap();
        let cell2_pos = topo_order.iter().position(|&id| id == cell2).unwrap();
        let cell3_pos = topo_order.iter().position(|&id| id == cell3).unwrap();
        
        println!("位置: cell1={}, cell2={}, cell3={}", cell1_pos, cell2_pos, cell3_pos);
        
        // cell1 应该在 cell2 之前（因为 cell2 依赖 cell1）
        assert!(cell1_pos < cell2_pos, "cell1 应该在 cell2 之前，但实际位置: cell1={}, cell2={}", cell1_pos, cell2_pos);
        // cell2 应该在 cell3 之前（因为 cell3 依赖 cell2）
        assert!(cell2_pos < cell3_pos, "cell2 应该在 cell3 之前，但实际位置: cell2={}, cell3={}", cell2_pos, cell3_pos);
        
        // 测试增量执行
        let mut modified = HashSet::new();
        modified.insert(cell1);
        
        let to_execute = graph.get_cells_to_execute(&modified);
        assert!(to_execute.contains(&cell1));
        assert!(to_execute.contains(&cell2));
        assert!(to_execute.contains(&cell3));
        
        // 测试依赖图统计
        let stats = graph.statistics();
        assert_eq!(stats.total_nodes, 3);
        assert_eq!(stats.total_edges, 2);
        assert!(!stats.has_cycles);
    }
    
    #[test]
    fn test_execution_queue_enhanced() {
        let mut queue = ExecutionQueue::new();
        let cell_id1 = uuid::Uuid::new_v4();
        let cell_id2 = uuid::Uuid::new_v4();
        
        // 添加项目到队列
        let item1 = ExecutionQueueItem {
            cell_id: cell_id1,
            priority: 1,
            dependencies: vec![],
            queued_at: SystemTime::now(),
            is_incremental: false,
            estimated_duration: None,
        };
        
        let item2 = ExecutionQueueItem {
            cell_id: cell_id2,
            priority: 0, // 更高优先级
            dependencies: vec![cell_id1], // 依赖 cell_id1
            queued_at: SystemTime::now(),
            is_incremental: true,
            estimated_duration: Some(Duration::from_secs(5)),
        };
        
        queue.enqueue(item1);
        queue.enqueue(item2);
        
        // 应该先获取 cell_id1（没有依赖）
        let next = queue.dequeue().unwrap();
        assert_eq!(next.cell_id, cell_id1);
        
        // 标记完成
        queue.mark_completed(cell_id1, true, Some(ExecutionResult::Success {
            value: "42".to_string(),
            format: FormatType::Standard,
            execution_time: Duration::from_millis(100),
        }));
        
        // 现在可以获取 cell_id2
        let next = queue.dequeue().unwrap();
        assert_eq!(next.cell_id, cell_id2);
        
        // 测试批量获取
        queue.clear();
        for i in 0..5 {
            let item = ExecutionQueueItem {
                cell_id: uuid::Uuid::new_v4(),
                priority: i,
                dependencies: vec![],
                queued_at: SystemTime::now(),
                is_incremental: false,
                estimated_duration: None,
            };
            queue.enqueue(item);
        }
        
        let batch = queue.dequeue_batch(3);
        assert_eq!(batch.len(), 3);
    }
    
    #[test]
    fn test_execution_cache() {
        let mut cache = ExecutionCache::new();
        let cell_id = uuid::Uuid::new_v4();
        let content = "2 + 3";
        
        // 缓存未命中
        assert!(cache.get(&cell_id, content).is_none());
        
        // 添加到缓存
        let result = ExecutionResult::Success {
            value: "5".to_string(),
            format: FormatType::Standard,
            execution_time: Duration::from_millis(10),
        };
        cache.put(cell_id, content, result.clone());
        
        // 缓存命中
        let cached = cache.get(&cell_id, content).unwrap();
        assert_eq!(cached.value(), Some("5"));
        
        // 内容更改后缓存失效
        let new_content = "3 + 4";
        assert!(cache.get(&cell_id, new_content).is_none());
        
        // 测试缓存清理
        cache.clear();
        assert!(cache.get(&cell_id, content).is_none());
    }
    
    #[test]
    fn test_execution_engine_enhanced() {
        let config = ExecutionEngineConfig {
            max_concurrent: 2,
            enable_cache: true,
            cache_file: None,
            cache_max_size: 100,
            cache_max_age: Duration::from_secs(3600),
            enable_progress: false, // 禁用进度报告以简化测试
            progress_interval: Duration::from_millis(100),
            execution_timeout: Some(Duration::from_secs(10)),
            max_retries: 2,
        };
        
        let mut engine = ExecutionEngine::with_config(config);
        let mut cell = NotebookCell::new_code("2 + 3".to_string());
        
        // 第一次执行
        let result1 = engine.execute_cell(&mut cell).unwrap();
        assert!(result1.is_success());
        
        // 第二次执行应该命中缓存
        let result2 = engine.execute_cell(&mut cell).unwrap();
        assert!(result2.is_success());
        
        // 检查缓存统计
        let status = engine.get_engine_status();
        assert!(status.cache_hit_rate > 0.0);
        
        // 测试非可执行单元格
        let mut text_cell = NotebookCell::new_text("这是文本".to_string());
        let result = engine.execute_cell(&mut text_cell).unwrap();
        assert!(matches!(result, ExecutionResult::Skipped));
    }
    
    #[test]
    fn test_incremental_execution() {
        let mut engine = ExecutionEngine::new();
        let mut cells = HashMap::new();
        
        // 创建相互依赖的单元格
        let mut cell1 = NotebookCell::new_code("x = 10".to_string());
        let mut cell2 = NotebookCell::new_code("y = x + 5".to_string());
        let mut cell3 = NotebookCell::new_code("z = y * 2".to_string());
        
        // 标记为需要执行
        cell1.metadata.mark_dirty();
        cell2.metadata.mark_dirty();
        cell3.metadata.mark_dirty();
        
        cells.insert(cell1.id, cell1);
        cells.insert(cell2.id, cell2);
        cells.insert(cell3.id, cell3);
        
        // 执行增量更新
        let results = engine.execute_cells_incremental(&mut cells);
        
        // 所有单元格都应该被执行
        assert_eq!(results.len(), 3);
        
        // 现在只修改第一个单元格
        if let Some(cell1) = cells.values_mut().next() {
            cell1.set_text("x = 20".to_string());
        }
        
        let results = engine.execute_cells_incremental(&mut cells);
        
        // 应该执行所有依赖的单元格
        assert!(results.len() > 0);
    }
    
    #[test]
    fn test_error_handling_and_recovery() {
        let mut engine = ExecutionEngine::new();
        
        // 测试除零错误
        let error = NotebookError::Execution(ComputeError::DivisionByZero);
        let cell_id = uuid::Uuid::new_v4();
        
        let can_recover = engine.handle_execution_error(cell_id, &error).unwrap();
        // 目前的实现返回 false，表示无法恢复
        assert!(!can_recover);
        
        // 测试未定义变量错误
        let error = NotebookError::Execution(ComputeError::UndefinedVariable { 
            name: "undefined_var".to_string() 
        });
        
        let can_recover = engine.handle_execution_error(cell_id, &error).unwrap();
        assert!(!can_recover);
    }
    
    #[test]
    fn test_execution_statistics() {
        let mut stats = ExecutionStatistics::default();
        
        stats.record_execution(true, Duration::from_millis(100));
        stats.record_execution(false, Duration::from_millis(50));
        stats.record_execution(true, Duration::from_millis(200));
        
        assert_eq!(stats.total_executions, 3);
        assert_eq!(stats.successful_executions, 2);
        assert_eq!(stats.failed_executions, 1);
        assert_eq!(stats.success_rate(), 2.0 / 3.0);
        assert_eq!(stats.max_execution_time, Duration::from_millis(200));
        assert_eq!(stats.min_execution_time, Duration::from_millis(50));
    }
    
    #[test]
    fn test_dependency_analysis() {
        let engine = ExecutionEngine::new();
        
        let cell1 = NotebookCell::new_code("x = 10".to_string());
        let cell2 = NotebookCell::new_code("y = x + 5".to_string());
        let cell3 = NotebookCell::new_code("z = y * 2".to_string());
        
        let cells = vec![cell1.clone(), cell2.clone(), cell3.clone()];
        let deps = engine.analyze_dependencies(&cells);
        
        // cell2 应该依赖 cell1
        assert!(deps.get(&cell2.id).unwrap().contains(&cell1.id));
        
        // cell3 应该依赖 cell2
        assert!(deps.get(&cell3.id).unwrap().contains(&cell2.id));
    }
    
    #[tokio::test]
    async fn test_async_execution() {
        let mut engine = ExecutionEngine::new();
        let mut cell = NotebookCell::new_code("2 + 3".to_string());
        
        let result = engine.execute_cell_async(&mut cell).await.unwrap();
        assert!(result.is_success());
    }
}