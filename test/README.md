## X 语言一致性测试集 (`test/`)

这个目录包含 **X 语言的一致性测试集**，用于从规格层面验证：

- 词法/语法是否按 `README.md`、`docs/x-keywords.md`、`docs/x-expressions.md`、`docs/x-types.md` 实现
- 解释器在给定程序上的行为（是否能运行、输出是否正确）
- 未来扩展：类型检查、代码生成后端等

通过这里的所有测试，才算是一个“完整”的 X 编译器 + 解释器实现。

---

## 目录结构

按特性分门别类，每个子目录下面放一到多个 TOML 测试文件：

- `test/lexical/`：标识符、数字字面量、字符串、注释、运算符
- `test/types/`：基础类型、字面量与类型、类型注解、（未来）泛型
- `test/expressions/`：算术/比较/逻辑、调用、成员访问、索引、管道
- `test/statements/`：`let` / `let mutable`、赋值、`if` / `while` / `for`、`return`
- `test/functions/`：函数定义、参数、返回值、递归、遮蔽
- 预留：
  - `test/modules/`
  - `test/patterns/`
  - `test/classes/`
  - `test/effects/`
  - `test/async/`

每个 `*.toml` 文件描述一个“章节”（section）及其下的多个用例。

---

## 测试用例格式（TOML）

一个最小的测试文件示例（以 `test/expressions/basic.toml` 为例）：

```toml
[section]
id = "expressions.basic"
name = "Basic expressions"
spec_chapter = "3"

[[case]]
name = "println_hello_world"
spec = ["x-expressions 3.3", "x-keywords println"]
source = """
println("Hello, World!")
"""

  [case.expect]
  parse = "pass"
  run = "pass"
  stdout = "Hello, World!\n"
```

### 顶层字段

- `[section]`：描述本文件对应的规格章节
  - `id`：内部 ID（如 `expressions.basic`）
  - `name`：可读名称
  - `spec_chapter`：可选，指向主规格章节号

- `[[case]]`：一个具体用例
  - `name`：用例名称（唯一）
  - `source`：要测试的 X 程序（多行字符串）
  - `spec`：可选，字符串数组，**追踪到规格文档**（如 `["x-keywords §4.1 let", "x-expressions §4.2 logic"]`）

> 兼容说明：旧的 `spec/x-spec/cases/*.toml` 仍然支持原有字段 `compile_fail`、`error_contains`、`exit_code`，
> 新 runner 会保持向后兼容。

### `[case.expect]` 表

扩展的 `expect` 小节用于精确描述各阶段的预期结果：

- `parse`：`"pass"` 或 `"fail"`
- `typecheck`：`"pass"` 或 `"fail"`（预留，当前类型检查器仍是桩实现）
- `run`：`"pass"` 或 `"fail"`（解释执行）
- `compile`：可选，未来用于指定后端（例如 `"c23"`、`"llvm"`）
- `error_contains`：字符串数组；当某阶段预期失败时，错误消息应包含这些子串
- `stdout`：期望的标准输出完整字符串（包含换行）
- `stdout_contains`：字符串数组；所有子串都必须出现在输出中
- `exit_code`：可选，用于未来编译/运行外部进程的退出码校验

如果未提供 `[case.expect]`，则使用向后兼容的旧语义：

- 默认认为 **解析必须成功**，且（如果配置）解释器运行不得报错
- 仅当旧字段 `compile_fail = true` 时才期望解析失败

---

## 与 runner 的关系

- 现有的 `spec/x-spec` runner 负责读取 `spec/x-spec/cases/*.toml`。
- 我们扩展 runner，使其同时读取根目录下的 `test/**/*.toml`，并统一按上述 schema 解释。
- runner 会按以下顺序执行每个用例：
  1. 解析（必须与 `expect.parse` 一致）
  2. （可选）类型检查（与 `expect.typecheck` 一致）
  3. （可选）解释执行（与 `expect.run`、`stdout`/`stdout_contains` 一致）

当有任一步与 `expect` 不符时，用例失败。

---

## 如何添加新的测试用例

1. 选择合适的子目录，比如：
   - 词法相关 → `test/lexical/xxx.toml`
   - 表达式相关 → `test/expressions/xxx.toml`
2. 如果该文件不存在，按上面的模板创建一个新的 `*.toml` 文件并写入 `[section]`。
3. 按照 schema 添加一个或多个 `[[case]]`，并根据需要填写 `[case.expect]`。
4. 在 `spec` 字段中引用相关规格文档小节，方便追踪。
5. 运行：
   - `cargo run -p x-spec` （或未来的 `x test --conformance`）

> 只有当 **所有 `test/` 用例都通过** 时，才认为当前 X 实现“在该版本规格下是一致的”。

