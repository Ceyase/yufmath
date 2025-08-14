//! # 外部函数接口（FFI）
//!
//! 本模块提供与其他语言（如 C++）的互操作接口，
//! 实现 C 兼容的 API 以供其他语言调用。

pub mod c_api;
pub mod types;

pub use c_api::*;
pub use types::*;