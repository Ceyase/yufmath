//! # 高精度数值计算模块
//!
//! 提供超高精度的数值计算功能，解决浮点数精度丢失问题

use bigdecimal::{BigDecimal, Zero, One};
use num_bigint::BigInt;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// 高精度数值类型，专门处理极小数值的精确计算
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighPrecisionDecimal {
    /// 内部使用 BigDecimal 存储
    value: BigDecimal,
    /// 最小精度要求（小数点后位数）
    min_precision: usize,
}

impl HighPrecisionDecimal {
    /// 创建新的高精度数值
    pub fn new(value: BigDecimal, min_precision: usize) -> Self {
        Self {
            value,
            min_precision,
        }
    }
    
    /// 从字符串创建高精度数值
    pub fn from_str_with_precision(s: &str, min_precision: usize) -> Result<Self, bigdecimal::ParseBigDecimalError> {
        let value = BigDecimal::from_str(s)?;
        Ok(Self::new(value, min_precision))
    }
    
    /// 从 BigDecimal 创建，自动检测精度
    pub fn from_bigdecimal(value: BigDecimal) -> Self {
        let precision = value.fractional_digit_count().max(0) as usize;
        Self::new(value, precision)
    }
    
    /// 创建极小数值（1 * 10^-scale）
    pub fn tiny(scale: i64) -> Self {
        let value = BigDecimal::new(BigInt::from(1), scale);
        Self::new(value, scale.max(0) as usize)
    }
    
    /// 获取内部值
    pub fn value(&self) -> &BigDecimal {
        &self.value
    }
    
    /// 获取最小精度
    pub fn min_precision(&self) -> usize {
        self.min_precision
    }
    
    /// 确保结果具有足够的精度
    fn ensure_precision(&self, result: BigDecimal) -> BigDecimal {
        let current_precision = result.fractional_digit_count();
        if current_precision < self.min_precision as i64 {
            // 如果精度不够，需要调整
            result.with_scale(self.min_precision as i64)
        } else {
            result
        }
    }
    
    /// 高精度加法
    pub fn add(&self, other: &Self) -> Self {
        let result_precision = self.min_precision.max(other.min_precision);
        let result = &self.value + &other.value;
        
        Self::new(result, result_precision)
    }
    
    /// 高精度减法
    pub fn sub(&self, other: &Self) -> Self {
        let result_precision = self.min_precision.max(other.min_precision);
        let result = &self.value - &other.value;
        
        Self::new(result, result_precision)
    }
    
    /// 高精度乘法
    pub fn mul(&self, other: &Self) -> Self {
        let result_precision = self.min_precision + other.min_precision;
        let result = &self.value * &other.value;
        
        Self::new(result, result_precision)
    }
    
    /// 高精度除法
    pub fn div(&self, other: &Self) -> Self {
        if other.value.is_zero() {
            panic!("除零错误");
        }
        
        let result_precision = self.min_precision.max(other.min_precision) + 10; // 额外精度
        let result = &self.value / &other.value;
        
        Self::new(result, result_precision)
    }
    
    /// 检查是否为零
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
    
    /// 检查是否为一
    pub fn is_one(&self) -> bool {
        self.value == BigDecimal::one()
    }
    
    /// 转换为标准 BigDecimal
    pub fn to_bigdecimal(&self) -> BigDecimal {
        self.value.clone()
    }
}

impl Display for HighPrecisionDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.value)
    }
}

impl From<BigDecimal> for HighPrecisionDecimal {
    fn from(value: BigDecimal) -> Self {
        Self::from_bigdecimal(value)
    }
}

impl From<&str> for HighPrecisionDecimal {
    fn from(s: &str) -> Self {
        let value = BigDecimal::from_str(s).expect("无效的数值字符串");
        Self::from_bigdecimal(value)
    }
}

impl From<i32> for HighPrecisionDecimal {
    fn from(n: i32) -> Self {
        Self::from_bigdecimal(BigDecimal::from(n))
    }
}

impl From<f64> for HighPrecisionDecimal {
    fn from(f: f64) -> Self {
        let value = BigDecimal::try_from(f).expect("无效的浮点数");
        Self::from_bigdecimal(value)
    }
}

/// 高精度数值运算函数
pub mod operations {
    use super::*;
    
    /// 高精度加法，确保结果精度
    pub fn precise_add(a: &BigDecimal, b: &BigDecimal, min_precision: usize) -> BigDecimal {
        let result = a + b;
        let current_precision = result.fractional_digit_count();
        
        if current_precision < min_precision as i64 {
            result.with_scale(min_precision as i64)
        } else {
            result
        }
    }
    
    /// 创建极小数值的精确表示
    pub fn create_tiny_decimal(scale: i64) -> BigDecimal {
        BigDecimal::new(BigInt::from(1), scale)
    }
    
    /// 检查两个高精度数值是否在指定精度下相等
    pub fn equals_with_precision(a: &BigDecimal, b: &BigDecimal, precision: usize) -> bool {
        let diff = (a - b).abs();
        let tolerance = BigDecimal::new(BigInt::from(1), precision as i64 + 1);
        diff < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_high_precision_creation() {
        let tiny = HighPrecisionDecimal::tiny(71);
        println!("极小数值: {}", tiny);
        assert_eq!(tiny.min_precision(), 71);
    }
    
    #[test]
    fn test_high_precision_addition() {
        let tiny = HighPrecisionDecimal::from_str_with_precision(
            "0.00000000000000000000000000000000000000000000000000000000000000000000001", 
            71
        ).unwrap();
        
        let point_one = HighPrecisionDecimal::from_str_with_precision("0.1", 1).unwrap();
        
        let result = tiny.add(&point_one);
        
        println!("高精度加法结果: {}", result);
        
        let expected = HighPrecisionDecimal::from_str_with_precision(
            "0.10000000000000000000000000000000000000000000000000000000000000000000001",
            71
        ).unwrap();
        
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_precision_preservation() {
        let a = HighPrecisionDecimal::tiny(50);
        let b = HighPrecisionDecimal::from("0.5");
        
        let result = a.add(&b);
        
        println!("精度保持测试: {} + {} = {}", a, b, result);
        
        // 结果应该保持最高精度
        assert!(result.min_precision() >= 50);
    }
    
    #[test]
    fn test_operations_module() {
        let tiny = operations::create_tiny_decimal(70);
        let point_one = BigDecimal::from_str("0.1").unwrap();
        
        let result = operations::precise_add(&tiny, &point_one, 70);
        
        println!("操作模块测试结果: {}", result);
        
        let expected = BigDecimal::from_str("0.1000000000000000000000000000000000000000000000000000000000000000000001").unwrap();
        
        assert!(operations::equals_with_precision(&result, &expected, 70));
    }
}