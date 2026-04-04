# CLAUDE.md — x-codegen-python

**Python 后端**：同时消费 **`x_parser::ast`** 与 **`x_lir::Program`**（见各 `generate_*` 实现）。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`PythonBackend`**、**`PythonBackendConfig`**。
- **`impl x_codegen::CodeGenerator for PythonBackend`**（含 `generate_from_lir`）。
- **`PythonResult<T>`** = `Result<T, x_codegen::CodeGenError>`。

## 测试

```bash
cd compiler && cargo test -p x-codegen-python
```
