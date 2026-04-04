# CLAUDE.md — x-syntax-gen

**语法高亮等资产生成器**：从 **`x-lexer`** 的 token 模型生成多编辑器语法定义。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 工作流程（`src/main.rs`）

1. **`token_mapping::build_syntax_model()`**：读取 / 映射 lexer 中的关键字与 token 分类。
2. 根据子命令调用 **`generators::{vscode,vim,neovim,sublime,emacs,jetbrains}::generate`**。
3. 默认输出目录：**`-o` / `--output`**，默认 `output/`。

## 子命令

`all`、`vscode`、`vim`、`neovim`、`sublime`、`emacs`、`jetbrains`。

## 何时需要重跑

- 修改 **`x-lexer/src/token.rs`** 中关键字或字面量类别时，应重新生成并提交或分发对应编辑器插件资产。

## 测试

```bash
cd tools && cargo test -p x-syntax-gen
```
