# 编译器目录结构重组计划

> 本文档描述编译器目录结构的重组计划，以匹配新的三层 IR 架构（HIR → MIR → LIR）。

## 一、当前目录结构

```
compiler/
├── x-lexer/              # 词法分析器
├── x-parser/             # 语法分析器
├── x-typechecker/        # 类型检查器
├── x-hir/                # 高级 IR
├── x-perceus/            # Perceus 内存管理（独立）
├── x-codegen/            # 代码生成 + XIR + 6 个后端（职责过重）
├── x-codegen-dotnet/     # .NET 后端（桩）
├── x-codegen-js/         # JavaScript 后端（桩）
├── x-codegen-jvm/        # JVM 后端（桩）
├── x-codegen-wasm/       # Wasm 后端（桩，应废弃）
├── x-interpreter/        # 解释器
└── x-stdlib/             # 标准库
```

### 问题分析

1. **x-perceus 独立存在**：根据新架构，Perceus 应在 MIR 阶段执行，应整合到 x-mir
2. **x-mir 缺失**：中端缺少 MIR 层
3. **x-codegen 职责过重**：同时包含 IR 定义、降级逻辑和 6 个后端实现
4. **后端组织混乱**：有的后端在 x-codegen 内部，有的是独立 crate
5. **x-codegen-wasm 冗余**：Wasm 由 Zig 后端支持，无需独立后端

## 二、目标目录结构

```
compiler/
├── Cargo.toml                      # 工作空间配置
│
├── x-lexer/                        # 词法分析器
├── x-parser/                       # 语法分析器
├── x-typechecker/                  # 类型检查器
│
├── x-hir/                          # 高级 IR
├── x-mir/                          # 中层 IR + Perceus 内存管理
├── x-lir/                          # 低级 IR (= XIR)
│
├── x-codegen/                      # 代码生成基础设施
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                  # trait 定义、工厂函数
│       ├── error.rs                # 错误类型
│       ├── target.rs               # 目标平台枚举
│       └── config.rs               # 配置类型
│
├── x-backend-c/                    # C 后端
├── x-backend-zig/                  # Zig 后端 ✅
├── x-backend-rust/                 # Rust 后端
├── x-backend-java/                 # Java 后端
├── x-backend-csharp/               # C# 后端
├── x-backend-typescript/           # TypeScript 后端
├── x-backend-python/               # Python 后端
├── x-backend-swift/                # Swift 后端（规划）
├── x-backend-llvm/                 # LLVM 后端（规划）
├── x-backend-native/               # 机器码后端（规划）
│
├── x-interpreter/                  # 解释器
└── x-stdlib/                       # 标准库
```

## 三、迁移步骤

### 阶段一：中端重组

#### 3.1.1 新建 x-mir crate

```bash
mkdir -p compiler/x-mir/src
```

**x-mir/Cargo.toml**:
```toml
[package]
name = "x-mir"
version = "0.1.0"
edition = "2021"

[dependencies]
x-hir = { path = "../x-hir" }
thiserror = "1.0"
log = "0.4"

[dev-dependencies]
```

**x-mir/src/lib.rs** 核心结构：
- MIR 定义（基本块、指令、控制流图）
- Perceus 分析（从 x-perceus 迁移）
- dup/drop/reuse 指令插入
- 优化 Pass（常量传播、DCE 等）

#### 3.1.2 新建 x-lir crate

```bash
mkdir -p compiler/x-lir/src
```

从 `x-codegen/src/xir.rs` 迁移到 `x-lir/src/lib.rs`。

**x-lir/Cargo.toml**:
```toml
[package]
name = "x-lir"
version = "0.1.0"
edition = "2021"

[dependencies]
x-mir = { path = "../x-mir" }
thiserror = "1.0"
log = "0.4"
```

#### 3.1.3 废弃 x-perceus

将 `x-perceus/src/lib.rs` 内容迁移到 `x-mir/src/perceus.rs`，然后在 `x-mir/src/lib.rs` 中：

```rust
pub mod mir;
pub mod perceus;

pub use mir::*;
pub use perceus::*;
```

#### 3.1.4 废弃 x-codegen-wasm

删除 `compiler/x-codegen-wasm/`，Wasm 支持由 Zig 后端提供。

### 阶段二：后端拆分

#### 3.2.1 创建后端 crate 模板

每个后端遵循统一结构：

```
x-backend-{name}/
├── Cargo.toml
└── src/
    └── lib.rs
```

**Cargo.toml 模板**:
```toml
[package]
name = "x-backend-{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
x-codegen = { path = "../x-codegen" }
x-lir = { path = "../x-lir" }
thiserror = "1.0"
log = "0.4"
```

#### 3.2.2 迁移后端代码

| 源文件 | 目标 crate |
|--------|------------|
| x-codegen/src/zig_backend.rs | x-backend-zig/src/lib.rs |
| x-codegen/src/rust_backend.rs | x-backend-rust/src/lib.rs |
| x-codegen/src/java_backend.rs | x-backend-java/src/lib.rs |
| x-codegen/src/csharp_backend.rs | x-backend-csharp/src/lib.rs |
| x-codegen/src/typescript_backend.rs | x-backend-typescript/src/lib.rs |
| x-codegen/src/python_backend.rs | x-backend-python/src/lib.rs |

#### 3.2.3 清理 x-codegen

迁移后，`x-codegen/src/` 仅保留：
- `lib.rs` - trait 定义、工厂函数
- `error.rs` - 错误类型
- `target.rs` - 目标平台枚举
- `config.rs` - 配置类型
- `lower.rs` - HIR/MIR/LIR 降级逻辑（如需要）

### 阶段三：更新依赖关系

#### 3.3.1 更新工作空间 Cargo.toml

```toml
[workspace]
members = [
    # 前端
    "x-lexer",
    "x-parser",
    "x-typechecker",

    # 中端
    "x-hir",
    "x-mir",
    "x-lir",

    # 后端
    "x-codegen",
    "x-backend-c",
    "x-backend-zig",
    "x-backend-rust",
    "x-backend-java",
    "x-backend-csharp",
    "x-backend-typescript",
    "x-backend-python",
    # "x-backend-swift",      # 规划
    # "x-backend-llvm",       # 规划
    # "x-backend-native",     # 规划

    # 其他
    "x-interpreter",
    "x-stdlib",
]
```

#### 3.3.2 更新依赖链

```
x-lexer
    ↓
x-parser
    ↓
x-typechecker
    ↓
x-hir
    ↓
x-mir (包含 perceus)
    ↓
x-lir
    ↓
x-backend-* (依赖 x-codegen trait 和 x-lir)
```

## 四、向后兼容

为保证 CLI 和其他依赖方不受影响：

1. **x-codegen 保留公共 API**：
   - `CodeGenerator` trait
   - `Target` 枚举
   - `CodeGenConfig` / `CodeGenOutput`
   - `get_code_generator()` 工厂函数

2. **重导出关键类型**：
   ```rust
   // x-codegen/src/lib.rs
   pub use x_lir::{Module, Function, BasicBlock, ...};
   ```

3. **废弃标记**：
   - x-perceus: 标记为 deprecated，重导出 x-mir
   - x-codegen-wasm: 直接删除

## 五、执行顺序

1. ✅ 创建本文档
2. [ ] 创建 x-mir crate
3. [ ] 迁移 x-perceus 到 x-mir
4. [ ] 创建 x-lir crate
5. [ ] 从 x-codegen 提取 XIR 到 x-lir
6. [ ] 创建各后端 crate
7. [ ] 迁移后端代码
8. [ ] 删除 x-codegen-wasm
9. [ ] 更新工作空间 Cargo.toml
10. [ ] 更新 x-cli 依赖
11. [ ] 运行测试验证
12. [ ] 更新文档

## 六、风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| 循环依赖 | 确保 IR 层单向依赖，后端只依赖 x-codegen 和 x-lir |
| 测试失败 | 每步迁移后运行 `cargo test` |
| CLI 中断 | 保持 x-codegen 公共 API 不变 |
| 文档过时 | 同步更新 CLAUDE.md |

---

*最后更新：2026-03-19*
