//! # 命令行命令实现
//!
//! 实现各种命令行命令的处理逻辑。

use super::args::{CliArgs, Commands, OutputFormat};
use crate::notebook::{Notebook, NotebookFormat, NotebookDeserializer, NotebookUI};
use crate::api::Yufmath;
use std::path::Path;

/// 运行命令行命令
pub fn run_command(args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    match args.command {
        Some(Commands::Notepad { file, title }) => {
            run_notepad_command(file, title)?;
        }
        Some(Commands::Interactive) => {
            crate::cli::run_interactive()?;
        }
        Some(Commands::Compute { expression }) => {
            let yuf = Yufmath::new();
            let result = yuf.compute(&expression)?;
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Simplify { expression }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let simplified = yuf.simplify(&expr)?;
            let result = yuf.format(&simplified);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Diff { expression, variable }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let derivative = yuf.diff(&expr, &variable)?;
            let result = yuf.format(&derivative);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Integrate { expression, variable }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let integral = yuf.integrate(&expr, &variable)?;
            let result = yuf.format(&integral);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Solve { equation, variable }) => {
            let yuf = Yufmath::new();
            let eq = yuf.parse(&equation)?;
            let solutions = yuf.solve(&eq, &variable)?;
            for (i, solution) in solutions.iter().enumerate() {
                let result = yuf.format(solution);
                println!("解 {}: {}", i + 1, format_output(&result, &args.format));
            }
        }
        Some(Commands::Factor { expression }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let factored = yuf.factor(&expr)?;
            let result = yuf.format(&factored);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Expand { expression }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let expanded = yuf.expand(&expr)?;
            let result = yuf.format(&expanded);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Limit { expression, variable, point }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let point_expr = yuf.parse(&point)?;
            let limit = yuf.limit(&expr, &variable, &point_expr)?;
            let result = yuf.format(&limit);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Series { expression, variable, point, order }) => {
            let yuf = Yufmath::new();
            let expr = yuf.parse(&expression)?;
            let point_expr = yuf.parse(&point)?;
            let series = yuf.series(&expr, &variable, &point_expr, order)?;
            let result = yuf.format(&series);
            println!("{}", format_output(&result, &args.format));
        }
        Some(Commands::Batch { ref input, ref output }) => {
            run_batch_command(input, output.as_deref(), &args)?;
        }
        None => {
            // 如果没有提供子命令，启动交互模式
            crate::cli::run_interactive()?;
        }
    }
    
    Ok(())
}

/// 运行笔记本命令
fn run_notepad_command(file: Option<String>, title: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let notebook = if let Some(file_path) = file {
        let path = Path::new(&file_path);
        
        if path.exists() {
            // 加载现有笔记本
            println!("正在加载笔记本: {}", file_path);
            NotebookDeserializer::load_from_file(path)?
        } else {
            // 创建新笔记本并保存到指定路径
            let notebook_title = title.unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("新笔记本")
                    .to_string()
            });
            
            println!("正在创建新笔记本: {} ({})", notebook_title, file_path);
            let mut notebook = NotebookFormat::create_template(&notebook_title);
            
            // 保存到文件
            crate::notebook::NotebookSerializer::save_to_file(&mut notebook, path)?;
            notebook
        }
    } else {
        // 创建临时笔记本
        let notebook_title = title.unwrap_or_else(|| "临时笔记本".to_string());
        println!("正在创建临时笔记本: {}", notebook_title);
        NotebookFormat::create_template(&notebook_title)
    };
    
    // 启动笔记本界面
    let mut ui = NotebookUI::with_notebook(notebook);
    ui.run()?;
    
    Ok(())
}

/// 运行批处理命令
fn run_batch_command(input: &str, output: Option<&str>, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Write;
    
    let input_content = fs::read_to_string(input)?;
    let lines: Vec<&str> = input_content.lines().collect();
    
    let yuf = Yufmath::new();
    let mut results = Vec::new();
    
    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // 跳过空行和注释
        }
        
        match yuf.compute(line) {
            Ok(result) => {
                let formatted = format_output(&result, &args.format);
                results.push(format!("输入 {}: {}", line_num + 1, line));
                results.push(format!("输出 {}: {}", line_num + 1, formatted));
                
                if !args.quiet {
                    println!("输入 {}: {}", line_num + 1, line);
                    println!("输出 {}: {}", line_num + 1, formatted);
                }
            }
            Err(e) => {
                let error_msg = format!("错误 {}: {}", line_num + 1, e);
                results.push(error_msg.clone());
                
                if !args.quiet {
                    eprintln!("{}", error_msg);
                }
            }
        }
    }
    
    // 如果指定了输出文件，写入结果
    if let Some(output_path) = output {
        let mut file = fs::File::create(output_path)?;
        for result in results {
            writeln!(file, "{}", result)?;
        }
        
        if !args.quiet {
            println!("结果已保存到: {}", output_path);
        }
    }
    
    Ok(())
}

/// 格式化输出
fn format_output(result: &str, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Standard => result.to_string(),
        OutputFormat::Latex => {
            // 如果结果已经是 LaTeX 格式，直接返回
            if result.starts_with('$') && result.ends_with('$') {
                result.to_string()
            } else {
                format!("${result}$")
            }
        }
        OutputFormat::Mathml => {
            // 简单的 MathML 包装
            format!("<math><mrow>{}</mrow></math>", result)
        }
    }
}