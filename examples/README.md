# X 语言示例程序

本目录包含 X 语言的示例程序，展示了语言的各种特性。

## 示例列表

### 基础示例
- **hello.x** - 经典的 "Hello, World!" 程序
- **variables.x** - 变量声明和基本数据类型
- **arithmetic.x** - 算术运算和比较运算
- **control-flow.x** - 控制流语句（if/else, while）

### 函数示例
- **functions.x** - 函数定义、递归和返回值
- **fibonacci.x** - 斐波那契数列（递归和迭代版本）
- **primes.x** - 质数判断和相关算法

## 运行示例

使用 X 语言工具链运行示例：

```bash
# 运行单个示例
cargo run -- run examples/hello.x

# 或从 tools/x-cli 目录运行
cd tools/x-cli
cargo run -- run ../../examples/hello.x
```

## 示例说明

### hello.x
最简单的 X 语言程序，展示了如何使用 `println` 函数输出文本。

### variables.x
展示了：
- 不可变绑定 (`let`)
- 可变变量 (`let mutable`)
- 基本数据类型（整数、浮点数、布尔值、字符）
- 多变量声明

### arithmetic.x
展示了：
- 基本算术运算（加、减、乘、除、取模）
- 比较运算（等于、不等于、大于、小于等）
- 逻辑运算（与、或、非）
- 自定义数学函数

### control-flow.x
展示了：
- `if` / `else` 条件语句
- `while` 循环
- 逻辑运算的实际应用
- 比较运算的实际应用

### functions.x
展示了：
- 无返回值函数
- 带返回值函数
- 递归函数（阶乘）
- 多参数函数
- 函数调用

### fibonacci.x
展示了：
- 递归计算斐波那契数列
- 迭代计算斐波那契数列（更高效）
- 序列生成和打印

### primes.x
展示了：
- 质数判断算法
- 范围查找质数
- 第 n 个质数计算
- 数学逻辑的实际应用

## 语言特性

### main 函数可选
X 语言支持两种编程风格：

1. **脚本风格**（推荐用于简单程序）：
   ```x
   println("Hello, World!")
   ```

2. **传统风格**（推荐用于复杂程序）：
   ```x
   function main() {
       println("Hello, World!")
   }

   main()
   ```

### 关键字
X 语言使用自然语言风格的关键字：
- `function` - 定义函数
- `let` - 定义不可变绑定
- `let mutable` - 定义可变变量
- `const` - 定义编译期常量
- `if` / `else` - 条件分支
- `while` - 循环
- `return` - 返回值
