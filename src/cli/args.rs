//! # 命令行参数定义
//!
//! 定义命令行工具的参数结构。

use clap::{Parser, Subcommand};

/// Yufmath 命令行工具
#[derive(Parser)]
#[command(name = "yufmath")]
#[command(about = "一个基于 Rust 的高性能计算机代数系统")]
#[command(version)]
pub struct CliArgs {
    /// 子命令
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// 输出格式
    #[arg(short, long, value_enum, default_value = "standard")]
    pub format: OutputFormat,
    
    /// 数值精度
    #[arg(short, long)]
    pub precision: Option<usize>,
    
    /// 详细输出
    #[arg(short, long)]
    pub verbose: bool,
    
    /// 静默模式
    #[arg(short, long)]
    pub quiet: bool,
    
    /// 显示计算进度条
    #[arg(long)]
    pub progress: bool,
    
    /// 禁用进度条
    #[arg(long)]
    pub no_progress: bool,
    
    /// 计算超时时间（秒）
    #[arg(long)]
    pub timeout: Option<u64>,
}

/// 子命令定义
#[derive(Subcommand)]
pub enum Commands {
    /// 计算表达式
    Compute {
        /// 要计算的表达式
        expression: String,
    },
    /// 简化表达式
    Simplify {
        /// 要简化的表达式
        expression: String,
    },
    /// 对变量求导
    Diff {
        /// 要求导的表达式
        expression: String,
        /// 求导变量
        variable: String,
    },
    /// 对变量积分
    Integrate {
        /// 要积分的表达式
        expression: String,
        /// 积分变量
        variable: String,
    },
    /// 启动交互模式
    Interactive,
}

/// 输出格式
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// 标准格式
    Standard,
    /// LaTeX 格式
    Latex,
    /// MathML 格式
    Mathml,
}