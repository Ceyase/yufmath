//! # 终端初始化模块
//!
//! 处理跨平台的终端初始化，特别是 Windows 系统上的 ANSI 颜色支持。

#[cfg(windows)]
use std::ffi::c_void;

/// 初始化终端以支持 ANSI 颜色输出
/// 
/// 在 Windows 系统上，需要启用 ENABLE_VIRTUAL_TERMINAL_PROCESSING 标志
/// 以支持 ANSI 转义序列（颜色输出）。
/// 
/// 在其他系统上，通常默认支持 ANSI 颜色，无需特殊处理。
pub fn init_terminal() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    {
        init_windows_terminal()?;
    }
    
    // 在非 Windows 系统上，通常不需要特殊初始化
    #[cfg(not(windows))]
    {
        // 可以在这里添加其他平台的特殊处理
    }
    
    Ok(())
}

#[cfg(windows)]
fn init_windows_terminal() -> Result<(), Box<dyn std::error::Error>> {
    // Windows API 常量
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5_u32; // -11 as u32
    const STD_ERROR_HANDLE: u32 = 0xFFFFFFF4_u32;  // -12 as u32
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    const ENABLE_PROCESSED_OUTPUT: u32 = 0x0001;
    
    // 动态链接 Windows API 函数
    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: *mut c_void, dwMode: u32) -> i32;
    }
    
    unsafe {
        // 处理标准输出
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if !stdout_handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                // 启用虚拟终端处理和处理输出
                let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT;
                if SetConsoleMode(stdout_handle, new_mode) == 0 {
                    eprintln!("警告: 无法为标准输出启用 ANSI 颜色支持");
                }
            }
        }
        
        // 处理标准错误输出
        let stderr_handle = GetStdHandle(STD_ERROR_HANDLE);
        if !stderr_handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(stderr_handle, &mut mode) != 0 {
                // 启用虚拟终端处理和处理输出
                let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT;
                if SetConsoleMode(stderr_handle, new_mode) == 0 {
                    eprintln!("警告: 无法为标准错误输出启用 ANSI 颜色支持");
                }
            }
        }
    }
    
    Ok(())
}

/// 检查终端是否支持颜色输出
/// 
/// 这个函数尝试检测当前终端是否支持 ANSI 颜色。
/// 在某些情况下（如重定向到文件），应该禁用颜色输出。
pub fn supports_color() -> bool {
    // 检查是否在交互式终端中运行
    if !atty::is(atty::Stream::Stdout) {
        return false;
    }
    
    // 检查环境变量
    if let Ok(term) = std::env::var("TERM") {
        if term == "dumb" {
            return false;
        }
    }
    
    // 检查 NO_COLOR 环境变量
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }
    
    // 检查 FORCE_COLOR 环境变量
    if std::env::var("FORCE_COLOR").is_ok() {
        return true;
    }
    
    // 在 Windows 上，检查是否成功启用了虚拟终端处理
    #[cfg(windows)]
    {
        return check_windows_color_support();
    }
    
    // 在其他系统上，默认支持颜色
    #[cfg(not(windows))]
    {
        true
    }
}

#[cfg(windows)]
fn check_windows_color_support() -> bool {
    const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5_u32; // -11 as u32
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
    
    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
        fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
    }
    
    unsafe {
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if !stdout_handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                return (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0;
            }
        }
    }
    
    false
}

/// 终端颜色配置
#[derive(Debug, Clone)]
pub struct ColorConfig {
    /// 是否强制启用颜色
    pub force_color: bool,
    /// 是否强制禁用颜色
    pub no_color: bool,
    /// 自动检测颜色支持
    pub auto_detect: bool,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            force_color: false,
            no_color: false,
            auto_detect: true,
        }
    }
}

impl ColorConfig {
    /// 根据配置和环境确定是否应该使用颜色
    pub fn should_use_color(&self) -> bool {
        if self.no_color {
            return false;
        }
        
        if self.force_color {
            return true;
        }
        
        if self.auto_detect {
            return supports_color();
        }
        
        false
    }
    
    /// 从环境变量创建颜色配置
    pub fn from_env() -> Self {
        let force_color = std::env::var("FORCE_COLOR").is_ok();
        let no_color = std::env::var("NO_COLOR").is_ok();
        
        Self {
            force_color,
            no_color,
            auto_detect: !force_color && !no_color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_config_default() {
        let config = ColorConfig::default();
        assert!(!config.force_color);
        assert!(!config.no_color);
        assert!(config.auto_detect);
    }
    
    #[test]
    fn test_color_config_force_color() {
        let config = ColorConfig {
            force_color: true,
            no_color: false,
            auto_detect: false,
        };
        assert!(config.should_use_color());
    }
    
    #[test]
    fn test_color_config_no_color() {
        let config = ColorConfig {
            force_color: false,
            no_color: true,
            auto_detect: false,
        };
        assert!(!config.should_use_color());
    }
    
    #[test]
    fn test_color_config_no_color_overrides_force() {
        let config = ColorConfig {
            force_color: true,
            no_color: true,
            auto_detect: false,
        };
        // no_color 应该覆盖 force_color
        assert!(!config.should_use_color());
    }
}