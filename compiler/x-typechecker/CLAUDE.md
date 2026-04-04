# CLAUDE.md — x-typechecker

**类型检查与语义分析**：在 `x_parser::ast::Program` 上工作，产出诊断或失败。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 源码布局

| 模块 | 路径 | 说明 |
|------|------|------|
| 核心 | `src/lib.rs`（体积大） | `type_check`、`type_check_with_env`、`TypeEnv`、`TypeError`、`TypeCheckResult` |
| 错误 | `src/errors.rs` | `TypeError`、`Severity`、`ErrorCategory` |
| 穷尽性等 | `src/exhaustiveness.rs`、`src/format.rs` | match 穷尽、格式化辅助 |

## 对外 API（调用方常搜这些名）

- **`type_check(program: &Program) -> Result<(), TypeError>`**：单错误即停风格包装。
- **`type_check_with_env(program) -> Result<TypeEnv, TypeError>`**：需要保留环境时用。
- **`TypeCheckResult`**：多错误收集（`errors: Vec<TypeError>`，`is_ok` / `to_result`）。

CLI 侧大栈包装见 `tools/x-cli/src/pipeline.rs` 的 `type_check_with_big_stack*`。

## 依赖

- **`x_lexer::span::Span`**：错误位置。
- **`x_parser::ast::*`**：被检查的树。

## 测试

```bash
cd compiler && cargo test -p x-typechecker
```
