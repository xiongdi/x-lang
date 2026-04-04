# CLAUDE.md — x-lir

**LIR / XIR（低层 IR）**：类 C 结构，**绝大多数代码生成后端的统一输入**。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 根类型（`src/lib.rs`）

- **`Program { declarations: Vec<Declaration> }`**
- **`Declaration`**：`Function`、`GlobalVar`、`Struct`、`Class`、`Enum`、`Import`、`ExternFunction` 等。
- **`Function`**：`name`、`parameters`、`return_type`、`body: Block`。
- **`Statement` / `Expression` / `Type`**：后端发射时最常匹配的结构。

## 降阶与优化

- **`lower_mir_to_lir`**（`src/lower.rs`）：MIR → LIR。
- **`peephole_optimize_program`** / `peephole_optimize_function`（`src/peephole.rs`）：窥孔优化。

## 调试

```bash
cd tools/x-cli && cargo run -- compile path/to/file.x --emit lir
```

## 测试

```bash
cd compiler && cargo test -p x-lir
```
