//! # 笔记本管理器
//!
//! 提供笔记本的创建、管理和操作功能。

use super::{NotebookCell, CellId, CellType, NotebookError, NotebookResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use uuid::Uuid;

/// 笔记本唯一标识符
pub type NotebookId = Uuid;

/// 笔记本元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookMetadata {
    /// 笔记本标题
    pub title: String,
    /// 作者
    pub author: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: SystemTime,
    /// 最后修改时间
    pub modified_at: SystemTime,
    /// 最后保存时间
    pub last_saved: Option<SystemTime>,
    /// 版本号
    pub version: String,
    /// 语言设置
    pub language: String,
    /// 自定义属性
    pub properties: HashMap<String, String>,
    /// 标签
    pub tags: Vec<String>,
}

impl Default for NotebookMetadata {
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            title: "未命名笔记本".to_string(),
            author: None,
            description: None,
            created_at: now,
            modified_at: now,
            last_saved: None,
            version: "1.0".to_string(),
            language: "zh-CN".to_string(),
            properties: HashMap::new(),
            tags: Vec::new(),
        }
    }
}

impl NotebookMetadata {
    /// 标记为已修改
    pub fn mark_modified(&mut self) {
        self.modified_at = SystemTime::now();
    }
    
    /// 标记为已保存
    pub fn mark_saved(&mut self) {
        self.last_saved = Some(SystemTime::now());
    }
    
    /// 检查是否需要保存
    pub fn needs_save(&self) -> bool {
        match self.last_saved {
            Some(saved_time) => self.modified_at > saved_time,
            None => true,
        }
    }
    
    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.mark_modified();
        }
    }
    
    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        if let Some(pos) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(pos);
            self.mark_modified();
        }
    }
}

/// 笔记本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    /// 笔记本唯一标识符
    pub id: NotebookId,
    /// 元数据
    pub metadata: NotebookMetadata,
    /// 单元格列表（按顺序）
    pub cells: Vec<NotebookCell>,
    /// 文件路径（如果已保存）
    pub file_path: Option<PathBuf>,
}

impl Notebook {
    /// 创建新的空笔记本
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            metadata: NotebookMetadata::default(),
            cells: Vec::new(),
            file_path: None,
        }
    }
    
    /// 创建带标题的笔记本
    pub fn with_title(title: String) -> Self {
        let mut notebook = Self::new();
        notebook.metadata.title = title;
        notebook
    }
    
    /// 获取单元格数量
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
    
    /// 检查笔记本是否为空
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
    
    /// 添加单元格到末尾
    pub fn add_cell(&mut self, cell: NotebookCell) {
        self.cells.push(cell);
        self.metadata.mark_modified();
    }
    
    /// 在指定位置插入单元格
    pub fn insert_cell(&mut self, index: usize, cell: NotebookCell) -> NotebookResult<()> {
        if index > self.cells.len() {
            return Err(NotebookError::Cell(format!("索引 {} 超出范围", index)));
        }
        
        self.cells.insert(index, cell);
        self.metadata.mark_modified();
        Ok(())
    }
    
    /// 移除指定位置的单元格
    pub fn remove_cell(&mut self, index: usize) -> NotebookResult<NotebookCell> {
        if index >= self.cells.len() {
            return Err(NotebookError::Cell(format!("索引 {} 超出范围", index)));
        }
        
        let cell = self.cells.remove(index);
        self.metadata.mark_modified();
        Ok(cell)
    }
    
    /// 根据 ID 查找单元格
    pub fn find_cell(&self, cell_id: &CellId) -> Option<(usize, &NotebookCell)> {
        self.cells
            .iter()
            .enumerate()
            .find(|(_, cell)| &cell.id == cell_id)
    }
    
    /// 根据 ID 查找可变单元格
    pub fn find_cell_mut(&mut self, cell_id: &CellId) -> Option<(usize, &mut NotebookCell)> {
        self.cells
            .iter_mut()
            .enumerate()
            .find(|(_, cell)| &cell.id == cell_id)
    }
    
    /// 获取指定位置的单元格
    pub fn get_cell(&self, index: usize) -> Option<&NotebookCell> {
        self.cells.get(index)
    }
    
    /// 获取指定位置的可变单元格
    pub fn get_cell_mut(&mut self, index: usize) -> Option<&mut NotebookCell> {
        if index < self.cells.len() {
            self.metadata.mark_modified();
        }
        self.cells.get_mut(index)
    }
    
    /// 移动单元格位置
    pub fn move_cell(&mut self, from: usize, to: usize) -> NotebookResult<()> {
        if from >= self.cells.len() || to >= self.cells.len() {
            return Err(NotebookError::Cell("索引超出范围".to_string()));
        }
        
        if from != to {
            let cell = self.cells.remove(from);
            self.cells.insert(to, cell);
            self.metadata.mark_modified();
        }
        
        Ok(())
    }
    
    /// 复制单元格
    pub fn duplicate_cell(&mut self, index: usize) -> NotebookResult<()> {
        if index >= self.cells.len() {
            return Err(NotebookError::Cell(format!("索引 {} 超出范围", index)));
        }
        
        let duplicated = self.cells[index].duplicate();
        self.cells.insert(index + 1, duplicated);
        self.metadata.mark_modified();
        Ok(())
    }
    
    /// 获取所有代码单元格
    pub fn get_code_cells(&self) -> Vec<(usize, &NotebookCell)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.cell_type == CellType::Code)
            .collect()
    }
    
    /// 获取需要执行的单元格
    pub fn get_dirty_cells(&self) -> Vec<(usize, &NotebookCell)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.needs_execution())
            .collect()
    }
    
    /// 清除所有输出
    pub fn clear_all_outputs(&mut self) {
        for cell in &mut self.cells {
            if cell.is_executable() {
                cell.clear_output();
            }
        }
        self.metadata.mark_modified();
    }
    
    /// 获取笔记本统计信息
    pub fn statistics(&self) -> NotebookStatistics {
        let mut stats = NotebookStatistics::default();
        
        for cell in &self.cells {
            match cell.cell_type {
                CellType::Code => {
                    stats.code_cells += 1;
                    if cell.needs_execution() {
                        stats.dirty_cells += 1;
                    }
                    if cell.get_output().is_some() {
                        stats.executed_cells += 1;
                    }
                }
                CellType::Text => stats.text_cells += 1,
                CellType::Markdown => stats.markdown_cells += 1,
                CellType::Output => stats.output_cells += 1,
            }
            
            stats.total_characters += cell.get_text().len();
        }
        
        stats.total_cells = self.cells.len();
        stats
    }
    
    /// 搜索单元格内容
    pub fn search(&self, query: &str, case_sensitive: bool) -> Vec<(usize, Vec<usize>)> {
        let mut results = Vec::new();
        
        for (index, cell) in self.cells.iter().enumerate() {
            let content = cell.get_text();
            let search_content = if case_sensitive {
                content.clone()
            } else {
                content.to_lowercase()
            };
            
            let search_query = if case_sensitive {
                query.to_string()
            } else {
                query.to_lowercase()
            };
            
            let mut matches = Vec::new();
            let mut start = 0;
            
            while let Some(pos) = search_content[start..].find(&search_query) {
                matches.push(start + pos);
                start += pos + search_query.len();
            }
            
            if !matches.is_empty() {
                results.push((index, matches));
            }
        }
        
        results
    }
    
    /// 设置文件路径
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }
    
    /// 获取文件路径
    pub fn get_file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }
    
    /// 检查是否需要保存
    pub fn needs_save(&self) -> bool {
        self.metadata.needs_save()
    }
    
    /// 标记为已保存
    pub fn mark_saved(&mut self) {
        self.metadata.mark_saved();
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

/// 笔记本统计信息
#[derive(Debug, Default, Clone)]
pub struct NotebookStatistics {
    pub total_cells: usize,
    pub code_cells: usize,
    pub text_cells: usize,
    pub markdown_cells: usize,
    pub output_cells: usize,
    pub executed_cells: usize,
    pub dirty_cells: usize,
    pub total_characters: usize,
}

impl NotebookStatistics {
    /// 获取执行完成率
    pub fn execution_rate(&self) -> f64 {
        if self.code_cells == 0 {
            1.0
        } else {
            self.executed_cells as f64 / self.code_cells as f64
        }
    }
    
    /// 获取平均单元格长度
    pub fn average_cell_length(&self) -> f64 {
        if self.total_cells == 0 {
            0.0
        } else {
            self.total_characters as f64 / self.total_cells as f64
        }
    }
}

/// 笔记本管理器
pub struct NotebookManager {
    /// 当前打开的笔记本
    notebooks: HashMap<NotebookId, Notebook>,
    /// 活动笔记本 ID
    active_notebook: Option<NotebookId>,
}

impl NotebookManager {
    /// 创建新的笔记本管理器
    pub fn new() -> Self {
        Self {
            notebooks: HashMap::new(),
            active_notebook: None,
        }
    }
    
    /// 创建新笔记本
    pub fn create_notebook(&mut self, title: Option<String>) -> NotebookId {
        let notebook = match title {
            Some(title) => Notebook::with_title(title),
            None => Notebook::new(),
        };
        
        let id = notebook.id;
        self.notebooks.insert(id, notebook);
        self.active_notebook = Some(id);
        id
    }
    
    /// 打开笔记本
    pub fn open_notebook(&mut self, notebook: Notebook) -> NotebookId {
        let id = notebook.id;
        self.notebooks.insert(id, notebook);
        self.active_notebook = Some(id);
        id
    }
    
    /// 关闭笔记本
    pub fn close_notebook(&mut self, notebook_id: &NotebookId) -> NotebookResult<Notebook> {
        let notebook = self.notebooks.remove(notebook_id)
            .ok_or_else(|| NotebookError::Cell("笔记本不存在".to_string()))?;
        
        if self.active_notebook == Some(*notebook_id) {
            self.active_notebook = self.notebooks.keys().next().copied();
        }
        
        Ok(notebook)
    }
    
    /// 获取活动笔记本
    pub fn get_active_notebook(&self) -> Option<&Notebook> {
        self.active_notebook
            .and_then(|id| self.notebooks.get(&id))
    }
    
    /// 获取活动笔记本（可变）
    pub fn get_active_notebook_mut(&mut self) -> Option<&mut Notebook> {
        self.active_notebook
            .and_then(|id| self.notebooks.get_mut(&id))
    }
    
    /// 设置活动笔记本
    pub fn set_active_notebook(&mut self, notebook_id: &NotebookId) -> NotebookResult<()> {
        if self.notebooks.contains_key(notebook_id) {
            self.active_notebook = Some(*notebook_id);
            Ok(())
        } else {
            Err(NotebookError::Cell("笔记本不存在".to_string()))
        }
    }
    
    /// 获取指定笔记本
    pub fn get_notebook(&self, notebook_id: &NotebookId) -> Option<&Notebook> {
        self.notebooks.get(notebook_id)
    }
    
    /// 获取指定笔记本（可变）
    pub fn get_notebook_mut(&mut self, notebook_id: &NotebookId) -> Option<&mut Notebook> {
        self.notebooks.get_mut(notebook_id)
    }
    
    /// 获取所有笔记本 ID
    pub fn get_notebook_ids(&self) -> Vec<NotebookId> {
        self.notebooks.keys().copied().collect()
    }
    
    /// 获取笔记本数量
    pub fn notebook_count(&self) -> usize {
        self.notebooks.len()
    }
    
    /// 检查是否有未保存的笔记本
    pub fn has_unsaved_notebooks(&self) -> bool {
        self.notebooks.values().any(|nb| nb.needs_save())
    }
    
    /// 获取未保存的笔记本列表
    pub fn get_unsaved_notebooks(&self) -> Vec<&Notebook> {
        self.notebooks.values().filter(|nb| nb.needs_save()).collect()
    }
}

impl Default for NotebookManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notebook::CellContent;
    
    #[test]
    fn test_notebook_metadata() {
        let mut metadata = NotebookMetadata::default();
        assert_eq!(metadata.title, "未命名笔记本");
        assert!(metadata.needs_save());
        
        metadata.mark_saved();
        assert!(!metadata.needs_save());
        
        metadata.mark_modified();
        assert!(metadata.needs_save());
    }
    
    #[test]
    fn test_notebook_creation() {
        let notebook = Notebook::new();
        assert!(notebook.is_empty());
        assert_eq!(notebook.cell_count(), 0);
        
        let titled_notebook = Notebook::with_title("测试笔记本".to_string());
        assert_eq!(titled_notebook.metadata.title, "测试笔记本");
    }
    
    #[test]
    fn test_notebook_cell_operations() {
        let mut notebook = Notebook::new();
        
        // 添加单元格
        let cell1 = NotebookCell::new_code("2 + 3".to_string());
        let cell1_id = cell1.id;
        notebook.add_cell(cell1);
        assert_eq!(notebook.cell_count(), 1);
        
        // 插入单元格
        let cell2 = NotebookCell::new_text("文本内容".to_string());
        notebook.insert_cell(0, cell2).unwrap();
        assert_eq!(notebook.cell_count(), 2);
        
        // 查找单元格
        let (index, _) = notebook.find_cell(&cell1_id).unwrap();
        assert_eq!(index, 1); // 现在在位置 1
        
        // 移动单元格
        notebook.move_cell(1, 0).unwrap();
        let (index, _) = notebook.find_cell(&cell1_id).unwrap();
        assert_eq!(index, 0); // 现在在位置 0
        
        // 复制单元格
        notebook.duplicate_cell(0).unwrap();
        assert_eq!(notebook.cell_count(), 3);
        
        // 移除单元格
        let removed = notebook.remove_cell(0).unwrap();
        assert_eq!(removed.id, cell1_id);
        assert_eq!(notebook.cell_count(), 2);
    }
    
    #[test]
    fn test_notebook_statistics() {
        let mut notebook = Notebook::new();
        
        notebook.add_cell(NotebookCell::new_code("x + y".to_string()));
        notebook.add_cell(NotebookCell::new_text("说明文本".to_string()));
        notebook.add_cell(NotebookCell::new_markdown("# 标题".to_string()));
        
        let stats = notebook.statistics();
        assert_eq!(stats.total_cells, 3);
        assert_eq!(stats.code_cells, 1);
        assert_eq!(stats.text_cells, 1);
        assert_eq!(stats.markdown_cells, 1);
        assert_eq!(stats.dirty_cells, 1); // 代码单元格默认为 dirty
    }
    
    #[test]
    fn test_notebook_search() {
        let mut notebook = Notebook::new();
        
        notebook.add_cell(NotebookCell::new_code("x + y = z".to_string()));
        notebook.add_cell(NotebookCell::new_text("这是一个测试".to_string()));
        notebook.add_cell(NotebookCell::new_code("y * 2".to_string()));
        
        // 搜索 "y"
        let results = notebook.search("y", true);
        assert_eq!(results.len(), 2); // 在两个单元格中找到
        assert_eq!(results[0].0, 0); // 第一个单元格
        assert_eq!(results[1].0, 2); // 第三个单元格
        
        // 搜索 "测试"
        let results = notebook.search("测试", true);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, 1); // 第二个单元格
    }
    
    #[test]
    fn test_notebook_manager() {
        let mut manager = NotebookManager::new();
        assert_eq!(manager.notebook_count(), 0);
        assert!(manager.get_active_notebook().is_none());
        
        // 创建笔记本
        let id1 = manager.create_notebook(Some("笔记本1".to_string()));
        assert_eq!(manager.notebook_count(), 1);
        assert!(manager.get_active_notebook().is_some());
        
        let id2 = manager.create_notebook(Some("笔记本2".to_string()));
        assert_eq!(manager.notebook_count(), 2);
        
        // 切换活动笔记本
        manager.set_active_notebook(&id1).unwrap();
        assert_eq!(manager.get_active_notebook().unwrap().metadata.title, "笔记本1");
        
        // 关闭笔记本
        let closed = manager.close_notebook(&id1).unwrap();
        assert_eq!(closed.metadata.title, "笔记本1");
        assert_eq!(manager.notebook_count(), 1);
    }
}