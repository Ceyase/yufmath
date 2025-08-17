# Yufmath 架构文档

## 项目结构

```
yufmath/
├── src/
│   ├── lib.rs              # 库入口点
│   ├── bin/
│   │   └── yufmath.rs      # 命令行工具入口
│   ├── core/               # 核心数据结构
│   │   ├── mod.rs
│   │   ├── expression.rs   # 表达式定义
│   │   ├── number.rs       # 数值类型
│   │   ├── constants.rs    # 数学常量
│   │   ├── operators.rs    # 运算符定义
│   │   └── types.rs        # 类型系统
│   ├── parser/             # 表达式解析器
│   │   ├── mod.rs
│   │   ├── lexer.rs        # 词法分析器
│   │   ├── syntax.rs       # 语法分析器
│   │   └── error.rs        # 解析错误
│   ├── engine/             # 计算引擎
│   │   ├── mod.rs
│   │   ├── compute.rs      # 基础计算
│   │   ├── simplify.rs     # 表达式简化
│   │   ├── calculus.rs     # 微积分运算
│   │   ├── algebra.rs      # 代数运算
│   │   └── error.rs        # 计算错误
│   ├── formatter/          # 格式化器
│   │   ├── mod.rs
│   │   ├── standard.rs     # 标准格式
│   │   ├── latex.rs        # LaTeX 格式
│   │   └── mathml.rs       # MathML 格式
│   ├── api/                # Rust API
│   │   ├── mod.rs
│   │   ├── yufmath.rs      # 主要 API
│   │   ├── config.rs       # 配置选项
│   │   ├── progress.rs     # 进度监控
│   │   └── error.rs        # 顶层错误
│   ├── cli/                # 命令行工具
│   │   ├── mod.rs
│   │   ├── args.rs         # 参数定义
│   │   ├── commands.rs     # 命令实现
│   │   └── interactive.rs  # 交互模式
│   └── ffi/                # 外部函数接口
│       ├── mod.rs
│       ├── c_api.rs        # C API
│       └── types.rs        # FFI 类型
├── benches/
│   └── benchmarks.rs       # 性能基准测试
├── Cargo.toml              # 项目配置
├── README.md               # 项目说明
├── LICENSE-MIT             # MIT 许可证
└── ARCHITECTURE.md         # 架构文档
```

## 核心组件

### 1. 核心数据结构 (core/)

- **Expression**: 数学表达式的抽象语法树表示
- **Number**: 支持多种精度的数值类型系统
- **MathConstant**: 数学常量定义（π、e、i 等）
- **Operators**: 二元和一元运算符定义
- **Types**: 表达式类型系统

### 2. 解析器 (parser/)

- **Lexer**: 词法分析器，将字符串分解为词法单元
- **SyntaxParser**: 语法分析器，构建抽象语法树
- **ParseError**: 解析错误处理

### 3. 计算引擎 (engine/)

- **ComputeEngine**: 核心计算接口
- **BasicComputeEngine**: 基础计算引擎实现
- **简化器**: 代数表达式简化
- **微积分**: 符号求导和积分
- **代数运算**: 多项式运算、因式分解等

### 4. 格式化器 (formatter/)

- **StandardFormatter**: 标准数学记号格式化
- **TerminalFormatter**: 终端彩色格式化，支持颜色输出和数值近似值显示
- **LaTeXFormatter**: LaTeX 格式输出
- **MathMLFormatter**: MathML 格式输出

### 5. API 接口 (api/)

- **Yufmath**: 主要的 Rust API 入口点
- **配置系统**: 计算配置和精度配置
- **进度监控**: 计算进度和性能统计

### 6. 命令行工具 (cli/)

- **参数解析**: 使用 clap 处理命令行参数
- **命令实现**: 各种数学运算命令
- **交互模式**: 增强的 REPL 环境，支持：
  - 彩色语法高亮
  - 数值近似值显示（如 √3 ≈ 1.732051）
  - 智能数学符号（×、÷、π、√ 等）
  - 可配置的显示选项

### 7. FFI 接口 (ffi/)

- **C API**: C 兼容的接口层
- **类型转换**: Rust 和 C 类型之间的转换

## 设计原则

### 1. 精确计算优先

- 默认使用任意精度数值类型
- 避免浮点数精度损失
- 无法精确计算时保持符号形式

### 2. 模块化设计

- 清晰的模块边界
- 可插拔的组件架构
- 易于扩展和维护

### 3. 类型安全

- 利用 Rust 的类型系统
- 编译时错误检查
- 内存安全保证

### 4. 高性能

- 智能缓存机制
- 惰性求值优化
- 并行计算支持

## 当前状态

### ✅ 已完成

- [x] 项目基础架构搭建
- [x] 核心数据结构定义
- [x] 模块结构创建
- [x] 错误处理系统
- [x] 基本的 trait 接口定义
- [x] 项目文档和配置

### ⏳ 开发中

- [ ] 基础数值系统实现
- [ ] 表达式解析器
- [ ] 计算引擎核心功能
- [ ] 格式化器实现
- [ ] 命令行工具

### 📋 计划中

- [ ] 微积分功能
- [ ] 高级代数运算
- [ ] 性能优化
- [ ] C++ 绑定
- [ ] 完整测试覆盖

## 依赖关系

### 核心依赖

- `num-bigint`: 任意精度整数
- `num-rational`: 任意精度有理数
- `num-complex`: 复数运算
- `num-traits`: 数值 trait
- `bigdecimal`: 任意精度实数
- `thiserror`: 错误处理

### 工具依赖

- `clap`: 命令行参数解析
- `rustyline`: 交互式输入支持
- `colored`: 终端颜色输出
- `indicatif`: 进度条支持
- `rayon`: 并行计算支持
- `criterion`: 性能基准测试

## 编译和测试

```bash
# 检查编译
cargo check

# 运行测试
cargo test

# 构建发布版本
cargo build --release

# 运行基准测试
cargo bench

# 构建命令行工具
cargo build --bin yufmath
```