//! # 运算符定义
//!
//! 定义数学表达式中使用的各种运算符。

/// 二元运算符
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // 基本算术运算
    /// 加法 +
    Add,
    /// 减法 -
    Subtract,
    /// 乘法 *
    Multiply,
    /// 除法 /
    Divide,
    /// 幂运算 ^
    Power,
    /// 取模 %
    Modulo,
    
    // 比较运算
    /// 等于 ==
    Equal,
    /// 不等于 !=
    NotEqual,
    /// 小于 <
    Less,
    /// 小于等于 <=
    LessEqual,
    /// 大于 >
    Greater,
    /// 大于等于 >=
    GreaterEqual,
    
    // 逻辑运算
    /// 逻辑与 &&
    And,
    /// 逻辑或 ||
    Or,
    
    // 集合运算
    /// 并集 ∪
    Union,
    /// 交集 ∩
    Intersection,
    /// 差集 \
    SetDifference,
    
    // 矩阵运算
    /// 矩阵乘法 @
    MatrixMultiply,
    /// 叉积 ×
    CrossProduct,
    /// 点积 ·
    DotProduct,
}

impl BinaryOperator {
    /// 获取运算符的符号表示
    pub fn symbol(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Power => "^",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::Union => "∪",
            BinaryOperator::Intersection => "∩",
            BinaryOperator::SetDifference => "\\",
            BinaryOperator::MatrixMultiply => "@",
            BinaryOperator::CrossProduct => "×",
            BinaryOperator::DotProduct => "·",
        }
    }
    
    /// 获取运算符的优先级
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Or => 1,
            BinaryOperator::And => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 3,
            BinaryOperator::Less | BinaryOperator::LessEqual 
            | BinaryOperator::Greater | BinaryOperator::GreaterEqual => 4,
            BinaryOperator::Union | BinaryOperator::Intersection 
            | BinaryOperator::SetDifference => 5,
            BinaryOperator::Add | BinaryOperator::Subtract => 6,
            BinaryOperator::Multiply | BinaryOperator::Divide 
            | BinaryOperator::Modulo => 7,
            BinaryOperator::MatrixMultiply | BinaryOperator::DotProduct 
            | BinaryOperator::CrossProduct => 8,
            BinaryOperator::Power => 9,
        }
    }
    
    /// 检查运算符是否为右结合
    pub fn is_right_associative(&self) -> bool {
        matches!(self, BinaryOperator::Power)
    }
    
    /// 获取运算符的名称
    pub fn name(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "加法",
            BinaryOperator::Subtract => "减法",
            BinaryOperator::Multiply => "乘法",
            BinaryOperator::Divide => "除法",
            BinaryOperator::Power => "幂运算",
            BinaryOperator::Modulo => "取模",
            BinaryOperator::Equal => "等于",
            BinaryOperator::NotEqual => "不等于",
            BinaryOperator::Less => "小于",
            BinaryOperator::LessEqual => "小于等于",
            BinaryOperator::Greater => "大于",
            BinaryOperator::GreaterEqual => "大于等于",
            BinaryOperator::And => "逻辑与",
            BinaryOperator::Or => "逻辑或",
            BinaryOperator::Union => "并集",
            BinaryOperator::Intersection => "交集",
            BinaryOperator::SetDifference => "差集",
            BinaryOperator::MatrixMultiply => "矩阵乘法",
            BinaryOperator::CrossProduct => "叉积",
            BinaryOperator::DotProduct => "点积",
        }
    }
}

/// 一元运算符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    // 基本运算
    /// 负号 -
    Negate,
    /// 正号 +
    Plus,
    
    // 数学函数
    /// 平方根 √
    Sqrt,
    /// 绝对值 |x|
    Abs,
    
    // 三角函数
    /// 正弦 sin
    Sin,
    /// 余弦 cos
    Cos,
    /// 正切 tan
    Tan,
    /// 反正弦 asin
    Asin,
    /// 反余弦 acos
    Acos,
    /// 反正切 atan
    Atan,
    /// 双曲正弦 sinh
    Sinh,
    /// 双曲余弦 cosh
    Cosh,
    /// 双曲正切 tanh
    Tanh,
    /// 反双曲正弦 asinh
    Asinh,
    /// 反双曲余弦 acosh
    Acosh,
    /// 反双曲正切 atanh
    Atanh,
    
    // 对数和指数
    /// 自然对数 ln
    Ln,
    /// 常用对数 log₁₀
    Log10,
    /// 二进制对数 log₂
    Log2,
    /// 指数函数 e^x
    Exp,
    
    // 特殊函数
    /// 阶乘 x!
    Factorial,
    /// 伽马函数 Γ(x)
    Gamma,
    
    // 逻辑运算
    /// 逻辑非 !
    Not,
    
    // 复数运算
    /// 实部 Re(x)
    Real,
    /// 虚部 Im(x)
    Imaginary,
    /// 共轭 x*
    Conjugate,
    /// 幅角 arg(x)
    Argument,
    
    // 矩阵运算
    /// 转置 x^T
    Transpose,
    /// 行列式 det(x)
    Determinant,
    /// 逆矩阵 x^(-1)
    Inverse,
    /// 矩阵的迹 tr(x)
    Trace,
}

impl UnaryOperator {
    /// 获取运算符的符号表示
    pub fn symbol(&self) -> &'static str {
        match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Plus => "+",
            UnaryOperator::Sqrt => "√",
            UnaryOperator::Abs => "abs",
            UnaryOperator::Sin => "sin",
            UnaryOperator::Cos => "cos",
            UnaryOperator::Tan => "tan",
            UnaryOperator::Asin => "asin",
            UnaryOperator::Acos => "acos",
            UnaryOperator::Atan => "atan",
            UnaryOperator::Sinh => "sinh",
            UnaryOperator::Cosh => "cosh",
            UnaryOperator::Tanh => "tanh",
            UnaryOperator::Asinh => "asinh",
            UnaryOperator::Acosh => "acosh",
            UnaryOperator::Atanh => "atanh",
            UnaryOperator::Ln => "ln",
            UnaryOperator::Log10 => "log10",
            UnaryOperator::Log2 => "log2",
            UnaryOperator::Exp => "exp",
            UnaryOperator::Factorial => "!",
            UnaryOperator::Gamma => "Γ",
            UnaryOperator::Not => "!",
            UnaryOperator::Real => "Re",
            UnaryOperator::Imaginary => "Im",
            UnaryOperator::Conjugate => "*",
            UnaryOperator::Argument => "arg",
            UnaryOperator::Transpose => "T",
            UnaryOperator::Determinant => "det",
            UnaryOperator::Inverse => "inv",
            UnaryOperator::Trace => "tr",
        }
    }
    
    /// 获取运算符的名称
    pub fn name(&self) -> &'static str {
        match self {
            UnaryOperator::Negate => "负号",
            UnaryOperator::Plus => "正号",
            UnaryOperator::Sqrt => "平方根",
            UnaryOperator::Abs => "绝对值",
            UnaryOperator::Sin => "正弦",
            UnaryOperator::Cos => "余弦",
            UnaryOperator::Tan => "正切",
            UnaryOperator::Asin => "反正弦",
            UnaryOperator::Acos => "反余弦",
            UnaryOperator::Atan => "反正切",
            UnaryOperator::Sinh => "双曲正弦",
            UnaryOperator::Cosh => "双曲余弦",
            UnaryOperator::Tanh => "双曲正切",
            UnaryOperator::Asinh => "反双曲正弦",
            UnaryOperator::Acosh => "反双曲余弦",
            UnaryOperator::Atanh => "反双曲正切",
            UnaryOperator::Ln => "自然对数",
            UnaryOperator::Log10 => "常用对数",
            UnaryOperator::Log2 => "二进制对数",
            UnaryOperator::Exp => "指数函数",
            UnaryOperator::Factorial => "阶乘",
            UnaryOperator::Gamma => "伽马函数",
            UnaryOperator::Not => "逻辑非",
            UnaryOperator::Real => "实部",
            UnaryOperator::Imaginary => "虚部",
            UnaryOperator::Conjugate => "共轭",
            UnaryOperator::Argument => "幅角",
            UnaryOperator::Transpose => "转置",
            UnaryOperator::Determinant => "行列式",
            UnaryOperator::Inverse => "逆矩阵",
            UnaryOperator::Trace => "矩阵的迹",
        }
    }
}