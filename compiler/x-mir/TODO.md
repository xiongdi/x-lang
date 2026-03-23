# x-mir TODO

## 项目概览

x-mir 是 X 语言的中层中间表示（Middle-level Intermediate Representation），包含控制流图（CFG）形式的表示和 Perceus 内存管理分析功能。

## 当前状态

- **完成度**: ~70%
- **核心功能**: MIR 数据结构、Perceus 分析、HIR → MIR 降级
- **集成状态**: 与 x-hir 和 x-lir 集成完成

## 待办清单

### 优先级 A - 核心功能
- [x] 实现完整的 MIR 数据结构（BasicBlock、Instruction、Operand）
- [x] 实现 Perceus 内存分析
- [x] 实现 HIR 到 MIR 的降级
- [x] 实现所有权状态追踪
- [x] 实现内存操作记录（dup/drop/reuse/alloc）
- [x] 完善循环和条件语句的分支分析
- [x] 实现跨函数的内存分析

### 优先级 B - 优化 Pass
- [x] 常量传播（Constant Propagation）
- [x] 死代码消除（Dead Code Elimination）
- [x] 公共子表达式消除（CSE）
- [x] 常量折叠（Constant Folding）
- [ ] 循环不变量外提（Loop Invariant Code Motion）
- [ ] 内联优化

### 优先级 C - 优化与完善
- [ ] 实现更精确的复用分析算法
- [ ] 实现增量分析
- [ ] 优化分析算法性能
- [ ] 实现分析结果的可视化输出
- [ ] 增加对复杂数据结构的分析支持

### 优先级 D - 测试与文档
- [x] 编写单元测试用例
- [ ] 完善 API 文档
- [ ] 编写 Perceus 内存管理技术文档

## 测试覆盖情况

### 现有测试
- ✅ MIR 降级测试
- ✅ Perceus 分析测试
- ✅ 优化 Pass 测试

## 功能覆盖评估

| 功能模块 | 已完成 | 待实现 | 完成度 |
|---------|--------|--------|--------|
| MIR 数据结构 | ✅ | ❌ | 100% |
| HIR → MIR 降级 | ✅ | ❌ | 90% |
| Perceus 分析器 | ✅ | ❌ | 100% |
| 所有权追踪 | ✅ | ❌ | 70% |
| 内存操作记录 | ✅ | ❌ | 60% |
| 复用分析 | 🚧 | ✅ | 40% |
| dup/drop 插入 | 🚧 | ✅ | 45% |
| 优化 Pass | 🚧 | ✅ | 60% |
| 控制流分析 | 🚧 | ✅ | 30% |

## 质量门禁

### 覆盖率目标
- **行覆盖率**：100%
- **分支覆盖率**：100%
- **测试通过率**：100%

### 验收步骤

```bash
cd compiler
cargo test -p x-mir
cargo llvm-cov -p x-mir --tests
```

## 备注

x-mir 合并了原 x-perceus 的所有功能，是 X 语言实现编译期内存管理的关键组件。
