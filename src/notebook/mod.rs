//! # 笔记本模式模块
//!
//! 提供类似 Mathematica 风格的笔记本界面，支持交互式数学计算和文档编写。
//!
//! ## 主要组件
//!
//! - `NotebookCell`: 单元格数据结构
//! - `Notebook`: 笔记本管理器
//! - `ExecutionEngine`: 单元格执行引擎
//! - `VariableScope`: 变量作用域管理
//! - `NotebookFormat`: 文件格式处理

pub mod cell;
pub mod notebook;
pub mod execution;
pub mod scope;
pub mod format;
pub mod ui;
pub mod gui;
pub mod autocomplete;
pub mod export;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod gui_tests;

pub use cell::{NotebookCell, CellType, CellContent, CellMetadata, CellId};
pub use notebook::{Notebook, NotebookManager, NotebookMetadata};
pub use execution::{ExecutionEngine, ExecutionResult, ExecutionContext, ExecutionQueue};
pub use scope::{VariableScope, ScopeManager, VariableBinding};
pub use format::{NotebookFormat, NotebookSerializer, NotebookDeserializer};
pub use ui::{NotebookUI, UIEvent, UICommand, KeyBinding};
pub use gui::{NotebookGUI, CellEditor};
pub use autocomplete::{AutoCompleteEngine, CompletionSuggestion, SuggestionType};
pub use export::{NotebookExporter, ExportFormat, ExportOptions};

use crate::engine::ComputeError;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use uuid::Uuid;

/// 笔记本错误类型
#[derive(Debug, thiserror::Error)]
pub enum NotebookError {
    #[error("单元格错误: {0}")]
    Cell(String),
    
    #[error("执行错误: {0}")]
    Execution(#[from] ComputeError),
    
    #[error("序列化错误: {0}")]
    Serialization(String),
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("格式错误: {0}")]
    Format(String),
    
    #[error("变量作用域错误: {0}")]
    Scope(String),
}

/// 笔记本结果类型
pub type NotebookResult<T> = Result<T, NotebookError>;

