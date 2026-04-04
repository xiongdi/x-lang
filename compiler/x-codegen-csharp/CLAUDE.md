# CLAUDE.md — x-codegen-csharp

**C# 后端**：生成 **C# 14 / .NET 10** 风格源码；**`impl x_codegen::CodeGenerator for CSharpBackend`**。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`CSharpBackend`**、**`CSharpConfig`**（可选 **`namespace`**）。
- 使用 **`x_parser::ast`** 与 **`x_lir::Program`**；内部跟踪 **`current_class`** 以生成实例方法。
- **`CSharpResult<T>`** = `Result<T, x_codegen::CodeGenError>`。

## 测试

```bash
cd compiler && cargo test -p x-codegen-csharp
```
