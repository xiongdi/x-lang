# X-Lang Bazel 迁移指南

## 概述

本文档描述了如何将 X-Lang 项目从 Cargo 迁移到 Bazel 构建系统（Bazel 9.0+，纯 Bzlmod）。

## 为什么选择 Bazel？

1. **多语言支持** - 同时处理 Rust、C/C++、K Framework、Python 等
2. **正确的增量构建** - 精细的依赖跟踪
3. **远程构建和缓存** - 支持团队共享构建缓存
4. **可扩展** - 适合大型 mono repo
5. **统一的工具链** - 所有语言使用相同的构建系统

## 技术栈

- **Bazel 9.0+** - 构建系统（纯 Bzlmod，无需 WORKSPACE）
- **rules_rust 0.50+** - Rust 构建规则
- **crate_universe** - Crates.io 依赖管理
- **rules_cc** - C/C++ 构建规则（用于 C23 后端）
- **rules_kotlin** (可选) - 用于 JVM 后端
- **rules_go** (可选) - 用于工具

## 目录结构

```
x-lang/
├── MODULE.bazel                 # Bazel 模块（Bzlmod，Bazel 9.0+）
├── BUILD.bazel                  # 根 BUILD 文件
├── .bazelrc                     # Bazel 配置
├── Cargo.lock                   # 保留用于 crate_universe
├── Cargo.toml                   # 保留用于 crate_universe
├── compiler/
│   ├── x-lexer/
│   │   ├── BUILD.bazel
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── x-parser/
│   │   ├── BUILD.bazel
│   │   └── ...
│   └── ...
├── library/
│   └── x-stdlib/
│       ├── BUILD.bazel
│       └── ...
├── tools/
│   └── x-cli/
│       ├── BUILD.bazel
│       └── ...
├── spec/
│   ├── x-spec/
│   │   └── BUILD.bazel
│   └── ...
├── examples/
│   ├── BUILD.bazel
│   └── ...
└── third_party/
    └── crates/
        └── BUILD.bazel
```

## 迁移步骤

### 阶段 1: 基础设置
1. 安装 Bazel 9.0+
2. 创建 MODULE.bazel
3. 配置 rules_rust 和 crate_universe
4. 生成第三方依赖的 BUILD 文件

### 阶段 2: 核心 crates
1. x-lexer
2. x-parser
3. x-typechecker
4. x-hir
5. x-perceus

### 阶段 3: 代码生成
1. x-codegen (基础)
2. x-codegen-llvm
3. x-codegen-js
4. x-codegen-jvm
5. x-codegen-dotnet

### 阶段 4: 工具和标准库
1. x-stdlib
2. x-interpreter
3. x-cli
4. x-spec

### 阶段 5: 测试和示例
1. 单元测试
2. 集成测试
3. 示例程序
4. 基准测试

## Bazel 命令

```bash
# 首次需要生成第三方依赖
bazel mod tidy

# 构建所有
bazel build //...

# 运行 CLI
bazel run //tools/x-cli:x -- run examples/hello.x

# 运行所有测试
bazel test //...

# 查看依赖图
bazel query --output=graph //compiler/x-codegen/... | dot -Tpng > deps.png

# 清理
bazel clean
bazel clean --expunge

# 查看模块依赖
bazel mod graph
bazel mod show @rules_rust
```

## 与 Cargo 互操作

- 保留 Cargo.toml 和 Cargo.lock 用于 crate_universe
- 使用 `cargo` 进行依赖更新，然后运行 `bazel mod tidy` 更新 Bazel 配置
- 可以并行使用两个构建系统

## 性能优化

- 使用 `--disk_cache` 启用磁盘缓存
- 配置远程执行 (Remote Execution)
- 使用 `--jobs` 控制并行任务数
- 利用增量分析和缓存

## 注意事项

1. **LLVM 依赖**: x-codegen 的 LLVM 功能需要 LLVM 21，通过 inkwell 绑定
2. **系统库**: 正确配置 LLVM 等系统库的依赖（设置 LLVM_SYS_211_PREFIX）
3. **Windows 支持**: rules_rust 对 Windows 支持良好，但需要测试
4. **K Framework**: 需要为 K 定义自定义规则
5. **Bzlmod**: Bazel 9.0+ 默认使用 Bzlmod，无需 WORKSPACE 文件
