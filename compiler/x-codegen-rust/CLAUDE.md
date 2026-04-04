# CLAUDE.md — x-codegen-rust

**Rust 后端**：从 **AST** 与 **LIR** 发射 Rust 源码（见 `src/lib.rs` 中的 `RustBackend` / `RustBackendConfig`）。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 说明

- 本 crate **当前未实现** `x_codegen::CodeGenerator` trait（与 Zig/TS 等不同），以 **`RustBackend`** 上的生成方法及 CLI 调用为准。
- 大量使用 **`x_lir`** 类型（`Program`、`Expression`、`Statement` 等）与 **`x_codegen::CodegenOutput`** / **`OutputFile`**。

## 测试

```bash
cd compiler && cargo test -p x-codegen-rust
```
