# 完善编译器计划

## 目标
完善X语言编译器，确保所有测试通过。

## 现状分析

### 测试失败原因
1. **函数定义的简写形式**：测试文件使用 `function add(a: Int, b: Int) -> Int = a + b`，但解析器只支持带大括号的形式
2. **管道运算符**：测试文件使用 `5 |> double |> square`，解析器不支持
3. **When 表达式**：测试文件使用 `when x > 5 then "greater" else "less or equal"`，解析器不支持
4. **Const 绑定**：测试文件使用 `const MAX_VALUE: Int = 100`，解析器不支持
5. **For 循环**：测试文件使用 for 循环，解析器不支持
6. **类型定义**：测试文件使用 `type Shape = | Circle { radius: Float } | Rect { width: Float, height: Float } | Point`，解析器不支持
7. **Option 和 Result 类型**：测试文件使用 `Some(42)` 和 `Ok(42)`，解析器不支持

### 编译器现状
- 词法分析器：基本功能完整，支持所有关键字和操作符
- 解析器：支持基本的函数定义、变量绑定、if 语句、while 循环、表达式等
- 解释器：支持基本的表达式求值和语句执行

## 实现计划

### [x] 任务 1: 支持函数定义的简写形式
- **优先级**: P0
- **Depends On**: None
- **Description**:
  - 修改解析器，支持 `function add(a: Int, b: Int) -> Int = a + b` 这种简写形式
  - 在 `parse_function` 方法中添加对 `=` 符号的处理
- **Success Criteria**:
  - 函数定义的简写形式能够被正确解析
- **Test Requirements**:
  - `programmatic` TR-1.1: `test\files\expressions\function_calls.x` 测试通过
  - `programmatic` TR-1.2: `test\files\expressions\pipe_operator.x` 测试通过

### [/] 任务 2: 支持 Const 绑定
- **优先级**: P0
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 `const MAX_VALUE: Int = 100` 这种 const 绑定
  - 在 `parse_program` 和 `parse_statement` 方法中添加对 `const` 关键字的处理
- **Success Criteria**:
  - Const 绑定能够被正确解析
- **Test Requirements**:
  - `programmatic` TR-2.1: `test\files\statements\const_bindings.x` 测试通过

### [ ] 任务 3: 支持管道运算符
- **优先级**: P1
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 `5 |> double |> square` 这种管道运算符
  - 在表达式解析中添加对 `|>` 操作符的支持
- **Success Criteria**:
  - 管道运算符能够被正确解析和执行
- **Test Requirements**:
  - `programmatic` TR-3.1: `test\files\expressions\pipe_operator.x` 测试通过

### [ ] 任务 4: 支持 When 表达式
- **优先级**: P1
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 `when x > 5 then "greater" else "less or equal"` 这种 when 表达式
  - 在表达式解析中添加对 `when` 关键字的处理
- **Success Criteria**:
  - When 表达式能够被正确解析和执行
- **Test Requirements**:
  - `programmatic` TR-4.1: `test\files\expressions\when_expression.x` 测试通过

### [ ] 任务 5: 支持 For 循环
- **优先级**: P1
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 for 循环
  - 在 `parse_statement` 方法中添加对 `for` 关键字的处理
- **Success Criteria**:
  - For 循环能够被正确解析和执行
- **Test Requirements**:
  - `programmatic` TR-5.1: `test\files\statements\for_loop.x` 测试通过
  - `programmatic` TR-5.2: `test\files\statements\for_inclusive_loop.x` 测试通过

### [ ] 任务 6: 支持类型定义
- **优先级**: P1
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 `type Shape = | Circle { radius: Float } | Rect { width: Float, height: Float } | Point` 这种类型定义
  - 在 `parse_program` 方法中添加对 `type` 关键字的处理
- **Success Criteria**:
  - 类型定义能够被正确解析
- **Test Requirements**:
  - `programmatic` TR-6.1: `test\files\types\adt_types.x` 测试通过
  - `programmatic` TR-6.2: `test\files\types\record_types.x` 测试通过

### [ ] 任务 7: 支持 Option 和 Result 类型
- **优先级**: P1
- **Depends On**: 任务 1
- **Description**:
  - 修改解析器，支持 `Some(42)` 和 `Ok(42)` 这种形式
  - 在 `parse_primary` 方法中添加对 `Some`、`None`、`Ok`、`Err` 关键字的处理
- **Success Criteria**:
  - Option 和 Result 类型能够被正确解析和执行
- **Test Requirements**:
  - `programmatic` TR-7.1: `test\files\types\option_type.x` 测试通过
  - `programmatic` TR-7.2: `test\files\types\result_type.x` 测试通过

### [ ] 任务 8: 运行完整测试套件
- **优先级**: P0
- **Depends On**: 任务 1-7
- **Description**:
  - 运行完整的测试套件
  - 确保所有测试都能通过
- **Success Criteria**:
  - 所有测试都通过
- **Test Requirements**:
  - `programmatic` TR-8.1: 所有测试文件都通过

## 执行时间估计

- 任务 1: 30 分钟
- 任务 2: 20 分钟
- 任务 3: 25 分钟
- 任务 4: 25 分钟
- 任务 5: 30 分钟
- 任务 6: 40 分钟
- 任务 7: 20 分钟
- 任务 8: 15 分钟

**总估计时间**: 205 分钟