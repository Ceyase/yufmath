//! # 单元格执行引擎
//!
//! 提供单元格的执行、调度和依赖管理功能。

use super::{NotebookCell, CellId, CellType, CellContent, ScopeManager, NotebookError, NotebookResult};
use crate::api::Yufmath;
use crate::core::Expression;
use crate::engine::ComputeError;
use crate::formatter::{FormatType, FormatOptions};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;

/// 执行结果
#[derive(Debug, Clone)]
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
}

/// 执行队列
pub struct ExecutionQueue {
    /// 待执行队列
    queue: VecDeque<ExecutionQueueItem>,
    /// 正在执行的单元格
    executing: HashSet<CellId>,
    /// 已完成的单元格
    completed: HashSet<CellId>,
    /// 失败的单元格
    failed: HashSet<CellId>,
}

impl ExecutionQueue {
    /// 创建新的执行队列
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            executing: HashSet::new(),
            completed: HashSet::new(),
            failed: HashSet::new(),
        }
    }
    
    /// 添加单元格到队列
    pub fn enqueue(&mut self, item: ExecutionQueueItem) {
        // 按优先级插入
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
    
    /// 获取下一个可执行的单元格
    pub fn dequeue(&mut self) -> Option<ExecutionQueueItem> {
        // 找到依赖都已满足的单元格
        for i in 0..self.queue.len() {
            let item = &self.queue[i];
            
            // 检查依赖是否都已完成
            let dependencies_satisfied = item.dependencies.iter()
                .all(|dep| self.completed.contains(dep) || self.failed.contains(dep));
            
            if dependencies_satisfied {
                let item = self.queue.remove(i).unwrap();
                self.executing.insert(item.cell_id);
                return Some(item);
            }
        }
        
        None
    }
    
    /// 标记单元格执行完成
    pub fn mark_completed(&mut self, cell_id: CellId, success: bool) {
        self.executing.remove(&cell_id);
        
        if success {
            self.completed.insert(cell_id);
        } else {
            self.failed.insert(cell_id);
        }
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
    
    /// 清空队列
    pub fn clear(&mut self) {
        self.queue.clear();
        self.executing.clear();
        self.completed.clear();
        self.failed.clear();
    }
    
    /// 获取统计信息
    pub fn statistics(&self) -> QueueStatistics {
        QueueStatistics {
            queued: self.queue.len(),
            executing: self.executing.len(),
            completed: self.completed.len(),
            failed: self.failed.len(),
        }
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
}

/// 执行引擎
pub struct ExecutionEngine {
    /// Yufmath 计算引擎
    yufmath: Yufmath,
    /// 作用域管理器
    scope_manager: ScopeManager,
    /// 执行队列
    execution_queue: ExecutionQueue,
    /// 是否正在执行
    is_running: bool,
    /// 取消标志
    cancel_flag: Arc<Mutex<bool>>,
    /// 执行统计
    statistics: ExecutionStatistics,
}

impl ExecutionEngine {
    /// 创建新的执行引擎
    pub fn new() -> Self {
        Self {
            yufmath: Yufmath::new(),
            scope_manager: ScopeManager::new(),
            execution_queue: ExecutionQueue::new(),
            is_running: false,
            cancel_flag: Arc::new(Mutex::new(false)),
            statistics: ExecutionStatistics::default(),
        }
    }
    
    /// 执行单个单元格
    pub fn execute_cell(&mut self, cell: &mut NotebookCell) -> NotebookResult<ExecutionResult> {
        let context = ExecutionContext::new(cell.id);
        self.execute_cell_with_context(cell, context)
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
        
        // 设置当前作用域
        self.scope_manager.set_current_scope(Some(cell.id));
        
        // 解析表达式
        let expression = match self.parse_cell_content(cell) {
            Ok(expr) => expr,
            Err(e) => {
                let execution_time = start_time.elapsed();
                self.statistics.record_execution(false, execution_time);
                
                return Ok(ExecutionResult::Error {
                    error: format!("解析错误: {}", e),
                    error_type: "ParseError".to_string(),
                    execution_time,
                });
            }
        };
        
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
        
        Ok(result)
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
    
    /// 异步执行单元格队列
    pub fn execute_queue(&mut self) -> NotebookResult<()> {
        self.is_running = true;
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
                self.execution_queue.mark_completed(queue_item.cell_id, true);
            } else {
                // 没有可执行的单元格，等待一段时间
                thread::sleep(Duration::from_millis(100));
            }
        }
        
        self.is_running = false;
        Ok(())
    }
    
    /// 添加单元格到执行队列
    pub fn queue_cell(&mut self, cell_id: CellId, dependencies: Vec<CellId>) {
        let item = ExecutionQueueItem {
            cell_id,
            priority: 0,
            dependencies,
            queued_at: SystemTime::now(),
        };
        
        self.execution_queue.enqueue(item);
    }
    
    /// 取消执行
    pub fn cancel_execution(&mut self) {
        *self.cancel_flag.lock().unwrap() = true;
    }
    
    /// 检查是否正在执行
    pub fn is_running(&self) -> bool {
        self.is_running
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
        let mut dependencies = HashMap::new();
        
        // 收集所有变量定义
        let mut variable_definitions: HashMap<String, CellId> = HashMap::new();
        
        for cell in cells {
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
        for cell in cells {
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
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 执行统计信息
#[derive(Debug, Default, Clone)]
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
    fn test_execution_queue() {
        let mut queue = ExecutionQueue::new();
        let cell_id1 = uuid::Uuid::new_v4();
        let cell_id2 = uuid::Uuid::new_v4();
        
        // 添加项目到队列
        let item1 = ExecutionQueueItem {
            cell_id: cell_id1,
            priority: 1,
            dependencies: vec![],
            queued_at: SystemTime::now(),
        };
        
        let item2 = ExecutionQueueItem {
            cell_id: cell_id2,
            priority: 0, // 更高优先级
            dependencies: vec![cell_id1], // 依赖 cell_id1
            queued_at: SystemTime::now(),
        };
        
        queue.enqueue(item1);
        queue.enqueue(item2);
        
        // 应该先获取 cell_id1（没有依赖）
        let next = queue.dequeue().unwrap();
        assert_eq!(next.cell_id, cell_id1);
        
        // 标记完成
        queue.mark_completed(cell_id1, true);
        
        // 现在可以获取 cell_id2
        let next = queue.dequeue().unwrap();
        assert_eq!(next.cell_id, cell_id2);
        
        assert!(queue.is_empty());
    }
    
    #[test]
    fn test_execution_engine() {
        let mut engine = ExecutionEngine::new();
        let mut cell = NotebookCell::new_code("2 + 3".to_string());
        
        let result = engine.execute_cell(&mut cell).unwrap();
        assert!(result.is_success());
        
        // 检查输出是否设置
        assert!(cell.get_output().is_some());
        
        // 测试非可执行单元格
        let mut text_cell = NotebookCell::new_text("这是文本".to_string());
        let result = engine.execute_cell(&mut text_cell).unwrap();
        assert!(matches!(result, ExecutionResult::Skipped));
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
}