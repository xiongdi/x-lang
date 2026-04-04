# CLAUDE.md — x-lexer

**X 词法分析**：`&str` → `Token` 流。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 源码布局

| 模块 | 文件 | 内容 |
|------|------|------|
| `token` | `src/token.rs` | `Token` 枚举（关键字、字面量、标点、`Ident`、`Eof` 等） |
| `span` | `src/span.rs` | `Span`（字节偏移），供诊断 |
| `errors` | `src/errors.rs` | `LexError` |
| 根 | `src/lib.rs` | `Lexer<'a>`、`LexerState`（Normal / String / 插值等）、`TokenIterator` |

**实现方式**：手写扫描器（**未**使用 `logos`）；处理 UTF-8 BOM、字符串插值 `${...}`、多行/原始字符串等状态在 `LexerState` / `state_stack` 中维护。

## 对外 API（解析器常用）

- `Lexer::new(input: &str)`：构造扫描器；自动去掉 UTF-8 BOM。
- `new_lexer(input)` → `TokenIterator`：按 token 迭代。
- 解析侧一般通过 `x_parser` 间接使用，不必在业务代码里直接拼 token 序列。

## 变更时注意

- 新增/改名 `Token` 变体 → 同步 `x-parser` 与 [spec/](../../spec/)。
- 错误信息尽量带 `Span`，与根目录 CLAUDE 的 `file:line:col` 风格一致（行号由 span + 源码换算）。

## 测试

```bash
cd compiler && cargo test -p x-lexer
```
