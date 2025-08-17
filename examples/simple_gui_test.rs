//! # 简单的 GUI 测试程序
//!
//! 测试 FLTK GUI 是否能正常工作。

use yufmath::notebook::{NotebookGUI, Notebook, NotebookCell, CellType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试 FLTK GUI 初始化...");
    
    // 检查图形环境
    if std::env::var("DISPLAY").is_err() && 
       std::env::var("WAYLAND_DISPLAY").is_err() && 
       !cfg!(target_os = "windows") && 
       !cfg!(target_os = "macos") {
        eprintln!("错误：没有检测到图形环境");
        eprintln!("请确保：");
        eprintln!("1. 在图形桌面环境中运行");
        eprintln!("2. 设置了 DISPLAY 或 WAYLAND_DISPLAY 环境变量");
        eprintln!("3. 如果使用 SSH，请使用 -X 或 -Y 参数启用 X11 转发");
        std::process::exit(1);
    }
    
    // 创建简单的测试笔记本
    let mut notebook = Notebook::with_title("GUI 测试笔记本".to_string());
    notebook.add_cell(NotebookCell::new_markdown("# GUI 测试\n\n这是一个简单的 GUI 测试。".to_string()));
    notebook.add_cell(NotebookCell::new_code("2 + 3".to_string()));
    notebook.add_cell(NotebookCell::new_code("sin(pi/2)".to_string()));
    
    println!("创建 GUI...");
    
    // 创建并运行 GUI
    let mut gui = NotebookGUI::new();
    gui.set_notebook(notebook)?;
    
    println!("启动 GUI 主循环...");
    gui.run()?;
    
    println!("GUI 已关闭");
    Ok(())
}