//! # Yufmath 命令行工具
//!
//! 提供 Yufmath 库的命令行接口。

use clap::Parser;
use yufmath::cli::{CliArgs, run_command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    
    // 目前只是占位符实现，将在后续任务中完成
    println!("Yufmath v{}", yufmath::VERSION);
    println!("命令行工具将在后续任务中实现");
    
    // run_command(args)?;
    
    Ok(())
}