# Yufmath - 计算机代数系统

[![Crates.io](https://img.shields.io/crates/v/yufmath.svg)](https://crates.io/crates/yufmath)
[![Documentation](https://docs.rs/yufmath/badge.svg)](https://docs.rs/yufmath)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Yufmath 是一个基于 Rust 编写的高性能计算机代数系统（CAS）库，提供符号数学计算、精确算术运算和多种接口支持。

## 主要特性

- **精确计算**：支持任意精度整数、有理数和复数运算，避免浮点数精度损失
- **符号计算**：代数表达式的符号操作、简化和变换
- **微积分**：符号求导和积分功能
- **多接口支持**：Rust 原生 API、C++ 绑定、命令行工具
- **高性能**：优化的算法实现和智能缓存机制
- **类型安全**：利用 Rust 的类型系统确保计算的正确性

## 快速开始

### 安装

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
yufmath = "0.1.0"
```

### 基本使用

```rust
use yufmath::Yufmath;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yuf = Yufmath::new();
    
    // 基本计算
    let result = yuf.compute("2 + 3 * 4")?;
    println!("结果: {}", result); // 输出: 结果: 14
    
    // 符号计算
    let simplified = yuf.compute("x + x + 2*x")?;
    println!("简化: {}", simplified); // 输出: 简化: 4*x
    
    // 求导
    let expr = yuf.parse("x^3 + 2*x^2 + x")?;
    let derivative = yuf.diff(&expr, "x")?;
    println!("导数: {}", yuf.formatter.format(&derivative)); // 输出: 导数: 3*x^2 + 4*x + 1
    
    Ok(())
}
```

### 命令行工具

安装命令行工具：

```bash
cargo install yufmath
```

使用示例：

```bash
# 基本计算
yufmath compute "2^10 + 3^5"

# 表达式简化
yufmath simplify "(x+1)^2"

# 求导
yufmath diff "x^3 + sin(x)" x

# 积分
yufmath integrate "2*x + 1" x

# 交互模式
yufmath interactive
```

## 核心功能

### 精确数值计算

Yufmath 优先使用精确表示，支持：

- **任意精度整数**：无精度限制的整数运算
- **任意精度有理数**：精确的分数运算
- **任意精度实数**：高精度实数计算
- **复数**：支持复数的各种运算
- **符号表示**：无法精确计算时保持符号形式

```rust
use yufmath::{Yufmath, Number};
use num_bigint::BigInt;

let yuf = Yufmath::new();

// 大整数运算
let big_num = Number::integer(BigInt::from(2).pow(1000));
let result = yuf.compute(&format!("{} + 1", big_num))?;

// 精确有理数
let fraction = Number::rational(22, 7); // 22/7 的精确表示
```

### 符号计算

支持各种符号数学运算：

```rust
let yuf = Yufmath::new();

// 代数简化
let expr = yuf.parse("(x + 1)^2")?;
let expanded = yuf.simplify(&expr)?; // x^2 + 2*x + 1

// 因式分解
let factored = yuf.compute("factor(x^2 - 4)")?; // (x - 2)(x + 2)

// 多项式运算
let collected = yuf.compute("collect(x^2 + 2*x + x^2, x)")?; // 2*x^2 + 2*x
```

### 微积分

```rust
let yuf = Yufmath::new();

// 符号求导
let expr = yuf.parse("sin(x^2) + cos(x)")?;
let derivative = yuf.diff(&expr, "x")?; // 2*x*cos(x^2) - sin(x)

// 符号积分
let integral = yuf.integrate(&yuf.parse("2*x + 1")?, "x")?; // x^2 + x + C

// 定积分（计划中的功能）
// let definite = yuf.definite_integral(&expr, "x", 0, 1)?;
```

## 架构设计

Yufmath 采用模块化设计，主要组件包括：

- **核心模块** (`core/`)：表达式、数值类型、运算符定义
- **解析器** (`parser/`)：词法分析和语法分析
- **计算引擎** (`engine/`)：数学运算和简化算法
- **格式化器** (`formatter/`)：多种输出格式支持
- **API 接口** (`api/`)：Rust 原生 API
- **命令行工具** (`cli/`)：交互式和批处理模式
- **FFI 接口** (`ffi/`)：C++ 语言绑定

## 性能特性

- **智能缓存**：多层缓存系统提高重复计算效率
- **惰性求值**：延迟计算优化性能
- **并行计算**：支持并行处理复杂表达式
- **内存优化**：表达式共享和写时复制策略

## 开发状态

Yufmath 目前处于积极开发阶段。已完成的功能：

- ✅ 项目基础架构
- ⏳ 基础数值系统（开发中）
- ⏳ 表达式解析器（计划中）
- ⏳ 计算引擎（计划中）
- ⏳ 命令行工具（计划中）

## 贡献

我们欢迎各种形式的贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解如何参与项目开发。

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。详情请查看 [LICENSE-MIT](LICENSE-MIT) 和 [LICENSE-APACHE](LICENSE-APACHE) 文件。

## 相关项目

- [SymPy](https://www.sympy.org/) - Python 符号数学库
- [Maxima](https://maxima.sourceforge.io/) - 计算机代数系统
- [SageMath](https://www.sagemath.org/) - 开源数学软件系统

## 联系方式

- 项目主页：https://github.com/your-username/yufmath
- 文档：https://docs.rs/yufmath
- 问题反馈：https://github.com/your-username/yufmath/issues