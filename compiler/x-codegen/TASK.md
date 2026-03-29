# x-codegen 任务清单

代码生成核心 - 公共接口和工具函数。

## 已完成 ✅

| 任务 | 状态 |
|------|------|
| `CodeGenerator` trait 定义 | ✅ 已完成 |
| `Target` 枚举（所有支持的后端） | ✅ 已完成 |
| `CodeGenConfig` 和 `CodegenOutput` | ✅ 已完成 |
| 错误类型定义 | ✅ 已完成 |
| 工具模块：buffer/escape/operators/symbols | ✅ 已完成 |
| XIR（旧 IR）兼容保留 | ✅ 已完成 |

## 已实现的后端

| 后端 | 状态 | generate_from_lir |
|------|------|-----------------|
| Zig | ✅ 成熟 | ✅ 已完成 |
| TypeScript | ✅ 已完成 | ✅ 已完成 |
| Python | ✅ 已完成 | ✅ 已完成 (2026-03-29) |
| Rust | ✅ 已完成 | ✅ 已完成 |
| Java (JVM) | ✅ 已完成 | ✅ 已完成 |
| C# (.NET) | ✅ 已完成 | ✅ 已完成 (2026-03-29) |
| Swift | ✅ 已完成 | ✅ 已完成 (2026-03-29) |
| Erlang | ✅ 已完成 | ✅ 已完成 (2026-03-29) |
| LLVM | ✅ 部分完成 | ✅ 已完成 |
| Native | 🚧 进行中 | ✅ 已完成 |

## 验收标准

- [x] 所有主要后端都能实现 CodeGenerator trait
- [x] 工具函数满足所有后端需求

## 依赖

- x-lir ✅ 已完成
