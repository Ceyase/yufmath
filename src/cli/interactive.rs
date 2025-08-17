//! # 交互式模式
//!
//! 实现 REPL 交互式计算环境。

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::collections::HashMap;
use colored::*;
use crate::Yufmath;
use crate::core::Number;
use crate::formatter::{FormatOptions, FormatType, TerminalFormatter};

/// 交互式会话状态
pub struct InteractiveSession {
    /// Yufmath 计算引擎
    yufmath: Yufmath,
    /// 变量存储
    variables: HashMap<String, Number>,
    /// 格式化选项
    format_options: FormatOptions,
    /// 终端格式化器
    terminal_formatter: TerminalFormatter,
    /// 是否显示详细信息
    verbose: bool,
    /// 是否启用颜色输出
    colors_enabled: bool,
    /// 是否显示数值近似值
    show_approximations: bool,
}

impl InteractiveSession {
    /// 创建新的交互式会话
    pub fn new() -> Self {
        let mut yufmath = Yufmath::new();
        let mut format_options = FormatOptions::default();
        format_options.format_type = FormatType::Terminal;
        yufmath.set_format_options(format_options.clone());
        
        let mut terminal_formatter = TerminalFormatter::new();
        terminal_formatter.set_colors_enabled(true);
        terminal_formatter.set_approximations_enabled(true);
        
        Self {
            yufmath,
            variables: HashMap::new(),
            format_options,
            terminal_formatter,
            verbose: false,
            colors_enabled: true,
            show_approximations: true,
        }
    }
    
    /// 处理用户输入的命令
    pub fn process_command(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.trim();
        
        // 处理空输入
        if input.is_empty() {
            return Ok(String::new());
        }
        
        // 处理特殊命令
        if let Some(result) = self.handle_special_commands(input)? {
            return Ok(result);
        }
        
        // 检查是否是变量赋值
        if let Some((var_name, expression)) = self.parse_assignment(input) {
            return self.handle_assignment(var_name, expression);
        }
        
        // 处理数学表达式
        self.handle_expression(input)
    }
    
    /// 处理特殊命令（如 help、quit、set 等）
    fn handle_special_commands(&mut self, input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match input.to_lowercase().as_str() {
            "help" | "?" => {
                Ok(Some(self.show_help()))
            }
            "quit" | "exit" | "q" => {
                // 退出命令不返回消息，由主循环处理
                Ok(None)
            }
            "clear" => {
                // 清空变量
                self.variables.clear();
                Ok(Some("变量已清空".to_string()))
            }
            "vars" | "variables" => {
                Ok(Some(self.show_variables()))
            }
            "verbose" => {
                self.verbose = !self.verbose;
                let status = if self.verbose { "开启".green() } else { "关闭".red() };
                Ok(Some(format!("详细模式: {}", status)))
            }
            "colors" => {
                self.colors_enabled = !self.colors_enabled;
                self.terminal_formatter.set_colors_enabled(self.colors_enabled);
                let status = if self.colors_enabled { "开启".green() } else { "关闭".red() };
                Ok(Some(format!("颜色输出: {}", status)))
            }
            "approx" | "approximations" => {
                self.show_approximations = !self.show_approximations;
                self.terminal_formatter.set_approximations_enabled(self.show_approximations);
                let status = if self.show_approximations { "开启".green() } else { "关闭".red() };
                Ok(Some(format!("数值近似值: {}", status)))
            }
            input if input.starts_with("format ") => {
                let format_type = input.strip_prefix("format ").unwrap().trim();
                self.set_format(format_type)
            }
            input if input.starts_with("precision ") => {
                let precision_str = input.strip_prefix("precision ").unwrap().trim();
                self.set_precision(precision_str)
            }
            input if input.starts_with("approx_precision ") => {
                let precision_str = input.strip_prefix("approx_precision ").unwrap().trim();
                self.set_approximation_precision(precision_str)
            }
            _ => Ok(None)
        }
    }
    
    /// 解析变量赋值
    fn parse_assignment(&self, input: &str) -> Option<(String, String)> {
        if let Some(eq_pos) = input.find('=') {
            let var_name = input[..eq_pos].trim().to_string();
            let expression = input[eq_pos + 1..].trim().to_string();
            
            // 检查变量名是否有效（简单的字母数字检查）
            if var_name.chars().all(|c| c.is_alphanumeric() || c == '_') && !var_name.is_empty() {
                Some((var_name, expression))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// 处理变量赋值
    fn handle_assignment(&mut self, var_name: String, expression: String) -> Result<String, Box<dyn std::error::Error>> {
        // 首先计算表达式
        match self.yufmath.compute(&expression) {
            Ok(result) => {
                // 尝试解析结果为数值并存储
                // 这里简化处理，实际应该存储 Expression 类型
                if self.verbose {
                    println!("计算 {} = {}", expression, result);
                }
                
                // 暂时使用字符串存储，后续应该改为存储实际的 Number 类型
                // self.variables.insert(var_name.clone(), parsed_number);
                
                Ok(format!("{} = {}", var_name, result))
            }
            Err(e) => {
                Err(format!("无法计算表达式 '{}': {}", expression, e).into())
            }
        }
    }
    
    /// 处理数学表达式
    fn handle_expression(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        if self.verbose {
            println!("正在计算: {}", input);
        }
        
        match self.yufmath.compute(input) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("计算错误: {}", e).into())
        }
    }
    
    /// 显示帮助信息
    fn show_help(&self) -> String {
        format!(r#"{}

{}:
  {}          显示此帮助信息
  {}    退出程序
  {}            清空所有变量
  {}  显示所有变量
  {}          切换详细模式
  {}           切换颜色输出
  {}     切换数值近似值显示

{}:
  {}    设置输出格式 (standard, terminal, latex, mathml)
  {}    设置数值精度
  {}  设置近似值显示精度

{}:
  {}            基本算术运算
  {}    代数表达式
  {}        三角函数和数学常量
  {}     求导 (暂未实现)
  {}  积分 (暂未实现)

{}:
  {}            将值赋给变量
  {}      使用变量的表达式

{}:
  {} 2 + 3
  {}
  
  {} x = 10
  {} x = 10
  
  {} sqrt(3)
  {} √(3) ≈ 1.732051
  
  {} sin(pi/2)
  {} sin(π ≈ 3.141593/2 ≈ 1.570796) ≈ 1.000000

输入多行表达式时，以空行结束输入。
"#,
            "Yufmath 交互式计算器帮助".bright_cyan().bold(),
            "基本命令".bright_yellow(),
            "help, ?".green(),
            "quit, exit, q".green(),
            "clear".green(),
            "vars, variables".green(),
            "verbose".green(),
            "colors".green(),
            "approx, approximations".green(),
            "格式化命令".bright_yellow(),
            "format <type>".green(),
            "precision <n>".green(),
            "approx_precision <n>".green(),
            "数学运算".bright_yellow(),
            "2 + 3".cyan(),
            "x^2 + 2*x + 1".cyan(),
            "sin(pi/2)".cyan(),
            "diff(x^2, x)".cyan(),
            "integrate(x, x)".cyan(),
            "变量赋值".bright_yellow(),
            "x = 5".cyan(),
            "y = x^2 + 1".cyan(),
            "示例".bright_yellow(),
            "yufmath>".bright_green(),
            "5".bright_cyan(),
            "yufmath>".bright_green(),
            "x = 10".bright_cyan(),
            "yufmath>".bright_green(),
            "√(3) ≈ 1.732051".bright_cyan(),
            "yufmath>".bright_green(),
            "sin(π ≈ 3.141593/2 ≈ 1.570796) ≈ 1.000000".bright_cyan(),
        )
    }
    
    /// 显示当前变量
    fn show_variables(&self) -> String {
        if self.variables.is_empty() {
            "没有定义变量".to_string()
        } else {
            let mut result = "当前变量:\n".to_string();
            for (name, value) in &self.variables {
                result.push_str(&format!("  {} = {:?}\n", name, value));
            }
            result
        }
    }
    
    /// 设置输出格式
    fn set_format(&mut self, format_type: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let new_format = match format_type.to_lowercase().as_str() {
            "standard" | "std" => FormatType::Standard,
            "terminal" | "term" => FormatType::Terminal,
            "latex" | "tex" => FormatType::LaTeX,
            "mathml" | "xml" => FormatType::MathML,
            _ => {
                return Ok(Some("无效的格式类型。可用格式: standard, terminal, latex, mathml".red().to_string()));
            }
        };
        
        self.format_options.format_type = new_format.clone();
        self.yufmath.set_format_options(self.format_options.clone());
        
        let format_name = match new_format {
            FormatType::Standard => "标准格式".cyan(),
            FormatType::Terminal => "终端彩色格式".cyan(),
            FormatType::LaTeX => "LaTeX 格式".cyan(),
            FormatType::MathML => "MathML 格式".cyan(),
        };
        
        Ok(Some(format!("输出格式已设置为: {}", format_name)))
    }
    
    /// 设置数值精度
    fn set_precision(&mut self, precision_str: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match precision_str.parse::<usize>() {
            Ok(precision) => {
                self.format_options.precision = Some(precision);
                self.yufmath.set_format_options(self.format_options.clone());
                Ok(Some(format!("数值精度已设置为: {}", precision.to_string().cyan())))
            }
            Err(_) => {
                Ok(Some("无效的精度值，请输入正整数".red().to_string()))
            }
        }
    }
    
    /// 设置近似值精度
    fn set_approximation_precision(&mut self, precision_str: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match precision_str.parse::<usize>() {
            Ok(precision) => {
                self.terminal_formatter.set_approximation_precision(precision);
                Ok(Some(format!("近似值精度已设置为: {}", precision.to_string().cyan())))
            }
            Err(_) => {
                Ok(Some("无效的精度值，请输入正整数".red().to_string()))
            }
        }
    }
}

/// 运行交互式模式
pub fn run_interactive() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {} - {}", 
        "Yufmath".bright_cyan().bold(),
        format!("v{}", crate::VERSION).bright_green(),
        "计算机代数系统".bright_white());
    println!("{}", "━".repeat(50).bright_black());
    println!("✨ {} - 支持彩色输出和数值近似值", "增强终端模式".bright_yellow());
    println!("📚 输入 {} 查看帮助，输入 {} 退出", "'help'".green(), "'quit'".red());
    println!();
    
    let mut rl = DefaultEditor::new()?;
    let mut session = InteractiveSession::new();
    
    // 尝试加载历史记录
    let history_file = "yufmath_history.txt";
    let _ = rl.load_history(history_file);
    
    loop {
        // 读取用户输入
        // 修复终端显示问题：使用简单的提示符，避免彩色输出导致的显示偏移
        let prompt = "yufmath> ";
        let readline = rl.readline(prompt);
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                let _ = rl.add_history_entry(line.as_str());
                
                // 处理多行输入
                let input = if line.trim().is_empty() {
                    continue;
                } else if line.trim().ends_with('\\') {
                    // 支持行尾反斜杠续行
                    handle_multiline_input(&mut rl, line)?
                } else {
                    line
                };
                
                // 检查是否是退出命令
                if input.trim().to_lowercase() == "quit" 
                    || input.trim().to_lowercase() == "exit" 
                    || input.trim().to_lowercase() == "q" {
                    println!("{} {}", "👋".bright_yellow(), "再见！".bright_cyan());
                    break;
                }
                
                // 处理其他命令
                match session.process_command(&input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            println!("{}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "❌ 错误:".bright_red(), e.to_string().red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".bright_yellow());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "^D".bright_yellow());
                break;
            }
            Err(err) => {
                eprintln!("{} {:?}", "❌ 错误:".bright_red(), err);
                break;
            }
        }
    }
    
    // 保存历史记录
    let _ = rl.save_history(history_file);
    
    Ok(())
}

/// 处理多行输入
fn handle_multiline_input(rl: &mut DefaultEditor, first_line: String) -> RustylineResult<String> {
    let mut input = first_line;
    
    // 移除行尾的反斜杠
    if input.trim().ends_with('\\') {
        input = input.trim_end_matches('\\').to_string();
    }
    
    loop {
        // 修复终端显示问题：使用简单的续行提示符
        let continuation_prompt = "     ... ";
        let line = rl.readline(continuation_prompt)?;
        
        if line.trim().is_empty() {
            // 空行表示输入结束
            break;
        } else if line.trim().ends_with('\\') {
            // 继续下一行
            input.push(' ');
            input.push_str(line.trim_end_matches('\\'));
        } else {
            // 最后一行
            input.push(' ');
            input.push_str(&line);
            break;
        }
    }
    
    Ok(input)
}