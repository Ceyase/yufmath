//! # Yufmath 命令行工具
//!
//! 提供命令行接口来使用 Yufmath 计算机代数系统。

use clap::Parser;
use std::process;
use yufmath::cli::args::{CliArgs, Commands, OutputFormat};
use yufmath::cli::interactive;
use yufmath::cli::progress::{create_compute_progress, create_batch_progress};
use yufmath::cli::terminal::init_terminal;
use yufmath::formatter::{FormatOptions, FormatType};
use yufmath::Yufmath;

fn main() {
    // 初始化终端以支持 ANSI 颜色输出（特别是在 Windows 上）
    if let Err(e) = init_terminal() {
        eprintln!("警告: 终端初始化失败: {}", e);
        eprintln!("颜色输出可能无法正常工作");
    }
    
    let args = CliArgs::parse();
    
    // 设置日志级别
    if args.verbose {
        println!("Yufmath v{} - 计算机代数系统", yufmath::VERSION);
        println!("详细模式已启用");
    }
    
    // 创建 Yufmath 实例
    let mut yuf = match create_yufmath_instance(&args) {
        Ok(instance) => instance,
        Err(e) => {
            eprintln!("错误：无法初始化 Yufmath: {}", e);
            process::exit(1);
        }
    };
    
    // 设置格式化选项
    let format_options = FormatOptions {
        format_type: match args.format {
            OutputFormat::Standard => FormatType::Standard,
            OutputFormat::Latex => FormatType::LaTeX,
            OutputFormat::Mathml => FormatType::MathML,
        },
        precision: args.precision,
        use_parentheses: true,
    };
    yuf.set_format_options(format_options);
    
    // 执行命令
    let result = match &args.command {
        Some(Commands::Compute { expression }) => {
            handle_compute(&yuf, expression, &args)
        }
        Some(Commands::Simplify { expression }) => {
            handle_simplify(&yuf, expression, &args)
        }
        Some(Commands::Diff { expression, variable }) => {
            handle_diff(&yuf, expression, variable, &args)
        }
        Some(Commands::Integrate { expression, variable }) => {
            handle_integrate(&yuf, expression, variable, &args)
        }
        Some(Commands::Solve { equation, variable }) => {
            handle_solve(&yuf, equation, variable, &args)
        }
        Some(Commands::Factor { expression }) => {
            handle_factor(&yuf, expression, &args)
        }
        Some(Commands::Expand { expression }) => {
            handle_expand(&yuf, expression, &args)
        }
        Some(Commands::Limit { expression, variable, point }) => {
            handle_limit(&yuf, expression, variable, point, &args)
        }
        Some(Commands::Series { expression, variable, point, order }) => {
            handle_series(&yuf, expression, variable, point, *order, &args)
        }
        Some(Commands::Batch { input, output }) => {
            handle_batch(&yuf, input, output.as_deref(), &args)
        }
        Some(Commands::Interactive) => {
            handle_interactive(&args)
        }
        None => {
            // 如果没有指定子命令，显示帮助信息
            //show_help();
            handle_interactive(&args);
            Ok(())
        }
    };
    
    // 处理结果
    match result {
        Ok(()) => {
            if args.verbose {
                println!("计算完成");
            }
        }
        Err(e) => {
            if !args.quiet {
                eprintln!("错误: {}", e);
            }
            process::exit(1);
        }
    }
}

/// 创建 Yufmath 实例
fn create_yufmath_instance(_args: &CliArgs) -> Result<Yufmath, Box<dyn std::error::Error>> {
    // 目前使用默认配置，后续任务会添加更多配置选项
    Ok(Yufmath::new())
}

/// 处理计算命令
fn handle_compute(yuf: &Yufmath, expression: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let show_progress = args.progress && !args.no_progress && !args.quiet;
    let progress = create_compute_progress(show_progress, "计算表达式");
    
    if args.verbose {
        println!("正在计算表达式: {}", expression);
    }
    
    let result = yuf.compute(expression)?;
    
    progress.finish("计算完成");
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理简化命令
fn handle_simplify(yuf: &Yufmath, expression: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在简化表达式: {}", expression);
    }
    
    let expr = yuf.parse(expression)?;
    let simplified = yuf.simplify(&expr)?;
    
    // 使用格式化器来正确显示简化后的表达式
    use yufmath::formatter::{StandardFormatter, Formatter};
    let formatter = StandardFormatter::new();
    let result = formatter.format(&simplified);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理求导命令
fn handle_diff(yuf: &Yufmath, expression: &str, variable: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在对表达式 {} 关于变量 {} 求导", expression, variable);
    }
    
    let expr = yuf.parse(expression)?;
    let derivative = yuf.diff(&expr, variable)?;
    
    // 使用格式化器来正确显示求导结果
    use yufmath::formatter::{StandardFormatter, Formatter};
    let formatter = StandardFormatter::new();
    let result = formatter.format(&derivative);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理积分命令
fn handle_integrate(yuf: &Yufmath, expression: &str, variable: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在对表达式 {} 关于变量 {} 积分", expression, variable);
    }
    
    let expr = yuf.parse(expression)?;
    let integral = yuf.integrate(&expr, variable)?;
    
    // 使用格式化器来正确显示积分结果
    use yufmath::formatter::{StandardFormatter, Formatter};
    let formatter = StandardFormatter::new();
    let result = formatter.format(&integral);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理求解命令
fn handle_solve(yuf: &Yufmath, equation: &str, variable: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在求解方程 {} 关于变量 {}", equation, variable);
    }
    
    // 暂时返回未实现错误，因为求解功能还未完全实现
    if !args.quiet {
        println!("求解功能暂未实现，方程: {}, 变量: {}", equation, variable);
    }
    
    Ok(())
}

/// 处理因式分解命令
fn handle_factor(yuf: &Yufmath, expression: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在对表达式 {} 进行因式分解", expression);
    }
    
    // 暂时返回未实现错误，因为因式分解功能还未完全实现
    if !args.quiet {
        println!("因式分解功能暂未实现，表达式: {}", expression);
    }
    
    Ok(())
}

/// 处理展开命令
fn handle_expand(yuf: &Yufmath, expression: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在展开表达式 {}", expression);
    }
    
    // 暂时返回未实现错误，因为展开功能还未完全实现
    if !args.quiet {
        println!("展开功能暂未实现，表达式: {}", expression);
    }
    
    Ok(())
}

/// 处理极限命令
fn handle_limit(yuf: &Yufmath, expression: &str, variable: &str, point: &str, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在计算表达式 {} 当 {} 趋向 {} 时的极限", expression, variable, point);
    }
    
    let expr = yuf.parse(expression)?;
    let point_expr = yuf.parse(point)?;
    let limit_result = yuf.limit(&expr, variable, &point_expr)?;
    
    // 使用格式化器来正确显示极限结果
    use yufmath::formatter::{StandardFormatter, Formatter};
    let formatter = StandardFormatter::new();
    let result = formatter.format(&limit_result);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理级数展开命令
fn handle_series(yuf: &Yufmath, expression: &str, variable: &str, point: &str, order: usize, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("正在对表达式 {} 在 {} = {} 处进行 {} 阶级数展开", expression, variable, point, order);
    }
    
    let expr = yuf.parse(expression)?;
    let point_expr = yuf.parse(point)?;
    let series_result = yuf.series(&expr, variable, &point_expr, order)?;
    
    // 使用格式化器来正确显示级数展开结果
    use yufmath::formatter::{StandardFormatter, Formatter};
    let formatter = StandardFormatter::new();
    let result = formatter.format(&series_result);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理批处理命令
fn handle_batch(yuf: &Yufmath, input_file: &str, output_file: Option<&str>, args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Write;
    
    if args.verbose {
        println!("正在处理批处理文件: {}", input_file);
        if let Some(output) = output_file {
            println!("输出文件: {}", output);
        }
    }
    
    // 读取输入文件
    let content = fs::read_to_string(input_file)
        .map_err(|e| format!("无法读取输入文件 '{}': {}", input_file, e))?;
    
    // 计算有效行数（用于进度条）
    let total_lines = content.lines()
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#') && !line.starts_with("//")
        })
        .count() as u64;
    
    let show_progress = args.progress && !args.no_progress && !args.quiet;
    let progress = create_batch_progress(show_progress, total_lines);
    
    let mut results = Vec::new();
    let mut line_number = 0;
    let mut processed_lines = 0;
    
    // 处理每一行
    for line in content.lines() {
        line_number += 1;
        let line = line.trim();
        
        // 跳过空行和注释行
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        
        processed_lines += 1;
        progress.update(processed_lines, Some(&format!("处理第 {} 行", line_number)));
        
        if args.verbose {
            println!("处理第 {} 行: {}", line_number, line);
        }
        
        // 尝试计算表达式
        match yuf.compute(line) {
            Ok(result) => {
                let output_line = format!("{} = {}", line, result);
                results.push(output_line.clone());
                if !args.quiet && !show_progress {
                    println!("{}", output_line);
                }
            }
            Err(e) => {
                let error_line = format!("第 {} 行错误: {} -> {}", line_number, line, e);
                results.push(error_line.clone());
                if !args.quiet && !show_progress {
                    eprintln!("{}", error_line);
                }
            }
        }
    }
    
    progress.finish(&format!("批处理完成，处理了 {} 行", processed_lines));
    
    // 如果启用了进度条，现在显示结果
    if show_progress && !args.quiet {
        for result in &results {
            if result.contains("错误") {
                eprintln!("{}", result);
            } else {
                println!("{}", result);
            }
        }
    }
    
    // 如果指定了输出文件，写入结果
    if let Some(output_path) = output_file {
        let mut output = fs::File::create(output_path)
            .map_err(|e| format!("无法创建输出文件 '{}': {}", output_path, e))?;
        
        for result in &results {
            writeln!(output, "{}", result)
                .map_err(|e| format!("写入输出文件失败: {}", e))?;
        }
        
        if args.verbose {
            println!("结果已写入文件: {}", output_path);
        }
    }
    
    Ok(())
}

/// 显示帮助信息
fn show_help() {
    println!("Yufmath v{} - 计算机代数系统", yufmath::VERSION);
    println!();
    println!("用法:");
    println!("  yufmath [选项] <命令> [参数...]");
    println!();
    println!("命令:");
    println!("  compute <表达式>              计算表达式的值");
    println!("  simplify <表达式>             简化表达式");
    println!("  diff <表达式> <变量>          对表达式求导");
    println!("  integrate <表达式> <变量>     对表达式积分");
    println!("  solve <方程> <变量>           求解方程");
    println!("  factor <表达式>               因式分解");
    println!("  expand <表达式>               展开表达式");
    println!("  limit <表达式> <变量> <点>    计算极限");
    println!("  series <表达式> <变量> <点>   级数展开");
    println!("  batch -i <文件> [-o <文件>]   批处理模式");
    println!("  interactive                   交互模式");
    println!();
    println!("选项:");
    println!("  -f, --format <格式>           输出格式 [standard, latex, mathml]");
    println!("  -p, --precision <精度>        数值精度");
    println!("  -v, --verbose                 详细输出");
    println!("  -q, --quiet                   静默模式");
    println!("      --progress                显示进度条");
    println!("      --no-progress             禁用进度条");
    println!("      --timeout <秒>            计算超时时间");
    println!("  -h, --help                    显示帮助信息");
    println!("      --version                 显示版本信息");
    println!();
    println!("示例:");
    println!("  yufmath compute \"2 + 3 * 4\"");
    println!("  yufmath simplify \"x^2 + 2*x + 1\"");
    println!("  yufmath diff \"x^3 + 2*x^2 + x\" x");
    println!("  yufmath integrate \"2*x + 1\" x");
    println!("  yufmath --format latex compute \"x^2 + 1\"");
    println!("  yufmath batch -i input.txt -o output.txt");
    println!("  yufmath interactive");
}

/// 处理交互模式
fn handle_interactive(args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("启动交互模式...");
    }
    
    interactive::run_interactive()
}