//! # 笔记本用户界面
//!
//! 提供基于终端的笔记本交互界面。

use super::{Notebook, NotebookCell, CellId, CellType, ExecutionEngine, NotebookError, NotebookResult};
use crate::formatter::FormatType;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Duration;

/// UI 事件类型
#[derive(Debug, Clone)]
pub enum UIEvent {
    /// 键盘输入
    KeyPress(KeyEvent),
    /// 鼠标事件
    Mouse(MouseEvent),
    /// 窗口大小改变
    Resize(u16, u16),
    /// 退出请求
    Quit,
}

/// 键盘事件
#[derive(Debug, Clone)]
pub struct KeyEvent {
    /// 按键代码
    pub key: Key,
    /// 修饰键
    pub modifiers: KeyModifiers,
}

/// 按键类型
#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    /// 字符键
    Char(char),
    /// 功能键
    Function(u8),
    /// 方向键
    Arrow(ArrowKey),
    /// 特殊键
    Special(SpecialKey),
}

/// 方向键
#[derive(Debug, Clone, PartialEq)]
pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}

/// 特殊键
#[derive(Debug, Clone, PartialEq)]
pub enum SpecialKey {
    Enter,
    Tab,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Escape,
}

/// 修饰键
#[derive(Debug, Clone, PartialEq)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: false,
        }
    }
    
    pub fn ctrl() -> Self {
        Self {
            ctrl: true,
            alt: false,
            shift: false,
        }
    }
    
    pub fn shift() -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: true,
        }
    }
}

/// 鼠标事件
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: u16,
    pub y: u16,
    pub button: MouseButton,
    pub action: MouseAction,
}

/// 鼠标按键
#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// 鼠标动作
#[derive(Debug, Clone, PartialEq)]
pub enum MouseAction {
    Press,
    Release,
    Move,
    Scroll(ScrollDirection),
}

/// 滚动方向
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

/// UI 命令
#[derive(Debug, Clone)]
pub enum UICommand {
    /// 创建新单元格
    CreateCell(CellType),
    /// 删除单元格
    DeleteCell(CellId),
    /// 移动单元格
    MoveCell(CellId, isize),
    /// 执行单元格
    ExecuteCell(CellId),
    /// 执行所有单元格
    ExecuteAll,
    /// 切换单元格类型
    ConvertCell(CellId, CellType),
    /// 保存笔记本
    Save,
    /// 另存为
    SaveAs(String),
    /// 打开笔记本
    Open(String),
    /// 导出笔记本
    Export(String, super::ExportFormat),
    /// 显示帮助
    ShowHelp,
    /// 退出
    Quit,
}

/// 键盘绑定
#[derive(Debug, Clone)]
pub struct KeyBinding {
    /// 键盘事件
    pub key_event: KeyEvent,
    /// 对应的命令
    pub command: UICommand,
    /// 描述
    pub description: String,
}

impl KeyBinding {
    /// 创建新的键盘绑定
    pub fn new(key: Key, modifiers: KeyModifiers, command: UICommand, description: &str) -> Self {
        Self {
            key_event: KeyEvent { key, modifiers },
            command,
            description: description.to_string(),
        }
    }
    
    /// 检查键盘事件是否匹配
    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.key_event.key == event.key && self.key_event.modifiers == event.modifiers
    }
}

/// 笔记本用户界面
pub struct NotebookUI {
    /// 当前笔记本
    notebook: Option<Notebook>,
    /// 执行引擎
    execution_engine: ExecutionEngine,
    /// 当前选中的单元格索引
    current_cell: Option<usize>,
    /// 编辑模式
    edit_mode: bool,
    /// 键盘绑定
    key_bindings: Vec<KeyBinding>,
    /// 终端大小
    terminal_size: (u16, u16),
    /// 滚动位置
    scroll_offset: usize,
    /// 状态消息
    status_message: Option<String>,
    /// 状态消息显示时间
    status_timeout: Option<std::time::Instant>,
}

impl NotebookUI {
    /// 创建新的笔记本 UI
    pub fn new() -> Self {
        let mut ui = Self {
            notebook: None,
            execution_engine: ExecutionEngine::new(),
            current_cell: None,
            edit_mode: false,
            key_bindings: Vec::new(),
            terminal_size: (80, 24),
            scroll_offset: 0,
            status_message: None,
            status_timeout: None,
        };
        
        ui.setup_default_key_bindings();
        ui
    }
    
    /// 创建带笔记本的 UI
    pub fn with_notebook(notebook: Notebook) -> Self {
        let mut ui = Self::new();
        ui.set_notebook(notebook);
        ui
    }
    
    /// 设置笔记本
    pub fn set_notebook(&mut self, notebook: Notebook) {
        self.notebook = Some(notebook);
        self.current_cell = None;
        self.edit_mode = false;
        self.scroll_offset = 0;
    }
    
    /// 设置默认键盘绑定
    fn setup_default_key_bindings(&mut self) {
        self.key_bindings = vec![
            // 单元格操作
            KeyBinding::new(
                Key::Char('n'),
                KeyModifiers::ctrl(),
                UICommand::CreateCell(CellType::Code),
                "创建新代码单元格"
            ),
            KeyBinding::new(
                Key::Char('m'),
                KeyModifiers::ctrl(),
                UICommand::CreateCell(CellType::Markdown),
                "创建新 Markdown 单元格"
            ),
            KeyBinding::new(
                Key::Char('d'),
                KeyModifiers::ctrl(),
                UICommand::DeleteCell(uuid::Uuid::nil()), // 占位符，实际使用时会替换
                "删除当前单元格"
            ),
            
            // 执行操作
            KeyBinding::new(
                Key::Special(SpecialKey::Enter),
                KeyModifiers::ctrl(),
                UICommand::ExecuteCell(uuid::Uuid::nil()),
                "执行当前单元格"
            ),
            KeyBinding::new(
                Key::Special(SpecialKey::Enter),
                KeyModifiers::shift(),
                UICommand::ExecuteCell(uuid::Uuid::nil()),
                "执行当前单元格并创建新单元格"
            ),
            KeyBinding::new(
                Key::Char('r'),
                KeyModifiers::ctrl(),
                UICommand::ExecuteAll,
                "执行所有单元格"
            ),
            
            // 文件操作
            KeyBinding::new(
                Key::Char('s'),
                KeyModifiers::ctrl(),
                UICommand::Save,
                "保存笔记本"
            ),
            KeyBinding::new(
                Key::Char('o'),
                KeyModifiers::ctrl(),
                UICommand::Open("".to_string()),
                "打开笔记本"
            ),
            
            // 导航
            KeyBinding::new(
                Key::Arrow(ArrowKey::Up),
                KeyModifiers::none(),
                UICommand::ShowHelp, // 占位符，实际处理在 handle_navigation 中
                "向上移动"
            ),
            KeyBinding::new(
                Key::Arrow(ArrowKey::Down),
                KeyModifiers::none(),
                UICommand::ShowHelp, // 占位符
                "向下移动"
            ),
            
            // 其他
            KeyBinding::new(
                Key::Char('h'),
                KeyModifiers::ctrl(),
                UICommand::ShowHelp,
                "显示帮助"
            ),
            KeyBinding::new(
                Key::Char('q'),
                KeyModifiers::ctrl(),
                UICommand::Quit,
                "退出"
            ),
        ];
    }
    

    
    /// 获取当前笔记本
    pub fn get_notebook(&self) -> Option<&Notebook> {
        self.notebook.as_ref()
    }
    
    /// 获取当前笔记本（可变）
    pub fn get_notebook_mut(&mut self) -> Option<&mut Notebook> {
        self.notebook.as_mut()
    }
    
    /// 运行主循环
    pub fn run(&mut self) -> NotebookResult<()> {
        self.initialize_terminal()?;
        
        loop {
            self.render()?;
            
            match self.read_event()? {
                UIEvent::KeyPress(key_event) => {
                    if let Some(command) = self.handle_key_event(&key_event) {
                        if matches!(command, UICommand::Quit) {
                            break;
                        }
                        self.execute_command(command)?;
                    }
                }
                UIEvent::Resize(width, height) => {
                    self.terminal_size = (width, height);
                }
                UIEvent::Quit => break,
                _ => {}
            }
        }
        
        self.cleanup_terminal()?;
        Ok(())
    }
    
    /// 初始化终端
    fn initialize_terminal(&mut self) -> NotebookResult<()> {
        // 进入原始模式，隐藏光标等
        print!("\x1b[?1049h"); // 切换到备用屏幕
        print!("\x1b[?25l");   // 隐藏光标
        io::stdout().flush().map_err(|e| NotebookError::Io(e))?;
        
        // 获取终端大小
        self.terminal_size = self.get_terminal_size();
        
        Ok(())
    }
    
    /// 清理终端
    fn cleanup_terminal(&mut self) -> NotebookResult<()> {
        print!("\x1b[?25h");   // 显示光标
        print!("\x1b[?1049l"); // 切换回主屏幕
        io::stdout().flush().map_err(|e| NotebookError::Io(e))?;
        Ok(())
    }
    
    /// 获取终端大小
    fn get_terminal_size(&self) -> (u16, u16) {
        // 简化实现，实际应该使用系统调用获取真实终端大小
        (80, 24)
    }
    
    /// 读取事件
    fn read_event(&self) -> NotebookResult<UIEvent> {
        // 简化实现，实际应该使用非阻塞 I/O 读取键盘输入
        // 这里返回一个模拟的退出事件
        Ok(UIEvent::Quit)
    }
    
    /// 处理键盘事件
    fn handle_key_event(&mut self, event: &KeyEvent) -> Option<UICommand> {
        // 首先检查是否是导航键
        if let Some(command) = self.handle_navigation(event) {
            return Some(command);
        }
        
        // 查找匹配的键盘绑定
        for binding in &self.key_bindings {
            if binding.matches(event) {
                let mut command = binding.command.clone();
                
                // 替换占位符
                command = self.replace_placeholders(command);
                
                return Some(command);
            }
        }
        
        None
    }
    
    /// 处理导航键
    fn handle_navigation(&mut self, event: &KeyEvent) -> Option<UICommand> {
        if self.edit_mode {
            return None; // 编辑模式下不处理导航
        }
        
        match &event.key {
            Key::Arrow(ArrowKey::Up) => {
                if let Some(current) = self.current_cell {
                    if current > 0 {
                        self.current_cell = Some(current - 1);
                        self.ensure_cell_visible();
                    }
                }
            }
            Key::Arrow(ArrowKey::Down) => {
                if let Some(notebook) = &self.notebook {
                    if let Some(current) = self.current_cell {
                        if current < notebook.cell_count() - 1 {
                            self.current_cell = Some(current + 1);
                            self.ensure_cell_visible();
                        }
                    }
                }
            }
            Key::Special(SpecialKey::Enter) => {
                self.edit_mode = true;
            }
            Key::Special(SpecialKey::Escape) => {
                self.edit_mode = false;
            }
            _ => return None,
        }
        
        None
    }
    
    /// 确保当前单元格可见
    fn ensure_cell_visible(&mut self) {
        if let Some(current) = self.current_cell {
            let visible_height = self.terminal_size.1 as usize - 3; // 减去状态栏等
            
            if current < self.scroll_offset {
                self.scroll_offset = current;
            } else if current >= self.scroll_offset + visible_height {
                self.scroll_offset = current - visible_height + 1;
            }
        }
    }
    
    /// 替换命令中的占位符
    fn replace_placeholders(&self, mut command: UICommand) -> UICommand {
        match &mut command {
            UICommand::DeleteCell(id) | UICommand::ExecuteCell(id) => {
                if let Some(current) = self.current_cell {
                    if let Some(notebook) = &self.notebook {
                        if let Some(cell) = notebook.get_cell(current) {
                            *id = cell.id;
                        }
                    }
                }
            }
            UICommand::ConvertCell(id, _) => {
                if let Some(current) = self.current_cell {
                    if let Some(notebook) = &self.notebook {
                        if let Some(cell) = notebook.get_cell(current) {
                            *id = cell.id;
                        }
                    }
                }
            }
            _ => {}
        }
        
        command
    }
    
    /// 执行命令
    fn execute_command(&mut self, command: UICommand) -> NotebookResult<()> {
        match command {
            UICommand::CreateCell(cell_type) => {
                self.create_cell(cell_type)?;
            }
            UICommand::DeleteCell(cell_id) => {
                self.delete_cell(cell_id)?;
            }
            UICommand::ExecuteCell(cell_id) => {
                self.execute_cell(cell_id)?;
            }
            UICommand::ExecuteAll => {
                self.execute_all_cells()?;
            }
            UICommand::Save => {
                self.save_notebook()?;
            }
            UICommand::ShowHelp => {
                self.show_help();
            }
            _ => {
                self.set_status_message("命令尚未实现".to_string());
            }
        }
        
        Ok(())
    }
    
    /// 创建新单元格
    fn create_cell(&mut self, cell_type: CellType) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            let cell = match cell_type {
                CellType::Code => NotebookCell::new_code("".to_string()),
                CellType::Text => NotebookCell::new_text("".to_string()),
                CellType::Markdown => NotebookCell::new_markdown("".to_string()),
                CellType::Output => return Err(NotebookError::Cell("不能手动创建输出单元格".to_string())),
            };
            
            let insert_index = self.current_cell.map(|i| i + 1).unwrap_or(0);
            notebook.insert_cell(insert_index, cell)?;
            self.current_cell = Some(insert_index);
            self.edit_mode = true;
            
            self.set_status_message(format!("创建了新的{}单元格", cell_type.display_name()));
        }
        
        Ok(())
    }
    
    /// 删除单元格
    fn delete_cell(&mut self, cell_id: CellId) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            if let Some((index, _)) = notebook.find_cell(&cell_id) {
                notebook.remove_cell(index)?;
                
                // 调整当前单元格索引
                if let Some(current) = self.current_cell {
                    if current >= notebook.cell_count() && notebook.cell_count() > 0 {
                        self.current_cell = Some(notebook.cell_count() - 1);
                    } else if notebook.cell_count() == 0 {
                        self.current_cell = None;
                    }
                }
                
                self.set_status_message("删除了单元格".to_string());
            }
        }
        
        Ok(())
    }
    
    /// 执行单元格
    fn execute_cell(&mut self, cell_id: CellId) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            if let Some((_, cell)) = notebook.find_cell_mut(&cell_id) {
                let result = self.execution_engine.execute_cell(cell)?;
                
                match result {
                    super::ExecutionResult::Success { execution_time, .. } => {
                        self.set_status_message(format!("执行成功 ({:.2}ms)", execution_time.as_millis()));
                    }
                    super::ExecutionResult::Error { error, .. } => {
                        self.set_status_message(format!("执行错误: {}", error));
                    }
                    super::ExecutionResult::Skipped => {
                        self.set_status_message("跳过执行（非代码单元格）".to_string());
                    }
                    super::ExecutionResult::Cancelled => {
                        self.set_status_message("执行被取消".to_string());
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行所有单元格
    fn execute_all_cells(&mut self) -> NotebookResult<()> {
        let cell_ids: Vec<_> = if let Some(notebook) = &self.notebook {
            notebook.get_code_cells().into_iter().map(|(_, cell)| cell.id).collect()
        } else {
            Vec::new()
        };
        
        let total = cell_ids.len();
        let mut executed = 0;
        
        for cell_id in cell_ids {
            self.execute_cell(cell_id)?;
            executed += 1;
            
            self.set_status_message(format!("执行进度: {}/{}", executed, total));
        }
        
        self.set_status_message(format!("执行完成: {}/{} 个单元格", executed, total));
        
        Ok(())
    }
    
    /// 保存笔记本
    fn save_notebook(&mut self) -> NotebookResult<()> {
        if let Some(notebook) = &mut self.notebook {
            if let Some(path) = notebook.get_file_path().cloned() {
                super::NotebookSerializer::save_to_file(notebook, path)?;
                self.set_status_message("笔记本已保存".to_string());
            } else {
                self.set_status_message("请先指定文件路径".to_string());
            }
        }
        
        Ok(())
    }
    
    /// 显示帮助
    fn show_help(&mut self) {
        let help_text = self.generate_help_text();
        self.set_status_message(help_text);
    }
    
    /// 生成帮助文本
    fn generate_help_text(&self) -> String {
        let mut help = String::from("快捷键帮助:\n");
        
        for binding in &self.key_bindings {
            let key_desc = self.format_key_event(&binding.key_event);
            help.push_str(&format!("  {} - {}\n", key_desc, binding.description));
        }
        
        help
    }
    
    /// 格式化键盘事件
    fn format_key_event(&self, event: &KeyEvent) -> String {
        let mut parts = Vec::new();
        
        if event.modifiers.ctrl {
            parts.push("Ctrl");
        }
        if event.modifiers.alt {
            parts.push("Alt");
        }
        if event.modifiers.shift {
            parts.push("Shift");
        }
        
        let key_str = match &event.key {
            Key::Char(c) => c.to_string(),
            Key::Function(n) => format!("F{}", n),
            Key::Arrow(arrow) => match arrow {
                ArrowKey::Up => "↑".to_string(),
                ArrowKey::Down => "↓".to_string(),
                ArrowKey::Left => "←".to_string(),
                ArrowKey::Right => "→".to_string(),
            },
            Key::Special(special) => match special {
                SpecialKey::Enter => "Enter".to_string(),
                SpecialKey::Tab => "Tab".to_string(),
                SpecialKey::Backspace => "Backspace".to_string(),
                SpecialKey::Delete => "Delete".to_string(),
                SpecialKey::Home => "Home".to_string(),
                SpecialKey::End => "End".to_string(),
                SpecialKey::PageUp => "PageUp".to_string(),
                SpecialKey::PageDown => "PageDown".to_string(),
                SpecialKey::Escape => "Escape".to_string(),
            },
        };
        
        parts.push(&key_str);
        parts.join("+")
    }
    
    /// 设置状态消息
    fn set_status_message(&mut self, message: String) {
        self.status_message = Some(message);
        self.status_timeout = Some(std::time::Instant::now() + Duration::from_secs(3));
    }
    
    /// 渲染界面
    fn render(&mut self) -> NotebookResult<()> {
        // 清屏
        print!("\x1b[2J\x1b[H");
        
        // 渲染标题栏
        self.render_title_bar()?;
        
        // 渲染单元格
        self.render_cells()?;
        
        // 渲染状态栏
        self.render_status_bar()?;
        
        io::stdout().flush().map_err(|e| NotebookError::Io(e))?;
        Ok(())
    }
    
    /// 渲染标题栏
    fn render_title_bar(&self) -> NotebookResult<()> {
        let title = if let Some(notebook) = &self.notebook {
            format!("Yufmath Notebook - {}", notebook.metadata.title)
        } else {
            "Yufmath Notebook".to_string()
        };
        
        let width = self.terminal_size.0 as usize;
        let padding = if title.len() < width {
            " ".repeat(width - title.len())
        } else {
            String::new()
        };
        
        println!("\x1b[7m{}{}\x1b[0m", title, padding);
        Ok(())
    }
    
    /// 渲染单元格
    fn render_cells(&self) -> NotebookResult<()> {
        if let Some(notebook) = &self.notebook {
            let visible_height = self.terminal_size.1 as usize - 3;
            let end_index = std::cmp::min(
                self.scroll_offset + visible_height,
                notebook.cell_count()
            );
            
            for i in self.scroll_offset..end_index {
                if let Some(cell) = notebook.get_cell(i) {
                    let is_current = Some(i) == self.current_cell;
                    self.render_cell(cell, is_current, i)?;
                }
            }
        } else {
            println!("没有打开的笔记本");
        }
        
        Ok(())
    }
    
    /// 渲染单个单元格
    fn render_cell(&self, cell: &NotebookCell, is_current: bool, index: usize) -> NotebookResult<()> {
        let prefix = if is_current { ">" } else { " " };
        let cell_type = cell.cell_type.display_name();
        
        println!("{} [{}] {}: {}", 
                prefix, 
                index + 1, 
                cell_type, 
                cell.get_text().lines().next().unwrap_or(""));
        
        // 如果有输出，显示输出
        if let Some(output) = cell.get_output() {
            println!("    输出: {}", output.get_text());
        }
        
        Ok(())
    }
    
    /// 渲染状态栏
    fn render_status_bar(&mut self) -> NotebookResult<()> {
        // 检查状态消息是否过期
        if let Some(timeout) = self.status_timeout {
            if std::time::Instant::now() > timeout {
                self.status_message = None;
                self.status_timeout = None;
            }
        }
        
        let status = if let Some(message) = &self.status_message {
            message.clone()
        } else {
            let mode = if self.edit_mode { "编辑" } else { "命令" };
            let cell_info = if let Some(current) = self.current_cell {
                format!("单元格 {}", current + 1)
            } else {
                "无单元格".to_string()
            };
            
            format!("{} 模式 | {} | Ctrl+H 显示帮助", mode, cell_info)
        };
        
        let width = self.terminal_size.0 as usize;
        let padding = if status.len() < width {
            " ".repeat(width - status.len())
        } else {
            String::new()
        };
        
        println!("\x1b[7m{}{}\x1b[0m", status, padding);
        Ok(())
    }
}

impl Default for NotebookUI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_key_modifiers() {
        let none = KeyModifiers::none();
        assert!(!none.ctrl && !none.alt && !none.shift);
        
        let ctrl = KeyModifiers::ctrl();
        assert!(ctrl.ctrl && !ctrl.alt && !ctrl.shift);
        
        let shift = KeyModifiers::shift();
        assert!(!shift.ctrl && !shift.alt && shift.shift);
    }
    
    #[test]
    fn test_key_binding() {
        let binding = KeyBinding::new(
            Key::Char('s'),
            KeyModifiers::ctrl(),
            UICommand::Save,
            "保存文件"
        );
        
        let matching_event = KeyEvent {
            key: Key::Char('s'),
            modifiers: KeyModifiers::ctrl(),
        };
        
        let non_matching_event = KeyEvent {
            key: Key::Char('s'),
            modifiers: KeyModifiers::none(),
        };
        
        assert!(binding.matches(&matching_event));
        assert!(!binding.matches(&non_matching_event));
    }
    
    #[test]
    fn test_notebook_ui_creation() {
        let ui = NotebookUI::new();
        assert!(ui.notebook.is_none());
        assert!(ui.current_cell.is_none());
        assert!(!ui.edit_mode);
        assert!(!ui.key_bindings.is_empty());
    }
    
    #[test]
    fn test_set_notebook() {
        let mut ui = NotebookUI::new();
        let mut notebook = Notebook::with_title("测试笔记本".to_string());
        notebook.add_cell(NotebookCell::new_code("test".to_string()));
        
        ui.set_notebook(notebook);
        assert!(ui.notebook.is_some());
        assert_eq!(ui.current_cell, Some(0));
        assert_eq!(ui.scroll_offset, 0);
    }
    
    #[test]
    fn test_status_message() {
        let mut ui = NotebookUI::new();
        
        ui.set_status_message("测试消息".to_string());
        assert_eq!(ui.status_message, Some("测试消息".to_string()));
        assert!(ui.status_timeout.is_some());
    }
}