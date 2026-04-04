# CLAUDE.md — x-interpreter

**树遍历解释器**：在 **`x_parser::ast::Program`** 上直接执行，不经过 MIR/LIR。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 入口类型

- **`Interpreter`**（`src/lib.rs`）：`Interpreter::new()` 构造；**`run(&mut self, program: &Program) -> Result<(), InterpreterError>`** 为执行入口。
- **`Value`**：运行时值（整型、浮点、字符串、`Array`、`Rc` 引用等）。
- **`InterpreterError`**：含 `runtime` / `undefined_variable` / `undefined_function` 等构造辅助。

## 与 CLI

- `tools/x-cli/src/commands/run.rs` 在类型检查后构造 `Interpreter` 并 `run`。
- 语义应对齐语言规范，但不必覆盖「仅后端才有」的能力（如某目标平台汇编）。

## 测试

```bash
cd compiler && cargo test -p x-interpreter
```
