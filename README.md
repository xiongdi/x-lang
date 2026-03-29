# X 语言

X 是一门**现代通用编程语言**，强调可读性第一、类型安全与内存安全，支持函数式、面向对象、过程式和声明式多种范式。采用 Perceus 编译期引用计数（无 GC、无手动管理），无 null、无异常，通过 `Option`/`Result` 与效果系统显式表达副作用与错误。

本仓库为 X 语言的**编译器、工具链与规格**实现与文档。

---

## 文档与规格

| 文档 | 说明 |
|------|------|
| [DESIGN_GOALS.md](DESIGN_GOALS.md) | **设计目标与原则**（宪法性文件，所有设计决策的最终依据） |
| [spec/](spec/) | **语言规格说明书**（语法、类型、语义的正式定义，见 [spec/README.md](spec/README.md)） |
| [docs/](docs/) | 教程、附录与专题文档（关键字、类型、表达式、编译器架构等） |
| [CLAUDE.md](CLAUDE.md) | 贡献与开发指南（构建、测试、目录结构） |

---

## 快速开始

- **环境要求**：Rust 工具链、Zig 0.13.0+（用于原生编译）

- **运行 .x 文件**（解析 + 解释执行）
  ```bash
  cd tools/x-cli && cargo run -- run <file.x>
  ```

- **检查语法与类型**
  ```bash
  cd tools/x-cli && cargo run -- check <file.x>
  ```

- **编译为可执行文件**（Zig 后端）
  ```bash
  cd tools/x-cli && cargo run -- compile <file.x> -o <output>
  ```

- **运行示例**
  ```bash
  cd tools/x-cli && cargo run -- run ../../examples/hello.x
  ```

更多命令、依赖说明和开发指南见 [CLAUDE.md](CLAUDE.md)。

---

## 项目结构

| 目录 | 说明 |
|------|------|
| `compiler/` | 编译器核心 |
| `compiler/x-lexer` | 词法分析 |
| `compiler/x-parser` | 语法分析 |
| `compiler/x-typechecker` | 类型检查 |
| `compiler/x-hir` / `x-mir` / `x-lir` | 多层中间表示 |
| `compiler/x-mir` | Perceus 内存分析（dup/drop/reuse） |
| `compiler/x-codegen*` | 多后端代码生成 |
| `compiler/x-interpreter` | AST 解释器 |
| `tools/x-cli` | 命令行工具（`run`、`check`、`compile` 等） |
| `spec/` | 语言规格（设计哲学、词法、类型、表达式、函数、效果、内存等） |
| `docs/` | 教程与专题文档 |
| `examples/` | 示例与基准程序 |
| `library/stdlib` | 核心标准库 |

---

## 代码生成后端状态

| 后端 | 状态 | 说明 |
|------|------|------|
| Zig | ✅ 成熟 | 编译到 Zig 源码，使用 Zig 编译器生成原生/Wasm，最成熟 |
| C | 🚧 早期 | 编译到 C 源码，最大可移植性 |
| Rust | 🚧 早期 | 编译到 Rust 源码，Rust 生态互操作 |
| JavaScript/TypeScript | 🚧 早期 | 编译到 JS/TS，浏览器/Node.js |
| JVM | 🚧 早期 | 编译到 JVM 字节码 |
| .NET | 🚧 早期 | 编译到 .NET CIL |
| Python | 🚧 早期 | 编译到 Python 源码 |
| LLVM | 🚧 早期 | 生成 LLVM IR |
| Swift | 📋 计划 | 编译到 Swift 源码，Apple 生态 |
| Native | 🚧 计划 | 直接机器码生成 |

---

## 编译器流水线

```
源代码 → 词法分析 → 语法分析 → AST → 类型检查 → HIR → MIR → Perceus 分析 → LIR → 代码生成 → 可执行文件
```

X 采用经典的三段式架构：**前端 → 中端 → 后端**，支持从解释执行到全编译多种模式。

---

## 主要特性

- **自然语言风格关键字**：使用 `needs`、`given`、`when`/`is`、`can`、`wait`、`atomic` 等可读性好的关键字
- **数学函数表示法**：`f(x) = x + 1` 简洁直观
- **显式效果系统 (R·E·A)**：清晰表达函数的错误、效果与区域
- **Perceus 内存管理**：编译期重用分析，无 GC，无手动内存管理
- **多范式支持**：函数式、面向对象、过程式、声明式
- **零成本抽象**：高级抽象不牺牲运行时性能
- **没有 null**：通过 `Option` 类型显式表示可空
- **没有异常**：通过 `Result` 类型显式表示错误

---

## 测试

```bash
# 运行所有单元测试
cd compiler && cargo test

# 运行规范测试
cargo run -p x-spec

# 或者一次性运行所有测试
./test.sh
```

---

## 版本控制

本项目默认使用 **Jujutsu (jj)** 进行版本控制，Git 作为替代方案被支持。

- 提交：`jj commit -m "message"`
- 拉取：`jj git fetch`
- 推送：`jj git push`

---

## 许可

本项目采用多重许可，你可任选其一使用、修改与分发：

- MIT License
- Apache License 2.0
- BSD 3-Clause License

详见 [LICENSES.md](LICENSES.md)。
