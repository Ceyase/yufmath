//! # 笔记本模块集成测试
//!
//! 测试笔记本模块各组件的集成功能。

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use tempfile::tempdir;
    use std::time::Duration;
    
    #[test]
    fn test_notebook_workflow() {
        // 创建笔记本
        let mut notebook = Notebook::with_title("集成测试笔记本".to_string());
        
        // 添加单元格
        let code_cell = NotebookCell::new_code("x = 10\ny = x + 5".to_string());
        let markdown_cell = NotebookCell::new_markdown("# 计算结果\n\n这是一个简单的计算示例。".to_string());
        
        notebook.add_cell(code_cell);
        notebook.add_cell(markdown_cell);
        
        assert_eq!(notebook.cell_count(), 2);
        
        // 测试统计信息
        let stats = notebook.statistics();
        assert_eq!(stats.total_cells, 2);
        assert_eq!(stats.code_cells, 1);
        assert_eq!(stats.markdown_cells, 1);
    }
    
    #[test]
    fn test_execution_engine_integration() {
        let mut engine = ExecutionEngine::new();
        let mut cell = NotebookCell::new_code("2 + 3".to_string());
        
        // 执行单元格
        let result = engine.execute_cell(&mut cell).unwrap();
        
        assert!(result.is_success());
        assert!(cell.get_output().is_some());
        
        // 检查执行统计
        let stats = engine.get_statistics();
        assert_eq!(stats.total_executions, 1);
        assert_eq!(stats.successful_executions, 1);
    }
    
    #[test]
    fn test_scope_management() {
        let mut manager = ScopeManager::new();
        let cell_id = uuid::Uuid::new_v4();
        
        // 定义全局变量
        manager.define_global_variable(
            "pi".to_string(),
            crate::core::Expression::Number(crate::core::Number::from(3.14159)),
            cell_id
        ).unwrap();
        
        // 创建单元格作用域
        manager.create_cell_scope(cell_id, "测试单元格".to_string());
        manager.set_current_scope(Some(cell_id));
        
        // 在单元格作用域定义变量
        manager.define_variable(
            "radius".to_string(),
            crate::core::Expression::Number(crate::core::Number::from(5)),
            cell_id
        ).unwrap();
        
        // 检查变量可见性
        assert!(manager.has_variable("pi"));
        assert!(manager.has_variable("radius"));
        
        // 导出变量用于计算
        let vars = manager.export_for_computation();
        assert!(vars.contains_key("pi"));
        assert!(vars.contains_key("radius"));
    }
    
    #[test]
    fn test_file_format_integration() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("integration_test.ynb");
        
        // 创建笔记本
        let mut notebook = Notebook::with_title("文件格式测试".to_string());
        notebook.metadata.author = Some("测试作者".to_string());
        notebook.metadata.description = Some("这是一个集成测试笔记本".to_string());
        
        // 添加各种类型的单元格
        notebook.add_cell(crate::NotebookCell::new_markdown("# 介绍\n\n这是测试笔记本。".to_string()));
        notebook.add_cell(crate::NotebookCell::new_code("result = 42".to_string()));
        notebook.add_cell(crate::NotebookCell::new_text("这是一个文本单元格。".to_string()));
        
        // 保存文件
        NotebookSerializer::save_to_file(&mut notebook, &file_path).unwrap();
        assert!(file_path.exists());
        assert!(!notebook.needs_save());
        
        // 加载文件
        let loaded_notebook = NotebookDeserializer::load_from_file(&file_path).unwrap();
        assert_eq!(loaded_notebook.metadata.title, "文件格式测试");
        assert_eq!(loaded_notebook.metadata.author, Some("测试作者".to_string()));
        assert_eq!(loaded_notebook.cell_count(), 3);
        
        // 验证文件信息
        let file_info = NotebookFormat::get_file_info(&file_path).unwrap();
        assert_eq!(file_info.title, "文件格式测试");
        assert_eq!(file_info.cell_count, 3);
        assert_eq!(file_info.code_cells, 1);
    }
    
    #[test]
    fn test_export_integration() {
        let mut notebook = Notebook::with_title("导出测试".to_string());
        notebook.add_cell(crate::NotebookCell::new_markdown("# 导出测试\n\n这是导出功能的测试。".to_string()));
        notebook.add_cell(crate::NotebookCell::new_code("x = 1 + 2".to_string()));
        
        let exporter = NotebookExporter::new();
        
        // 测试导出为 Markdown
        let markdown = exporter.export_to_string(&notebook, ExportFormat::Markdown).unwrap();
        assert!(markdown.contains("# 导出测试"));
        assert!(markdown.contains("x = 1 + 2"));
        
        // 测试导出为 HTML
        let html = exporter.export_to_string(&notebook, ExportFormat::Html).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("导出测试"));
        
        // 测试导出为代码
        let code = exporter.export_to_string(&notebook, ExportFormat::Code).unwrap();
        assert!(code.contains("x = 1 + 2"));
        
        // 测试文件导出
        let dir = tempdir().unwrap();
        let html_path = dir.path().join("export_test.html");
        
        exporter.export_to_file(&notebook, &html_path, ExportFormat::Html).unwrap();
        assert!(html_path.exists());
    }
    
    #[test]
    fn test_notebook_manager_integration() {
        let mut manager = NotebookManager::new();
        
        // 创建多个笔记本
        let id1 = manager.create_notebook(Some("笔记本1".to_string()));
        let id2 = manager.create_notebook(Some("笔记本2".to_string()));
        
        assert_eq!(manager.notebook_count(), 2);
        assert_eq!(manager.get_active_notebook().unwrap().metadata.title, "笔记本2");
        
        // 切换活动笔记本
        manager.set_active_notebook(&id1).unwrap();
        assert_eq!(manager.get_active_notebook().unwrap().metadata.title, "笔记本1");
        
        // 修改笔记本
        if let Some(notebook) = manager.get_active_notebook_mut() {
            notebook.add_cell(crate::NotebookCell::new_code("test = 123".to_string()));
        }
        
        // 检查未保存状态
        assert!(manager.has_unsaved_notebooks());
        let unsaved = manager.get_unsaved_notebooks();
        assert_eq!(unsaved.len(), 1);
        
        // 关闭笔记本
        let closed = manager.close_notebook(&id1).unwrap();
        assert_eq!(closed.metadata.title, "笔记本1");
        assert_eq!(manager.notebook_count(), 1);
    }
    
    #[test]
    fn test_execution_queue_integration() {
        let mut engine = ExecutionEngine::new();
        
        // 创建测试单元格
        let cell1_id = uuid::Uuid::new_v4();
        let cell2_id = uuid::Uuid::new_v4();
        let cell3_id = uuid::Uuid::new_v4();
        
        // 添加到执行队列（cell2 依赖 cell1）
        engine.queue_cell(cell1_id, vec![]);
        engine.queue_cell(cell2_id, vec![cell1_id]);
        engine.queue_cell(cell3_id, vec![]);
        
        // 测试依赖分析
        let cells = vec![
            crate::NotebookCell::new_code("x = 10".to_string()),
            crate::NotebookCell::new_code("y = x + 5".to_string()),
            crate::NotebookCell::new_code("z = 20".to_string()),
        ];
        
        let dependencies = engine.analyze_dependencies(&cells);
        
        // cell[1] 应该依赖 cell[0]（因为使用了变量 x）
        assert!(dependencies.get(&cells[1].id).unwrap().contains(&cells[0].id));
        
        // cell[2] 不应该有依赖
        assert!(dependencies.get(&cells[2].id).unwrap().is_empty());
    }
    
    #[test]
    fn test_template_and_backup() {
        // 测试模板创建
        let template = NotebookFormat::create_template("模板测试");
        assert_eq!(template.metadata.title, "模板测试");
        assert_eq!(template.cell_count(), 2); // 欢迎单元格 + 示例代码
        
        // 测试备份功能
        let dir = tempdir().unwrap();
        let original_path = dir.path().join("original.ynb");
        let backup_path = dir.path().join("original.ynb.backup");
        
        let notebook = Notebook::with_title("备份测试".to_string());
        
        NotebookSerializer::create_backup(&notebook, &original_path).unwrap();
        assert!(backup_path.exists());
        
        // 验证备份内容
        let backup_notebook = NotebookDeserializer::load_from_file(&backup_path).unwrap();
        assert_eq!(backup_notebook.metadata.title, "备份测试");
    }
    
    #[test]
    fn test_search_and_statistics() {
        let mut notebook = Notebook::with_title("搜索测试".to_string());
        
        notebook.add_cell(crate::NotebookCell::new_code("x = 42".to_string()));
        notebook.add_cell(crate::NotebookCell::new_text("这里包含数字 42".to_string()));
        notebook.add_cell(crate::NotebookCell::new_markdown("# 标题\n\n没有特殊内容".to_string()));
        notebook.add_cell(crate::NotebookCell::new_code("y = x * 2".to_string()));
        
        // 搜索 "42"
        let results = notebook.search("42", true);
        assert_eq!(results.len(), 2); // 在两个单元格中找到
        
        // 搜索 "x"
        let results = notebook.search("x", true);
        assert_eq!(results.len(), 2); // 在两个代码单元格中找到
        
        // 获取统计信息
        let stats = notebook.statistics();
        assert_eq!(stats.total_cells, 4);
        assert_eq!(stats.code_cells, 2);
        assert_eq!(stats.text_cells, 1);
        assert_eq!(stats.markdown_cells, 1);
        assert!(stats.total_characters > 0);
        
        // 测试执行率
        assert_eq!(stats.execution_rate(), 0.0); // 没有执行过
        assert!(stats.average_cell_length() > 0.0);
    }
}