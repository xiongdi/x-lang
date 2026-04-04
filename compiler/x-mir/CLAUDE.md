# CLAUDE.md — x-mir

**MIR（中层 IR）**：控制流图形式；**Perceus** 风格内存分析（dup/drop/reuse）主要在这里。全局规则见 [../../CLAUDE.md](../../CLAUDE.md)、[../../DESIGN_GOALS.md](../../DESIGN_GOALS.md)。

## 模块与入口

| 模块 | 作用 |
|------|------|
| `lower` | **`lower_hir_to_mir`** → `MirLowerResult` / `MirLowerError` |
| `mir` | MIR 数据结构（`pub use mir::*`） |
| `perceus` | **`analyze_hir`**、`PerceusAnalyzer`、`OwnershipFact`、`ReuseAnalysis` 等 |
| `const_prop` | `constant_propagation` |
| `dead_code` | `dead_code_elimination` |

架构位置：**HIR → MIR → LIR**；MIR 输出作为 `x-lir::lower::lower_mir_to_lir` 的输入。

## 与 x-hir 的关系

Perceus 分析既可填充 HIR 的 `perceus_info`，也在本 crate 内对 CFG 做细粒度事实推导（以当前实现为准）；改内存策略时优先读本目录 `perceus/` 与 `lower/`。

## 测试

```bash
cd compiler && cargo test -p x-mir
```
