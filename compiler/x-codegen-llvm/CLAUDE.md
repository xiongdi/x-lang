# CLAUDE.md — x-codegen-llvm

**LLVM IR 后端**：生成 **LLVM IR 文本**（不依赖 `inkwell`，直接 `write!` 拼接）。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`LlvmBackend`**、**`LlvmBackendConfig`**（`target_triple`、`module_name` 等）。
- **`impl x_codegen::CodeGenerator for LlvmBackend`**：从 **LIR** 与 AST 路径发射；大量 `use x_lir::{...}` 解构指令级结构。

后续用 **`llc` / `clang`** 等外部工具将 `.ll` 转为目标文件（由调用方/CLI 决定）。

## 测试

```bash
cd compiler && cargo test -p x-codegen-llvm
```
