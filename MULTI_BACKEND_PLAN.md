# X语言多后端工具链 - 实现计划与进度

## 已完成的工作

### ✅ 阶段1a: 语法修改
- 变量声明: `val`/`var` → `let`/`let mut`
- 注释语法: `--`/`{- -}` → `//`/`/** */`
- 更新所有10个 examples
- 更新语言规范 (README.md)
- 更新词法分析器和解析器

### ✅ 阶段1b: 验证文档
- 完整的验证报告 (VERIFICATION.md)
- 变更总结 (CHANGES_SUMMARY.md)
- 测试程序 (verify_changes.rs)

### 🔄 阶段1c: 架构重构（进行中）

#### 已创建的文件:
1. `crates/x-codegen/src/lib.rs` - 公共接口抽象层
   - `CodeGenerator` trait
   - `DynamicCodeGenerator` trait
   - `CodegenConfig`, `CodegenOutput`, `OutputFile` 结构
   - `get_code_generator()` 工厂函数
   - 各后端的配置占位符

2. `crates/x-codegen/src/error.rs` - 统一错误类型
   - `CodeGenError` 枚举
   - 各后端特定错误（feature-gated）

3. `crates/x-codegen/src/target.rs` - 目标平台定义
   - `Target` 枚举 (Native/Jvm/DotNet/JavaScript/TypeScript/Wasm/LlvmIr)
   - `FileType` 枚举
   - 工具方法 (`as_str()`, `from_str()`, `default_extension()` 等)

4. `crates/x-codegen-llvm/Cargo.toml` - LLVM后端配置
5. `crates/x-codegen-llvm/src/lib.rs` - LLVM代码生成器框架

---

## 完整的Crate组织架构

```
crates/
├── x-cli/                    # 命令行工具（协调编译流程）
├── x-lexer/                  # 词法分析器（共享）✅
├── x-parser/                 # 语法分析器（共享）✅
├── x-typechecker/            # 类型检查器（共享）
├── x-hir/                    # 高级中间表示（共享）
├── x-perceus/                # Perceus内存管理分析（共享）
├── x-codegen/                # 代码生成公共接口和抽象层 ✨ NEW ✨
│   ├── src/
│   │   ├── lib.rs           # 公共接口、trait定义
│   │   ├── error.rs         # 统一错误类型
│   │   └── target.rs        # 目标平台定义
│
├── x-codegen-llvm/           # LLVM代码生成器（Native后端）✨ NEW ✨
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # LLVM代码生成器实现
│       └── lower.rs         # AST → LLVM IR lowering（从旧x-codegen迁移）
│
├── x-codegen-jvm/            # JVM字节码生成（JVM后端）⭐ TO-DO ⭐
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs           # JVM代码生成器
│
├── x-codegen-dotnet/         # .NET CIL生成（.NET后端）⭐ TO-DO ⭐
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs           # .NET代码生成器
│
├── x-codegen-js/             # JavaScript/TypeScript生成（JS/TS后端）⭐ TO-DO ⭐
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs           # JS/TS代码生成器
│
├── x-interpreter/            # 树遍历解释器（共享）✅
├── x-stdlib/                 # 标准库定义（共享）
├── x-spec/                   # 规格测试运行器（共享）
│
└── x-runtime/                # 运行时支持库（按目标平台分类）⭐ TO-DO ⭐
    ├── x-runtime-native/     # C++运行时（Native后端）
    ├── x-runtime-jvm/        # Java运行时（JVM后端）
    ├── x-runtime-dotnet/     # C#运行时（.NET后端）
    └── x-runtime-js/         # JavaScript运行时（JS/TS后端）
```

---

## 下一步工作（优先级顺序）

### 高优先级

1. **完成 x-codegen-llvm**
   - 将原 x-codegen 的 lower.rs 移动到 x-codegen-llvm
   - 实现完整的 `DynamicCodeGenerator` trait
   - 更新旧的 x-codegen 提供兼容性 facade（可选）

2. **更新 Cargo.toml 工作区配置**
   - 添加所有新 crates 到 workspace
   - 配置 feature 系统 (llvm, jvm, dotnet, js)
   - 设置正确的依赖关系

3. **扩展 x-cli**
   - 添加 `--target` 选项 (native/jvm/dotnet/js/ts/wasm/llvm-ir)
   - 扩展 `--emit` 支持各后端
   - 更新 `compile_command` 使用新的多后端 API

### 中优先级

4. **x-codegen-jvm 骨架**
   - 创建基本的 JVM 代码生成器结构
   - 使用 jvm-rs 或 bytecode 库
   - 支持生成简单的 .class 文件

5. **x-codegen-dotnet 骨架**
   - 创建基本的 .NET 代码生成器结构
   - 使用 dnlib 或类似库
   - 支持生成简单的 .NET 程序集

6. **x-codegen-js 骨架**
   - 创建基本的 JS/TS 代码生成器结构
   - 支持生成 ES6+ JavaScript
   - 支持生成 TypeScript 声明

### 低优先级

7. **运行时库**
   - 创建各平台的运行时支持
   - 标准库绑定
   - 内存管理支持

---

## 核心 Trait 设计

### CodeGenerator trait
```rust
pub trait CodeGenerator {
    type Config;
    type Error;

    fn new(config: Self::Config) -> Self;
    fn generate_from_ast(&mut self, program: &Program) -> Result<CodegenOutput, Self::Error>;
    fn generate_from_hir(&mut self, hir: &()) -> Result<CodegenOutput, Self::Error>;
    fn generate_from_pir(&mut self, pir: &()) -> Result<CodegenOutput, Self::Error>;
}
```

### 工厂函数
```rust
pub fn get_code_generator(target: Target, config: CodeGenConfig)
    -> CodeGenResult<Box<dyn DynamicCodeGenerator>>;
```

---

## CLI 使用示例

```bash
# 编译到原生可执行文件（默认）
x compile examples/binary_trees.x -o binary_trees

# 编译到JVM JAR
x compile examples/binary_trees.x --target jvm -o binary_trees.jar

# 编译到.NET DLL
x compile examples/binary_trees.x --target dotnet -o binary_trees.dll

# 编译到JavaScript
x compile examples/binary_trees.x --target js -o binary_trees.js

# 编译到TypeScript
x compile examples/binary_trees.x --target ts -o binary_trees.ts

# 输出LLVM IR（调试）
x compile examples/binary_trees.x --emit llvm-ir

# 输出JVM字节码（调试）
x compile examples/binary_trees.x --target jvm --emit jvm-bytecode
```

---

## 特性系统设计

```toml
# Cargo.toml - x-cli
[dependencies]
x-codegen = { path = "../x-codegen" }
x-codegen-llvm = { path = "../x-codegen-llvm", optional = true }
x-codegen-jvm = { path = "../x-codegen-jvm", optional = true }
x-codegen-dotnet = { path = "../x-codegen-dotnet", optional = true }
x-codegen-js = { path = "../x-codegen-js", optional = true }

[features]
default = ["llvm"]
llvm = ["x-codegen-llvm"]
jvm = ["x-codegen-jvm"]
dotnet = ["x-codegen-dotnet"]
js = ["x-codegen-js"]
all = ["llvm", "jvm", "dotnet", "js"]
```

使用方式：
```bash
# 默认：仅LLVM后端
cargo build

# 启用所有后端
cargo build --features all

# 启用特定后端组合
cargo build --features "llvm,jvm"
```

---

## 文件迁移清单

### 从 x-codegen 迁移到 x-codegen-llvm

- [x] `lower.rs` → `x-codegen-llvm/src/lower.rs`
- [ ] 更新 import 路径
- [ ] 更新错误类型使用
- [ ] 实现 CodeGenerator trait

### 旧 x-codegen 的兼容性处理

选项A：完全替换
- 弃用旧 x-codegen，要求用户迁移到新的多后端 API

选项B：兼容性 facade
- 保留旧的 `generate_code` 函数
- 内部调用新的 x-codegen-llvm
- 提供平滑迁移路径

---

## 后续阶段计划

### 阶段2：完善Native后端（4周）
- 完整的 C++ 运行时 (x-runtime-native)
- 完善 LLVM 代码生成
- 完整的标准库绑定
- 链接优化

### 阶段3：JVM后端（6周）
- x-codegen-jvm 完整实现
- x-runtime-jvm Java 运行时
- 标准库到 Java API 绑定
- JAR 生成和执行

### 阶段4：.NET后端（6周）
- x-codegen-dotnet 完整实现
- x-runtime-dotnet C# 运行时
- 标准库到 .NET BCL 绑定
- 程序集生成和执行

### 阶段5：JS/TS后端（6周）
- x-codegen-js 完整实现
- x-runtime-js JavaScript 运行时
- 标准库到 Node.js/Browser API 绑定
- 直接执行和打包

---

## 总结

我们已经成功开始了多后端架构的重构工作：

✅ **已完成**：
1. 语法修改（let/let mut, // 注释）
2. 验证文档和测试
3. x-codegen 公共抽象层设计
4. x-codegen-llvm 框架创建
5. 完整的架构设计文档

🚧 **进行中**：
- x-codegen-llvm 完整实现和迁移

📋 **待做**：
- 其他三个后端的骨架
- CLI 扩展
- 运行时库

这个架构设计为X语言提供了清晰、可扩展的多后端支持，每个后端都可以独立开发和演进。
