//! # 笔记本单元格
//!
//! 定义笔记本中的单元格数据结构和相关操作。

use crate::core::Expression;
use crate::formatter::{FormatOptions, FormatType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

/// 单元格唯一标识符
pub type CellId = Uuid;

/// 单元格类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    /// 代码单元格 - 包含可执行的数学表达式
    Code,
    /// 文本单元格 - 包含纯文本内容
    Text,
    /// Markdown 单元格 - 包含 Markdown 格式的文档
    Markdown,
    /// 输出单元格 - 显示计算结果（只读）
    Output,
}

impl CellType {
    /// 获取单元格类型的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            CellType::Code => "代码",
            CellType::Text => "文本",
            CellType::Markdown => "Markdown",
            CellType::Output => "输出",
        }
    }
    
    /// 检查单元格类型是否可执行
    pub fn is_executable(&self) -> bool {
        matches!(self, CellType::Code)
    }
    
    /// 检查单元格类型是否可编辑
    pub fn is_editable(&self) -> bool {
        !matches!(self, CellType::Output)
    }
}

/// 单元格内容
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellContent {
    /// 原始文本内容
    Text(String),
    /// 解析后的表达式（仅用于代码单元格）
    Expression(Expression),
    /// 执行结果（仅用于输出单元格）
    Result {
        value: String,
        format: FormatType,
        execution_time: Option<std::time::Duration>,
    },
}

impl CellContent {
    /// 获取内容的文本表示
    pub fn as_text(&self) -> String {
        match self {
            CellContent::Text(text) => text.clone(),
            CellContent::Expression(expr) => format!("{:?}", expr),
            CellContent::Result { value, .. } => value.clone(),
        }
    }
    
    /// 检查内容是否为空
    pub fn is_empty(&self) -> bool {
        match self {
            CellContent::Text(text) => text.trim().is_empty(),
            CellContent::Expression(_) => false,
            CellContent::Result { value, .. } => value.trim().is_empty(),
        }
    }
    
    /// 获取内容长度
    pub fn len(&self) -> usize {
        self.as_text().len()
    }
}

/// 单元格元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellMetadata {
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后修改时间
    pub modified_at: SystemTime,
    /// 最后执行时间
    pub last_executed: Option<SystemTime>,
    /// 执行次数
    pub execution_count: u64,
    /// 是否已修改（需要重新执行）
    pub is_dirty: bool,
    /// 自定义标签
    pub tags: Vec<String>,
    /// 自定义属性
    pub properties: HashMap<String, String>,
}

impl Default for CellMetadata {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            created_at: now,
            modified_at: now,
            last_executed: None,
            execution_count: 0,
            is_dirty: false,
            tags: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

impl CellMetadata {
    /// 标记单元格为已修改
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.modified_at = SystemTime::now();
    }
    
    /// 标记单元格为已执行
    pub fn mark_executed(&mut self) {
        self.is_dirty = false;
        self.last_executed = Some(SystemTime::now());
        self.execution_count += 1;
    }
    
    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.mark_dirty();
        }
    }
    
    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.mark_dirty();
        }
    }
    
    /// 设置属性
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
        self.mark_dirty();
    }
    
    /// 获取属性
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

/// 笔记本单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookCell {
    /// 单元格唯一标识符
    pub id: CellId,
    /// 单元格类型
    pub cell_type: CellType,
    /// 单元格内容
    pub content: CellContent,
    /// 单元格元数据
    pub metadata: CellMetadata,
    /// 输出单元格（如果有）
    pub output: Option<Box<NotebookCell>>,
}

impl NotebookCell {
    /// 创建新的代码单元格
    pub fn new_code(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            cell_type: CellType::Code,
            content: CellContent::Text(content),
            metadata: CellMetadata::default(),
            output: None,
        }
    }
    
    /// 创建新的文本单元格
    pub fn new_text(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            cell_type: CellType::Text,
            content: CellContent::Text(content),
            metadata: CellMetadata::default(),
            output: None,
        }
    }
    
    /// 创建新的 Markdown 单元格
    pub fn new_markdown(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            cell_type: CellType::Markdown,
            content: CellContent::Text(content),
            metadata: CellMetadata::default(),
            output: None,
        }
    }
    
    /// 创建输出单元格
    pub fn new_output(value: String, format: FormatType, execution_time: Option<std::time::Duration>) -> Self {
        Self {
            id: Uuid::new_v4(),
            cell_type: CellType::Output,
            content: CellContent::Result { value, format, execution_time },
            metadata: CellMetadata::default(),
            output: None,
        }
    }
    
    /// 设置单元格内容
    pub fn set_content(&mut self, content: CellContent) {
        self.content = content;
        self.metadata.mark_dirty();
    }
    
    /// 设置文本内容
    pub fn set_text(&mut self, text: String) {
        self.content = CellContent::Text(text);
        self.metadata.mark_dirty();
    }
    
    /// 获取文本内容
    pub fn get_text(&self) -> String {
        self.content.as_text()
    }
    
    /// 检查单元格是否可执行
    pub fn is_executable(&self) -> bool {
        self.cell_type.is_executable()
    }
    
    /// 检查单元格是否可编辑
    pub fn is_editable(&self) -> bool {
        self.cell_type.is_editable()
    }
    
    /// 检查单元格是否需要重新执行
    pub fn needs_execution(&self) -> bool {
        self.metadata.is_dirty && self.is_executable()
    }
    
    /// 设置输出结果
    pub fn set_output(&mut self, output: NotebookCell) {
        self.output = Some(Box::new(output));
        self.metadata.mark_executed();
    }
    
    /// 清除输出结果
    pub fn clear_output(&mut self) {
        self.output = None;
    }
    
    /// 获取输出结果
    pub fn get_output(&self) -> Option<&NotebookCell> {
        self.output.as_ref().map(|b| b.as_ref())
    }
    
    /// 转换单元格类型
    pub fn convert_to(&mut self, new_type: CellType) {
        if self.cell_type != new_type {
            self.cell_type = new_type;
            self.metadata.mark_dirty();
            
            // 清除输出（如果不再是代码单元格）
            if !self.is_executable() {
                self.clear_output();
            }
        }
    }
    
    /// 复制单元格
    pub fn duplicate(&self) -> Self {
        let mut new_cell = self.clone();
        new_cell.id = Uuid::new_v4();
        new_cell.metadata = CellMetadata::default();
        new_cell.clear_output();
        new_cell
    }
    
    /// 获取单元格摘要信息
    pub fn summary(&self) -> String {
        let content_preview = {
            let text = self.get_text();
            if text.len() > 50 {
                format!("{}...", &text[..47])
            } else {
                text
            }
        };
        
        format!(
            "{} [{}] - {}",
            self.cell_type.display_name(),
            if self.metadata.is_dirty { "已修改" } else { "已保存" },
            content_preview
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cell_type_properties() {
        assert_eq!(CellType::Code.display_name(), "代码");
        assert!(CellType::Code.is_executable());
        assert!(CellType::Code.is_editable());
        
        assert_eq!(CellType::Output.display_name(), "输出");
        assert!(!CellType::Output.is_executable());
        assert!(!CellType::Output.is_editable());
    }
    
    #[test]
    fn test_cell_content() {
        let content = CellContent::Text("2 + 3".to_string());
        assert_eq!(content.as_text(), "2 + 3");
        assert!(!content.is_empty());
        assert_eq!(content.len(), 5);
        
        let empty_content = CellContent::Text("   ".to_string());
        assert!(empty_content.is_empty());
    }
    
    #[test]
    fn test_cell_metadata() {
        let mut metadata = CellMetadata::default();
        assert!(!metadata.is_dirty);
        assert_eq!(metadata.execution_count, 0);
        
        metadata.mark_dirty();
        assert!(metadata.is_dirty);
        
        metadata.mark_executed();
        assert!(!metadata.is_dirty);
        assert_eq!(metadata.execution_count, 1);
        assert!(metadata.last_executed.is_some());
    }
    
    #[test]
    fn test_notebook_cell_creation() {
        let cell = NotebookCell::new_code("x + y".to_string());
        assert!(cell.is_executable());
        assert!(cell.is_editable());
        assert_eq!(cell.get_text(), "x + y");
        
        let text_cell = NotebookCell::new_text("这是一个文本单元格".to_string());
        assert!(!text_cell.is_executable());
        assert!(text_cell.is_editable());
    }
    
    #[test]
    fn test_cell_output() {
        let mut cell = NotebookCell::new_code("2 + 3".to_string());
        assert!(cell.get_output().is_none());
        
        let output = NotebookCell::new_output(
            "5".to_string(), 
            FormatType::Standard, 
            Some(std::time::Duration::from_millis(10))
        );
        
        cell.set_output(output);
        assert!(cell.get_output().is_some());
        assert_eq!(cell.get_output().unwrap().get_text(), "5");
    }
    
    #[test]
    fn test_cell_conversion() {
        let mut cell = NotebookCell::new_code("# 标题".to_string());
        assert_eq!(cell.cell_type, CellType::Code);
        
        cell.convert_to(CellType::Markdown);
        assert_eq!(cell.cell_type, CellType::Markdown);
        assert!(cell.metadata.is_dirty);
    }
    
    #[test]
    fn test_cell_duplication() {
        let original = NotebookCell::new_code("原始内容".to_string());
        let duplicate = original.duplicate();
        
        assert_ne!(original.id, duplicate.id);
        assert_eq!(original.get_text(), duplicate.get_text());
        assert_eq!(duplicate.metadata.execution_count, 0);
    }
    
    #[test]
    fn test_cell_tags_and_properties() {
        let mut metadata = CellMetadata::default();
        
        metadata.add_tag("重要".to_string());
        metadata.add_tag("数学".to_string());
        assert_eq!(metadata.tags.len(), 2);
        
        metadata.remove_tag("重要");
        assert_eq!(metadata.tags.len(), 1);
        assert_eq!(metadata.tags[0], "数学");
        
        metadata.set_property("难度".to_string(), "中等".to_string());
        assert_eq!(metadata.get_property("难度"), Some(&"中等".to_string()));
    }
}