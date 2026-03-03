# X-Lang K Framework 规范总结

## 概述

本文档总结了使用 K Framework 为 X-Lang 语言编写的形式化语义规范。

## 目录结构

```
spec/k/
├── x-lang-syntax.k      # 语法定义
├── x-lang.k             # 语义定义
├── Makefile             # 构建脚本
├── README.md            # 详细文档
├── SUMMARY.md           # 本文档
└── tests/               # 测试用例
    ├── basic.x
    ├── let-binding.x
    ├── function.x
    ├── if.x
    ├── list.x
    ├── record.x
    ├── option.x
    ├── result.x
    ├── when.x
    ├── pipe.x
    ├── loop.x
    └── class.x
```

## 已实现的特性

### ✅ 语法特性

| 特性 | 状态 | 说明 |
|------|------|------|
| 标识符和关键字 | ✅ | 完整的词法结构 |
| 基本类型 | ✅ | Int, Float, Bool, String, Unit, Never |
| 复合类型 | ✅ | 函数类型、记录类型、联合类型 |
| 泛型类型 | ✅ | Option<T>, Result<T, E>, async<T> |
| 列表和字典类型 | ✅ | [T], {K: V} |
| 字面量 | ✅ | 整数、浮点数、布尔、字符串、单元 |
| 算术运算 | ✅ | +, -, *, /, %, ^ |
| 逻辑运算 | ✅ | and, or, not, &&, \|\|, ! |
| 比较运算 | ✅ | ==, !=, <, >, <=, >= |
| 变量声明 | ✅ | let, let mutable |
| 函数定义 | ✅ | function, async function, 分段函数 |
| 函数调用 | ✅ | 普通调用、方法调用 |
| Lambda 表达式 | ✅ | (x) -> x * 2 |
| If 表达式/语句 | ✅ | if/then/else, if/else 块 |
| 模式匹配 | ✅ | match, 守卫条件 |
| 列表表达式 | ✅ | [1, 2, 3], 范围, 推导式 |
| 记录表达式 | ✅ | {x: 1, y: 2}, copy-with |
| 管道操作符 | ✅ | \|>, \|>> |
| 循环 | ✅ | while, for/in, do/while |
| 错误处理 | ✅ | Option/Result + ? + or |
| 类定义 | ✅ | class, extends, new |
| 接口定义 | ✅ | trait, implements |
| Effect 系统 | ✅ | needs, given |
| 异步 | ✅ | wait, wait together, wait race |
| 原子事务 | ✅ | atomic, retry |
| 模块系统 | ✅ | module, import, export |

### ✅ 语义特性

| 特性 | 状态 | 说明 |
|------|------|------|
| 环境和存储 | ✅ | 变量环境和存储分离 |
| 闭包 | ✅ | 函数闭包捕获定义时的环境 |
| 调用栈 | ✅ | 函数调用栈保存和恢复 |
| 模式匹配 | ✅ | 匹配检查和变量绑定 |
| Option/Result | ✅ | 错误传播 (? 操作符) |
| 默认值 | ✅ | or 操作符 |
| 输出 | ✅ | print 语句收集输出 |
| 严格性属性 | ✅ | 控制求值顺序 |

## 配置结构

```k
<T>
  <k> $PGM:Program </k>        <!-- 当前继续 -->
  <env> .Map </env>             <!-- 变量环境 (Id -> Loc) -->
  <store> .Map </store>         <!-- 存储 (Loc -> Value) -->
  <stack> .List </stack>        <!-- 调用栈 -->
  <effects> .Map </effects>     <!-- Effect 实现 -->
  <output> .List </output>      <!-- 输出列表 -->
</T>
```

## 测试覆盖

当前包含 12 个测试用例：

1. `basic.x` - 基础算术运算
2. `let-binding.x` - 不可变和可变绑定
3. `function.x` - 函数定义和递归
4. `if.x` - 条件表达式和语句
5. `list.x` - 列表操作
6. `record.x` - 记录（结构体）操作
7. `option.x` - Option 类型使用
8. `result.x` - Result 类型使用
9. `when.x` - 模式匹配
10. `pipe.x` - 管道操作符
11. `loop.x` - 循环（for/while）
12. `class.x` - 类和继承

## 下一步计划

### 短期

1. **Perceus 内存管理语义**
   - 添加 dup/drop 操作语义
   - 实现重用分析规则
   - FBIP（Functional But In-Place）语义

2. **类型系统语义**
   - 添加类型环境
   - 实现类型检查规则
   - 类型推断规则

3. **更多测试用例**
   - 覆盖剩余语言特性
   - 添加预期输出文件
   - 边界条件测试

### 中期

1. **形式化证明**
   - 安全性证明（类型安全、内存安全）
   - 程序等价性证明
   - 优化正确性证明

2. **与实现的对应**
   - 连接 K 规范与实际编译器
   - 生成测试向量
   - 差异检查

3. **性能优化**
   - 使用更高效的 K 后端
   - 优化重写规则
   - 并行求值

### 长期

1. **完整语言覆盖**
   - 所有 X-Lang 特性
   - 完整的标准库语义
   - 并发语义

2. **验证工具**
   - 交互式验证环境
   - 自动定理证明集成
   - 反例生成

## 使用示例

### 构建和测试

```bash
# 构建规范
make build

# 运行所有测试
make test

# 运行单个测试
make run FILE=tests/basic.x
```

### 示例：基础算术

输入 (`tests/basic.x`):
```x
fun main() {
  let x = 1 + 2
  let y = x * 3
  print(y)
}
```

预期输出:
```
9
```

### 示例：递归函数

输入 (`tests/function.x`):
```x
fun factorial(n) =
  if n <= 1 then
    1
  else
    n * factorial(n - 1)

fun main() {
  let fact = factorial(5)
  print(fact)
}
```

预期输出:
```
120
```

## 技术细节

### K Framework 特性使用

- **Modules**: `XLANG-SYNTAX`, `XLANG`
- **Configuration**: 6 个 cell 的 T 配置
- **Rewrite Rules**: 约 150+ 条重写规则
- **Strictness Attributes**: 控制求值顺序
- **Functions**: 辅助函数定义
- **Maps/Lists**: 使用内置集合类型

### 设计决策

1. **环境与存储分离**：遵循 CEK 机风格，便于推理
2. **闭包表示**：显式捕获定义时的环境
3. **调用栈**：使用帧列表保存恢复信息
4. **输出收集**：使用列表而非直接 IO，便于测试
5. **模块化**：语法和语义分离，便于维护

## 相关资源

- [K Framework 官网](https://kframework.org)
- [K 文档](https://docs.kframework.org)
- [X-Lang 主规范](../README.md)
- [K Tutorial](https://github.com/runtimeverification/k/tree/master/k-distribution/tutorial)

## 贡献指南

欢迎贡献！请确保：

1. 添加新特性时同步更新语法和语义
2. 添加对应的测试用例
3. 更新本文档
4. 运行 `make test` 确保通过

## 联系方式

如有问题或建议，请通过 X-Lang 项目的 Issue  tracker 联系。
