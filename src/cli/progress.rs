//! # 进度条显示模块
//!
//! 提供命令行进度条显示功能。

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// 进度条管理器
pub struct ProgressManager {
    bar: Option<ProgressBar>,
    enabled: bool,
}

impl ProgressManager {
    /// 创建新的进度条管理器
    pub fn new(enabled: bool) -> Self {
        Self {
            bar: None,
            enabled,
        }
    }
    
    /// 开始显示进度条
    pub fn start(&mut self, message: &str, total_steps: Option<u64>) {
        if !self.enabled {
            return;
        }
        
        let bar = if let Some(total) = total_steps {
            ProgressBar::new(total)
        } else {
            ProgressBar::new_spinner()
        };
        
        // 设置进度条样式
        if total_steps.is_some() {
            bar.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-")
            );
        } else {
            bar.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {msg}")
                    .unwrap()
            );
        }
        
        bar.set_message(message.to_string());
        bar.enable_steady_tick(Duration::from_millis(100));
        
        self.bar = Some(bar);
    }
    
    /// 更新进度
    pub fn update(&self, step: u64, message: Option<&str>) {
        if let Some(ref bar) = self.bar {
            bar.set_position(step);
            if let Some(msg) = message {
                bar.set_message(msg.to_string());
            }
        }
    }
    
    /// 增加进度
    pub fn increment(&self, delta: u64) {
        if let Some(ref bar) = self.bar {
            bar.inc(delta);
        }
    }
    
    /// 设置消息
    pub fn set_message(&self, message: &str) {
        if let Some(ref bar) = self.bar {
            bar.set_message(message.to_string());
        }
    }
    
    /// 完成进度条
    pub fn finish(&self, message: &str) {
        if let Some(ref bar) = self.bar {
            bar.finish_with_message(message.to_string());
        }
    }
    
    /// 完成并清除进度条
    pub fn finish_and_clear(&self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
    
    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Drop for ProgressManager {
    fn drop(&mut self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
}

/// 创建计算进度条
pub fn create_compute_progress(enabled: bool, operation: &str) -> ProgressManager {
    let mut progress = ProgressManager::new(enabled);
    if enabled {
        progress.start(&format!("正在{}...", operation), None);
    }
    progress
}

/// 创建批处理进度条
pub fn create_batch_progress(enabled: bool, total_lines: u64) -> ProgressManager {
    let mut progress = ProgressManager::new(enabled);
    if enabled {
        progress.start("正在处理批处理文件", Some(total_lines));
    }
    progress
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_progress_manager_creation() {
        let progress = ProgressManager::new(true);
        assert!(progress.is_enabled());
        
        let progress = ProgressManager::new(false);
        assert!(!progress.is_enabled());
    }
    
    #[test]
    fn test_progress_manager_disabled() {
        let mut progress = ProgressManager::new(false);
        progress.start("测试", Some(100));
        progress.update(50, Some("进行中"));
        progress.finish("完成");
        // 禁用状态下不应该有任何输出
    }
    
    #[test]
    fn test_create_compute_progress() {
        let progress = create_compute_progress(false, "计算");
        assert!(!progress.is_enabled());
    }
    
    #[test]
    fn test_create_batch_progress() {
        let progress = create_batch_progress(false, 100);
        assert!(!progress.is_enabled());
    }
}