# X-Lang K Framework 形式化规范

本目录包含使用 [K Framework](https://kframework.org) 编写的 X-Lang 形式化语义规范。

## 关于 K Framework

K Framework 是一个用于定义编程语言形式化语义的框架。它支持：

- **BNF 风格的语法定义**
- **配置（Configuration）** 表示程序状态
- **重写规则（Rewrite Rules）** 定义操作语义
- **形式化验证** 使用可达性逻辑（Reachability Logic）

## 文件结构

```
spec/k/
├── x-lang-syntax.k      # 语法定义（语法模块）
├── x-lang.k             # 语义定义（主模块）
├── Makefile             # 构建和测试脚本
├── README.md            # 本文件
└── tests/               # 测试用例
    ├── basic.x          # 基础算术
    ├── let-binding.x    # 变量绑定
    ├── function.x       # 函数定义和调用
    ├── if.x             # 条件语句
    ├── list.x           # 列表操作
    ├── record.x         # 记录（结构体）
    ├── option.x         # Option 类型
    ├── result.x         # Result 类型
    ├── when.x           # 模式匹配（match）
    ├── pipe.x           # 管道操作符
    ├── loop.x           # 循环
    └── class.x          # 类和继承
```

## 前置要求

1. 安装 K Framework：
   ```bash
   # 使用包管理器
   bash <(curl https://kframework.org/install)
   kup install k
   ```

   或从 [GitHub Releases](https://github.com/runtimeverification/k/releases) 下载。

2. 验证安装：
   ```bash
   kompile --version
   ```

## 使用方法

### 构建规范

```bash
make build
```

### 运行测试

```bash
# 运行所有测试
make test

# 运行单个测试文件
make run FILE=tests/basic.x

# 交互式 K shell
make shell
```

### 生成预期输出

```bash
make gen-expected FILE=tests/basic.x
```

### 清理构建文件

```bash
make clean
```

### 查看帮助

```bash
make help
```

## 规范概述

### 语法模块 (`x-lang-syntax.k`)

定义了 X-Lang 的完整语法，包括：

- **词法结构**：标识符、关键字、字面量
- **类型系统**：基本类型、函数类型、记录类型、联合类型等
- **表达式**：字面量、变量、运算、函数调用、模式匹配等
- **语句**：声明、控制流、异常处理等
- **声明**：函数、类、接口、类型别名等
- **操作符优先级**

### 语义模块 (`x-lang.k`)

定义了 X-Lang 的操作语义，配置结构：

```k
configuration
  <T>
    <k> $PGM:Program </k>        <!-- 当前执行的代码 -->
    <env> .Map </env>             <!-- 变量环境（名称→位置） -->
    <store> .Map </store>         <!-- 存储（位置→值） -->
    <stack> .List </stack>        <!-- 调用栈 -->
    <effects> .Map </effects>     <!-- Effect 环境 -->
    <output> .List </output>      <!-- 输出 -->
  </T>
```

### 核心语义规则

1. **变量绑定和查找**
2. **算术和逻辑运算**
3. **函数定义和调用**（包括闭包）
4. **控制流**（if/while/for）
5. **模式匹配**（match）
6. **错误处理**（Option/Result + ? + or）
7. **列表和记录操作**
8. **Option/Result 类型**
9. **管道操作符**
10. **Effect 系统**（needs/given）
11. **异步**（wait）
12. **原子事务**（atomic）

## 与 X-Lang 规范的对应

| README 章节 | K 规范部分 |
|------------|-----------|
| 1. 词法结构 | `XLANG-SYNTAX` 词法规则 |
| 2. 类型系统 | `Type` 产生式 |
| 3. 名字与作用域 | 环境（env）规则 |
| 4. 变量 | `LetDecl` 规则 |
| 6. 表达式 | 表达式求值规则 |
| 7. 函数 | 函数闭包和应用规则 |
| 8. 类和接口 | 类定义规则 |
| 13. 模式匹配 | `match` 规则 |
| 16. Effect 系统 | needs/given 配置和规则 |
| 17. 异步与并发 | `async`/`await` 规则 |
| 18. 原子事务 | `atomic` 规则 |
| 19. Perceus | （待实现） |

## 形式化验证

K Framework 支持使用可达性逻辑进行形式化验证。例如，可以证明：

```k
// 证明加法交换律
rule <k> a + b => b + a ...</k>
     [reachability]
```

## 扩展和完善

当前的 K 规范是一个基础版本，可以进一步扩展：

1. **Perceus 内存管理**：添加 dup/drop 语义
2. **完整的类型检查**：添加类型系统语义
3. **更多测试用例**：覆盖更多语言特性
4. **形式化证明**：添加可达性逻辑证明
5. **优化**：使用更高效的 K 特性

## 参考资源

- [K Framework 官方文档](https://docs.kframework.org)
- [K Tutorial](https://github.com/runtimeverification/k/tree/master/k-distribution/tutorial)
- [K 论文集](https://www.kframework.org/index.php/Publications)

## 许可证

与 X-Lang 项目使用相同的许可证。
