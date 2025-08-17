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
    /// 求解方程
    Solve {
        /// 要求解的方程
        equation: String,
        /// 求解变量
        variable: String,
    },
    /// 因式分解
    Factor {
        /// 要分解的表达式
        expression: String,
    },
    /// 展开表达式
    Expand {
        /// 要展开的表达式
        expression: String,
    },
    /// 计算极限
    Limit {
        /// 表达式
        expression: String,
        /// 变量
        variable: String,
        /// 趋向点
        point: String,
    },
    /// 级数展开
    Series {
        /// 表达式
        expression: String,
        /// 变量
        variable: String,
        /// 展开点
        point: String,
        /// 展开阶数
        #[arg(short, long, default_value = "5")]
        order: usize,
    },
    /// 批处理模式
    Batch {
        /// 输入文件路径
        #[arg(short, long)]
        input: String,
        /// 输出文件路径（可选）
        #[arg(short, long)]
        output: Option<String>,
    },
    /// 启动交互模式
    Interactive,
    /// 启动笔记本模式
    Notepad {
        /// 笔记本文件路径（可选，如果不提供则创建新笔记本）
        file: Option<String>,
        /// 笔记本标题（创建新笔记本时使用）
        #[arg(short, long)]
        title: Option<String>,
        /// 使用终端界面模式（默认使用图形界面）
        #[arg(long)]
        terminal: bool,
    },
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