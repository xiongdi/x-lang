# CLAUDE.md — x-parser

**语法分析**：源码 → `ast::Program`。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 源码布局

| 模块 | 路径 | 内容 |
|------|------|------|
| `ast` | `src/ast/` | AST 节点：`Program`、`Declaration`、`Statement`、`Expression`、`Pattern`、`Type` 等 |
| `parser` | `src/parser.rs`（及生成/辅助文件） | `XParser`：内部驱动 `x_lexer` |
| `errors` | `src/errors.rs` | `ParseError` |

## 对外 API

- **`parse_program(input: &str) -> Result<Program, ParseError>`**（`src/lib.rs`）：最常用的入口。
- **`Parser` 类型别名** = `XParser`；需要自定义时可 `XParser::new().parse(input)`。

## 与上下游

- **上游**：`x-lexer`（`Token`、`Span`）。
- **下游**：`x-typechecker`（`type_check` 等）、`x-hir::ast_to_hir`、`x-interpreter::Interpreter::run`、各 `x-codegen-*`（多数仍消费 AST 或经 LIR）。

## 测试

```bash
cd compiler && cargo test -p x-parser
```
