# CLAUDE.md — x-codegen-asm

**原生 / Wasm 汇编后端**：**LIR → 汇编文本**（必要时再经外部汇编器/链接器）。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 模块布局（`src/lib.rs` 导出）

| 模块 | 作用 |
|------|------|
| `arch` | 架构相关常量与 ABI 辅助 |
| `assembly` | 按目标生成指令串（如 `assembly/x86_64.rs`、`aarch64.rs`、`riscv`、`wasm`） |
| `assembler` / `emitter` / `encoding` | 汇编工具链对接或直接编码 |

## 主要类型

- **`NativeBackend`**、**`NativeBackendConfig`**：**`TargetArch`**（`X86_64`、`AArch64`、RISC-V、Wasm 等）、**`TargetOS`**、**`OutputFormat`**（汇编 vs 可执行路径）。
- **`impl x_codegen::CodeGenerator for NativeBackend`**：以 **`generate_from_lir`** 为主；CLI `compile` 默认 **`Target::Asm`** 走此后端。

## 维护注意

- 多架构共享逻辑（**字符串常量池、聚合体初始化、字段偏移**）改动时，避免只修一条后端路径导致行为分叉。
- macOS/Linux/Windows 链接在 **`tools/x-cli/src/commands/compile.rs`**（`assemble_and_link_*`）。

## 测试

```bash
cd compiler && cargo test -p x-codegen-asm
```
