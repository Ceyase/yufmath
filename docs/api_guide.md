# Yufmath API 使用指南

## 概述

Yufmath 是一个基于 Rust 编写的高性能计算机代数系统（CAS）库，提供符号数学计算、精确算术运算和多种接口支持。

## 快速开始

### 基本使用

```rust
use yufmath::Yufmath;

// 创建 Yufmath 实例
let yuf = Yufmath::new();

// 基本计算
let result = yuf.compute("2 + 3 * 4").unwrap();
println!("计算结果: {}", result); // 输出: 14

// 符号计算
let result = yuf.compute("x + x").unwrap();
println!("简化结果: {}", result); // 输出: 2*x
```

### 配置使用

```rust
use yufmath::{Yufmath, ComputeConfig, PrecisionConfig};
use std::time::Duration;

// 创建自定义配置
let precision_config = PrecisionConfig::new()
    .with_force_exact(true)
    .with_max_precision(1000);

let config = ComputeConfig::new()
    .with_progress(true)
    .with_max_compute_time(Duration::from_secs(60))
    .with_precision(precision_config);

// 使用配置创建实例
let yuf = Yufmath::with_config(config);
```

## 核心 API

### Yufmath 主结构

`Yufmath` 是库的主要入口点，提供所有计算功能。

#### 创建实例

```rust
// 使用默认配置
let yuf = Yufmath::new();

// 使用自定义配置
let yuf = Yufmath::with_config(config);

// 使用 Default trait
let yuf = Yufmath::default();
```

#### 基本计算方法

```rust
// 解析并计算表达式
let result = yuf.compute("x^2 + 2*x + 1")?;

// 仅解析表达式
let expr = yuf.parse("x^2 + 2*x + 1")?;

// 简化表达式
let simplified = yuf.simplify(&expr)?;

// 求导
let derivative = yuf.diff(&expr, "x")?;
let derivative = yuf.differentiate(&expr, "x")?; // 别名方法

// 积分
let integral = yuf.integrate(&expr, "x")?;
```

#### 高级数学功能

```rust
// 极限计算
let limit_result = yuf.limit(&expr, "x", &point)?;

// 级数展开
let series_result = yuf.series(&expr, "x", &point, order)?;

// 数值计算
let numerical_result = yuf.numerical_evaluate(&expr, &vars)?;

// 精确计算
let exact_result = yuf.evaluate(&expr, &exact_vars)?;
```

#### 多项式运算

```rust
// 展开
let expanded = yuf.expand(&expr)?;

// 因式分解
let factored = yuf.factor(&expr)?;

// 收集同类项
let collected = yuf.collect(&expr, "x")?;
```

#### 方程求解

```rust
// 单变量方程求解
let solutions = yuf.solve(&equation, "x")?;

// 方程组求解
let system_solutions = yuf.solve_system(&equations, &vars)?;
```

#### 矩阵运算

```rust
// 矩阵加法
let sum = yuf.matrix_add(&matrix_a, &matrix_b)?;

// 矩阵乘法
let product = yuf.matrix_multiply(&matrix_a, &matrix_b)?;

// 行列式
let det = yuf.matrix_determinant(&matrix)?;

// 逆矩阵
let inverse = yuf.matrix_inverse(&matrix)?;
```

#### 数论函数

```rust
// 最大公约数
let gcd_result = yuf.gcd(&a, &b)?;

// 最小公倍数
let lcm_result = yuf.lcm(&a, &b)?;

// 素数判断
let is_prime_result = yuf.is_prime(&n)?;

// 质因数分解
let factors = yuf.prime_factors(&n)?;
```

#### 组合数学

```rust
// 二项式系数
let binomial_result = yuf.binomial(&n, &k)?;

// 排列数
let permutation_result = yuf.permutation(&n, &k)?;
```

#### 复数运算

```rust
// 复数共轭
let conjugate = yuf.complex_conjugate(&expr)?;

// 复数模长
let modulus = yuf.complex_modulus(&expr)?;

// 复数幅角
let argument = yuf.complex_argument(&expr)?;
```

#### 向量运算

```rust
// 点积
let dot_product = yuf.vector_dot(&a, &b)?;

// 叉积
let cross_product = yuf.vector_cross(&a, &b)?;

// 范数
let norm = yuf.vector_norm(&v)?;
```

#### 集合运算

```rust
// 并集
let union = yuf.set_union(&a, &b)?;

// 交集
let intersection = yuf.set_intersection(&a, &b)?;

// 差集
let difference = yuf.set_difference(&a, &b)?;
```

#### 统计函数

```rust
// 平均值
let mean_result = yuf.mean(&values)?;

// 方差
let variance_result = yuf.variance(&values)?;

// 标准差
let std_dev_result = yuf.standard_deviation(&values)?;
```

### 批量操作

```rust
// 批量计算
let results = yuf.batch_compute(&expressions);

// 批量解析
let parsed_results = yuf.batch_parse(&expressions);

// 批量简化
let simplified_results = yuf.batch_simplify(&expressions);
```

### 进度监控

```rust
// 设置进度回调
yuf.set_progress_callback(Box::new(|progress| {
    println!("进度: {:.1}% - {}", 
            progress.progress * 100.0, 
            progress.current_step);
    true // 返回 true 继续计算，false 取消计算
}));

// 带进度的计算
let result = yuf.compute_with_progress("integrate(sin(x^2), x)")?;

// 带进度的简化
let simplified = yuf.simplify_with_progress(&expr)?;

// 带进度的积分
let integral = yuf.integrate_with_progress(&expr, "x")?;

// 取消计算
yuf.cancel_computation();

// 检查是否被取消
let is_cancelled = yuf.is_cancelled();
```

### 配置管理

```rust
// 获取当前配置
let config = yuf.get_config();

// 更新配置
yuf.update_config(new_config);

// 设置格式化选项
yuf.set_format_options(FormatOptions {
    format_type: FormatType::LaTeX,
    precision: Some(10),
    use_parentheses: true,
});
```

### 性能统计

```rust
// 获取性能统计
if let Some(stats) = yuf.get_performance_stats() {
    println!("总计算次数: {}", stats.total_computations);
    println!("成功率: {:.2}%", stats.success_rate() * 100.0);
    println!("平均计算时间: {:?}", stats.avg_compute_time);
    println!("精确计算比例: {:.2}%", stats.exact_computation_ratio * 100.0);
}

// 重置性能统计
yuf.reset_performance_stats();
```

## 配置选项

### ComputeConfig

计算配置控制计算行为和性能选项。

```rust
let config = ComputeConfig::new()
    .with_progress(true)                                    // 启用进度报告
    .with_progress_interval(100)                           // 进度更新间隔（毫秒）
    .with_max_compute_time(Duration::from_secs(300))       // 最大计算时间
    .with_cancellation(true)                               // 允许取消计算
    .with_precision(precision_config);                     // 精度配置
```

### PrecisionConfig

精度配置控制数值计算的精确性。

```rust
let precision_config = PrecisionConfig::new()
    .with_force_exact(true)                    // 强制使用精确计算
    .with_max_precision(1000)                  // 最大精度位数
    .with_symbolic(true)                       // 允许符号表示
    .with_approximation_threshold(1e-10);      // 数值近似阈值
```

### FormatOptions

格式化选项控制输出格式。

```rust
let format_options = FormatOptions {
    format_type: FormatType::LaTeX,    // 输出格式：Standard, LaTeX, MathML
    precision: Some(10),               // 数值精度
    use_parentheses: true,             // 使用括号
};
```

## 数据类型

### Expression

表达式是 Yufmath 中的核心数据类型，表示数学表达式。

```rust
// 创建表达式
let expr = Expression::variable("x");
let num_expr = Expression::number(Number::from(42));
let binary_expr = Expression::binary_op(
    BinaryOperator::Add,
    Box::new(Expression::variable("x")),
    Box::new(Expression::number(Number::from(1)))
);
```

### Number

数值类型支持多种数值表示。

```rust
// 创建数值
let int_num = Number::from(42);
let float_num = Number::from(3.14);
let rational_num = Number::rational(22, 7);
let complex_num = Number::complex(3.0, 4.0);
```

### MathConstant

数学常量表示常用的数学常数。

```rust
// 使用数学常量
let pi = MathConstant::Pi;
let e = MathConstant::E;
let i = MathConstant::I;

println!("π 的符号: {}", pi.symbol());
println!("π 的名称: {}", pi.name());
println!("π 的近似值: {}", pi.approximate_value());
```

## 错误处理

Yufmath 提供了完善的错误处理机制。

```rust
use yufmath::YufmathError;

match yuf.compute("2 + + 3") {
    Ok(result) => println!("结果: {}", result),
    Err(e) => {
        println!("错误: {}", e.user_friendly_message());
        println!("建议: {:?}", e.suggestions());
        println!("严重程度: {:?}", e.severity());
        println!("可恢复: {}", e.is_recoverable());
        
        // 生成完整的错误报告
        println!("{}", e.format_error_report(Some("2 + + 3")));
    }
}
```

### 错误类型

- `ParseError`: 解析错误
- `ComputeError`: 计算错误
- `FormatError`: 格式化错误
- `ConfigError`: 配置错误
- `InternalError`: 内部错误

## 进度监控详解

### ComputeProgress

进度信息结构包含计算进度的详细信息。

```rust
pub struct ComputeProgress {
    pub current_step: String,           // 当前步骤描述
    pub progress: f64,                  // 完成百分比 (0.0 - 1.0)
    pub estimated_remaining: Option<Duration>, // 预估剩余时间
    pub expression_size: usize,         // 表达式大小
    pub completed_subtasks: usize,      // 已完成子任务数
    pub total_subtasks: usize,          // 总子任务数
}
```

### ProgressCallback

进度回调函数类型定义。

```rust
pub type ProgressCallback = Box<dyn Fn(&ComputeProgress) -> bool + Send + Sync>;
```

回调函数返回 `true` 继续计算，返回 `false` 取消计算。

## 性能统计详解

### PerformanceStats

性能统计结构包含详细的性能信息。

```rust
pub struct PerformanceStats {
    pub cache_hit_rate: f64,            // 缓存命中率
    pub avg_compute_time: Duration,     // 平均计算时间
    pub memory_usage: usize,            // 内存使用量
    pub exact_computation_ratio: f64,   // 精确计算比例
    pub last_progress: Option<ComputeProgress>, // 最近进度
    pub total_computations: usize,      // 总计算次数
    pub successful_computations: usize, // 成功计算次数
}

impl PerformanceStats {
    pub fn success_rate(&self) -> f64;          // 成功率
    pub fn failed_computations(&self) -> usize; // 失败次数
}
```

## 最佳实践

### 1. 配置优化

```rust
// 对于高精度计算
let precision_config = PrecisionConfig::new()
    .with_force_exact(true)
    .with_max_precision(2000);

// 对于快速计算
let precision_config = PrecisionConfig::new()
    .with_force_exact(false)
    .with_approximation_threshold(1e-6);
```

### 2. 错误处理

```rust
// 推荐的错误处理模式
match yuf.compute(expression) {
    Ok(result) => {
        // 处理成功结果
        println!("结果: {}", result);
    }
    Err(e) if e.is_recoverable() => {
        // 处理可恢复的错误
        eprintln!("警告: {}", e.user_friendly_message());
        for suggestion in e.suggestions() {
            eprintln!("建议: {}", suggestion);
        }
    }
    Err(e) => {
        // 处理不可恢复的错误
        eprintln!("严重错误: {}", e);
        return Err(e.into());
    }
}
```

### 3. 批量处理

```rust
// 对于大量计算，使用批量方法
let expressions: Vec<&str> = /* ... */;
let results = yuf.batch_compute(&expressions);

// 处理结果
for (expr, result) in expressions.iter().zip(results.iter()) {
    match result {
        Ok(value) => println!("{} = {}", expr, value),
        Err(e) => eprintln!("{} 计算失败: {}", expr, e),
    }
}
```

### 4. 进度监控

```rust
// 对于长时间计算，使用进度监控
yuf.set_progress_callback(Box::new(|progress| {
    // 更新 UI 或打印进度
    if progress.progress > 0.0 {
        println!("进度: {:.1}% - {}", 
                progress.progress * 100.0, 
                progress.current_step);
    }
    
    // 检查用户是否请求取消
    !should_cancel()
}));
```

### 5. 内存管理

```rust
// 定期重置性能统计以释放内存
if let Some(stats) = yuf.get_performance_stats() {
    if stats.total_computations > 10000 {
        yuf.reset_performance_stats();
    }
}
```

## 线程安全

Yufmath 的只读操作是线程安全的，可以在多线程环境中安全使用：

```rust
use std::sync::Arc;
use std::thread;

let yuf = Arc::new(Yufmath::new());

let handles: Vec<_> = (0..4).map(|i| {
    let yuf_clone = Arc::clone(&yuf);
    thread::spawn(move || {
        yuf_clone.compute(&format!("{} + {}", i, i + 1))
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap();
    println!("结果: {:?}", result);
}
```

注意：修改操作（如设置进度回调、更新配置）不是线程安全的，需要适当的同步机制。

## 示例项目

完整的示例项目可以在 `examples/` 目录中找到：

- `api_demo.rs`: 基本 API 使用示例
- `advanced_demo.rs`: 高级功能示例
- `performance_demo.rs`: 性能优化示例
- `error_handling_demo.rs`: 错误处理示例

## 版本兼容性

当前版本：0.1.0

API 稳定性：
- 核心 API（Yufmath 主结构）：稳定
- 配置选项：可能在未来版本中扩展
- 错误类型：可能在未来版本中细化
- 进度监控：可能在未来版本中增强

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。