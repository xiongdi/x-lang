# 移除C源码并使用Zig替代 - 实现计划

## 任务概述

移除所有C源码，使用Zig替代C后端，包括：

1. 移除C运行时头文件
2. 移除C后端实现
3. 创建Zig后端实现
4. 更新相关配置和文档

## 具体任务

### [x] 任务 1: 移除C运行时头文件
- **优先级**: P0
- **依赖**: 无
- **描述**: 
  - 删除 `x-codegen/runtime/x_runtime.h` 文件
  - 清理所有引用该头文件的代码
- **成功标准**:
  - `x_runtime.h` 文件被删除
  - 代码中不再引用C运行时头文件
- **测试要求**:
  - `programmatic` TR-1.1: 代码编译通过
  - `human-judgement` TR-1.2: 确认C头文件已被完全移除
- **备注**: 该文件包含C运行时的定义，将被Zig运行时代替

### [x] 任务 2: 移除C后端实现
- **优先级**: P0
- **依赖**: 任务 1
- **描述**: 
  - 删除 `x-codegen/src/c_backend.rs` 文件
  - 从 `x-codegen/src/lib.rs` 中移除对C后端的引用
  - 从 `x-codegen/src/target.rs` 中移除C目标相关代码
- **成功标准**:
  - `c_backend.rs` 文件被删除
  - 代码中不再引用C后端
  - 编译通过
- **测试要求**:
  - `programmatic` TR-2.1: 代码编译通过
  - `human-judgement` TR-2.2: 确认C后端代码已被完全移除
- **备注**: C后端将被Zig后端替代

### [/] 任务 3: 创建Zig后端实现
- **优先级**: P0
- **依赖**: 任务 2
- **描述**: 
  - 创建 `x-codegen/src/zig_backend.rs` 文件
  - 实现Zig代码生成和编译逻辑
  - 支持Native和Wasm目标
  - 更新 `x-codegen/src/lib.rs` 以使用Zig后端
  - 更新 `x-codegen/src/target.rs` 以支持Zig目标
- **成功标准**:
  - Zig后端实现完成
  - 代码编译通过
  - 能够生成Zig源码并编译为Native和Wasm
- **测试要求**:
  - `programmatic` TR-3.1: 代码编译通过
  - `programmatic` TR-3.2: 能够成功生成Zig源码
  - `programmatic` TR-3.3: 能够通过Zig编译器编译为Native和Wasm
- **备注**: 参考之前架构文档中的Zig后端设计

### [ ] 任务 4: 移除JavaScript后端
- **优先级**: P1
- **依赖**: 任务 3
- **描述**: 
  - 删除 `x-codegen-js` 目录
  - 清理所有引用JavaScript后端的代码
- **成功标准**:
  - `x-codegen-js` 目录被删除
  - 代码中不再引用JavaScript后端
  - 编译通过
- **测试要求**:
  - `programmatic` TR-4.1: 代码编译通过
  - `human-judgement` TR-4.2: 确认JavaScript后端已被完全移除
- **备注**: JavaScript功能将由Wasm提供

### [ ] 任务 5: 更新Cargo配置
- **优先级**: P1
- **依赖**: 任务 3, 任务 4
- **描述**: 
  - 更新 `compiler/Cargo.toml` 和相关Cargo配置文件
  - 移除对C后端和JavaScript后端的依赖
  - 添加对Zig后端的支持
- **成功标准**:
  - Cargo配置文件更新完成
  - 依赖关系正确
  - 编译通过
- **测试要求**:
  - `programmatic` TR-5.1: `cargo build` 成功
  - `programmatic` TR-5.2: `cargo test` 成功
- **备注**: 确保所有依赖关系正确更新

### [ ] 任务 6: 测试Zig后端
- **优先级**: P0
- **依赖**: 任务 5
- **描述**: 
  - 创建测试文件验证Zig后端功能
  - 测试Native目标编译
  - 测试Wasm目标编译
  - 运行生成的可执行文件
- **成功标准**:
  - Zig后端能够成功编译代码
  - 生成的可执行文件能够正常运行
  - Wasm编译产物能够正常加载
- **测试要求**:
  - `programmatic` TR-6.1: 能够编译简单的X语言程序到Native
  - `programmatic` TR-6.2: 能够编译简单的X语言程序到Wasm
  - `programmatic` TR-6.3: 生成的程序能够正常运行
- **备注**: 测试基本功能，确保Zig后端工作正常

## 预期结果

完成后，X语言编译器将：

1. 完全移除C相关的源码和后端
2. 使用Zig作为新的Native和Wasm后端
3. 保留JVM和.NET后端
4. 移除JavaScript后端（由Wasm替代）
5. 所有代码能够正常编译和运行
