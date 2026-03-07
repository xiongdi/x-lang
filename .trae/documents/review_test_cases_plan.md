# 重新审视测试用例计划

## 目标
重新审视所有测试用例，确保它们符合X语言的设计规范，并保持一致性。

## 现状分析

### 测试目录结构
- `/test` 目录包含多个子目录，每个子目录对应不同的测试类别
- 每个子目录包含一个 `basic.toml` 文件，定义了测试用例
- 测试文件存放在 `/test/files` 目录下，按类别组织

### 测试用例格式
- 使用 TOML 格式定义测试用例
- 每个测试用例包含名称、规范引用、源代码和预期结果
- 源代码可以直接在 TOML 文件中定义，也可以引用外部文件

### 设计规范
- 函数定义使用 `function` 关键字，而不是 `fun`
- 类型使用 `Integer`、`String` 等，而不是 `Int`、`str`
- 代码风格应该保持一致

## 实现计划

### [x] 任务 1: 检查所有测试文件中的关键字使用
- **优先级**: P0
- **Depends On**: None
- **Description**:
  - 检查所有测试文件，确保使用 `function` 关键字而不是 `fun`
  - 确保类型名称使用正确的格式（如 `Integer` 而不是 `Int`）
- **Success Criteria**:
  - 所有测试文件都使用 `function` 关键字
  - 所有类型名称都符合设计规范
- **Test Requirements**:
  - `programmatic` TR-1.1: 所有 `.x` 文件中不存在 `fun` 关键字
  - `programmatic` TR-1.2: 所有类型名称都使用正确的格式

### [x] 任务 2: 检查测试配置文件中的源代码
- **优先级**: P0
- **Depends On**: 任务 1
- **Description**:
  - 检查所有 `basic.toml` 文件中的内联源代码
  - 确保它们使用 `function` 关键字和正确的类型名称
- **Success Criteria**:
  - 所有测试配置文件中的内联源代码都使用 `function` 关键字
  - 所有类型名称都符合设计规范
- **Test Requirements**:
  - `programmatic` TR-2.1: 所有 `basic.toml` 文件中不存在 `fun` 关键字
  - `programmatic` TR-2.2: 所有类型名称都使用正确的格式

### [x] 任务 3: 检查代码风格一致性
- **优先级**: P1
- **Depends On**: 任务 1 和 2
- **Description**:
  - 检查所有测试文件的代码风格
  - 确保缩进、空格、命名等方面保持一致
- **Success Criteria**:
  - 所有测试文件的代码风格一致
- **Test Requirements**:
  - `human-judgement` TR-3.1: 代码风格一致，易于阅读

### [x] 任务 4: 运行测试套件
- **优先级**: P0
- **Depends On**: 任务 1、2 和 3
- **Description**:
  - 运行完整的测试套件
  - 确保所有测试都能通过
- **Success Criteria**:
  - 所有测试都通过
- **Test Requirements**:
  - `programmatic` TR-4.1: 所有测试都通过

### [x] 任务 5: 生成测试报告
- **优先级**: P2
- **Depends On**: 任务 4
- **Description**:
  - 生成详细的测试报告
  - 记录所有测试结果
- **Success Criteria**:
  - 生成完整的测试报告
- **Test Requirements**:
  - `programmatic` TR-5.1: 生成包含所有测试结果的报告

## 执行时间估计

- 任务 1: 15 分钟
- 任务 2: 15 分钟
- 任务 3: 10 分钟
- 任务 4: 20 分钟
- 任务 5: 5 分钟

**总估计时间**: 65 分钟