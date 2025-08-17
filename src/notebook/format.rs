//! # 笔记本文件格式
//!
//! 处理 .ynb (Yufmath Notebook) 文件格式的序列化和反序列化。

use super::{Notebook, NotebookError, NotebookResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 笔记本文件格式版本
pub const NOTEBOOK_FORMAT_VERSION: &str = "1.0";

/// 笔记本文件扩展名
pub const NOTEBOOK_EXTENSION: &str = "ynb";

/// 笔记本序列化器
pub struct NotebookSerializer;

impl NotebookSerializer {
    /// 将笔记本序列化为 TOML 字符串
    pub fn serialize(notebook: &Notebook) -> NotebookResult<String> {
        // 创建序列化包装器
        let wrapper = NotebookWrapper {
            format_version: NOTEBOOK_FORMAT_VERSION.to_string(),
            notebook: notebook.clone(),
        };
        
        toml::to_string_pretty(&wrapper)
            .map_err(|e| NotebookError::Serialization(format!("序列化失败: {}", e)))
    }
    
    /// 将笔记本保存到文件
    pub fn save_to_file<P: AsRef<Path>>(notebook: &mut Notebook, path: P) -> NotebookResult<()> {
        let path = path.as_ref();
        
        // 确保文件扩展名正确
        if path.extension().and_then(|s| s.to_str()) != Some(NOTEBOOK_EXTENSION) {
            return Err(NotebookError::Format(
                format!("文件扩展名必须是 .{}", NOTEBOOK_EXTENSION)
            ));
        }
        
        let content = Self::serialize(notebook)?;
        
        fs::write(path, content)
            .map_err(|e| NotebookError::Io(e))?;
        
        // 更新笔记本的文件路径和保存状态
        notebook.set_file_path(path.to_path_buf());
        notebook.mark_saved();
        
        Ok(())
    }
    
    /// 创建备份文件
    pub fn create_backup<P: AsRef<Path>>(notebook: &Notebook, original_path: P) -> NotebookResult<()> {
        let original_path = original_path.as_ref();
        let backup_path = original_path.with_extension(format!("{}.backup", NOTEBOOK_EXTENSION));
        
        let content = Self::serialize(notebook)?;
        fs::write(backup_path, content)
            .map_err(|e| NotebookError::Io(e))?;
        
        Ok(())
    }
}

/// 笔记本反序列化器
pub struct NotebookDeserializer;

impl NotebookDeserializer {
    /// 从 TOML 字符串反序列化笔记本
    pub fn deserialize(content: &str) -> NotebookResult<Notebook> {
        let wrapper: NotebookWrapper = toml::from_str(content)
            .map_err(|e| NotebookError::Serialization(format!("反序列化失败: {}", e)))?;
        
        // 检查格式版本兼容性
        Self::check_version_compatibility(&wrapper.format_version)?;
        
        Ok(wrapper.notebook)
    }
    
    /// 从文件加载笔记本
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> NotebookResult<Notebook> {
        let path = path.as_ref();
        
        // 检查文件扩展名
        if path.extension().and_then(|s| s.to_str()) != Some(NOTEBOOK_EXTENSION) {
            return Err(NotebookError::Format(
                format!("不支持的文件格式，期望 .{} 文件", NOTEBOOK_EXTENSION)
            ));
        }
        
        // 检查文件是否存在
        if !path.exists() {
            return Err(NotebookError::Io(
                std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在")
            ));
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| NotebookError::Io(e))?;
        
        let mut notebook = Self::deserialize(&content)?;
        notebook.set_file_path(path.to_path_buf());
        
        Ok(notebook)
    }
    
    /// 检查版本兼容性
    fn check_version_compatibility(version: &str) -> NotebookResult<()> {
        // 简单的版本检查，实际实现可能需要更复杂的版本比较
        let supported_versions = ["1.0"];
        
        if supported_versions.contains(&version) {
            Ok(())
        } else {
            Err(NotebookError::Format(
                format!("不支持的笔记本格式版本: {}，支持的版本: {:?}", 
                       version, supported_versions)
            ))
        }
    }
    
    /// 尝试修复损坏的笔记本文件
    pub fn repair_notebook(content: &str) -> NotebookResult<Notebook> {
        // 尝试正常反序列化
        if let Ok(notebook) = Self::deserialize(content) {
            return Ok(notebook);
        }
        
        // 如果失败，尝试修复常见问题
        let repaired_content = Self::repair_common_issues(content);
        
        Self::deserialize(&repaired_content)
            .map_err(|_| NotebookError::Format("无法修复损坏的笔记本文件".to_string()))
    }
    
    /// 修复常见的格式问题
    fn repair_common_issues(content: &str) -> String {
        let mut repaired = content.to_string();
        
        // 修复常见的 TOML 格式问题
        // 例如：缺少引号、转义字符问题等
        
        // 这里可以添加更多的修复逻辑
        repaired
    }
}

/// 笔记本格式处理器
pub struct NotebookFormat;

impl NotebookFormat {
    /// 验证笔记本文件格式
    pub fn validate_file<P: AsRef<Path>>(path: P) -> NotebookResult<()> {
        let path = path.as_ref();
        
        // 检查文件扩展名
        if path.extension().and_then(|s| s.to_str()) != Some(NOTEBOOK_EXTENSION) {
            return Err(NotebookError::Format("文件扩展名不正确".to_string()));
        }
        
        // 尝试加载文件
        NotebookDeserializer::load_from_file(path)?;
        
        Ok(())
    }
    
    /// 获取笔记本文件信息
    pub fn get_file_info<P: AsRef<Path>>(path: P) -> NotebookResult<NotebookFileInfo> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)
            .map_err(|e| NotebookError::Io(e))?;
        
        let notebook = NotebookDeserializer::load_from_file(path)?;
        let stats = notebook.statistics();
        
        Ok(NotebookFileInfo {
            file_path: path.to_path_buf(),
            file_size: metadata.len(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
            title: notebook.metadata.title,
            cell_count: stats.total_cells,
            code_cells: stats.code_cells,
            format_version: NOTEBOOK_FORMAT_VERSION.to_string(),
        })
    }
    
    /// 转换旧版本笔记本格式
    pub fn migrate_format(old_content: &str, from_version: &str) -> NotebookResult<String> {
        match from_version {
            "0.9" => Self::migrate_from_0_9(old_content),
            _ => Err(NotebookError::Format(
                format!("不支持从版本 {} 迁移", from_version)
            )),
        }
    }
    
    /// 从 0.9 版本迁移
    fn migrate_from_0_9(content: &str) -> NotebookResult<String> {
        // 实现从旧版本的迁移逻辑
        // 这里是示例实现
        Ok(content.to_string())
    }
    
    /// 创建默认笔记本模板
    pub fn create_template(title: &str) -> Notebook {
        let mut notebook = Notebook::with_title(title.to_string());
        
        // 添加欢迎单元格
        let welcome_cell = crate::NotebookCell::new_markdown(format!(
            "# {}\n\n欢迎使用 Yufmath 笔记本！\n\n这是一个交互式数学计算环境。",
            title
        ));
        notebook.add_cell(welcome_cell);
        
        // 添加示例代码单元格
        let example_cell = crate::NotebookCell::new_code("2 + 3".to_string());
        notebook.add_cell(example_cell);
        
        notebook
    }
}

/// 笔记本序列化包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NotebookWrapper {
    /// 格式版本
    format_version: String,
    /// 笔记本数据
    notebook: Notebook,
}

/// 笔记本文件信息
#[derive(Debug, Clone)]
pub struct NotebookFileInfo {
    /// 文件路径
    pub file_path: std::path::PathBuf,
    /// 文件大小（字节）
    pub file_size: u64,
    /// 创建时间
    pub created: Option<std::time::SystemTime>,
    /// 修改时间
    pub modified: Option<std::time::SystemTime>,
    /// 笔记本标题
    pub title: String,
    /// 单元格总数
    pub cell_count: usize,
    /// 代码单元格数
    pub code_cells: usize,
    /// 格式版本
    pub format_version: String,
}

impl NotebookFileInfo {
    /// 获取文件大小的人类可读格式
    pub fn human_readable_size(&self) -> String {
        let size = self.file_size as f64;
        
        if size < 1024.0 {
            format!("{} B", size)
        } else if size < 1024.0 * 1024.0 {
            format!("{:.1} KB", size / 1024.0)
        } else if size < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB", size / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size / (1024.0 * 1024.0 * 1024.0))
        }
    }
    
    /// 获取文件摘要
    pub fn summary(&self) -> String {
        format!(
            "{} - {} 个单元格 ({} 代码) - {}",
            self.title,
            self.cell_count,
            self.code_cells,
            self.human_readable_size()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_notebook_serialization() {
        let notebook = Notebook::with_title("测试笔记本".to_string());
        
        let serialized = NotebookSerializer::serialize(&notebook).unwrap();
        assert!(serialized.contains("测试笔记本"));
        assert!(serialized.contains("format_version"));
        
        let deserialized = NotebookDeserializer::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.metadata.title, "测试笔记本");
    }
    
    #[test]
    fn test_file_operations() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.ynb");
        
        let mut notebook = Notebook::with_title("文件测试".to_string());
        notebook.add_cell(crate::NotebookCell::new_code("x = 42".to_string()));
        
        // 保存文件
        NotebookSerializer::save_to_file(&mut notebook, &file_path).unwrap();
        assert!(file_path.exists());
        assert!(notebook.get_file_path().is_some());
        assert!(!notebook.needs_save());
        
        // 加载文件
        let loaded = NotebookDeserializer::load_from_file(&file_path).unwrap();
        assert_eq!(loaded.metadata.title, "文件测试");
        assert_eq!(loaded.cell_count(), 1);
    }
    
    #[test]
    fn test_file_validation() {
        let dir = tempdir().unwrap();
        
        // 测试错误的扩展名
        let wrong_ext = dir.path().join("test.txt");
        fs::write(&wrong_ext, "content").unwrap();
        
        let result = NotebookFormat::validate_file(&wrong_ext);
        assert!(result.is_err());
        
        // 测试正确的文件
        let correct_file = dir.path().join("test.ynb");
        let notebook = Notebook::with_title("验证测试".to_string());
        let content = NotebookSerializer::serialize(&notebook).unwrap();
        fs::write(&correct_file, content).unwrap();
        
        let result = NotebookFormat::validate_file(&correct_file);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_file_info() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("info_test.ynb");
        
        let mut notebook = Notebook::with_title("信息测试".to_string());
        notebook.add_cell(crate::NotebookCell::new_code("1 + 1".to_string()));
        notebook.add_cell(crate::NotebookCell::new_text("说明".to_string()));
        
        NotebookSerializer::save_to_file(&mut notebook, &file_path).unwrap();
        
        let info = NotebookFormat::get_file_info(&file_path).unwrap();
        assert_eq!(info.title, "信息测试");
        assert_eq!(info.cell_count, 2);
        assert_eq!(info.code_cells, 1);
        assert!(info.file_size > 0);
    }
    
    #[test]
    fn test_template_creation() {
        let template = NotebookFormat::create_template("模板测试");
        assert_eq!(template.metadata.title, "模板测试");
        assert_eq!(template.cell_count(), 2); // 欢迎单元格 + 示例代码单元格
        
        let cells: Vec<_> = template.cells.iter().collect();
        assert_eq!(cells[0].cell_type, crate::CellType::Markdown);
        assert_eq!(cells[1].cell_type, crate::CellType::Code);
    }
    
    #[test]
    fn test_backup_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("backup_test.ynb");
        let backup_path = dir.path().join("backup_test.ynb.backup");
        
        let notebook = Notebook::with_title("备份测试".to_string());
        
        NotebookSerializer::create_backup(&notebook, &file_path).unwrap();
        assert!(backup_path.exists());
        
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        let restored = NotebookDeserializer::deserialize(&backup_content).unwrap();
        assert_eq!(restored.metadata.title, "备份测试");
    }
    
    #[test]
    fn test_version_compatibility() {
        let notebook = Notebook::with_title("版本测试".to_string());
        let content = NotebookSerializer::serialize(&notebook).unwrap();
        
        // 正常版本应该可以加载
        let result = NotebookDeserializer::deserialize(&content);
        assert!(result.is_ok());
        
        // 修改版本号测试不兼容的版本
        let incompatible_content = content.replace("1.0", "2.0");
        let result = NotebookDeserializer::deserialize(&incompatible_content);
        assert!(result.is_err());
    }
}