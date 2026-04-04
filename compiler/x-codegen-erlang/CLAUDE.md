# CLAUDE.md — x-codegen-erlang

**Erlang 后端**：生成 **Erlang/OTP** 模块源码；变量需符合 **大写/下划线开头** 等语法规则。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`ErlangBackend`**、**`ErlangBackendConfig`**（`module_name` 等）。
- **`ErlangBackend::new`**：默认模块名 **`x_module`**（可配置）。
- **`impl x_codegen::CodeGenerator for ErlangBackend`**：**`generate_from_ast`** 与 **`generate_from_lir`** 均存在；`while`/`loop` 等通过尾递归辅助函数实现。
- 内部状态：**`loop_counter`**（唯一循环辅助函数名）、**`exports`** 列表。

## 路线图

- 见同目录 **`TODO.md`**（若有）：如 HIR 直出等待办项。

## 测试

```bash
cd compiler && cargo test -p x-codegen-erlang
```
