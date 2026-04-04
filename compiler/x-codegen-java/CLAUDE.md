# CLAUDE.md — x-codegen-java

**Java 后端**：生成 **Java 25 LTS** 风格源码；**`impl x_codegen::CodeGenerator for JavaBackend`**。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`JavaBackend`**、**`JavaConfig`**（含 **`class_name`**，默认 `"Main"`）。
- 同时匹配 **`x_parser::ast`** 与 **`x_lir`**（`generate_from_lir` 等）。
- **`JavaResult<T>`** = `Result<T, x_codegen::CodeGenError>`。

## 测试

```bash
cd compiler && cargo test -p x-codegen-java
```
