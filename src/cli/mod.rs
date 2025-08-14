//! # 命令行工具
//!
//! 本模块实现 Yufmath 的命令行界面，
//! 提供交互式计算和批处理功能。

pub mod commands;
pub mod interactive;
pub mod args;

pub use args::CliArgs;
pub use commands::run_command;
pub use interactive::run_interactive;