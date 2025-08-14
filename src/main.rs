//! # Yufmath 命令行工具
//!
//! 提供命令行接口来使用 Yufmath 计算机代数系统。

use clap::Parser;
use std::process;
use yufmath::cli::args::{CliArgs, Commands, OutputFormat};
use yufmath::cli::interactive;
use yufmath::formatter::{FormatOptions, FormatType};
use yufmath::Yufmath;

fn main() {
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
        Some(Commands::Interactive) => {
            handle_interactive(&args)
        }
        None => {
            // 如果没有指定子命令，显示帮助信息
            println!("Yufmath v{} - 计算机代数系统", yufmath::VERSION);
            println!("使用 --help 查看可用命令");
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
    if args.verbose {
        println!("正在计算表达式: {}", expression);
    }
    
    let result = yuf.compute(expression)?;
    
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
    // 注意：这里需要使用 formatter 来格式化结果
    // 由于当前 Yufmath 结构体没有公开 formatter，我们暂时使用 Debug 格式
    let result = format!("{:?}", simplified);
    
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
    // 暂时使用 Debug 格式，后续会添加 format 方法
    let result = format!("{:?}", derivative);
    
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
    // 暂时使用 Debug 格式，后续会添加 format 方法
    let result = format!("{:?}", integral);
    
    if !args.quiet {
        println!("{}", result);
    }
    
    Ok(())
}

/// 处理交互模式
fn handle_interactive(args: &CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.verbose {
        println!("启动交互模式...");
    }
    
    interactive::run_interactive()
}