# X 语言测试套件

本测试套件系统性测试 X 语言的所有特性，验证编译流水线每个步骤的输出正确性。

## 目录结构

```
tests/
├── README.md                    # 本文档
├── run_tests.py                 # 测试运行脚本
├── config.toml                  # 测试配置文件
│
├── lexical/                     # 词法分析测试 (规范第1章)
├── types/                       # 类型系统测试 (规范第2章)
├── expressions/                 # 表达式测试 (规范第3章)
├── statements/                  # 语句测试 (规范第4章)
├── functions/                   # 函数测试 (规范第5章)
├── oop/                         # 面向对象测试 (规范第6章)
├── effects/                     # 效果系统测试 (规范第7章)
├── modules/                     # 模块系统测试 (规范第8章)
├── patterns/                    # 模式匹配测试 (规范第9章)
├── memory/                      # 内存管理测试 (规范第10章)
└── metaprogramming/             # 元编程测试 (规范第11章)
```

## 测试文件格式

每个测试用例使用 TOML 格式：

```toml
# 测试元数据
name = "test_name"
description = "测试描述"
category = "lexical/keywords"
spec = ["spec/docs/01-lexical.md"]

# 测试源代码
source = """
let x = 42
println(x)
"""

# 预期行为
[expect]
compile = true
exit_code = 0

# 流水线阶段验证（可选）
[expect.tokens]
contains = ["Let", "Ident", "DecimalInt"]

[expect.ast]
has_declaration = true

# 运行时验证
[expect.runtime]
output = "42\n"
```

## 运行测试

### 运行所有测试

```bash
python tests/run_tests.py
```

### 运行特定类别

```bash
# 词法分析测试
python tests/run_tests.py --category lexical

# 类型系统测试
python tests/run_tests.py --category types

# 表达式测试
python tests/run_tests.py --category expressions
```

### 运行单个测试

```bash
python tests/run_tests.py tests/lexical/keywords/basic.toml
```

### 详细输出

```bash
python tests/run_tests.py --verbose
```

### 列出测试

```bash
python tests/run_tests.py --list
```

## 测试类型

### 1. 编译成功测试

验证代码能够成功通过编译流水线：

```toml
[expect]
compile = true
exit_code = 0

[expect.runtime]
output = "expected output\n"
```

### 2. 编译失败测试

验证错误检测：

```toml
compile_fail = true
error_contains = ["type mismatch"]

[expect]
error_contains = ["undefined variable"]
```

### 3. 流水线阶段测试

验证特定阶段的输出：

```toml
[expect.tokens]
contains = ["Let", "Ident"]
not_contains = ["Error"]

[expect.ast]
has_declaration = true
nodes = ["Variable", "Function"]

[expect.hir]
# HIR 验证

[expect.mir]
# MIR 验证

[expect.lir]
# LIR 验证
```

## 测试覆盖范围

| 章节 | 目录 | 覆盖特性 |
|------|------|----------|
| 第1章 | lexical/ | 关键字、标识符、字面量、运算符、注释 |
| 第2章 | types/ | 基本类型、复合类型、泛型、Option/Result |
| 第3章 | expressions/ | 算术、逻辑、比较、管道、错误处理 |
| 第4章 | statements/ | 变量声明、赋值、控制流、循环 |
| 第5章 | functions/ | 函数定义、参数、闭包、效果 |
| 第6章 | oop/ | 类、继承、Trait、访问控制 |
| 第7章 | effects/ | 效果注解、Async、needs/given |
| 第8章 | modules/ | 导入、导出、可见性 |
| 第9章 | patterns/ | 模式匹配、穷尽性检查 |
| 第10章 | memory/ | 所有权、引用、弱引用 |
| 第11章 | metaprogramming/ | 常量、泛型、宏 |

## 添加新测试

1. 在对应目录下创建 `.toml` 文件
2. 填写测试元数据和源代码
3. 定义预期行为
4. 运行测试验证

### 示例：添加新的关键字测试

```toml
# tests/lexical/keywords/function_keyword.toml
name = "function_keyword"
description = "测试 function 关键字解析"
spec = ["spec/docs/01-lexical.md"]

source = """
function add(a, b) = a + b
"""

[expect]
compile = true

[expect.tokens]
contains = ["Function"]
```

## 与规范关联

每个测试通过 `spec` 字段关联语言规范章节：

```toml
spec = ["spec/docs/01-lexical.md", "spec/docs/02-types.md"]
```

这确保测试覆盖规范定义的所有语言特性。

## 故障排除

### CLI 找不到

确保在项目根目录运行测试，CLI 路径为 `tools/x-cli`。

### 编译超时

某些复杂测试可能需要更长时间，可在 `config.toml` 中调整超时设置。

### 依赖问题

测试运行器需要 Python 3.11+ 和 `tomli` 包（用于解析 TOML）：

```bash
pip install tomli
```
