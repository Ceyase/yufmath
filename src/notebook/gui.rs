//! # 笔记本图形用户界面
//!
//! 基于 FLTK 实现的笔记本交互界面，类似 Jupyter 的文本模式。

use super::{Notebook, NotebookCell, CellId, CellType, ExecutionEngine, NotebookError, NotebookResult, AutoCompleteEngine, CompletionSuggestion};
use crate::formatter::FormatType;
use fltk::{prelude::*, *};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;

/// 单元格编辑器组件
pub struct CellEditor {
    /// 主容器
    container: group::Group,
    /// 单元格类型标签
    type_label: frame::Frame,
    /// 输入编辑器
    editor: text::TextEditor,
    /// 输出显示区域
    output_display: text::TextDisplay,
    /// 单元格 ID
    cell_id: CellId,
    /// 单元格类型
    cell_type: CellType,
    /// 是否选中
    is_selected: bool,
    /// 语法高亮缓冲区
    syntax_buffer: Option<text::TextBuffer>,
    /// 自动补全引擎
    autocomplete_engine: AutoCompleteEngine,
}

impl CellEditor {
    /// 创建新的单元格编辑器
    pub fn new(x: i32, y: i32, width: i32, height: i32, cell: &NotebookCell) -> Self {
        let mut container = group::Group::new(x, y, width, height, None);
        container.set_frame(enums::FrameType::BorderBox);
        
        // 类型标签
        let mut type_label = frame::Frame::new(x + 5, y + 5, 80, 25, None);
        type_label.set_label(&format!("[{}]", cell.cell_type.display_name()));
        type_label.set_label_color(enums::Color::Blue);
        type_label.set_label_font(enums::Font::CourierBold);
        
        // 输入编辑器
        let mut editor = text::TextEditor::new(x + 5, y + 35, width - 10, height / 2 - 40, None);
        let mut input_buffer = text::TextBuffer::default();
        input_buffer.set_text(&cell.get_text());
        editor.set_buffer(input_buffer);
        editor.set_text_font(enums::Font::Courier);
        editor.set_text_size(12);
        
        // 输出显示区域
        let mut output_display = text::TextDisplay::new(x + 5, y + height / 2, width - 10, height / 2 - 5, None);
        let mut output_buffer = text::TextBuffer::default();
        if let Some(output) = cell.get_output() {
            output_buffer.set_text(&output.get_text());
        }
        output_display.set_buffer(output_buffer);
        output_display.set_text_font(enums::Font::Courier);
        output_display.set_text_size(11);
        output_display.set_color(enums::Color::from_rgb(248, 248, 248));
        
        container.end();
        
        let mut cell_editor = Self {
            container,
            type_label,
            editor,
            output_display,
            cell_id: cell.id,
            cell_type: cell.cell_type.clone(),
            is_selected: false,
            syntax_buffer: None,
            autocomplete_engine: AutoCompleteEngine::new(),
        };
        
        cell_editor.setup_autocomplete();
        cell_editor
    }
    
    /// 获取单元格 ID
    pub fn cell_id(&self) -> CellId {
        self.cell_id
    }
    
    /// 获取单元格类型
    pub fn cell_type(&self) -> &CellType {
        &self.cell_type
    }
    
    /// 获取输入文本
    pub fn get_input_text(&self) -> String {
        self.editor.buffer().unwrap().text()
    }
    
    /// 设置输入文本
    pub fn set_input_text(&mut self, text: &str) {
        if let Some(mut buffer) = self.editor.buffer() {
            buffer.set_text(text);
        }
    }
    
    /// 设置输出文本
    pub fn set_output_text(&mut self, text: &str) {
        if let Some(mut buffer) = self.output_display.buffer() {
            buffer.set_text(text);
        }
    }
    
    /// 设置选中状态
    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
        if selected {
            self.container.set_color(enums::Color::from_rgb(230, 240, 255));
        } else {
            self.container.set_color(enums::Color::White);
        }
        self.container.redraw();
    }
    
    /// 是否选中
    pub fn is_selected(&self) -> bool {
        self.is_selected
    }
    
    /// 设置单元格类型
    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.cell_type = cell_type.clone();
        self.type_label.set_label(&format!("[{}]", cell_type.display_name()));
        self.setup_syntax_highlighting();
        self.type_label.redraw();
    }
    
    /// 设置语法高亮
    fn setup_syntax_highlighting(&mut self) {
        match self.cell_type {
            CellType::Code => {
                // 为代码单元格设置语法高亮
                self.editor.set_highlight_data(
                    text::TextBuffer::default(),
                    vec![
                        text::StyleTableEntry {
                            color: enums::Color::Blue,
                            font: enums::Font::Courier,
                            size: 12,
                        },
                        text::StyleTableEntry {
                            color: enums::Color::Red,
                            font: enums::Font::Courier,
                            size: 12,
                        },
                        text::StyleTableEntry {
                            color: enums::Color::DarkGreen,
                            font: enums::Font::Courier,
                            size: 12,
                        },
                    ]
                );
            }
            CellType::Markdown => {
                // Markdown 语法高亮
                self.editor.set_text_color(enums::Color::DarkBlue);
            }
            _ => {
                // 普通文本
                self.editor.set_text_color(enums::Color::Black);
            }
        }
    }
    
    /// 获取容器组件
    pub fn container(&self) -> &group::Group {
        &self.container
    }
    
    /// 获取容器组件（可变）
    pub fn container_mut(&mut self) -> &mut group::Group {
        &mut self.container
    }
    
    /// 聚焦到编辑器
    pub fn focus(&mut self) {
        self.editor.take_focus();
    }
    
    /// 获取编辑器组件
    pub fn editor(&self) -> &text::TextEditor {
        &self.editor
    }
    
    /// 获取编辑器组件（可变）
    pub fn editor_mut(&mut self) -> &mut text::TextEditor {
        &mut self.editor
    }
    
    /// 设置自动补全
    fn setup_autocomplete(&mut self) {
        // 简化的自动补全设置
        // 由于 FLTK 的事件处理复杂性，这里暂时使用简化实现
        // 实际应用中可以通过其他方式实现自动补全
    }
    
    /// 显示自动补全建议
    pub fn show_completions(&mut self) {
        if let Some(buffer) = self.editor.buffer() {
            let text = buffer.text();
            let cursor_pos = self.editor.insert_position();
            
            let suggestions = self.autocomplete_engine.get_completions(&text, cursor_pos as usize);
            
            if !suggestions.is_empty() {
                self.show_completion_popup(suggestions);
            }
        }
    }
    
    /// 显示补全弹窗
    fn show_completion_popup(&self, suggestions: Vec<CompletionSuggestion>) {
        // 创建补全弹窗
        let mut popup = menu::MenuButton::new(
            self.editor.x() + 10,
            self.editor.y() + 30,
            200,
            20,
            None
        );
        
        // 添加建议项
        for (i, suggestion) in suggestions.iter().enumerate() {
            popup.add_emit(
                &suggestion.label,
                enums::Shortcut::None,
                menu::MenuFlag::Normal,
                app::Sender::<String>::get(),
                suggestion.text.clone(),
            );
            
            if i >= 10 {
                break; // 限制显示数量
            }
        }
        
        popup.popup();
    }
    
    /// 获取自动补全引擎
    pub fn autocomplete_engine(&self) -> &AutoCompleteEngine {
        &self.autocomplete_engine
    }
    
    /// 获取自动补全引擎（可变）
    pub fn autocomplete_engine_mut(&mut self) -> &mut AutoCompleteEngine {
        &mut self.autocomplete_engine
    }
}

/// 笔记本 GUI 主窗口
pub struct NotebookGUI {
    /// 主窗口
    window: window::Window,
    /// 菜单栏
    menu_bar: menu::MenuBar,
    /// 工具栏
    toolbar: group::Group,
    /// 滚动区域
    scroll: group::Scroll,
    /// 单元格容器
    cell_container: group::Pack,
    /// 状态栏
    status_bar: frame::Frame,
    /// 当前笔记本
    notebook: Option<Notebook>,
    /// 执行引擎
    execution_engine: ExecutionEngine,
    /// 单元格编辑器列表
    cell_editors: Vec<CellEditor>,
    /// 当前选中的单元格索引
    current_cell: Option<usize>,
    /// 快捷键映射
    shortcuts: HashMap<i32, Box<dyn Fn(&mut Self)>>,
}

impl NotebookGUI {
    /// 创建新的笔记本 GUI
    pub fn new() -> Self {
        let app = app::App::default();
        let mut window = window::Window::new(100, 100, 1000, 700, "Yufmath Notebook");
        
        // 创建菜单栏
        let mut menu_bar = menu::MenuBar::new(0, 0, 1000, 30, None);
        
        // 创建工具栏
        let mut toolbar = group::Group::new(0, 30, 1000, 40, None);
        toolbar.set_color(enums::Color::from_rgb(240, 240, 240));
        
        // 添加工具栏按钮
        let mut new_code_btn = button::Button::new(10, 35, 80, 30, "新建代码");
        let mut new_md_btn = button::Button::new(100, 35, 80, 30, "新建文档");
        let mut run_btn = button::Button::new(190, 35, 60, 30, "运行");
        let mut run_all_btn = button::Button::new(260, 35, 80, 30, "运行全部");
        let mut save_btn = button::Button::new(350, 35, 60, 30, "保存");
        
        toolbar.end();
        
        // 创建滚动区域
        let mut scroll = group::Scroll::new(0, 70, 1000, 600, None);
        scroll.set_type(group::ScrollType::Vertical);
        
        // 创建单元格容器
        let mut cell_container = group::Pack::new(10, 80, 980, 580, None);
        cell_container.set_type(group::PackType::Vertical);
        cell_container.set_spacing(10);
        
        cell_container.end();
        scroll.end();
        
        // 创建状态栏
        let mut status_bar = frame::Frame::new(0, 670, 1000, 30, "就绪");
        status_bar.set_color(enums::Color::from_rgb(240, 240, 240));
        status_bar.set_align(enums::Align::Left | enums::Align::Inside);
        
        window.end();
        window.show();
        
        let mut gui = Self {
            window,
            menu_bar,
            toolbar,
            scroll,
            cell_container,
            status_bar,
            notebook: None,
            execution_engine: ExecutionEngine::new(),
            cell_editors: Vec::new(),
            current_cell: None,
            shortcuts: HashMap::new(),
        };
        
        gui.setup_menu();
        gui.setup_callbacks();
        gui.setup_shortcuts();
        
        gui
    }
    
    /// 设置菜单
    fn setup_menu(&mut self) {
        self.menu_bar.add_emit(
            "文件/新建\t",
            enums::Shortcut::Ctrl | 'n',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            1001,
        );
        
        self.menu_bar.add_emit(
            "文件/打开...\t",
            enums::Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            1002,
        );
        
        self.menu_bar.add_emit(
            "文件/保存\t",
            enums::Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            1003,
        );
        
        self.menu_bar.add_emit(
            "文件/另存为...\t",
            enums::Shortcut::Ctrl | enums::Shortcut::Shift | 's',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            1004,
        );
        
        self.menu_bar.add_emit(
            "文件/退出\t",
            enums::Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            1099,
        );
        
        self.menu_bar.add_emit(
            "编辑/新建代码单元格\t",
            enums::Shortcut::Ctrl | 'n',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            2001,
        );
        
        self.menu_bar.add_emit(
            "编辑/新建文档单元格\t",
            enums::Shortcut::Ctrl | 'm',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            2002,
        );
        
        self.menu_bar.add_emit(
            "编辑/删除单元格\t",
            enums::Shortcut::Ctrl | 'd',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            2003,
        );
        
        self.menu_bar.add_emit(
            "运行/执行当前单元格\t",
            enums::Shortcut::Ctrl | enums::Shortcut::from_key(enums::Key::Enter),
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            3001,
        );
        
        self.menu_bar.add_emit(
            "运行/执行并新建单元格\t",
            enums::Shortcut::Shift | enums::Shortcut::from_key(enums::Key::Enter),
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            3002,
        );
        
        self.menu_bar.add_emit(
            "运行/执行所有单元格\t",
            enums::Shortcut::Ctrl | 'r',
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            3003,
        );
        
        self.menu_bar.add_emit(
            "帮助/关于\t",
            enums::Shortcut::None,
            menu::MenuFlag::Normal,
            app::Sender::<i32>::get(),
            9001,
        );
    }
    
    /// 设置回调函数
    fn setup_callbacks(&mut self) {
        // 这里需要使用 Rc<RefCell<>> 来处理 Rust 的借用检查
        // 实际实现中需要更复杂的回调处理
    }
    
    /// 设置快捷键
    fn setup_shortcuts(&mut self) {
        // 由于 Rust 的借用检查限制，这里使用简化的实现
        // 实际应用中需要使用消息传递或其他模式
    }
    
    /// 设置笔记本
    pub fn set_notebook(&mut self, notebook: Notebook) -> NotebookResult<()> {
        self.notebook = Some(notebook);
        self.refresh_cells()?;
        self.update_title();
        Ok(())
    }
    
    /// 获取当前笔记本
    pub fn get_notebook(&self) -> Option<&Notebook> {
        self.notebook.as_ref()
    }
    
    /// 刷新单元格显示
    fn refresh_cells(&mut self) -> NotebookResult<()> {
        // 清除现有的单元格编辑器
        self.cell_container.clear();
        self.cell_editors.clear();
        
        if let Some(notebook) = &self.notebook {
            let mut y_offset = 10;
            
            for (index, cell) in notebook.get_cells().iter().enumerate() {
                let mut cell_editor = CellEditor::new(
                    10, 
                    y_offset, 
                    960, 
                    150, 
                    cell
                );
                
                // 设置选中状态
                if Some(index) == self.current_cell {
                    cell_editor.set_selected(true);
                }
                
                self.cell_container.add(&cell_editor.container);
                self.cell_editors.push(cell_editor);
                
                y_offset += 160;
            }
        }
        
        self.cell_container.redraw();
        self.scroll.redraw();
        Ok(())
    }
    
    /// 更新窗口标题
    fn update_title(&mut self) {
        let title = if let Some(notebook) = &self.notebook {
            format!("Yufmath Notebook - {}", notebook.metadata.title)
        } else {
            "Yufmath Notebook".to_string()
        };
        
        self.window.set_label(&title);
    }
    
    /// 创建新的代码单元格
    pub fn create_code_cell(&mut self) -> NotebookResult<()> {
        self.create_cell(CellType::Code)
    }
    
    /// 创建新的文档单元格
    pub fn create_markdown_cell(&mut self) -> NotebookResult<()> {
        self.create_cell(CellType::Markdown)
    }
    
    /// 创建新单元格
    fn create_cell(&mut self, cell_type: CellType) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            let cell = match cell_type {
                CellType::Code => NotebookCell::new_code("".to_string()),
                CellType::Markdown => NotebookCell::new_markdown("".to_string()),
                CellType::Text => NotebookCell::new_text("".to_string()),
                CellType::Output => return Err(NotebookError::Cell("不能手动创建输出单元格".to_string())),
            };
            
            let insert_index = self.current_cell.map(|i| i + 1).unwrap_or(notebook.cell_count());
            notebook.insert_cell(insert_index, cell)?;
            
            self.refresh_cells()?;
            self.current_cell = Some(insert_index);
            
            // 聚焦到新创建的单元格
            if let Some(editor) = self.cell_editors.get_mut(insert_index) {
                editor.focus();
            }
            
            self.set_status(&format!("创建了新的{}单元格", cell_type.display_name()));
        }
        
        Ok(())
    }
    
    /// 删除当前单元格
    pub fn delete_current_cell(&mut self) -> NotebookResult<()> {
        if let Some(current_index) = self.current_cell {
            if let Some(notebook) = &mut self.notebook {
                notebook.remove_cell(current_index)?;
                
                // 调整当前单元格索引
                if current_index >= notebook.cell_count() && notebook.cell_count() > 0 {
                    self.current_cell = Some(notebook.cell_count() - 1);
                } else if notebook.cell_count() == 0 {
                    self.current_cell = None;
                }
                
                self.refresh_cells()?;
                self.set_status("删除了单元格");
            }
        }
        
        Ok(())
    }
    
    /// 执行当前单元格
    pub fn execute_current_cell(&mut self) -> NotebookResult<()> {
        if let Some(current_index) = self.current_cell {
            self.execute_cell(current_index)
        } else {
            Ok(())
        }
    }
    
    /// 执行指定单元格
    fn execute_cell(&mut self, index: usize) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            if let Some(cell) = notebook.get_cell_mut(index) {
                // 更新单元格内容
                if let Some(editor) = self.cell_editors.get(index) {
                    let input_text = editor.get_input_text();
                    cell.set_text(input_text);
                }
                
                // 执行单元格
                let result = self.execution_engine.execute_cell(cell)?;
                
                // 更新输出显示
                if let Some(editor) = self.cell_editors.get_mut(index) {
                    match &result {
                        super::ExecutionResult::Success { value, execution_time, .. } => {
                            editor.set_output_text(&value);
                            self.set_status(&format!("执行成功 ({:.2}ms)", execution_time.as_millis()));
                        }
                        super::ExecutionResult::Error { error, .. } => {
                            editor.set_output_text(&format!("错误: {}", error));
                            self.set_status(&format!("执行错误: {}", error));
                        }
                        super::ExecutionResult::Skipped => {
                            self.set_status("跳过执行（非代码单元格）");
                        }
                        super::ExecutionResult::Cancelled => {
                            self.set_status("执行被取消");
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行当前单元格并创建新单元格
    pub fn execute_and_create_cell(&mut self) -> NotebookResult<()> {
        self.execute_current_cell()?;
        self.create_code_cell()?;
        Ok(())
    }
    
    /// 执行所有单元格
    pub fn execute_all_cells(&mut self) -> NotebookResult<()> {
        if let Some(notebook) = &self.notebook {
            let cell_count = notebook.cell_count();
            
            for i in 0..cell_count {
                self.execute_cell(i)?;
                self.set_status(&format!("执行进度: {}/{}", i + 1, cell_count));
                
                // 更新界面
                app::App::default().wait();
            }
            
            self.set_status(&format!("执行完成: {} 个单元格", cell_count));
        }
        
        Ok(())
    }
    
    /// 保存笔记本
    pub fn save_notebook(&mut self) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            // 更新所有单元格内容
            for (index, editor) in self.cell_editors.iter().enumerate() {
                if let Some(cell) = notebook.get_cell_mut(index) {
                    let input_text = editor.get_input_text();
                    cell.set_text(input_text);
                }
            }
            
            if let Some(path) = notebook.get_file_path().cloned() {
                super::NotebookSerializer::save_to_file(notebook, path)?;
                self.set_status("笔记本已保存");
            } else {
                // 显示保存对话框
                self.show_save_dialog()?;
            }
        }
        
        Ok(())
    }
    
    /// 显示保存对话框
    fn show_save_dialog(&mut self) -> NotebookResult<()> {
        let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
        dialog.set_filter("*.ynb");
        dialog.show();
        
        if let Some(path) = dialog.filename().to_str() {
            if let Some(notebook) = &mut self.notebook {
                notebook.set_file_path(std::path::PathBuf::from(path));
                super::NotebookSerializer::save_to_file(notebook, path.to_string())?;
                self.set_status("笔记本已保存");
                self.update_title();
            }
        }
        
        Ok(())
    }
    
    /// 显示打开对话框
    pub fn show_open_dialog(&mut self) -> NotebookResult<()> {
        let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
        dialog.set_filter("*.ynb");
        dialog.show();
        
        if let Some(path) = dialog.filename().to_str() {
            let notebook = super::NotebookDeserializer::load_from_file(path.to_string())?;
            self.set_notebook(notebook)?;
            self.set_status(&format!("已打开笔记本: {}", path));
        }
        
        Ok(())
    }
    
    /// 切换单元格类型
    pub fn convert_current_cell(&mut self, new_type: CellType) -> NotebookResult<()> {
        if let Some(current_index) = self.current_cell {
            if let Some(notebook) = &mut self.notebook {
                if let Some(cell) = notebook.get_cell_mut(current_index) {
                    // 更新单元格内容
                    if let Some(editor) = self.cell_editors.get(current_index) {
                        let input_text = editor.get_input_text();
                        cell.set_text(input_text);
                    }
                    
                    // 转换单元格类型
                    cell.cell_type = new_type.clone();
                    
                    // 更新编辑器
                    if let Some(editor) = self.cell_editors.get_mut(current_index) {
                        editor.set_cell_type(new_type.clone());
                    }
                    
                    self.set_status(&format!("单元格已转换为{}", new_type.display_name()));
                }
            }
        }
        
        Ok(())
    }
    
    /// 移动到上一个单元格
    pub fn move_to_previous_cell(&mut self) {
        if let Some(current) = self.current_cell {
            if current > 0 {
                self.set_current_cell(Some(current - 1));
            }
        }
    }
    
    /// 移动到下一个单元格
    pub fn move_to_next_cell(&mut self) {
        if let Some(current) = self.current_cell {
            if let Some(notebook) = &self.notebook {
                if current < notebook.cell_count() - 1 {
                    self.set_current_cell(Some(current + 1));
                }
            }
        }
    }
    
    /// 设置当前单元格
    fn set_current_cell(&mut self, index: Option<usize>) {
        // 取消之前的选中状态
        if let Some(old_index) = self.current_cell {
            if let Some(editor) = self.cell_editors.get_mut(old_index) {
                editor.set_selected(false);
            }
        }
        
        // 设置新的选中状态
        self.current_cell = index;
        if let Some(new_index) = index {
            if let Some(editor) = self.cell_editors.get_mut(new_index) {
                editor.set_selected(true);
                editor.focus();
            }
        }
    }
    
    /// 设置状态消息
    fn set_status(&mut self, message: &str) {
        self.status_bar.set_label(message);
        self.status_bar.redraw();
    }
    
    /// 显示关于对话框
    pub fn show_about(&self) {
        dialog::message_default("关于 Yufmath Notebook\n\n基于 FLTK 的计算机代数系统笔记本界面\n版本 0.1.0");
    }
    
    /// 运行主循环
    pub fn run(&mut self) -> NotebookResult<()> {
        let app = app::App::default();
        
        // 处理消息循环
        while app.wait() {
            if let Some(msg) = app::Receiver::<i32>::get().recv() {
                match msg {
                    1001 => { // 新建
                        let notebook = Notebook::with_title("新笔记本".to_string());
                        self.set_notebook(notebook).ok();
                    }
                    1002 => { // 打开
                        self.show_open_dialog().ok();
                    }
                    1003 => { // 保存
                        self.save_notebook().ok();
                    }
                    1004 => { // 另存为
                        self.show_save_dialog().ok();
                    }
                    1099 => { // 退出
                        break;
                    }
                    2001 => { // 新建代码单元格
                        self.create_code_cell().ok();
                    }
                    2002 => { // 新建文档单元格
                        self.create_markdown_cell().ok();
                    }
                    2003 => { // 删除单元格
                        self.delete_current_cell().ok();
                    }
                    3001 => { // 执行当前单元格
                        self.execute_current_cell().ok();
                    }
                    3002 => { // 执行并新建单元格
                        self.execute_and_create_cell().ok();
                    }
                    3003 => { // 执行所有单元格
                        self.execute_all_cells().ok();
                    }
                    9001 => { // 关于
                        self.show_about();
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
}

impl Default for NotebookGUI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cell_editor_creation() {
        let cell = NotebookCell::new_code("test code".to_string());
        let editor = CellEditor::new(0, 0, 400, 200, &cell);
        
        assert_eq!(editor.cell_id(), cell.id);
        assert_eq!(editor.cell_type(), &CellType::Code);
        assert_eq!(editor.get_input_text(), "test code");
        assert!(!editor.is_selected());
    }
    
    #[test]
    fn test_cell_editor_selection() {
        let cell = NotebookCell::new_code("test".to_string());
        let mut editor = CellEditor::new(0, 0, 400, 200, &cell);
        
        assert!(!editor.is_selected());
        
        editor.set_selected(true);
        assert!(editor.is_selected());
        
        editor.set_selected(false);
        assert!(!editor.is_selected());
    }
    
    #[test]
    fn test_cell_editor_type_change() {
        let cell = NotebookCell::new_code("test".to_string());
        let mut editor = CellEditor::new(0, 0, 400, 200, &cell);
        
        assert_eq!(editor.cell_type(), &CellType::Code);
        
        editor.set_cell_type(CellType::Markdown);
        assert_eq!(editor.cell_type(), &CellType::Markdown);
    }
    
    #[test]
    fn test_notebook_gui_creation() {
        // 注意：这个测试需要在有图形环境的情况下运行
        // 在 CI 环境中可能会失败
        if std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok() {
            let gui = NotebookGUI::new();
            assert!(gui.notebook.is_none());
            assert!(gui.current_cell.is_none());
            assert!(gui.cell_editors.is_empty());
        }
    }
}