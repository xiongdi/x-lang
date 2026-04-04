# CLAUDE.md — x-codegen-swift

**Swift 后端**：生成 **Swift 6.x** 风格源码；**`impl x_codegen::CodeGenerator for SwiftBackend`**。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 主要类型（`src/lib.rs`）

- **`SwiftBackend`**、**`SwiftBackendConfig`**、**`SwiftTarget`**（`MacOS`、`IOS`、`WatchOS`、`TvOS`、`Linux`）。
- 消费 **`x_parser::ast`** 与 **`x_lir::Program`**（含 `WaitType` 等 X 特有构造的映射）。

## 测试

```bash
cd compiler && cargo test -p x-codegen-swift
```
