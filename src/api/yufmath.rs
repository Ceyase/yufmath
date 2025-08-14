//! # Yufmath 主要 API 接口
//!
//! 提供 Yufmath 库的主要入口点和核心功能。

use std::collections::HashMap;
use crate::core::{Expression, Number};
use crate::parser::{Parser, ParseError};
use crate::engine::{ComputeEngine, ComputeError};
use crate::formatter::{Formatter, FormatOptions};
use super::{YufmathError, ComputeConfig, PerformanceMonitor};

/// Yufmath 库的主要入口点
pub struct Yufmath {
    parser: Box<dyn Parser>,
    engine: Box<dyn ComputeEngine>,
    formatter: Box<dyn Formatter>,
    monitor: PerformanceMonitor,
    config: ComputeConfig,
}

impl Yufmath {
    /// 创建新的 Yufmath 实例
    pub fn new() -> Self {
        // 这里暂时使用占位符实现，后续任务会实现具体的组件
        todo!("将在后续任务中实现具体的解析器、引擎和格式化器")
    }
    
    /// 创建带配置的 Yufmath 实例
    pub fn with_config(config: ComputeConfig) -> Self {
        // 这里暂时使用占位符实现
        todo!("将在后续任务中实现")
    }
    
    /// 解析并计算表达式
    pub fn compute(&self, input: &str) -> Result<String, YufmathError> {
        let expr = self.parser.parse(input)?;
        let simplified = self.engine.simplify(&expr)?;
        Ok(self.formatter.format(&simplified))
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
    
    /// 积分
    pub fn integrate(&self, expr: &Expression, var: &str) -> Result<Expression, YufmathError> {
        Ok(self.engine.integrate(expr, var)?)
    }
    
    /// 计算表达式的数值
    pub fn evaluate(&self, expr: &Expression, vars: &HashMap<String, Number>) -> Result<Number, YufmathError> {
        Ok(self.engine.evaluate(expr, vars)?)
    }
    
    /// 设置格式化选项
    pub fn set_format_options(&mut self, options: FormatOptions) {
        self.formatter.set_options(options);
    }
    
    /// 获取性能统计信息
    pub fn get_performance_stats(&self) -> &crate::api::PerformanceStats {
        self.monitor.get_stats()
    }
}

impl Default for Yufmath {
    fn default() -> Self {
        Self::new()
    }
}