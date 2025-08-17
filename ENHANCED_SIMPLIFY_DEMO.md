# 增强化简功能演示

Yufmath 现在支持强大的运行时化简增强功能，可以自动化简各种数学表达式。

## 功能特性

### 1. 自动化简开关

在交互模式中，可以使用 `enhanced` 命令来开启或关闭增强化简功能：

```
yufmath> enhanced
增强化简功能: 开启
```

### 2. 根号表达式化简

#### 同类根号合并
```
yufmath> sqrt(2) + sqrt(8)
3*sqrt(2)

yufmath> sqrt(18) + sqrt(2)
4*sqrt(2)
```

#### 完全平方因子提取
```
yufmath> sqrt(8)
2*sqrt(2)

yufmath> sqrt(18)
3*sqrt(2)

yufmath> sqrt(50)
5*sqrt(2)
```

#### 根号乘除法化简
```
yufmath> sqrt(3) * sqrt(12)
6

yufmath> sqrt(2) * sqrt(8)
4
```

### 3. 三角函数化简

#### 诱导公式
```
yufmath> sin(-x)
-sin(x)

yufmath> cos(-x)
cos(x)

yufmath> tan(-x)
-tan(x)
```

#### 特殊角度值
```
yufmath> sin(0)
0

yufmath> cos(0)
1

yufmath> sin(pi/6)
1/2
```

#### 三角恒等式
```
yufmath> sin(x)^2 + cos(x)^2
1
```

### 4. 代数化简增强

#### 分数运算
```
yufmath> 1/2 + 1/3
5/6

yufmath> 2/4 + 3/6
1

yufmath> 6/9
2/3
```

#### 二项式展开识别
```
yufmath> (x + 1) * (x - 1)
x^2 - 1
```

#### 基本代数规则
```
yufmath> x + 0
x

yufmath> 1 * x
x

yufmath> x^1
x

yufmath> 0 * x
0
```

## 使用方法

### 命令行模式

```bash
# 启动交互模式
yufmath interactive

# 直接计算（自动启用增强化简）
yufmath compute "sqrt(2) + sqrt(8)"
```

### 编程接口

```rust
use yufmath::Yufmath;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut yuf = Yufmath::new();
    
    // 检查增强化简状态
    println!("增强化简: {}", yuf.is_enhanced_simplify_enabled());
    
    // 计算表达式（自动应用增强化简）
    let result = yuf.compute("sqrt(2) + sqrt(8)")?;
    println!("结果: {}", result); // 输出: 3*sqrt(2)
    
    // 关闭增强化简
    yuf.set_enhanced_simplify(false);
    
    // 重新计算（不应用增强化简）
    let result = yuf.compute("sqrt(2) + sqrt(8)")?;
    println!("结果: {}", result); // 输出: sqrt(2) + sqrt(8)
    
    Ok(())
}
```

## 技术实现

### 架构设计

1. **EnhancedComputeEngine**: 集成增强化简功能的计算引擎
2. **EnhancedSimplifier**: 专门的增强化简器，包含：
   - 根号化简规则
   - 三角函数化简规则
   - 高级代数化简规则
3. **自动化简机制**: 每次运算后自动应用化简规则
4. **迭代化简**: 多轮化简直到表达式不再变化

### 化简规则

#### 根号化简
- 完全平方因子提取：√(a²×b) = a√b
- 同类根号合并：c₁√a + c₂√a = (c₁+c₂)√a
- 根号乘法：√a × √b = √(a×b)
- 根号除法：√a ÷ √b = √(a÷b)

#### 三角函数化简
- 诱导公式：sin(-x) = -sin(x), cos(-x) = cos(x)
- 特殊角度：sin(0) = 0, cos(0) = 1, sin(π/6) = 1/2
- 恒等式：sin²x + cos²x = 1

#### 代数化简
- 分数通分：a/b + c/d = (ad+bc)/(bd)
- 二项式展开：(a+b)(a-b) = a²-b²
- 基本规则：x+0=x, 1×x=x, x¹=x, 0×x=0

## 测试覆盖

项目包含完整的测试套件，覆盖所有化简规则：

```bash
# 运行增强化简测试
cargo test enhanced_simplify

# 运行所有测试
cargo test
```

测试包括：
- 根号化简测试（9个测试用例）
- 三角函数化简测试
- 分数运算测试
- 二项式展开测试
- 自动化简开关测试

## 性能特性

- **智能缓存**: 化简结果缓存，避免重复计算
- **迭代控制**: 防止无限循环，最多10轮迭代
- **按需化简**: 可以运行时开启/关闭增强化简功能
- **内存优化**: 使用共享表达式减少内存占用

## 未来扩展

计划添加更多化简规则：
- 对数化简：log(a×b) = log(a) + log(b)
- 指数化简：a^m × a^n = a^(m+n)
- 复数化简：(a+bi) + (c+di) = (a+c) + (b+d)i
- 矩阵化简：矩阵运算的基本化简规则