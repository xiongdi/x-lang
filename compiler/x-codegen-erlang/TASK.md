# x-codegen-erlang 任务清单

Erlang 后端 - 生成 Erlang 源代码，供 BEAM/OTP 运行。

## 已完成 ✅

| 任务 | 状态 |
|------|------|
| 框架结构完成 | ✅ 已完成 |
| 基本类型映射框架 | 🚧 部分完成 |
| **generate_from_lir 实现** | ✅ 已完成 (2026-03-29) |

## 完成度

**60%** - generate_from_lir 已实现

## 已实现功能

- [x] CodeGenerator trait 实现
- [x] LIR 函数 → Erlang 函数
- [x] LIR 变量 → Erlang 变量赋值
- [x] LIR 控制流 → Erlang if/while
- [x] LIR 字面量 (Integer, Float, String, Char, Bool)
- [x] LIR 二元运算符 (+, -, *, /, rem, ==, !=, <, >, =<, >=, =:=, /=, band, bor, bxor, bsl, bsr, and, or)
- [x] LIR 一元运算符 (-, not, bnot)
- [x] LIR 表达式 (Call, Member, Index)

## 待完成 ⬜

| 序号 | 任务 | 优先级 | 描述 |
|------|------|--------|------|
| 1 | For 循环支持 | 中 | For 循环语句 |
| 2 | Match 模式匹配支持 | 中 | Match 语句 |
| 3 | 复合类型生成 | 中 | 需要实现 |
| 4 | Perceus 引用计数集成（适配 BEAM GC） | 中 | 需要适配 |

## 验收标准

- [x] generate_from_lir 方法已实现
- [x] 单元测试通过
- [ ] 生成的 Erlang 代码能被 `erlc` 编译
- [ ] BEAM 中能正确运行

## 依赖

- x-lir ✅ 已完成

## 完成度

**约 20%** - 基于 PLAN.md (2026-03-29)
