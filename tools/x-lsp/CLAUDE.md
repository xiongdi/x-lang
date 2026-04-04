# CLAUDE.md — x-lsp

**Language Server**：基于 **`lsp-server`** + **`lsp-types`**，stdio 与编辑器通信。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 源码布局

| 模块 | 路径 | 说明 |
|------|------|------|
| `main` | `src/main.rs` | `env_logger::init`，构造 **`server::LspServer::new()?.run()`** |
| `server` | `src/server.rs` | 连接 stdin/stdout 与消息循环 |
| `handlers` | `src/handlers.rs` | LSP 请求处理（completion、diagnostics 等，随实现扩展） |
| `analysis` | `src/analysis.rs` | 调用 `x-lexer` / `x-parser` / `x-typechecker` 做增量或单次分析 |
| `state` | `src/state.rs` | 打开文档、版本、已发布诊断 |
| `utils` | `src/utils.rs` | 辅助函数 |

实现未完成部分可能带 `#[allow(dead_code)]`；改前端 crate API 时优先保证 **`cargo build -p x-lsp`** 通过。

## 测试

```bash
cd tools && cargo test -p x-lsp
```
