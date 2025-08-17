//! # 自动补全功能
//!
//! 为笔记本编辑器提供智能自动补全功能。

use std::collections::HashMap;

/// 自动补全建议
#[derive(Debug, Clone)]
pub struct CompletionSuggestion {
    /// 建议的文本
    pub text: String,
    /// 显示的标签
    pub label: String,
    /// 建议类型
    pub suggestion_type: SuggestionType,
    /// 详细描述
    pub description: Option<String>,
    /// 插入位置的偏移量
    pub insert_offset: usize,
    /// 替换长度
    pub replace_length: usize,
}

/// 建议类型
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionType {
    /// 函数
    Function,
    /// 变量
    Variable,
    /// 常量
    Constant,
    /// 关键字
    Keyword,
    /// 运算符
    Operator,
}

/// 自动补全引擎
pub struct AutoCompleteEngine {
    /// 函数定义
    functions: HashMap<String, FunctionInfo>,
    /// 常量定义
    constants: HashMap<String, ConstantInfo>,
    /// 关键字列表
    keywords: Vec<String>,
    /// 运算符列表
    operators: Vec<String>,
    /// 用户定义的变量
    user_variables: HashMap<String, String>,
}

/// 函数信息
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// 函数名
    pub name: String,
    /// 参数列表
    pub parameters: Vec<String>,
    /// 返回类型
    pub return_type: String,
    /// 描述
    pub description: String,
    /// 示例
    pub example: Option<String>,
}

/// 常量信息
#[derive(Debug, Clone)]
pub struct ConstantInfo {
    /// 常量名
    pub name: String,
    /// 值
    pub value: String,
    /// 描述
    pub description: String,
}

impl AutoCompleteEngine {
    /// 创建新的自动补全引擎
    pub fn new() -> Self {
        let mut engine = Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
            keywords: Vec::new(),
            operators: Vec::new(),
            user_variables: HashMap::new(),
        };
        
        engine.initialize_builtin_functions();
        engine.initialize_builtin_constants();
        engine.initialize_keywords();
        engine.initialize_operators();
        
        engine
    }
    
    /// 初始化内置函数
    fn initialize_builtin_functions(&mut self) {
        let functions = vec![
            FunctionInfo {
                name: "sin".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算正弦值".to_string(),
                example: Some("sin(pi/2) = 1".to_string()),
            },
            FunctionInfo {
                name: "cos".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算余弦值".to_string(),
                example: Some("cos(0) = 1".to_string()),
            },
            FunctionInfo {
                name: "tan".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算正切值".to_string(),
                example: Some("tan(pi/4) = 1".to_string()),
            },
            FunctionInfo {
                name: "sqrt".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算平方根".to_string(),
                example: Some("sqrt(16) = 4".to_string()),
            },
            FunctionInfo {
                name: "exp".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算指数函数 e^x".to_string(),
                example: Some("exp(1) = e".to_string()),
            },
            FunctionInfo {
                name: "ln".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算自然对数".to_string(),
                example: Some("ln(e) = 1".to_string()),
            },
            FunctionInfo {
                name: "log".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算常用对数（以10为底）".to_string(),
                example: Some("log(100) = 2".to_string()),
            },
            FunctionInfo {
                name: "abs".to_string(),
                parameters: vec!["x".to_string()],
                return_type: "Number".to_string(),
                description: "计算绝对值".to_string(),
                example: Some("abs(-5) = 5".to_string()),
            },
            FunctionInfo {
                name: "diff".to_string(),
                parameters: vec!["expr".to_string(), "var".to_string()],
                return_type: "Expression".to_string(),
                description: "对表达式求导".to_string(),
                example: Some("diff(x^2, x) = 2*x".to_string()),
            },
            FunctionInfo {
                name: "integrate".to_string(),
                parameters: vec!["expr".to_string(), "var".to_string()],
                return_type: "Expression".to_string(),
                description: "对表达式积分".to_string(),
                example: Some("integrate(2*x, x) = x^2 + C".to_string()),
            },
            FunctionInfo {
                name: "simplify".to_string(),
                parameters: vec!["expr".to_string()],
                return_type: "Expression".to_string(),
                description: "简化表达式".to_string(),
                example: Some("simplify((x+1)^2) = x^2 + 2*x + 1".to_string()),
            },
            FunctionInfo {
                name: "expand".to_string(),
                parameters: vec!["expr".to_string()],
                return_type: "Expression".to_string(),
                description: "展开表达式".to_string(),
                example: Some("expand((x+1)*(x-1)) = x^2 - 1".to_string()),
            },
            FunctionInfo {
                name: "factor".to_string(),
                parameters: vec!["expr".to_string()],
                return_type: "Expression".to_string(),
                description: "因式分解".to_string(),
                example: Some("factor(x^2 - 1) = (x-1)*(x+1)".to_string()),
            },
            FunctionInfo {
                name: "solve".to_string(),
                parameters: vec!["equation".to_string(), "var".to_string()],
                return_type: "List".to_string(),
                description: "求解方程".to_string(),
                example: Some("solve(x^2 - 4 = 0, x) = [2, -2]".to_string()),
            },
            FunctionInfo {
                name: "limit".to_string(),
                parameters: vec!["expr".to_string(), "var".to_string(), "point".to_string()],
                return_type: "Expression".to_string(),
                description: "计算极限".to_string(),
                example: Some("limit(sin(x)/x, x, 0) = 1".to_string()),
            },
            FunctionInfo {
                name: "series".to_string(),
                parameters: vec!["expr".to_string(), "var".to_string(), "point".to_string(), "order".to_string()],
                return_type: "Expression".to_string(),
                description: "级数展开".to_string(),
                example: Some("series(e^x, x, 0, 3) = 1 + x + x^2/2! + x^3/3! + O(x^4)".to_string()),
            },
            FunctionInfo {
                name: "matrix".to_string(),
                parameters: vec!["elements".to_string()],
                return_type: "Matrix".to_string(),
                description: "创建矩阵".to_string(),
                example: Some("matrix([[1,2],[3,4]])".to_string()),
            },
            FunctionInfo {
                name: "det".to_string(),
                parameters: vec!["matrix".to_string()],
                return_type: "Number".to_string(),
                description: "计算矩阵行列式".to_string(),
                example: Some("det([[1,2],[3,4]]) = -2".to_string()),
            },
            FunctionInfo {
                name: "gcd".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                return_type: "Number".to_string(),
                description: "计算最大公约数".to_string(),
                example: Some("gcd(48, 18) = 6".to_string()),
            },
            FunctionInfo {
                name: "lcm".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                return_type: "Number".to_string(),
                description: "计算最小公倍数".to_string(),
                example: Some("lcm(12, 8) = 24".to_string()),
            },
        ];
        
        for func in functions {
            self.functions.insert(func.name.clone(), func);
        }
    }
    
    /// 初始化内置常量
    fn initialize_builtin_constants(&mut self) {
        let constants = vec![
            ConstantInfo {
                name: "pi".to_string(),
                value: "3.14159265358979323846...".to_string(),
                description: "圆周率".to_string(),
            },
            ConstantInfo {
                name: "e".to_string(),
                value: "2.71828182845904523536...".to_string(),
                description: "自然常数".to_string(),
            },
            ConstantInfo {
                name: "i".to_string(),
                value: "√(-1)".to_string(),
                description: "虚数单位".to_string(),
            },
            ConstantInfo {
                name: "gamma".to_string(),
                value: "0.57721566490153286060...".to_string(),
                description: "欧拉-马歇罗尼常数".to_string(),
            },
            ConstantInfo {
                name: "phi".to_string(),
                value: "1.61803398874989484820...".to_string(),
                description: "黄金比例".to_string(),
            },
            ConstantInfo {
                name: "inf".to_string(),
                value: "∞".to_string(),
                description: "正无穷".to_string(),
            },
        ];
        
        for constant in constants {
            self.constants.insert(constant.name.clone(), constant);
        }
    }
    
    /// 初始化关键字
    fn initialize_keywords(&mut self) {
        self.keywords = vec![
            "if".to_string(),
            "then".to_string(),
            "else".to_string(),
            "for".to_string(),
            "while".to_string(),
            "do".to_string(),
            "let".to_string(),
            "in".to_string(),
            "where".to_string(),
            "true".to_string(),
            "false".to_string(),
            "and".to_string(),
            "or".to_string(),
            "not".to_string(),
        ];
    }
    
    /// 初始化运算符
    fn initialize_operators(&mut self) {
        self.operators = vec![
            "+".to_string(),
            "-".to_string(),
            "*".to_string(),
            "/".to_string(),
            "^".to_string(),
            "**".to_string(),
            "%".to_string(),
            "=".to_string(),
            "==".to_string(),
            "!=".to_string(),
            "<".to_string(),
            "<=".to_string(),
            ">".to_string(),
            ">=".to_string(),
            "&&".to_string(),
            "||".to_string(),
            "!".to_string(),
        ];
    }
    
    /// 添加用户定义的变量
    pub fn add_user_variable(&mut self, name: String, description: String) {
        self.user_variables.insert(name, description);
    }
    
    /// 移除用户定义的变量
    pub fn remove_user_variable(&mut self, name: &str) {
        self.user_variables.remove(name);
    }
    
    /// 获取自动补全建议
    pub fn get_completions(&self, text: &str, cursor_position: usize) -> Vec<CompletionSuggestion> {
        let mut suggestions = Vec::new();
        
        // 找到当前单词的开始位置
        let word_start = self.find_word_start(text, cursor_position);
        let current_word = &text[word_start..cursor_position];
        
        if current_word.is_empty() {
            return suggestions;
        }
        
        // 搜索函数
        for (name, info) in &self.functions {
            if name.starts_with(current_word) {
                let params = info.parameters.join(", ");
                let label = format!("{}({})", name, params);
                let insert_text = format!("{}({})", name, 
                    info.parameters.iter()
                        .enumerate()
                        .map(|(i, p)| format!("${{{i}:{p}}}", i = i + 1, p = p))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                
                suggestions.push(CompletionSuggestion {
                    text: insert_text,
                    label,
                    suggestion_type: SuggestionType::Function,
                    description: Some(format!("{}\n\n示例: {}", 
                        info.description, 
                        info.example.as_deref().unwrap_or("无")
                    )),
                    insert_offset: word_start,
                    replace_length: current_word.len(),
                });
            }
        }
        
        // 搜索常量
        for (name, info) in &self.constants {
            if name.starts_with(current_word) {
                suggestions.push(CompletionSuggestion {
                    text: name.clone(),
                    label: format!("{} = {}", name, info.value),
                    suggestion_type: SuggestionType::Constant,
                    description: Some(info.description.clone()),
                    insert_offset: word_start,
                    replace_length: current_word.len(),
                });
            }
        }
        
        // 搜索关键字
        for keyword in &self.keywords {
            if keyword.starts_with(current_word) {
                suggestions.push(CompletionSuggestion {
                    text: keyword.clone(),
                    label: keyword.clone(),
                    suggestion_type: SuggestionType::Keyword,
                    description: None,
                    insert_offset: word_start,
                    replace_length: current_word.len(),
                });
            }
        }
        
        // 搜索用户变量
        for (name, description) in &self.user_variables {
            if name.starts_with(current_word) {
                suggestions.push(CompletionSuggestion {
                    text: name.clone(),
                    label: name.clone(),
                    suggestion_type: SuggestionType::Variable,
                    description: Some(description.clone()),
                    insert_offset: word_start,
                    replace_length: current_word.len(),
                });
            }
        }
        
        // 按相关性排序
        suggestions.sort_by(|a, b| {
            // 优先显示完全匹配的
            let a_exact = a.text == current_word;
            let b_exact = b.text == current_word;
            
            if a_exact && !b_exact {
                return std::cmp::Ordering::Less;
            }
            if !a_exact && b_exact {
                return std::cmp::Ordering::Greater;
            }
            
            // 然后按前缀匹配长度排序
            let a_prefix_len = self.common_prefix_length(&a.text, current_word);
            let b_prefix_len = self.common_prefix_length(&b.text, current_word);
            
            b_prefix_len.cmp(&a_prefix_len)
                .then_with(|| a.text.len().cmp(&b.text.len()))
                .then_with(|| a.text.cmp(&b.text))
        });
        
        // 限制建议数量
        suggestions.truncate(20);
        
        suggestions
    }
    
    /// 查找单词开始位置
    fn find_word_start(&self, text: &str, cursor_position: usize) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let mut start = cursor_position;
        
        while start > 0 {
            let ch = chars[start - 1];
            if ch.is_alphanumeric() || ch == '_' {
                start -= 1;
            } else {
                break;
            }
        }
        
        start
    }
    
    /// 计算公共前缀长度
    fn common_prefix_length(&self, s1: &str, s2: &str) -> usize {
        s1.chars()
            .zip(s2.chars())
            .take_while(|(a, b)| a == b)
            .count()
    }
    
    /// 获取函数签名帮助
    pub fn get_function_signature(&self, function_name: &str) -> Option<String> {
        self.functions.get(function_name).map(|info| {
            let params = info.parameters.join(", ");
            format!("{}({}) -> {}\n\n{}", 
                function_name, 
                params, 
                info.return_type,
                info.description
            )
        })
    }
    
    /// 获取上下文相关的建议
    pub fn get_context_suggestions(&self, text: &str, cursor_position: usize) -> Vec<CompletionSuggestion> {
        let mut suggestions = Vec::new();
        
        // 分析上下文
        let context = self.analyze_context(text, cursor_position);
        
        match context {
            CompletionContext::FunctionCall(func_name) => {
                // 在函数调用中，建议参数
                if let Some(info) = self.functions.get(&func_name) {
                    for (i, param) in info.parameters.iter().enumerate() {
                        suggestions.push(CompletionSuggestion {
                            text: param.clone(),
                            label: format!("参数 {}: {}", i + 1, param),
                            suggestion_type: SuggestionType::Variable,
                            description: Some(format!("{}函数的第{}个参数", func_name, i + 1)),
                            insert_offset: cursor_position,
                            replace_length: 0,
                        });
                    }
                }
            }
            CompletionContext::MathExpression => {
                // 在数学表达式中，建议数学函数和常量
                for (name, info) in &self.functions {
                    if matches!(info.return_type.as_str(), "Number" | "Expression") {
                        suggestions.push(CompletionSuggestion {
                            text: name.clone(),
                            label: name.clone(),
                            suggestion_type: SuggestionType::Function,
                            description: Some(info.description.clone()),
                            insert_offset: cursor_position,
                            replace_length: 0,
                        });
                    }
                }
                
                for (name, info) in &self.constants {
                    suggestions.push(CompletionSuggestion {
                        text: name.clone(),
                        label: name.clone(),
                        suggestion_type: SuggestionType::Constant,
                        description: Some(info.description.clone()),
                        insert_offset: cursor_position,
                        replace_length: 0,
                    });
                }
            }
            CompletionContext::General => {
                // 一般情况，返回所有建议
                return self.get_completions(text, cursor_position);
            }
        }
        
        suggestions
    }
    
    /// 分析上下文
    fn analyze_context(&self, text: &str, cursor_position: usize) -> CompletionContext {
        let chars: Vec<char> = text.chars().collect();
        
        // 向后查找，寻找函数调用模式
        let mut i = cursor_position;
        let mut paren_count = 0;
        
        while i > 0 {
            i -= 1;
            let ch = chars[i];
            
            match ch {
                ')' => paren_count += 1,
                '(' => {
                    if paren_count == 0 {
                        // 找到了函数调用的开始
                        let func_start = self.find_function_name_start(&chars, i);
                        if func_start < i {
                            let func_name: String = chars[func_start..i].iter().collect();
                            if self.functions.contains_key(&func_name) {
                                return CompletionContext::FunctionCall(func_name);
                            }
                        }
                        break;
                    } else {
                        paren_count -= 1;
                    }
                }
                _ => {}
            }
        }
        
        // 检查是否在数学表达式中
        if self.is_in_math_expression(text, cursor_position) {
            CompletionContext::MathExpression
        } else {
            CompletionContext::General
        }
    }
    
    /// 查找函数名开始位置
    fn find_function_name_start(&self, chars: &[char], end: usize) -> usize {
        let mut start = end;
        
        while start > 0 {
            let ch = chars[start - 1];
            if ch.is_alphanumeric() || ch == '_' {
                start -= 1;
            } else {
                break;
            }
        }
        
        start
    }
    
    /// 检查是否在数学表达式中
    fn is_in_math_expression(&self, text: &str, cursor_position: usize) -> bool {
        let before = &text[..cursor_position];
        
        // 简单的启发式检查
        before.contains('+') || before.contains('-') || before.contains('*') || 
        before.contains('/') || before.contains('^') || before.contains('=')
    }
}

/// 补全上下文
#[derive(Debug, Clone)]
enum CompletionContext {
    /// 函数调用中
    FunctionCall(String),
    /// 数学表达式中
    MathExpression,
    /// 一般情况
    General,
}

impl Default for AutoCompleteEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_autocomplete_engine_creation() {
        let engine = AutoCompleteEngine::new();
        
        // 验证内置函数已加载
        assert!(engine.functions.contains_key("sin"));
        assert!(engine.functions.contains_key("cos"));
        assert!(engine.functions.contains_key("sqrt"));
        
        // 验证内置常量已加载
        assert!(engine.constants.contains_key("pi"));
        assert!(engine.constants.contains_key("e"));
        
        // 验证关键字已加载
        assert!(!engine.keywords.is_empty());
        
        // 验证运算符已加载
        assert!(!engine.operators.is_empty());
    }
    
    #[test]
    fn test_function_completion() {
        let engine = AutoCompleteEngine::new();
        
        // 测试函数补全
        let suggestions = engine.get_completions("si", 2);
        
        // 应该包含 sin 函数
        let sin_suggestion = suggestions.iter().find(|s| s.text.starts_with("sin"));
        assert!(sin_suggestion.is_some());
        
        let sin_suggestion = sin_suggestion.unwrap();
        assert_eq!(sin_suggestion.suggestion_type, SuggestionType::Function);
        assert!(sin_suggestion.description.is_some());
    }
    
    #[test]
    fn test_constant_completion() {
        let engine = AutoCompleteEngine::new();
        
        // 测试常量补全
        let suggestions = engine.get_completions("p", 1);
        
        // 应该包含 pi 常量
        let pi_suggestion = suggestions.iter().find(|s| s.text == "pi");
        assert!(pi_suggestion.is_some());
        
        let pi_suggestion = pi_suggestion.unwrap();
        assert_eq!(pi_suggestion.suggestion_type, SuggestionType::Constant);
        assert!(pi_suggestion.description.is_some());
    }
    
    #[test]
    fn test_user_variable_completion() {
        let mut engine = AutoCompleteEngine::new();
        
        // 添加用户变量
        engine.add_user_variable("my_var".to_string(), "用户定义的变量".to_string());
        
        // 测试用户变量补全
        let suggestions = engine.get_completions("my", 2);
        
        // 应该包含用户变量
        let var_suggestion = suggestions.iter().find(|s| s.text == "my_var");
        assert!(var_suggestion.is_some());
        
        let var_suggestion = var_suggestion.unwrap();
        assert_eq!(var_suggestion.suggestion_type, SuggestionType::Variable);
    }
    
    #[test]
    fn test_word_start_finding() {
        let engine = AutoCompleteEngine::new();
        
        // 测试单词开始位置查找
        assert_eq!(engine.find_word_start("hello world", 5), 0);
        assert_eq!(engine.find_word_start("hello world", 11), 6);
        assert_eq!(engine.find_word_start("sin(x)", 3), 0);
        assert_eq!(engine.find_word_start("x + sin(y)", 7), 4);
    }
    
    #[test]
    fn test_function_signature() {
        let engine = AutoCompleteEngine::new();
        
        // 测试函数签名获取
        let signature = engine.get_function_signature("sin");
        assert!(signature.is_some());
        
        let signature = signature.unwrap();
        assert!(signature.contains("sin(x)"));
        assert!(signature.contains("正弦值"));
    }
    
    #[test]
    fn test_completion_sorting() {
        let engine = AutoCompleteEngine::new();
        
        // 测试补全结果排序
        let suggestions = engine.get_completions("s", 1);
        
        // 结果应该按相关性排序
        assert!(!suggestions.is_empty());
        
        // 更短的匹配应该排在前面
        for i in 1..suggestions.len() {
            let prev_len = suggestions[i-1].text.len();
            let curr_len = suggestions[i].text.len();
            
            // 如果前缀长度相同，较短的应该排在前面
            if suggestions[i-1].text.starts_with('s') && suggestions[i].text.starts_with('s') {
                assert!(prev_len <= curr_len || suggestions[i-1].text < suggestions[i].text);
            }
        }
    }
    
    #[test]
    fn test_context_analysis() {
        let engine = AutoCompleteEngine::new();
        
        // 测试上下文分析
        let context_suggestions = engine.get_context_suggestions("sin(", 4);
        
        // 在函数调用中应该有参数建议
        assert!(!context_suggestions.is_empty());
        
        let math_suggestions = engine.get_context_suggestions("x + ", 4);
        
        // 在数学表达式中应该有数学函数建议
        assert!(!math_suggestions.is_empty());
    }
}