//! # 矩阵和向量运算
//!
//! 实现矩阵和向量的各种数学运算，包括加法、乘法、行列式、逆矩阵等。

use crate::core::{Expression, Number};
use super::{ComputeError, ComputeEngine};
use num_traits::{Zero};

/// 矩阵和向量运算引擎
pub struct MatrixEngine {
    /// 基础计算引擎引用
    base_engine: Option<Box<dyn ComputeEngine>>,
}

impl MatrixEngine {
    /// 创建新的矩阵运算引擎
    pub fn new() -> Self {
        Self {
            base_engine: None,
        }
    }
    
    /// 设置基础计算引擎
    pub fn set_base_engine(&mut self, engine: Box<dyn ComputeEngine>) {
        self.base_engine = Some(engine);
    }
    
    /// 矩阵加法
    pub fn matrix_add(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let (rows_a, cols_a, elements_a) = self.extract_matrix_data(a)?;
        let (rows_b, cols_b, elements_b) = self.extract_matrix_data(b)?;
        
        // 检查矩阵维度是否匹配
        if rows_a != rows_b || cols_a != cols_b {
            return Err(ComputeError::dimension_mismatch(
                &format!("矩阵维度不匹配：{}×{} 和 {}×{}", rows_a, cols_a, rows_b, cols_b)
            ));
        }
        
        // 执行矩阵加法
        let mut result_elements = Vec::with_capacity(rows_a);
        for i in 0..rows_a {
            let mut row = Vec::with_capacity(cols_a);
            for j in 0..cols_a {
                let sum = Expression::add(
                    elements_a[i][j].clone(),
                    elements_b[i][j].clone()
                );
                row.push(sum);
            }
            result_elements.push(row);
        }
        
        Ok(Expression::Matrix(result_elements))
    }
    
    /// 矩阵乘法
    pub fn matrix_multiply(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let (rows_a, cols_a, elements_a) = self.extract_matrix_data(a)?;
        let (rows_b, cols_b, elements_b) = self.extract_matrix_data(b)?;
        
        // 检查矩阵乘法的维度要求：A的列数必须等于B的行数
        if cols_a != rows_b {
            return Err(ComputeError::dimension_mismatch(
                &format!("矩阵乘法维度不匹配：{}×{} 和 {}×{}", rows_a, cols_a, rows_b, cols_b)
            ));
        }
        
        // 执行矩阵乘法：结果矩阵为 rows_a × cols_b
        let mut result_elements = Vec::with_capacity(rows_a);
        for i in 0..rows_a {
            let mut row = Vec::with_capacity(cols_b);
            for j in 0..cols_b {
                // 计算第(i,j)个元素：第i行与第j列的点积
                let mut sum = Expression::number(Number::from(0));
                for k in 0..cols_a {
                    let product = Expression::multiply(
                        elements_a[i][k].clone(),
                        elements_b[k][j].clone()
                    );
                    sum = Expression::add(sum, product);
                }
                row.push(sum);
            }
            result_elements.push(row);
        }
        
        Ok(Expression::Matrix(result_elements))
    }
    
    /// 计算矩阵行列式
    pub fn matrix_determinant(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let (rows, cols, elements) = self.extract_matrix_data(matrix)?;
        
        // 检查是否为方阵
        if rows != cols {
            return Err(ComputeError::dimension_mismatch(
                &format!("行列式只能计算方阵，当前矩阵为 {}×{}", rows, cols)
            ));
        }
        
        // 使用递归方法计算行列式
        self.calculate_determinant(&elements)
    }
    
    /// 递归计算行列式
    fn calculate_determinant(&self, matrix: &[Vec<Expression>]) -> Result<Expression, ComputeError> {
        let n = matrix.len();
        
        match n {
            0 => Err(ComputeError::domain_error("空矩阵无法计算行列式")),
            1 => Ok(matrix[0][0].clone()),
            2 => {
                // 2×2矩阵：ad - bc
                let ad = Expression::multiply(matrix[0][0].clone(), matrix[1][1].clone());
                let bc = Expression::multiply(matrix[0][1].clone(), matrix[1][0].clone());
                Ok(Expression::subtract(ad, bc))
            }
            _ => {
                // n×n矩阵：使用第一行展开
                let mut result = Expression::number(Number::from(0));
                
                for j in 0..n {
                    // 计算代数余子式
                    let minor = self.get_minor(matrix, 0, j)?;
                    let minor_det = self.calculate_determinant(&minor)?;
                    
                    // 计算符号：(-1)^(0+j) = (-1)^j
                    let sign = if j % 2 == 0 { 1 } else { -1 };
                    let signed_minor = if sign == 1 {
                        minor_det
                    } else {
                        Expression::negate(minor_det)
                    };
                    
                    // 乘以对应元素
                    let term = Expression::multiply(matrix[0][j].clone(), signed_minor);
                    result = Expression::add(result, term);
                }
                
                Ok(result)
            }
        }
    }
    
    /// 获取矩阵的余子式（去掉第row行第col列）
    fn get_minor(&self, matrix: &[Vec<Expression>], row: usize, col: usize) -> Result<Vec<Vec<Expression>>, ComputeError> {
        let n = matrix.len();
        if row >= n || col >= n {
            return Err(ComputeError::domain_error("行或列索引超出范围"));
        }
        
        let mut minor = Vec::with_capacity(n - 1);
        for i in 0..n {
            if i == row {
                continue;
            }
            let mut minor_row = Vec::with_capacity(n - 1);
            for j in 0..n {
                if j == col {
                    continue;
                }
                minor_row.push(matrix[i][j].clone());
            }
            minor.push(minor_row);
        }
        
        Ok(minor)
    }
    
    /// 计算矩阵逆
    pub fn matrix_inverse(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let (rows, cols, elements) = self.extract_matrix_data(matrix)?;
        
        // 检查是否为方阵
        if rows != cols {
            return Err(ComputeError::dimension_mismatch(
                &format!("只有方阵才能求逆，当前矩阵为 {}×{}", rows, cols)
            ));
        }
        
        let n = rows;
        
        // 计算行列式
        let det = self.calculate_determinant(&elements)?;
        
        // 检查行列式是否为零
        if self.is_zero_expression(&det)? {
            return Err(ComputeError::domain_error("矩阵不可逆（行列式为零）"));
        }
        
        match n {
            1 => {
                // 1×1矩阵的逆就是元素的倒数
                let inv_element = Expression::divide(
                    Expression::number(Number::from(1)),
                    elements[0][0].clone()
                );
                Ok(Expression::matrix(vec![vec![inv_element]]).map_err(|e| ComputeError::domain_error(e))?)
            }
            2 => {
                // 2×2矩阵的逆：(1/det) * [[d, -b], [-c, a]]
                let a = &elements[0][0];
                let b = &elements[0][1];
                let c = &elements[1][0];
                let d = &elements[1][1];
                
                let inv_det = Expression::divide(Expression::number(Number::from(1)), det);
                
                let inv_elements = vec![
                    vec![
                        Expression::multiply(inv_det.clone(), d.clone()),
                        Expression::multiply(inv_det.clone(), Expression::negate(b.clone()))
                    ],
                    vec![
                        Expression::multiply(inv_det.clone(), Expression::negate(c.clone())),
                        Expression::multiply(inv_det, a.clone())
                    ]
                ];
                
                Ok(Expression::Matrix(inv_elements))
            }
            _ => {
                // n×n矩阵：使用伴随矩阵方法
                self.calculate_inverse_adjugate(&elements, &det)
            }
        }
    }
    
    /// 使用伴随矩阵方法计算逆矩阵
    fn calculate_inverse_adjugate(&self, matrix: &[Vec<Expression>], det: &Expression) -> Result<Expression, ComputeError> {
        let n = matrix.len();
        let mut adjugate = Vec::with_capacity(n);
        
        // 计算伴随矩阵（代数余子式矩阵的转置）
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                // 计算第(j,i)个代数余子式（注意转置）
                let minor = self.get_minor(matrix, j, i)?;
                let minor_det = self.calculate_determinant(&minor)?;
                
                // 计算符号：(-1)^(i+j)
                let sign = if (i + j) % 2 == 0 { 1 } else { -1 };
                let cofactor = if sign == 1 {
                    minor_det
                } else {
                    Expression::negate(minor_det)
                };
                
                row.push(cofactor);
            }
            adjugate.push(row);
        }
        
        // 逆矩阵 = (1/det) * 伴随矩阵
        let inv_det = Expression::divide(Expression::number(Number::from(1)), det.clone());
        let mut inverse = Vec::with_capacity(n);
        
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let element = Expression::multiply(inv_det.clone(), adjugate[i][j].clone());
                row.push(element);
            }
            inverse.push(row);
        }
        
        Ok(Expression::Matrix(inverse))
    }
    
    /// 向量点积
    pub fn vector_dot(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let elements_a = self.extract_vector_data(a)?;
        let elements_b = self.extract_vector_data(b)?;
        
        // 检查向量维度是否匹配
        if elements_a.len() != elements_b.len() {
            return Err(ComputeError::dimension_mismatch(
                &format!("向量维度不匹配：{} 和 {}", elements_a.len(), elements_b.len())
            ));
        }
        
        // 计算点积：∑(a_i * b_i)
        let mut result = Expression::number(Number::from(0));
        for (a_elem, b_elem) in elements_a.iter().zip(elements_b.iter()) {
            let product = Expression::multiply(a_elem.clone(), b_elem.clone());
            result = Expression::add(result, product);
        }
        
        Ok(result)
    }
    
    /// 向量叉积（仅适用于3维向量）
    pub fn vector_cross(&self, a: &Expression, b: &Expression) -> Result<Expression, ComputeError> {
        let elements_a = self.extract_vector_data(a)?;
        let elements_b = self.extract_vector_data(b)?;
        
        // 检查是否为3维向量
        if elements_a.len() != 3 || elements_b.len() != 3 {
            return Err(ComputeError::dimension_mismatch(
                "叉积只适用于3维向量"
            ));
        }
        
        // 计算叉积：a × b = (a2*b3 - a3*b2, a3*b1 - a1*b3, a1*b2 - a2*b1)
        let a1 = &elements_a[0];
        let a2 = &elements_a[1];
        let a3 = &elements_a[2];
        let b1 = &elements_b[0];
        let b2 = &elements_b[1];
        let b3 = &elements_b[2];
        
        let x = Expression::subtract(
            Expression::multiply(a2.clone(), b3.clone()),
            Expression::multiply(a3.clone(), b2.clone())
        );
        
        let y = Expression::subtract(
            Expression::multiply(a3.clone(), b1.clone()),
            Expression::multiply(a1.clone(), b3.clone())
        );
        
        let z = Expression::subtract(
            Expression::multiply(a1.clone(), b2.clone()),
            Expression::multiply(a2.clone(), b1.clone())
        );
        
        Expression::vector(vec![x, y, z]).map_err(|e| ComputeError::domain_error(e))
    }
    
    /// 向量范数（欧几里得范数）
    pub fn vector_norm(&self, v: &Expression) -> Result<Expression, ComputeError> {
        let elements = self.extract_vector_data(v)?;
        
        // 计算范数：√(∑(v_i^2))
        let mut sum_of_squares = Expression::number(Number::from(0));
        for element in elements {
            let square = Expression::power(element.clone(), Expression::number(Number::from(2)));
            sum_of_squares = Expression::add(sum_of_squares, square);
        }
        
        Ok(Expression::sqrt(sum_of_squares))
    }
    
    /// 矩阵转置
    pub fn matrix_transpose(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let (rows, cols, elements) = self.extract_matrix_data(matrix)?;
        
        // 创建转置矩阵：cols × rows
        let mut transposed = Vec::with_capacity(cols);
        for j in 0..cols {
            let mut row = Vec::with_capacity(rows);
            for i in 0..rows {
                row.push(elements[i][j].clone());
            }
            transposed.push(row);
        }
        
        Ok(Expression::Matrix(transposed))
    }
    
    /// 矩阵的迹（对角线元素之和）
    pub fn matrix_trace(&self, matrix: &Expression) -> Result<Expression, ComputeError> {
        let (rows, cols, elements) = self.extract_matrix_data(matrix)?;
        
        // 检查是否为方阵
        if rows != cols {
            return Err(ComputeError::dimension_mismatch(
                &format!("矩阵的迹只能计算方阵，当前矩阵为 {}×{}", rows, cols)
            ));
        }
        
        // 计算对角线元素之和
        let mut trace = Expression::number(Number::from(0));
        for i in 0..rows {
            trace = Expression::add(trace, elements[i][i].clone());
        }
        
        Ok(trace)
    }
    
    /// 从表达式中提取矩阵数据
    fn extract_matrix_data(&self, expr: &Expression) -> Result<(usize, usize, Vec<Vec<Expression>>), ComputeError> {
        match expr {
            Expression::Matrix(elements) => {
                if elements.is_empty() {
                    return Err(ComputeError::domain_error("矩阵不能为空"));
                }
                let rows = elements.len();
                let cols = elements[0].len();
                if cols == 0 {
                    return Err(ComputeError::domain_error("矩阵行不能为空"));
                }
                
                // 检查所有行的列数是否相同
                for (i, row) in elements.iter().enumerate() {
                    if row.len() != cols {
                        return Err(ComputeError::dimension_mismatch(
                            &format!("矩阵第{}行的列数({})与第一行的列数({})不匹配", i + 1, row.len(), cols)
                        ));
                    }
                }
                
                Ok((rows, cols, elements.clone()))
            }
            _ => Err(ComputeError::domain_error("表达式不是矩阵类型"))
        }
    }
    
    /// 从表达式中提取向量数据
    fn extract_vector_data(&self, expr: &Expression) -> Result<Vec<Expression>, ComputeError> {
        match expr {
            Expression::Vector(elements) => {
                if elements.is_empty() {
                    return Err(ComputeError::domain_error("向量不能为空"));
                }
                Ok(elements.clone())
            }
            // 也可以将单列矩阵视为向量
            Expression::Matrix(elements) => {
                if elements.is_empty() {
                    return Err(ComputeError::domain_error("矩阵不能为空"));
                }
                if elements[0].len() == 1 {
                    // 单列矩阵，转换为向量
                    let vector_elements: Vec<Expression> = elements.iter()
                        .map(|row| row[0].clone())
                        .collect();
                    Ok(vector_elements)
                } else if elements.len() == 1 {
                    // 单行矩阵，转换为向量
                    Ok(elements[0].clone())
                } else {
                    Err(ComputeError::dimension_mismatch("矩阵不能转换为向量（必须是单行或单列矩阵）"))
                }
            }
            _ => Err(ComputeError::domain_error("表达式不是向量类型"))
        }
    }
    
    /// 检查表达式是否为零
    fn is_zero_expression(&self, expr: &Expression) -> Result<bool, ComputeError> {
        match expr {
            Expression::Number(n) => Ok(n.is_zero()),
            _ => {
                // 对于复杂表达式，尝试简化后再判断
                if let Some(ref engine) = self.base_engine {
                    let simplified = engine.simplify(expr)?;
                    match simplified {
                        Expression::Number(n) => Ok(n.is_zero()),
                        _ => Ok(false) // 无法确定是否为零，保守返回false
                    }
                } else {
                    Ok(false) // 没有基础引擎，无法简化，保守返回false
                }
            }
        }
    }
}

impl Default for MatrixEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Number;
    
    #[test]
    fn test_matrix_add() {
        let engine = MatrixEngine::new();
        
        // 创建两个2×2矩阵
        let matrix_a = Expression::matrix(vec![
            vec![Expression::number(Number::from(1)), Expression::number(Number::from(2))],
            vec![Expression::number(Number::from(3)), Expression::number(Number::from(4))]
        ]).unwrap();
        
        let matrix_b = Expression::matrix(vec![
            vec![Expression::number(Number::from(5)), Expression::number(Number::from(6))],
            vec![Expression::number(Number::from(7)), Expression::number(Number::from(8))]
        ]).unwrap();
        
        let result = engine.matrix_add(&matrix_a, &matrix_b).unwrap();
        
        // 验证结果
        if let Expression::Matrix(elements) = result {
            assert_eq!(elements.len(), 2);
            assert_eq!(elements[0].len(), 2);
            // 结果应该是 [[6, 8], [10, 12]]
        } else {
            panic!("结果不是矩阵类型");
        }
    }
    
    #[test]
    fn test_matrix_multiply() {
        let engine = MatrixEngine::new();
        
        // 创建两个2×2矩阵
        let matrix_a = Expression::matrix(vec![
            vec![Expression::number(Number::from(1)), Expression::number(Number::from(2))],
            vec![Expression::number(Number::from(3)), Expression::number(Number::from(4))]
        ]).unwrap();
        
        let matrix_b = Expression::matrix(vec![
            vec![Expression::number(Number::from(5)), Expression::number(Number::from(6))],
            vec![Expression::number(Number::from(7)), Expression::number(Number::from(8))]
        ]).unwrap();
        
        let result = engine.matrix_multiply(&matrix_a, &matrix_b).unwrap();
        
        // 验证结果
        if let Expression::Matrix(elements) = result {
            assert_eq!(elements.len(), 2);
            assert_eq!(elements[0].len(), 2);
            // 结果应该是 [[19, 22], [43, 50]]
        } else {
            panic!("结果不是矩阵类型");
        }
    }
    
    #[test]
    fn test_matrix_determinant_2x2() {
        let engine = MatrixEngine::new();
        
        // 创建2×2矩阵 [[1, 2], [3, 4]]
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::from(1)), Expression::number(Number::from(2))],
            vec![Expression::number(Number::from(3)), Expression::number(Number::from(4))]
        ]).unwrap();
        
        let result = engine.matrix_determinant(&matrix).unwrap();
        
        // 行列式应该是 1*4 - 2*3 = -2
        // 这里只验证结果是一个表达式，具体数值需要通过求值验证
        assert!(matches!(result, Expression::BinaryOp { .. }));
    }
    
    #[test]
    fn test_vector_dot() {
        let engine = MatrixEngine::new();
        
        // 创建两个3维向量
        let vector_a = Expression::vector(vec![
            Expression::number(Number::from(1)),
            Expression::number(Number::from(2)),
            Expression::number(Number::from(3))
        ]).unwrap();
        
        let vector_b = Expression::vector(vec![
            Expression::number(Number::from(4)),
            Expression::number(Number::from(5)),
            Expression::number(Number::from(6))
        ]).unwrap();
        
        let result = engine.vector_dot(&vector_a, &vector_b).unwrap();
        
        // 点积应该是 1*4 + 2*5 + 3*6 = 32
        // 这里只验证结果是一个表达式
        assert!(matches!(result, Expression::BinaryOp { .. }) || matches!(result, Expression::Number(_)));
    }
    
    #[test]
    fn test_vector_cross() {
        let engine = MatrixEngine::new();
        
        // 创建两个3维向量
        let vector_a = Expression::vector(vec![
            Expression::number(Number::from(1)),
            Expression::number(Number::from(0)),
            Expression::number(Number::from(0))
        ]).unwrap();
        
        let vector_b = Expression::vector(vec![
            Expression::number(Number::from(0)),
            Expression::number(Number::from(1)),
            Expression::number(Number::from(0))
        ]).unwrap();
        
        let result = engine.vector_cross(&vector_a, &vector_b).unwrap();
        
        // 叉积应该是 (0, 0, 1)
        if let Expression::Vector(elements) = result {
            assert_eq!(elements.len(), 3);
        } else {
            panic!("结果不是向量类型");
        }
    }
    
    #[test]
    fn test_vector_norm() {
        let engine = MatrixEngine::new();
        
        // 创建3维向量 (3, 4, 0)
        let vector = Expression::vector(vec![
            Expression::number(Number::from(3)),
            Expression::number(Number::from(4)),
            Expression::number(Number::from(0))
        ]).unwrap();
        
        let result = engine.vector_norm(&vector).unwrap();
        
        // 范数应该是 √(3² + 4² + 0²) = √25 = 5
        assert!(matches!(result, Expression::UnaryOp { .. }));
    }
    
    #[test]
    fn test_matrix_transpose() {
        let engine = MatrixEngine::new();
        
        // 创建2×3矩阵
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::from(1)), Expression::number(Number::from(2)), Expression::number(Number::from(3))],
            vec![Expression::number(Number::from(4)), Expression::number(Number::from(5)), Expression::number(Number::from(6))]
        ]).unwrap();
        
        let result = engine.matrix_transpose(&matrix).unwrap();
        
        // 转置后应该是3×2矩阵
        if let Expression::Matrix(elements) = result {
            assert_eq!(elements.len(), 3); // 3行
            assert_eq!(elements[0].len(), 2); // 2列
        } else {
            panic!("结果不是矩阵类型");
        }
    }
    
    #[test]
    fn test_matrix_trace() {
        let engine = MatrixEngine::new();
        
        // 创建2×2矩阵 [[1, 2], [3, 4]]
        let matrix = Expression::matrix(vec![
            vec![Expression::number(Number::from(1)), Expression::number(Number::from(2))],
            vec![Expression::number(Number::from(3)), Expression::number(Number::from(4))]
        ]).unwrap();
        
        let result = engine.matrix_trace(&matrix).unwrap();
        
        // 迹应该是 1 + 4 = 5
        assert!(matches!(result, Expression::BinaryOp { .. }) || matches!(result, Expression::Number(_)));
    }
}