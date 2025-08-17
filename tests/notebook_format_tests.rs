//! # 笔记本文件格式测试
//!
//! 测试 .ynb 文件格式的序列化和反序列化功能。

use yufmath::notebook::{
    Notebook, NotebookCell, NotebookFormat, NotebookSerializer, NotebookDeserializer,
    NotebookError, CellType
};
use tempfile::tempdir;
use std::fs;

#[test]
fn test_notebook_serialization_roundtrip() {
    // 创建一个包含多种单元格类型的笔记本
    let mut notebook = Notebook::with_title("测试笔记本".to_string());
    
    // 添加不同类型的单元格
    notebook.add_cell(NotebookCell::new_markdown("# 欢迎使用 Yufmath\n\n这是一个测试笔记本。".to_string()));
    notebook.add_cell(NotebookCell::new_code("x = 2 + 3".to_string()));
    notebook.add_cell(NotebookCell::new_text("这是一个说明文本。".to_string()));
    notebook.add_cell(NotebookCell::new_code("y = x^2 + 1".to_string()));
    
    // 序列化
    let serialized = NotebookSerializer::serialize(&notebook).unwrap();
    
    // 验证序列化内容包含预期的信息
    assert!(serialized.contains("测试笔记本"));
    assert!(serialized.contains("format_version"));
    assert!(serialized.contains("欢迎使用 Yufmath"));
    assert!(serialized.contains("x = 2 + 3"));
    
    // 反序列化
    let deserialized = NotebookDeserializer::deserialize(&serialized).unwrap();
    
    // 验证反序列化结果
    assert_eq!(deserialized.metadata.title, "测试笔记本");
    assert_eq!(deserialized.cell_count(), 4);
    
    // 验证单元格类型和内容
    let cells: Vec<_> = deserialized.cells.iter().collect();
    assert_eq!(cells[0].cell_type, CellType::Markdown);
    assert!(cells[0].get_text().contains("欢迎使用 Yufmath"));
    
    assert_eq!(cells[1].cell_type, CellType::Code);
    assert_eq!(cells[1].get_text(), "x = 2 + 3");
    
    assert_eq!(cells[2].cell_type, CellType::Text);
    assert_eq!(cells[2].get_text(), "这是一个说明文本。");
    
    assert_eq!(cells[3].cell_type, CellType::Code);
    assert_eq!(cells[3].get_text(), "y = x^2 + 1");
}

#[test]
fn test_notebook_file_operations() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_notebook.ynb");
    
    // 创建测试笔记本
    let mut notebook = Notebook::with_title("文件测试笔记本".to_string());
    notebook.add_cell(NotebookCell::new_code("a = 1 + 2".to_string()));
    notebook.add_cell(NotebookCell::new_markdown("## 计算结果\n\na 的值应该是 3。".to_string()));
    
    // 保存到文件
    NotebookSerializer::save_to_file(&mut notebook, &file_path).unwrap();
    
    // 验证文件存在
    assert!(file_path.exists());
    assert!(notebook.get_file_path().is_some());
    assert!(!notebook.needs_save());
    
    // 从文件加载
    let loaded_notebook = NotebookDeserializer::load_from_file(&file_path).unwrap();
    
    // 验证加载的内容
    assert_eq!(loaded_notebook.metadata.title, "文件测试笔记本");
    assert_eq!(loaded_notebook.cell_count(), 2);
    assert_eq!(loaded_notebook.get_file_path(), Some(&file_path));
    
    // 验证单元格内容
    let cells: Vec<_> = loaded_notebook.cells.iter().collect();
    assert_eq!(cells[0].get_text(), "a = 1 + 2");
    assert!(cells[1].get_text().contains("计算结果"));
}

#[test]
fn test_file_extension_validation() {
    let dir = tempdir().unwrap();
    
    // 测试错误的文件扩展名
    let wrong_ext_path = dir.path().join("test.txt");
    let notebook = Notebook::with_title("扩展名测试".to_string());
    
    let result = NotebookSerializer::save_to_file(&mut notebook.clone(), &wrong_ext_path);
    assert!(result.is_err());
    
    // 测试正确的文件扩展名
    let correct_path = dir.path().join("test.ynb");
    let result = NotebookSerializer::save_to_file(&mut notebook.clone(), &correct_path);
    assert!(result.is_ok());
}

#[test]
fn test_notebook_file_info() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("info_test.ynb");
    
    // 创建包含多种单元格的笔记本
    let mut notebook = Notebook::with_title("信息测试笔记本".to_string());
    notebook.add_cell(NotebookCell::new_code("x = 42".to_string()));
    notebook.add_cell(NotebookCell::new_code("y = x * 2".to_string()));
    notebook.add_cell(NotebookCell::new_text("这是一个文本单元格".to_string()));
    notebook.add_cell(NotebookCell::new_markdown("# 标题\n\n这是 Markdown 内容。".to_string()));
    
    // 保存文件
    NotebookSerializer::save_to_file(&mut notebook, &file_path).unwrap();
    
    // 获取文件信息
    let info = NotebookFormat::get_file_info(&file_path).unwrap();
    
    // 验证文件信息
    assert_eq!(info.title, "信息测试笔记本");
    assert_eq!(info.cell_count, 4);
    assert_eq!(info.code_cells, 2);
    assert!(info.file_size > 0);
    assert_eq!(info.format_version, "1.0");
    
    // 测试摘要信息
    let summary = info.summary();
    assert!(summary.contains("信息测试笔记本"));
    assert!(summary.contains("4 个单元格"));
    assert!(summary.contains("2 代码"));
}

#[test]
fn test_notebook_template_creation() {
    let template = NotebookFormat::create_template("模板测试笔记本");
    
    // 验证模板基本信息
    assert_eq!(template.metadata.title, "模板测试笔记本");
    assert_eq!(template.cell_count(), 2); // 欢迎单元格 + 示例代码单元格
    
    // 验证单元格类型和内容
    let cells: Vec<_> = template.cells.iter().collect();
    assert_eq!(cells[0].cell_type, CellType::Markdown);
    assert!(cells[0].get_text().contains("模板测试笔记本"));
    assert!(cells[0].get_text().contains("欢迎使用 Yufmath 笔记本"));
    
    assert_eq!(cells[1].cell_type, CellType::Code);
    assert_eq!(cells[1].get_text(), "2 + 3");
}

#[test]
fn test_backup_creation() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("backup_test.ynb");
    let backup_path = dir.path().join("backup_test.ynb.backup");
    
    // 创建测试笔记本
    let notebook = Notebook::with_title("备份测试笔记本".to_string());
    
    // 创建备份
    NotebookSerializer::create_backup(&notebook, &file_path).unwrap();
    
    // 验证备份文件存在
    assert!(backup_path.exists());
    
    // 验证备份内容
    let backup_content = fs::read_to_string(&backup_path).unwrap();
    let restored_notebook = NotebookDeserializer::deserialize(&backup_content).unwrap();
    assert_eq!(restored_notebook.metadata.title, "备份测试笔记本");
}

#[test]
fn test_version_compatibility() {
    let notebook = Notebook::with_title("版本测试笔记本".to_string());
    let content = NotebookSerializer::serialize(&notebook).unwrap();
    
    // 正常版本应该可以加载
    let result = NotebookDeserializer::deserialize(&content);
    assert!(result.is_ok());
    
    // 测试不兼容的版本
    let incompatible_content = content.replace("1.0", "2.0");
    let result = NotebookDeserializer::deserialize(&incompatible_content);
    assert!(result.is_err());
    
    // 验证错误类型
    match result.unwrap_err() {
        NotebookError::Format(msg) => {
            assert!(msg.contains("不支持的笔记本格式版本"));
        }
        _ => panic!("期望格式错误"),
    }
}

#[test]
fn test_file_validation() {
    let dir = tempdir().unwrap();
    
    // 测试不存在的文件
    let nonexistent_path = dir.path().join("nonexistent.ynb");
    let result = NotebookFormat::validate_file(&nonexistent_path);
    assert!(result.is_err());
    
    // 测试错误的扩展名
    let wrong_ext_path = dir.path().join("test.txt");
    fs::write(&wrong_ext_path, "some content").unwrap();
    let result = NotebookFormat::validate_file(&wrong_ext_path);
    assert!(result.is_err());
    
    // 测试有效的笔记本文件
    let valid_path = dir.path().join("valid.ynb");
    let notebook = Notebook::with_title("有效笔记本".to_string());
    let content = NotebookSerializer::serialize(&notebook).unwrap();
    fs::write(&valid_path, content).unwrap();
    
    let result = NotebookFormat::validate_file(&valid_path);
    assert!(result.is_ok());
}

#[test]
fn test_empty_notebook_serialization() {
    let empty_notebook = Notebook::new();
    
    // 序列化空笔记本
    let serialized = NotebookSerializer::serialize(&empty_notebook).unwrap();
    
    // 反序列化
    let deserialized = NotebookDeserializer::deserialize(&serialized).unwrap();
    
    // 验证空笔记本的属性
    assert_eq!(deserialized.metadata.title, "未命名笔记本");
    assert_eq!(deserialized.cell_count(), 0);
    assert!(deserialized.is_empty());
}

#[test]
fn test_large_notebook_serialization() {
    let mut large_notebook = Notebook::with_title("大型笔记本测试".to_string());
    
    // 添加大量单元格
    for i in 0..100 {
        if i % 3 == 0 {
            large_notebook.add_cell(NotebookCell::new_code(format!("x_{} = {}", i, i * 2)));
        } else if i % 3 == 1 {
            large_notebook.add_cell(NotebookCell::new_text(format!("这是第 {} 个文本单元格", i)));
        } else {
            large_notebook.add_cell(NotebookCell::new_markdown(format!("## 第 {} 个标题", i)));
        }
    }
    
    // 序列化和反序列化
    let serialized = NotebookSerializer::serialize(&large_notebook).unwrap();
    let deserialized = NotebookDeserializer::deserialize(&serialized).unwrap();
    
    // 验证大型笔记本
    assert_eq!(deserialized.metadata.title, "大型笔记本测试");
    assert_eq!(deserialized.cell_count(), 100);
    
    // 验证统计信息
    let stats = deserialized.statistics();
    assert_eq!(stats.total_cells, 100);
    assert_eq!(stats.code_cells, 34); // 0, 3, 6, ..., 99 (34 个)
    assert_eq!(stats.text_cells, 33);  // 1, 4, 7, ..., 97 (33 个)
    assert_eq!(stats.markdown_cells, 33); // 2, 5, 8, ..., 98 (33 个)
}

#[test]
fn test_unicode_content_serialization() {
    let mut unicode_notebook = Notebook::with_title("Unicode 测试笔记本 🧮".to_string());
    
    // 添加包含 Unicode 字符的单元格
    unicode_notebook.add_cell(NotebookCell::new_code("π = 3.14159".to_string()));
    unicode_notebook.add_cell(NotebookCell::new_text("数学符号：∑, ∫, ∂, ∇, ∞".to_string()));
    unicode_notebook.add_cell(NotebookCell::new_markdown("# 中文标题\n\n包含中文内容的 Markdown 单元格。\n\n数学公式：$E = mc^2$".to_string()));
    
    // 序列化和反序列化
    let serialized = NotebookSerializer::serialize(&unicode_notebook).unwrap();
    let deserialized = NotebookDeserializer::deserialize(&serialized).unwrap();
    
    // 验证 Unicode 内容
    assert_eq!(deserialized.metadata.title, "Unicode 测试笔记本 🧮");
    
    let cells: Vec<_> = deserialized.cells.iter().collect();
    assert_eq!(cells[0].get_text(), "π = 3.14159");
    assert!(cells[1].get_text().contains("∑, ∫, ∂, ∇, ∞"));
    assert!(cells[2].get_text().contains("中文标题"));
    assert!(cells[2].get_text().contains("$E = mc^2$"));
}