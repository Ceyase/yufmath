//! # 交互式模式
//!
//! 实现 REPL 交互式计算环境。

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::collections::HashMap;
use crate::Yufmath;
use crate::core::Number;
use crate::formatter::{FormatOptions, FormatType};

/// 交互式会话状态
pub struct InteractiveSession {
    /// Yufmath 计算引擎
    yufmath: Yufmath,
    /// 变量存储
    variables: HashMap<String, Number>,
    /// 格式化选项
    format_options: FormatOptions,
    /// 是否显示详细信息
    verbose: bool,
}

impl InteractiveSession {
    /// 创建新的交互式会话
    pub fn new() -> Self {
        let mut yufmath = Yufmath::new();
        let format_options = FormatOptions::default();
        yufmath.set_format_options(format_options.clone());
        
        Self {
            yufmath,
            variables: HashMap::new(),
            format_options,
            verbose: false,
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
                Ok(Some(format!("详细模式: {}", if self.verbose { "开启" } else { "关闭" })))
            }
            input if input.starts_with("format ") => {
                let format_type = input.strip_prefix("format ").unwrap().trim();
                self.set_format(format_type)
            }
            input if input.starts_with("precision ") => {
                let precision_str = input.strip_prefix("precision ").unwrap().trim();
                self.set_precision(precision_str)
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
        r#"Yufmath 交互式计算器帮助

基本命令:
  help, ?          显示此帮助信息
  quit, exit, q    退出程序
  clear            清空所有变量
  vars, variables  显示所有变量
  verbose          切换详细模式

格式化命令:
  format <type>    设置输出格式 (standard, latex, mathml)
  precision <n>    设置数值精度

数学运算:
  2 + 3            基本算术运算
  x^2 + 2*x + 1    代数表达式
  sin(pi/2)        三角函数
  diff(x^2, x)     求导 (暂未实现)
  integrate(x, x)  积分 (暂未实现)

变量赋值:
  x = 5            将值赋给变量
  y = x^2 + 1      使用变量的表达式

示例:
  yufmath> 2 + 3
  5
  
  yufmath> x = 10
  x = 10
  
  yufmath> x^2 + 2*x + 1
  121

输入多行表达式时，以空行结束输入。
"#.to_string()
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
            "latex" | "tex" => FormatType::LaTeX,
            "mathml" | "xml" => FormatType::MathML,
            _ => {
                return Ok(Some("无效的格式类型。可用格式: standard, latex, mathml".to_string()));
            }
        };
        
        self.format_options.format_type = new_format.clone();
        self.yufmath.set_format_options(self.format_options.clone());
        
        Ok(Some(format!("输出格式已设置为: {:?}", new_format)))
    }
    
    /// 设置数值精度
    fn set_precision(&mut self, precision_str: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match precision_str.parse::<usize>() {
            Ok(precision) => {
                self.format_options.precision = Some(precision);
                self.yufmath.set_format_options(self.format_options.clone());
                Ok(Some(format!("数值精度已设置为: {}", precision)))
            }
            Err(_) => {
                Ok(Some("无效的精度值，请输入正整数".to_string()))
            }
        }
    }
}

/// 运行交互式模式
pub fn run_interactive() -> Result<(), Box<dyn std::error::Error>> {
    println!("Yufmath v{} - 计算机代数系统", crate::VERSION);
    println!("输入 'help' 查看帮助，输入 'quit' 退出\n");
    
    let mut rl = DefaultEditor::new()?;
    let mut session = InteractiveSession::new();
    
    // 尝试加载历史记录
    let history_file = "yufmath_history.txt";
    let _ = rl.load_history(history_file);
    
    loop {
        // 读取用户输入
        let readline = rl.readline("yufmath> ");
        
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
                    println!("再见！");
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
                        eprintln!("错误: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                eprintln!("错误: {:?}", err);
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
        let line = rl.readline("     ... ")?;
        
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