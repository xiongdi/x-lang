# CLAUDE.md — tools 工作区

本目录是 **CLI、LSP、语法高亮生成** 的 Cargo workspace（成员见 [Cargo.toml](./Cargo.toml)）。全局规则见仓库根 [CLAUDE.md](../CLAUDE.md)、[DESIGN_GOALS.md](../DESIGN_GOALS.md)。

## 成员与路径

| Crate | 目录 | 二进制 / 用途 |
|--------|------|----------------|
| `x-cli` | [x-cli/](x-cli/) | 包名 `x`（见 `x-cli/Cargo.toml` 的 `[[bin]]`）：`run`、`check`、`compile`、`build` 等 |
| `x-lsp` | [x-lsp/](x-lsp/) | `x-lsp`：stdio LSP |
| `x-syntax-gen` | [x-syntax-gen/](x-syntax-gen/) | 从 `x-lexer` 的 token 模型生成各编辑器语法文件 |

## 与 compiler 的依赖关系

`tools/Cargo.toml` 里 **`[workspace.dependencies]`** 用 **`path = "../compiler/..."`** 引用 `x-lexer`、`x-parser`、`x-typechecker`、`x-hir`、`x-mir`、`x-lir`、`x-codegen`、`x-interpreter` 等。改 API 时需同时编译 **`cd tools && cargo build`**。

## 常用命令

```bash
cd tools/x-cli && cargo build
cd tools/x-cli && cargo run -- run ../../examples/hello.x
cd tools && cargo test -p x-cli
```

各子目录另有 **CLAUDE.md**（更细的模块说明）。
