//! # Yufmath 主要 API 接口
//!
//! 提供 Yufmath 库的主要入口点和核心功能。

use std::collections::HashMap;
use crate::core::{Expression, Number};
use crate::parser::{Parser, ParseError};
use crate::engine::{ComputeEngine, ComputeError, compute::BasicComputeEngine};
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
        // 临时实现，仅用于测试求导功能
        Self {
            parser: Box::new(DummyParser),
            engine: Box::new(BasicComputeEngine::new()),
            formatter: Box::new(DummyFormatter),
            monitor: PerformanceMonitor::new(),
            config: ComputeConfig::default(),
        }
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
// 临时实现，仅用于测试
struct DummyParser;

impl Parser for DummyParser {
    fn parse(&self, _input: &str) -> Result<Expression, crate::parser::ParseError> {
        // 临时实现，仅返回错误
        Err(crate::parser::ParseError::Syntax { 
            pos: 0, 
            message: "临时解析器未实现".to_string() 
        })
    }
    
    fn validate(&self, _input: &str) -> Result<(), crate::parser::ParseError> {
        Ok(())
    }
}

struct DummyFormatter;

impl Formatter for DummyFormatter {
    fn format(&self, _expr: &Expression) -> String {
        "临时格式化器未实现".to_string()
    }
    
    fn set_options(&mut self, _options: FormatOptions) {
        // 临时实现，什么都不做
    }
}