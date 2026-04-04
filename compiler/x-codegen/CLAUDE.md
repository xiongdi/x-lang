# CLAUDE.md — x-codegen

**代码生成公共层**：trait、配置、共享缓冲与 XIR 重导出。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 核心 trait（`src/lib.rs`）

```rust
pub trait CodeGenerator {
    type Config;
    type Error;
    fn new(config: Self::Config) -> Self;
    fn generate_from_ast(&mut self, program: &AstProgram) -> Result<CodegenOutput, Self::Error>;
    fn generate_from_hir(&mut self, hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error>;
    fn generate_from_lir(&mut self, lir: &x_lir::Program) -> Result<CodegenOutput, Self::Error>;
}
```

- **`CodegenOutput`**：`files: Vec<OutputFile>`，每个含 `path`、`content`、`file_type`。
- **`DynamicCodeGenerator`**：类型擦除、供 CLI 动态分发；部分后端默认 `generate_from_lir` 未实现会返回 `UnsupportedFeature`。

## 其他常用项

- **`CodeBuffer`**（`src/utils.rs`）：缩进与拼接，后端普遍持有一个实例。
- **`headers`** / **`generate_header_with_version`**：生成文件头注释。
- **`src/xir/`**：XIR 相关定义（`pub use xir::*`）。
- **`Target` / `OutputFormat` / `FileType`**（`src/target.rs`）：CLI `compile` 与后端选择。

## 测试

```bash
cd compiler && cargo test -p x-codegen
```
