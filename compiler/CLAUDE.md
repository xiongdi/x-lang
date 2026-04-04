# CLAUDE.md — compiler 工作区

本目录是 X 编译器的 **Cargo workspace**（成员列表见 [Cargo.toml](./Cargo.toml)）。全局协作规则以仓库根目录 [CLAUDE.md](../CLAUDE.md) 与 [DESIGN_GOALS.md](../DESIGN_GOALS.md) 为准。

## 流水线（与 CLI 一致）

`tools/x-cli/src/pipeline.rs` 中 **`run_pipeline(source)`** 返回 **`PipelineOutput`**：

1. 解析（`x_parser::parser::XParser`）→ **`ast: Program`**
2. 类型检查：完整流水线里为 **`x_typechecker::type_check_with_env`**（`pipeline::run_pipeline`）；**`run` / `check` 子命令**单独用大栈包装 **`type_check_with_big_stack*`**，避免深 AST 栈溢出
3. **`x_hir::ast_to_hir_with_type_env`** → **`hir: Hir`**
4. **`x_mir::lower_hir_to_mir`** → **`mir: MirModule`**
5. **`x_lir::lower_mir_to_lir`** → **`lir: Program`**

`run` / `check` 在 AST 或类型检查处结束；`compile --emit hir|mir|lir` 打印对应阶段。

标准库：`pipeline::find_stdlib_path`、`read_std_prelude`、`parse_std_prelude`；模块解析 **`ModuleResolver`**（`{}.x` 按路径解析）。

## 成员 crate

| 阶段 | Crate | 子文档 |
|------|--------|--------|
| 词法 | `x-lexer` | [x-lexer/CLAUDE.md](./x-lexer/CLAUDE.md) |
| 语法 / AST | `x-parser` | [x-parser/CLAUDE.md](./x-parser/CLAUDE.md) |
| 类型检查 | `x-typechecker` | [x-typechecker/CLAUDE.md](./x-typechecker/CLAUDE.md) |
| HIR | `x-hir` | [x-hir/CLAUDE.md](./x-hir/CLAUDE.md) |
| MIR + Perceus | `x-mir` | [x-mir/CLAUDE.md](./x-mir/CLAUDE.md) |
| LIR | `x-lir` | [x-lir/CLAUDE.md](./x-lir/CLAUDE.md) |
| 代码生成核心 | `x-codegen` | [x-codegen/CLAUDE.md](./x-codegen/CLAUDE.md) |
| 各语言后端 | `x-codegen-*` | 各目录 `CLAUDE.md` |
| 解释器 | `x-interpreter` | [x-interpreter/CLAUDE.md](./x-interpreter/CLAUDE.md) |

## 常用命令

```bash
cd compiler && cargo test
cd compiler && cargo test -p x-parser
cd compiler && cargo fmt
```

## examples

勿修改 `examples/` 下用户维护的 `.x` / `.zig`；若示例暴露编译器问题，应改编译器（见根目录 CLAUDE.md）。
