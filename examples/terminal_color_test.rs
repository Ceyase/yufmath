//! # 终端颜色支持测试
//!
//! 测试 Windows 终端的 ANSI 颜色支持是否正常工作。

use yufmath::cli::terminal::{init_terminal, supports_color, ColorConfig};
use ansi_term::Colour;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Yufmath 终端颜色支持测试 ===\n");
    
    // 初始化终端
    println!("1. 初始化终端...");
    match init_terminal() {
        Ok(()) => println!("   ✓ 终端初始化成功"),
        Err(e) => {
            println!("   ✗ 终端初始化失败: {}", e);
            println!("   注意: 这可能会影响颜色输出");
        }
    }
    
    // 检查颜色支持
    println!("\n2. 检查颜色支持...");
    let color_supported = supports_color();
    if color_supported {
        println!("   ✓ 终端支持颜色输出");
    } else {
        println!("   ✗ 终端不支持颜色输出或颜色已被禁用");
    }
    
    // 测试颜色配置
    println!("\n3. 测试颜色配置...");
    let color_config = ColorConfig::from_env();
    println!("   强制颜色: {}", color_config.force_color);
    println!("   禁用颜色: {}", color_config.no_color);
    println!("   自动检测: {}", color_config.auto_detect);
    println!("   应该使用颜色: {}", color_config.should_use_color());
    
    // 测试基本颜色输出
    println!("\n4. 测试基本颜色输出...");
    if color_config.should_use_color() {
        println!("   {}", Colour::Red.paint("红色文本"));
        println!("   {}", Colour::Green.paint("绿色文本"));
        println!("   {}", Colour::Blue.paint("蓝色文本"));
        println!("   {}", Colour::Yellow.paint("黄色文本"));
        println!("   {}", Colour::Cyan.paint("青色文本"));
        println!("   {}", Colour::Purple.paint("紫色文本"));
        println!("   {}", Colour::White.bold().paint("粗体白色文本"));
        println!("   {}", Colour::Black.bold().paint("粗体黑色文本"));
    } else {
        println!("   颜色输出已禁用，显示纯文本:");
        println!("   红色文本");
        println!("   绿色文本");
        println!("   蓝色文本");
        println!("   黄色文本");
        println!("   青色文本");
        println!("   紫色文本");
        println!("   粗体白色文本");
        println!("   粗体黑色文本");
    }
    
    // 测试数学符号和颜色组合
    println!("\n5. 测试数学符号和颜色组合...");
    if color_config.should_use_color() {
        println!("   表达式: {} {} {} {} {}",
            Colour::Cyan.bold().paint("2"),
            Colour::Yellow.bold().paint("+"),
            Colour::Cyan.bold().paint("3"),
            Colour::Yellow.bold().paint("×"),
            Colour::Green.bold().paint("x"));
        
        println!("   函数: {}{}{}{}",
            Colour::Blue.bold().paint("sin"),
            Colour::White.paint("("),
            Colour::Purple.bold().paint("π"),
            Colour::White.paint(")"));
        
        println!("   根号: {}{}{}{}",
            Colour::Blue.bold().paint("√"),
            Colour::White.paint("("),
            Colour::Cyan.bold().paint("2"),
            Colour::White.paint(")"));
    } else {
        println!("   表达式: 2 + 3 × x");
        println!("   函数: sin(π)");
        println!("   根号: √(2)");
    }
    
    // 环境变量信息
    println!("\n6. 环境变量信息...");
    if let Ok(term) = std::env::var("TERM") {
        println!("   TERM = {}", term);
    } else {
        println!("   TERM 未设置");
    }
    
    if std::env::var("NO_COLOR").is_ok() {
        println!("   NO_COLOR 已设置 (禁用颜色)");
    }
    
    if std::env::var("FORCE_COLOR").is_ok() {
        println!("   FORCE_COLOR 已设置 (强制颜色)");
    }
    
    // 平台信息
    println!("\n7. 平台信息...");
    println!("   操作系统: {}", std::env::consts::OS);
    println!("   架构: {}", std::env::consts::ARCH);
    
    #[cfg(windows)]
    {
        println!("   Windows 平台: 已启用 ENABLE_VIRTUAL_TERMINAL_PROCESSING 支持");
    }
    
    #[cfg(not(windows))]
    {
        println!("   非 Windows 平台: 通常默认支持 ANSI 颜色");
    }
    
    println!("\n=== 测试完成 ===");
    
    if color_config.should_use_color() {
        println!("\n{}", Colour::Green.bold().paint("如果您能看到上面的彩色文本，说明颜色支持正常工作！"));
    } else {
        println!("\n如果您看不到彩色文本，这可能是因为:");
        println!("- 终端不支持 ANSI 颜色");
        println!("- 设置了 NO_COLOR 环境变量");
        println!("- 输出被重定向到文件");
        println!("- Windows 终端需要启用虚拟终端处理");
    }
    
    Ok(())
}