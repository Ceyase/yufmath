//! # 类型系统定义
//!
//! 定义表达式的类型信息和数值类型系统。

/// 表达式的类型信息
#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    /// 数值类型
    Numeric(NumericType),
    /// 符号类型
    Symbolic,
    /// 函数类型
    Function(Vec<ExprType>, Box<ExprType>),
    /// 矩阵类型
    Matrix(usize, usize, Box<ExprType>), // 行数、列数、元素类型
    /// 向量类型
    Vector(usize, Box<ExprType>), // 维数、元素类型
    /// 集合类型
    Set(Box<ExprType>), // 元素类型
    /// 区间类型
    Interval(Box<ExprType>), // 端点类型
    /// 未知类型
    Unknown,
}

/// 数值类型
#[derive(Debug, Clone, PartialEq)]
pub enum NumericType {
    /// 整数类型
    Integer,
    /// 有理数类型
    Rational,
    /// 实数类型
    Real,
    /// 复数类型
    Complex,
    /// 浮点数类型（非精确）
    Float,
}

impl ExprType {
    /// 检查类型是否为数值类型
    pub fn is_numeric(&self) -> bool {
        matches!(self, ExprType::Numeric(_))
    }
    
    /// 检查类型是否为符号类型
    pub fn is_symbolic(&self) -> bool {
        matches!(self, ExprType::Symbolic)
    }
    
    /// 检查类型是否为函数类型
    pub fn is_function(&self) -> bool {
        matches!(self, ExprType::Function(_, _))
    }
    
    /// 检查类型是否为矩阵类型
    pub fn is_matrix(&self) -> bool {
        matches!(self, ExprType::Matrix(_, _, _))
    }
    
    /// 检查类型是否为向量类型
    pub fn is_vector(&self) -> bool {
        matches!(self, ExprType::Vector(_, _))
    }
    
    /// 获取类型的字符串表示
    pub fn to_string(&self) -> String {
        match self {
            ExprType::Numeric(nt) => nt.to_string(),
            ExprType::Symbolic => "符号".to_string(),
            ExprType::Function(args, ret) => {
                let arg_types: Vec<String> = args.iter().map(|t| t.to_string()).collect();
                format!("({}) -> {}", arg_types.join(", "), ret.to_string())
            }
            ExprType::Matrix(rows, cols, elem_type) => {
                format!("矩阵[{}×{}]<{}>", rows, cols, elem_type.to_string())
            }
            ExprType::Vector(dim, elem_type) => {
                format!("向量[{}]<{}>", dim, elem_type.to_string())
            }
            ExprType::Set(elem_type) => {
                format!("集合<{}>", elem_type.to_string())
            }
            ExprType::Interval(elem_type) => {
                format!("区间<{}>", elem_type.to_string())
            }
            ExprType::Unknown => "未知".to_string(),
        }
    }
}

impl NumericType {
    /// 检查是否为精确类型
    pub fn is_exact(&self) -> bool {
        !matches!(self, NumericType::Float)
    }
    
    /// 检查是否为整数类型
    pub fn is_integer(&self) -> bool {
        matches!(self, NumericType::Integer)
    }
    
    /// 检查是否为有理数类型
    pub fn is_rational(&self) -> bool {
        matches!(self, NumericType::Rational)
    }
    
    /// 检查是否为实数类型
    pub fn is_real(&self) -> bool {
        matches!(self, NumericType::Real | NumericType::Rational | NumericType::Integer | NumericType::Float)
    }
    
    /// 检查是否为复数类型
    pub fn is_complex(&self) -> bool {
        matches!(self, NumericType::Complex)
    }
    
    /// 获取类型的字符串表示
    pub fn to_string(&self) -> String {
        match self {
            NumericType::Integer => "整数".to_string(),
            NumericType::Rational => "有理数".to_string(),
            NumericType::Real => "实数".to_string(),
            NumericType::Complex => "复数".to_string(),
            NumericType::Float => "浮点数".to_string(),
        }
    }
    
    /// 获取两个数值类型的公共类型
    pub fn common_type(&self, other: &NumericType) -> NumericType {
        match (self, other) {
            (NumericType::Complex, _) | (_, NumericType::Complex) => NumericType::Complex,
            (NumericType::Real, _) | (_, NumericType::Real) => NumericType::Real,
            (NumericType::Float, _) | (_, NumericType::Float) => NumericType::Float,
            (NumericType::Rational, _) | (_, NumericType::Rational) => NumericType::Rational,
            (NumericType::Integer, NumericType::Integer) => NumericType::Integer,
        }
    }
}