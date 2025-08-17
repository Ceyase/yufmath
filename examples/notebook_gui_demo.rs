//! # 笔记本 GUI 演示程序
//!
//! 演示如何使用 Yufmath 的笔记本图形用户界面。

use yufmath::notebook::{NotebookGUI, Notebook, NotebookCell, CellType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("启动 Yufmath 笔记本 GUI 演示...");
    
    // 检查是否有图形环境
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        eprintln!("错误：没有检测到图形环境");
        eprintln!("请确保：");
        eprintln!("1. 在图形桌面环境中运行");
        eprintln!("2. 设置了 DISPLAY 或 WAYLAND_DISPLAY 环境变量");
        eprintln!("3. 如果使用 SSH，请使用 -X 或 -Y 参数启用 X11 转发");
        std::process::exit(1);
    }
    
    // 创建演示笔记本
    let mut notebook = create_demo_notebook();
    
    // 创建并启动 GUI
    let mut gui = NotebookGUI::new();
    gui.set_notebook(notebook)?;
    
    println!("GUI 已启动，请在图形界面中操作...");
    println!("快捷键提示：");
    println!("  Ctrl+N  - 新建代码单元格");
    println!("  Ctrl+M  - 新建文档单元格");
    println!("  Ctrl+Enter - 执行当前单元格");
    println!("  Shift+Enter - 执行当前单元格并新建");
    println!("  Ctrl+S  - 保存笔记本");
    println!("  Ctrl+O  - 打开笔记本");
    println!("  Ctrl+Q  - 退出");
    
    // 运行 GUI 主循环
    gui.run()?;
    
    println!("GUI 已关闭");
    Ok(())
}

/// 创建演示笔记本
fn create_demo_notebook() -> Notebook {
    let mut notebook = Notebook::with_title("Yufmath 演示笔记本".to_string());
    
    // 添加欢迎文档
    notebook.add_cell(NotebookCell::new_markdown(
        "# 欢迎使用 Yufmath 笔记本\n\n这是一个基于 FLTK 的图形界面笔记本，支持交互式数学计算。".to_string()
    ));
    
    // 添加基本计算示例
    notebook.add_cell(NotebookCell::new_markdown(
        "## 基本计算\n\n让我们从一些简单的计算开始：".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "2 + 3 * 4".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "sqrt(16)".to_string()
    ));
    
    // 添加代数运算示例
    notebook.add_cell(NotebookCell::new_markdown(
        "## 代数运算\n\n符号计算和表达式简化：".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "x^2 + 2*x + 1".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "simplify((x+1)^2)".to_string()
    ));
    
    // 添加微积分示例
    notebook.add_cell(NotebookCell::new_markdown(
        "## 微积分\n\n求导和积分运算：".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "diff(x^3 + 2*x^2 + x, x)".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "integrate(2*x + 1, x)".to_string()
    ));
    
    // 添加三角函数示例
    notebook.add_cell(NotebookCell::new_markdown(
        "## 三角函数\n\n三角函数计算：".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "sin(pi/2)".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "cos(0)".to_string()
    ));
    
    // 添加矩阵运算示例
    notebook.add_cell(NotebookCell::new_markdown(
        "## 矩阵运算\n\n矩阵和向量计算：".to_string()
    ));
    
    notebook.add_cell(NotebookCell::new_code(
        "matrix([[1,2],[3,4]]) * matrix([[5,6],[7,8]])".to_string()
    ));
    
    // 添加使用说明
    notebook.add_cell(NotebookCell::new_markdown(
        "## 使用说明\n\n### 快捷键\n\n- **Ctrl+N**: 新建代码单元格\n- **Ctrl+M**: 新建文档单元格\n- **Ctrl+Enter**: 执行当前单元格\n- **Shift+Enter**: 执行当前单元格并新建\n- **Ctrl+D**: 删除当前单元格\n- **Ctrl+S**: 保存笔记本\n- **Ctrl+O**: 打开笔记本\n\n### 单元格类型\n\n- **代码单元格**: 用于输入和执行数学表达式\n- **文档单元格**: 用于编写 Markdown 格式的文档\n- **文本单元格**: 用于编写纯文本\n\n### 语法高亮\n\n代码单元格支持语法高亮，帮助您更好地编写数学表达式。".to_string()
    ));
    
    notebook
}