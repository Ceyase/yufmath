//! # 异步计算支持
//!
//! 提供异步和并发计算功能。

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};
use super::{YufmathError, ComputeProgress};

/// 异步计算结果
pub type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T, YufmathError>> + Send>>;

/// 计算任务状态
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    /// 等待中
    Pending,
    /// 运行中
    Running,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
    /// 出错
    Error,
}

/// 异步计算任务
pub struct AsyncTask<T> {
    /// 任务ID
    pub id: u64,
    /// 任务状态
    pub status: TaskStatus,
    /// 计算结果
    pub result: Option<Result<T, YufmathError>>,
    /// 进度信息
    pub progress: Option<ComputeProgress>,
    /// 开始时间
    pub start_time: Option<Instant>,
    /// 完成时间
    pub end_time: Option<Instant>,
    /// 唤醒器
    waker: Option<Waker>,
}

impl<T> AsyncTask<T> {
    /// 创建新的异步任务
    pub fn new(id: u64) -> Self {
        Self {
            id,
            status: TaskStatus::Pending,
            result: None,
            progress: None,
            start_time: None,
            end_time: None,
            waker: None,
        }
    }
    
    /// 开始任务
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
        self.start_time = Some(Instant::now());
    }
    
    /// 完成任务
    pub fn complete(&mut self, result: Result<T, YufmathError>) {
        self.status = TaskStatus::Completed;
        self.end_time = Some(Instant::now());
        self.result = Some(result);
        
        // 唤醒等待的 Future
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
    
    /// 取消任务
    pub fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.end_time = Some(Instant::now());
        
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
    
    /// 更新进度
    pub fn update_progress(&mut self, progress: ComputeProgress) {
        self.progress = Some(progress);
        
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }
    
    /// 获取执行时间
    pub fn execution_time(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(Instant::now().duration_since(start)),
            _ => None,
        }
    }
}

/// 异步计算 Future
pub struct AsyncComputation<T> {
    task: Arc<Mutex<AsyncTask<T>>>,
}

impl<T> AsyncComputation<T> {
    /// 创建新的异步计算
    pub fn new(task: Arc<Mutex<AsyncTask<T>>>) -> Self {
        Self { task }
    }
    
    /// 获取任务状态
    pub fn status(&self) -> TaskStatus {
        if let Ok(task) = self.task.lock() {
            task.status.clone()
        } else {
            TaskStatus::Error
        }
    }
    
    /// 获取进度信息
    pub fn progress(&self) -> Option<ComputeProgress> {
        if let Ok(task) = self.task.lock() {
            task.progress.clone()
        } else {
            None
        }
    }
    
    /// 取消计算
    pub fn cancel(&self) {
        if let Ok(mut task) = self.task.lock() {
            task.cancel();
        }
    }
    
    /// 获取执行时间
    pub fn execution_time(&self) -> Option<Duration> {
        if let Ok(task) = self.task.lock() {
            task.execution_time()
        } else {
            None
        }
    }
}

impl<T: Clone> Future for AsyncComputation<T> {
    type Output = Result<T, YufmathError>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Ok(mut task) = self.task.lock() {
            match task.status {
                TaskStatus::Completed => {
                    if let Some(result) = task.result.take() {
                        Poll::Ready(result)
                    } else {
                        Poll::Ready(Err(YufmathError::internal("任务已完成但没有结果")))
                    }
                }
                TaskStatus::Cancelled => {
                    Poll::Ready(Err(YufmathError::internal("任务已被取消")))
                }
                TaskStatus::Error => {
                    Poll::Ready(Err(YufmathError::internal("任务执行出错")))
                }
                _ => {
                    // 保存 waker 以便任务完成时唤醒
                    task.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        } else {
            Poll::Ready(Err(YufmathError::internal("无法获取任务锁")))
        }
    }
}

/// 批量异步计算管理器
pub struct BatchAsyncComputer {
    /// 任务计数器
    task_counter: Arc<Mutex<u64>>,
    /// 活跃任务
    active_tasks: Arc<Mutex<Vec<Arc<Mutex<AsyncTask<String>>>>>>,
    /// 最大并发数
    max_concurrent: usize,
}

impl BatchAsyncComputer {
    /// 创建新的批量异步计算管理器
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            task_counter: Arc::new(Mutex::new(0)),
            active_tasks: Arc::new(Mutex::new(Vec::new())),
            max_concurrent,
        }
    }
    
    /// 提交批量计算任务
    pub fn submit_batch(&self, expressions: Vec<String>) -> Vec<AsyncComputation<String>> {
        let mut computations = Vec::new();
        
        for expr in expressions {
            let task_id = {
                let mut counter = self.task_counter.lock().unwrap();
                *counter += 1;
                *counter
            };
            
            let task = Arc::new(Mutex::new(AsyncTask::new(task_id)));
            let computation = AsyncComputation::new(Arc::clone(&task));
            
            // 添加到活跃任务列表
            if let Ok(mut active_tasks) = self.active_tasks.lock() {
                active_tasks.push(Arc::clone(&task));
            }
            
            // 启动计算任务
            self.spawn_computation_task(task, expr);
            
            computations.push(computation);
        }
        
        computations
    }
    
    /// 启动计算任务
    fn spawn_computation_task(&self, task: Arc<Mutex<AsyncTask<String>>>, expression: String) {
        thread::spawn(move || {
            // 开始任务
            if let Ok(mut t) = task.lock() {
                t.start();
            }
            
            // 模拟计算过程
            let result = Self::simulate_computation(&expression, &task);
            
            // 完成任务
            if let Ok(mut t) = task.lock() {
                t.complete(result);
            }
        });
    }
    
    /// 模拟计算过程
    fn simulate_computation(
        expression: &str, 
        task: &Arc<Mutex<AsyncTask<String>>>
    ) -> Result<String, YufmathError> {
        // 模拟计算步骤
        let steps = vec![
            ("解析表达式", 0.2),
            ("简化表达式", 0.5),
            ("计算结果", 0.8),
            ("格式化输出", 1.0),
        ];
        
        for (step_name, progress) in steps {
            // 检查是否被取消
            if let Ok(task_guard) = task.lock() {
                if task_guard.status == TaskStatus::Cancelled {
                    return Err(YufmathError::internal("计算被取消"));
                }
            }
            
            // 更新进度
            let progress_info = ComputeProgress::new(step_name)
                .with_progress(progress);
            
            if let Ok(mut task_guard) = task.lock() {
                task_guard.update_progress(progress_info);
            }
            
            // 模拟计算时间
            thread::sleep(Duration::from_millis(100));
        }
        
        // 返回模拟结果
        Ok(format!("计算结果: {}", expression))
    }
    
    /// 获取活跃任务数量
    pub fn active_task_count(&self) -> usize {
        if let Ok(active_tasks) = self.active_tasks.lock() {
            active_tasks.len()
        } else {
            0
        }
    }
    
    /// 取消所有任务
    pub fn cancel_all(&self) {
        if let Ok(active_tasks) = self.active_tasks.lock() {
            for task in active_tasks.iter() {
                if let Ok(mut t) = task.lock() {
                    t.cancel();
                }
            }
        }
    }
    
    /// 清理已完成的任务
    pub fn cleanup_completed(&self) {
        if let Ok(mut active_tasks) = self.active_tasks.lock() {
            active_tasks.retain(|task| {
                if let Ok(t) = task.lock() {
                    !matches!(t.status, TaskStatus::Completed | TaskStatus::Cancelled | TaskStatus::Error)
                } else {
                    false
                }
            });
        }
    }
}

/// 异步计算配置
#[derive(Debug, Clone)]
pub struct AsyncConfig {
    /// 最大并发任务数
    pub max_concurrent_tasks: usize,
    /// 任务超时时间
    pub task_timeout: Duration,
    /// 是否启用进度报告
    pub enable_progress: bool,
    /// 进度更新间隔
    pub progress_interval: Duration,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 4,
            task_timeout: Duration::from_secs(300), // 5分钟
            enable_progress: true,
            progress_interval: Duration::from_millis(100),
        }
    }
}

impl AsyncConfig {
    /// 创建新的异步配置
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置最大并发任务数
    pub fn with_max_concurrent_tasks(mut self, max_tasks: usize) -> Self {
        self.max_concurrent_tasks = max_tasks;
        self
    }
    
    /// 设置任务超时时间
    pub fn with_task_timeout(mut self, timeout: Duration) -> Self {
        self.task_timeout = timeout;
        self
    }
    
    /// 设置是否启用进度报告
    pub fn with_progress(mut self, enable: bool) -> Self {
        self.enable_progress = enable;
        self
    }
    
    /// 设置进度更新间隔
    pub fn with_progress_interval(mut self, interval: Duration) -> Self {
        self.progress_interval = interval;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_async_task_creation() {
        let task = AsyncTask::<String>::new(1);
        assert_eq!(task.id, 1);
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.result.is_none());
    }
    
    #[test]
    fn test_async_task_lifecycle() {
        let mut task = AsyncTask::<String>::new(1);
        
        // 开始任务
        task.start();
        assert_eq!(task.status, TaskStatus::Running);
        assert!(task.start_time.is_some());
        
        // 完成任务
        task.complete(Ok("test result".to_string()));
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.end_time.is_some());
        assert!(task.result.is_some());
    }
    
    #[test]
    fn test_batch_async_computer() {
        let computer = BatchAsyncComputer::new(2);
        let expressions = vec!["2+3".to_string(), "x^2".to_string()];
        
        let computations = computer.submit_batch(expressions);
        assert_eq!(computations.len(), 2);
        assert_eq!(computer.active_task_count(), 2);
    }
    
    #[test]
    fn test_async_config() {
        let config = AsyncConfig::new()
            .with_max_concurrent_tasks(8)
            .with_task_timeout(Duration::from_secs(600))
            .with_progress(false);
        
        assert_eq!(config.max_concurrent_tasks, 8);
        assert_eq!(config.task_timeout, Duration::from_secs(600));
        assert!(!config.enable_progress);
    }
}