# CLAUDE.md — x-hir

**HIR（高层 IR）**：带类型环境、可去糖、可接 Perceus 元数据的中间层。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 根类型

- **`Hir`**（`src/lib.rs`）：`module_name`、`declarations`、`statements`、`type_env: HirTypeEnv`、`perceus_info: HirPerceusInfo`。
- **`HirDeclaration` / `HirStatement` / `HirExpression`**：与 AST 平行但更适合中端。

## 常用入口函数（按流水线顺序）

| 函数 | 作用 |
|------|------|
| `ast_to_hir(program)` / `ast_to_hir_with_type_env` | AST → HIR |
| `analyze_semantics` | 语义分析 |
| `optimize_hir` / `constant_fold_hir` / `dead_code_eliminate_hir` | 可选优化 |
| `analyze_ownership` 相关（见 `perceus_info`） | 与所有权/ drop 信息衔接 |

子模块 `constant_folding` 导出 `constant_fold` 等。

## 下游

- **`x_mir::lower_hir_to_mir`**：HIR → MIR（CFG）。
- 少数后端仍实现 `CodeGenerator::generate_from_hir`。

## 测试

```bash
cd compiler && cargo test -p x-hir
```
