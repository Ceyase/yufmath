//! # Rust API 接口
//!
//! 本模块提供 Yufmath 库的主要 Rust API 接口，
//! 包括主要的 Yufmath 结构和相关配置选项。

pub mod yufmath;
pub mod config;
pub mod progress;
pub mod error;

pub use yufmath::Yufmath;
pub use config::{ComputeConfig, PrecisionConfig};
pub use progress::{ComputeProgress, PerformanceStats, PerformanceMonitor, ProgressCallback};
pub use error::YufmathError;