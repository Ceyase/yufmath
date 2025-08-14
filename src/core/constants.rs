//! # 数学常量定义
//!
//! 定义常用的数学常量，如 π、e、i 等。

/// 数学常量类型
#[derive(Debug, Clone, PartialEq)]
pub enum MathConstant {
    /// 圆周率 π
    Pi,
    /// 自然常数 e
    E,
    /// 虚数单位 i
    I,
    /// 欧拉-马歇罗尼常数 γ
    EulerGamma,
    /// 黄金比例 φ
    GoldenRatio,
    /// 卡塔兰常数 G
    Catalan,
    /// 正无穷
    PositiveInfinity,
    /// 负无穷
    NegativeInfinity,
    /// 未定义（NaN）
    Undefined,
}

impl MathConstant {
    /// 获取常量的数值近似值
    pub fn approximate_value(&self) -> f64 {
        match self {
            MathConstant::Pi => std::f64::consts::PI,
            MathConstant::E => std::f64::consts::E,
            MathConstant::I => f64::NAN, // 复数单位需要特殊处理
            MathConstant::EulerGamma => 0.5772156649015329,
            MathConstant::GoldenRatio => 1.618033988749895,
            MathConstant::Catalan => 0.915965594177219,
            MathConstant::PositiveInfinity => f64::INFINITY,
            MathConstant::NegativeInfinity => f64::NEG_INFINITY,
            MathConstant::Undefined => f64::NAN,
        }
    }
    
    /// 获取常量的符号表示
    pub fn symbol(&self) -> &'static str {
        match self {
            MathConstant::Pi => "π",
            MathConstant::E => "e",
            MathConstant::I => "i",
            MathConstant::EulerGamma => "γ",
            MathConstant::GoldenRatio => "φ",
            MathConstant::Catalan => "G",
            MathConstant::PositiveInfinity => "∞",
            MathConstant::NegativeInfinity => "-∞",
            MathConstant::Undefined => "undefined",
        }
    }
    
    /// 从字符串解析常量
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pi" | "π" => Some(MathConstant::Pi),
            "e" => Some(MathConstant::E),
            "i" => Some(MathConstant::I),
            "gamma" | "γ" => Some(MathConstant::EulerGamma),
            "phi" | "φ" => Some(MathConstant::GoldenRatio),
            "catalan" | "g" => Some(MathConstant::Catalan),
            "inf" | "infinity" | "∞" => Some(MathConstant::PositiveInfinity),
            "-inf" | "-infinity" | "-∞" => Some(MathConstant::NegativeInfinity),
            "nan" | "undefined" => Some(MathConstant::Undefined),
            _ => None,
        }
    }
    
    /// 获取常量的完整名称
    pub fn name(&self) -> &'static str {
        match self {
            MathConstant::Pi => "圆周率",
            MathConstant::E => "自然常数",
            MathConstant::I => "虚数单位",
            MathConstant::EulerGamma => "欧拉-马歇罗尼常数",
            MathConstant::GoldenRatio => "黄金比例",
            MathConstant::Catalan => "卡塔兰常数",
            MathConstant::PositiveInfinity => "正无穷",
            MathConstant::NegativeInfinity => "负无穷",
            MathConstant::Undefined => "未定义",
        }
    }
}