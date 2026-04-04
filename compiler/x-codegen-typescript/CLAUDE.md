# CLAUDE.md — x-codegen-typescript

**TypeScript 后端**：**AST → TS 源码**（`generate_from_lir` 等以 `impl CodeGenerator` 为准）。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`TypeScriptBackend`**、**`TypeScriptBackendConfig`**（`output_dir`、`optimize`、`debug_info`）。
- **`impl x_codegen::CodeGenerator for TypeScriptBackend`**。
- 错误类型：**`TypeScriptResult<T>`** = `Result<T, x_codegen::CodeGenError>`。

依赖 **`x_parser::ast`**；输出经 **`CodeBuffer`** 拼接。

## CLI

- `x compile --emit ts` / `--target ts` 等路径在 `tools/x-cli/src/commands/compile.rs` 中选后端。

## 测试

```bash
cd compiler && cargo test -p x-codegen-typescript
```
