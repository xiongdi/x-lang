# x-codegen-python 任务清单

Python 后端 - 生成 Python 源代码。

## 已完成 ✅

| 任务 | 状态 |
|------|------|
| 完整框架结构 | ✅ 已完成 |
| 基本代码生成骨架 | ✅ 已完成 |
| 基本类型映射 | ✅ 部分完成 |
| **CodeGenerator trait 实现** | ✅ 已完成 (2026-03-29) |
| **generate_from_lir 实现** | ✅ 已完成 (2026-03-29) |

## 完成度

**60%** - CodeGenerator trait 和 generate_from_lir 已实现

## 已实现功能

- [x] CodeGenerator trait 实现
- [x] LIR 函数 → Python 函数
- [x] LIR 变量 → Python 变量赋值
- [x] LIR 控制流 → Python if/while
- [x] LIR 字面量 (Integer, Float, String, Char, Bool)
- [x] LIR 二元运算符 (+, -, *, /, %, ==, !=, <, >, <=, >=, and, or, &, |, ^, <<, >>)
- [x] LIR 一元运算符 (-, not, ~)
- [x] LIR 表达式 (Call, Member, Index)

## 部分完成 🚧

| 任务 | 状态 |
|------|------|
| 基本类型映射 | 部分完成 |
| 语句和表达式生成 | 部分完成 |
| 控制流生成 | 部分完成 |

## 待完成 ⬜

| 序号 | 任务 | 优先级 | 描述 |
|------|------|--------|------|
| 1 | For 循环支持 | 中 | For 循环语句 |
| 2 | Match 模式匹配支持 | 中 | Match 语句 |
| 3 | 复合类型生成 | 中 | 需要实现 |
| 4 | 类/枚举/记录生成 | 中 | 需要实现 |
| 5 | Perceus 引用计数集成（转为 Python GC） | 中 | 需要适配 |
| 6 | 标准库映射 | 低 | 需要实现 |

## 验收标准

- [x] generate_from_lir 方法已实现
- [x] 单元测试通过
- [ ] 生成的 Python 代码能被 CPython 正确运行
- [ ] 结果正确

## 依赖

- x-lir ✅ 已完成
