# X语言规范测试框架

本目录包含基于TOML的X语言规范测试用例。

## 测试用例格式

每个测试用例是一个TOML文件，包含以下字段：

### 必需字段

```toml
name = "test_name"           # 测试名称
source = "println(42)"       # X语言源代码
```

### 可选字段

```toml
# 预期行为
exit_code = 0                # 预期退出码（默认0）
stdout = "42\n"              # 预期标准输出
stderr = ""                  # 预期标准错误输出

# 编译相关
compile_fail = false         # 预期编译失败（默认false）
error_contains = "type mismatch"  # 编译错误应包含此文本

# 规范引用
spec = ["02-types", "03-expressions"]  # 引用的规范章节

# 分类
category = "types"           # 测试分类：types, expressions, functions, etc.
tags = ["generics", "inference"]  # 标签

# 条件执行
target = "zig"               # 仅在特定后端运行：zig, llvm, js等
skip = false                 # 跳过此测试
skip_reason = "未实现"       # 跳过原因
```

## 测试类型

### 1. 成功编译并运行

```toml
name = "simple_print"
source = "println(42)"
exit_code = 0
stdout = "42\n"
```

### 2. 编译失败

```toml
name = "type_mismatch"
source = "let x: integer = 3.14"
compile_fail = true
error_contains = "type mismatch"
```

### 3. 类型推断测试

```toml
name = "type_inference"
source = """
let x = 42
let y = x + 1
println(y)
"""
stdout = "43\n"
spec = ["02-types"]
tags = ["inference"]
```

### 4. 模式匹配测试

```toml
name = "match_exhaustive"
source = """
enum Option<T> { None, Some(T) }

function unwrap<T>(opt: Option<T>) -> T {
    match opt {
        Some(value) => value,
        None => panic("unwrapped None")
    }
}

println(unwrap(Some(42)))
"""
stdout = "42\n"
spec = ["09-patterns"]
tags = ["match", "enum"]
```

## 目录结构

```
tests/spec/
├── README.md           # 本文档
├── 00-philosophy/      # 设计哲学相关测试
├── 01-lexical/         # 词法结构测试
├── 02-types/           # 类型系统测试
├── 03-expressions/     # 表达式测试
├── 04-statements/      # 语句测试
├── 05-functions/       # 函数测试
├── 06-classes/         # 类与接口测试
├── 07-effects/         # 效果系统测试
├── 08-modules/         # 模块系统测试
├── 09-patterns/        # 模式匹配测试
├── 10-memory/          # 内存模型测试
└── 11-metaprogramming/ # 元编程测试
```

## 运行测试

```bash
# 运行所有规范测试
x test spec

# 运行特定目录的测试
x test spec/02-types

# 运行特定测试
x test spec/02-types/inference.toml

# 仅运行特定标签的测试
x test spec --tag inference

# 运行并生成覆盖率报告
x test spec --coverage
```

## 测试命名约定

- 使用描述性名称：`type_inference_int_literal` 而非 `test1`
- 使用snake_case
- 包含测试的关键特征
- 如果测试特定错误，名称应反映错误类型

## 最佳实践

1. **最小化测试用例**：只包含验证行为所需的最少代码
2. **明确规范引用**：每个测试应引用相关的规范章节
3. **避免依赖**：测试应尽可能独立，不依赖外部状态
4. **清晰的错误消息**：error_contains 应该足够具体以区分不同错误
5. **合理的标签**：使用标签帮助分类和过滤测试
