//! # 解析错误定义
//!
//! 定义表达式解析过程中可能出现的各种错误类型。

use thiserror::Error;

/// 解析错误
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseError {
    /// 语法错误
    #[error("语法错误：位置 {pos}，{message}")]
    Syntax { pos: usize, message: String },
    
    /// 未知函数
    #[error("未知函数：{name}")]
    UnknownFunction { name: String },
    
    /// 参数数量错误
    #[error("参数数量错误：期望 {expected}，实际 {actual}")]
    ArgumentCount { expected: usize, actual: usize },
    
    /// 无效的数值格式
    #[error("无效的数值格式：{value}")]
    InvalidNumber { value: String },
    
    /// 无效的变量名
    #[error("无效的变量名：{name}")]
    InvalidVariable { name: String },
    
    /// 括号不匹配
    #[error("括号不匹配：位置 {pos}")]
    UnmatchedParenthesis { pos: usize },
    
    /// 意外的输入结束
    #[error("意外的输入结束")]
    UnexpectedEndOfInput,
    
    /// 意外的字符
    #[error("意外的字符：位置 {pos}，字符 '{ch}'")]
    UnexpectedCharacter { pos: usize, ch: char },
    
    /// 空表达式
    #[error("空表达式")]
    EmptyExpression,
}

impl ParseError {
    /// 创建语法错误
    pub fn syntax(pos: usize, message: impl Into<String>) -> Self {
        ParseError::Syntax {
            pos,
            message: message.into(),
        }
    }
    
    /// 创建未知函数错误
    pub fn unknown_function(name: impl Into<String>) -> Self {
        ParseError::UnknownFunction {
            name: name.into(),
        }
    }
    
    /// 创建参数数量错误
    pub fn argument_count(expected: usize, actual: usize) -> Self {
        ParseError::ArgumentCount { expected, actual }
    }
    
    /// 创建无效数值错误
    pub fn invalid_number(value: impl Into<String>) -> Self {
        ParseError::InvalidNumber {
            value: value.into(),
        }
    }
    
    /// 创建无效变量名错误
    pub fn invalid_variable(name: impl Into<String>) -> Self {
        ParseError::InvalidVariable {
            name: name.into(),
        }
    }
    
    /// 创建括号不匹配错误
    pub fn unmatched_parenthesis(pos: usize) -> Self {
        ParseError::UnmatchedParenthesis { pos }
    }
    
    /// 创建意外字符错误
    pub fn unexpected_character(pos: usize, ch: char) -> Self {
        ParseError::UnexpectedCharacter { pos, ch }
    }
    
    /// 获取错误位置（如果有）
    pub fn position(&self) -> Option<usize> {
        match self {
            ParseError::Syntax { pos, .. } => Some(*pos),
            ParseError::UnmatchedParenthesis { pos } => Some(*pos),
            ParseError::UnexpectedCharacter { pos, .. } => Some(*pos),
            _ => None,
        }
    }
    
    /// 获取用户友好的错误消息
    pub fn user_friendly_message(&self) -> String {
        match self {
            ParseError::Syntax { pos, message } => {
                format!("语法错误（位置 {}）：{}", pos + 1, message)
            }
            ParseError::UnknownFunction { name } => {
                format!("未知函数 '{}'。您是否想要使用：sin, cos, tan, exp, ln, sqrt？", name)
            }
            ParseError::ArgumentCount { expected, actual } => {
                format!("函数参数数量错误：期望 {} 个参数，但提供了 {} 个", expected, actual)
            }
            ParseError::InvalidNumber { value } => {
                format!("无效的数值格式 '{}'。请检查数值是否正确输入", value)
            }
            ParseError::InvalidVariable { name } => {
                format!("无效的变量名 '{}'。变量名应以字母开头，只包含字母、数字和下划线", name)
            }
            ParseError::UnmatchedParenthesis { pos } => {
                format!("括号不匹配（位置 {}）。请检查是否有未闭合的括号", pos + 1)
            }
            ParseError::UnexpectedEndOfInput => {
                "表达式不完整。请检查是否缺少操作数或运算符".to_string()
            }
            ParseError::UnexpectedCharacter { pos, ch } => {
                format!("意外的字符 '{}' （位置 {}）。请检查输入是否正确", ch, pos + 1)
            }
            ParseError::EmptyExpression => {
                "表达式为空。请输入一个有效的数学表达式".to_string()
            }
        }
    }
    
    /// 获取修复建议
    pub fn suggestions(&self) -> Vec<String> {
        match self {
            ParseError::UnknownFunction { name } => {
                let mut suggestions = Vec::new();
                
                // 基于相似性提供函数建议
                let common_functions = [
                    "sin", "cos", "tan", "asin", "acos", "atan",
                    "sinh", "cosh", "tanh", "exp", "ln", "log",
                    "sqrt", "abs", "factorial", "gamma"
                ];
                
                for func in &common_functions {
                    if levenshtein_distance(name, func) <= 2 {
                        suggestions.push(format!("您是否想要使用 '{}'？", func));
                    }
                }
                
                if suggestions.is_empty() {
                    suggestions.push("请检查函数名是否正确拼写".to_string());
                    suggestions.push("查看支持的函数列表：sin, cos, tan, exp, ln, sqrt 等".to_string());
                }
                
                suggestions
            }
            ParseError::InvalidNumber { value } => {
                vec![
                    "请检查数值格式是否正确".to_string(),
                    "支持的格式：整数（123）、小数（3.14）、科学记数法（1.23e-4）".to_string(),
                    format!("如果 '{}' 是变量名，请确保它不以数字开头", value),
                ]
            }
            ParseError::UnmatchedParenthesis { .. } => {
                vec![
                    "检查每个左括号 '(' 是否有对应的右括号 ')'".to_string(),
                    "检查每个右括号 ')' 是否有对应的左括号 '('".to_string(),
                    "使用文本编辑器的括号匹配功能来检查括号配对".to_string(),
                ]
            }
            ParseError::Syntax { .. } => {
                vec![
                    "检查运算符是否正确使用".to_string(),
                    "确保每个运算符都有适当的操作数".to_string(),
                    "检查是否有连续的运算符（如 '++' 或 '--'）".to_string(),
                ]
            }
            ParseError::ArgumentCount { expected, .. } => {
                vec![
                    format!("该函数需要 {} 个参数", expected),
                    "检查函数调用中的逗号分隔符".to_string(),
                    "确保每个参数都是有效的表达式".to_string(),
                ]
            }
            _ => {
                vec![
                    "请检查表达式的语法是否正确".to_string(),
                    "参考文档了解正确的语法格式".to_string(),
                ]
            }
        }
    }
    
    /// 生成带有位置指示的错误报告
    pub fn format_with_context(&self, input: &str) -> String {
        let mut result = String::new();
        result.push_str(&format!("错误：{}\n", self.user_friendly_message()));
        
        if let Some(pos) = self.position() {
            if pos < input.len() {
                result.push_str(&format!("输入：{}\n", input));
                result.push_str(&format!("位置：{}{}\n", " ".repeat(pos + 3), "^"));
            }
        }
        
        let suggestions = self.suggestions();
        if !suggestions.is_empty() {
            result.push_str("\n建议：\n");
            for (i, suggestion) in suggestions.iter().enumerate() {
                result.push_str(&format!("  {}. {}\n", i + 1, suggestion));
            }
        }
        
        result
    }
}

/// 计算两个字符串之间的编辑距离（用于函数名建议）
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    // 初始化第一行和第一列
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // 删除
                    matrix[i][j - 1] + 1       // 插入
                ),
                matrix[i - 1][j - 1] + cost    // 替换
            );
        }
    }
    
    matrix[len1][len2]
}