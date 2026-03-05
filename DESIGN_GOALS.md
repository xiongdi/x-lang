# X 语言设计目标

> **本文档是 X 语言的最高准则与宪法性文件。**
> 所有语言设计决策、实现优先级和路线图规划都必须以本文档为依据。
> 若其他文档与本文档冲突，以本文档为准。

---

## 1. 现代通用编程语言

X 是一门**现代的、通用的编程语言**，适用于：

- 系统编程（OS、嵌入式、驱动）
- 应用开发（桌面、服务端、CLI 工具）
- 高性能计算（科学计算、数据处理）
- Web 后端与基础设施

X 不是领域特定语言（DSL），而是能覆盖从底层系统到上层应用的全栈语言。

---

## 2. 类型安全

X 拥有**完整的、健全的类型系统**：

- **Hindley-Milner 类型推断**：绝大多数类型注解可省略，编译器自动推断
- **代数数据类型（ADT）**：`enum`（sum type）+ `record`（product type）
- **参数多态**：泛型函数与泛型类型
- **无 null**：用 `Option<T>` 代替 null，编译器强制处理
- **无异常**：用 `Result<T, E>` 代替异常，错误路径显式可见
- **穷尽匹配**：`match` 模式匹配必须覆盖所有情况

**设计准则**：如果编译通过，程序就不应出现类型错误。

---

## 3. 内存安全

X 采用与 **Koka** 相同的 **Perceus** 算法实现内存管理：

- **编译时引用计数**：编译器在编译期精确插入 `dup`（引用复制）和 `drop`（引用释放）
- **重用分析（Reuse Analysis）**：当值的引用计数为 1 时，函数式的"创建新值"可优化为原地更新
- **无垃圾回收器（No GC）**：无 stop-the-world 停顿，延迟确定
- **无手动管理**：开发者不需要手写 `malloc`/`free`，不需要标注生命周期
- **线程安全**：引用计数操作在必要时为原子操作

**设计准则**：安全的内存管理不应以牺牲性能或开发体验为代价。

---

## 4. 多范式支持

X 支持**函数式编程（FP）** 与 **面向对象编程（OOP）** 的融合，以及过程式和声明式风格：

| 范式 | 核心特性 |
|------|---------|
| **函数式** | 纯函数、不可变数据、高阶函数、管道运算符 `\|>`、模式匹配 |
| **面向对象** | 类、继承、接口/trait、方法链、封装 |
| **过程式** | 可变变量 `let mutable`、`for`/`while` 循环、顺序执行 |
| **声明式** | `where`/`sort by` 查询语法、表达式导向 |

**设计准则**：为问题选择最合适的范式，而非强迫开发者适应单一范式。

---

## 5. 参考语言

X 的设计广泛参考以下语言的优秀实践：

| 参考语言 | 借鉴领域 |
|---------|---------|
| **Python** | 可读性、简洁语法、开发体验 |
| **Rust** | 所有权与安全性、模式匹配、枚举、`Result`/`Option`、cargo 工具链 |
| **Go** | goroutine 并发模型、简洁性、快速编译 |
| **Kotlin** | 表达式导向、空安全、扩展函数、协程 |
| **TypeScript** | 结构化类型、渐进类型化、联合类型 |
| **Swift** | 值类型、可选类型、协议导向编程 |
| **Haskell** | HM 类型推断、类型类、纯函数式、Effect 系统思想 |
| **Scala 3** | FP+OOP 融合、ADT、上下文参数、元编程 |
| **F#** | 管道运算符、类型推断、FP-first 设计、计算表达式 |
| **Zig** | 编译期计算、无隐藏控制流、手动底层控制 |
| **Koka** | Perceus 内存管理、代数效果系统 |

---

## 6. Hindley-Milner 类型推断

X 的类型推断基于 **Hindley-Milner（HM）算法**及其扩展：

```x
// 无需类型注解，编译器自动推断
let x = 42                      // x : Integer
let f = (a, b) -> a + b         // f : (Integer, Integer) -> Integer
let xs = [1, 2, 3]              // xs : List<Integer>
let result = xs |> map(f(_, 1)) // result : List<Integer>
```

- **局部类型推断**：函数体内几乎不需要任何类型注解
- **顶层签名推荐但可选**：公共 API 建议写类型签名以提升可读性
- **双向类型检查**：结合自上而下（期望类型）与自下而上（推断类型）两个方向
- **约束求解**：支持类型类约束、子类型约束、效果约束的统一求解

---

## 7. 默认不可变

X 中所有绑定**默认不可变**：

```x
let x = 10              // 不可变
let mutable y = 20      // 需要显式 mutable 才可变
y = 30                   // OK
x = 40                   // 编译错误
```

- **值语义优先**：数据默认为不可变值，有利于推理和并发安全
- **显式可变**：可变性是显式选择（`let mutable`），而非默认行为
- **Perceus 优化**：不可变性是重用分析的前提——唯一引用时可安全原地更新
- **函数式友好**：鼓励使用纯函数和不可变数据流

---

## 8. 多种并发模型

X 提供**三种并发模型**，开发者可根据场景选择：

### 8.1 Go 风格：轻量级协程（goroutine-like）
```x
go function() {
    // 轻量级协程
    let result = compute_heavy()
    channel.send(result)
}
```

### 8.2 Actor 模型：消息传递
```x
actor Counter {
    let mutable count = 0
    receive Increment => count += 1
    receive GetCount(reply) => reply.send(count)
}
```

### 8.3 Async/Await：结构化并发
```x
async function fetchData() -> Data {
    let a = await fetch("/api/users")
    let b = await fetch("/api/posts")
    combine(a, b)
}

// 结构化并发原语
let (users, posts) = await together {
    fetch("/api/users"),
    fetch("/api/posts")
}
```

**设计准则**：并发应该简单、安全、高效。不同场景用不同模型，而非一刀切。

---

## 9. 工具链：1:1 对标 Cargo

X 的工具链 **`x`** 在功能上 **1:1 对标 Rust 的 Cargo**：

| Cargo 命令 | X 等价命令 | 功能 |
|-----------|-----------|------|
| `cargo new` | `x new` | 创建新项目 |
| `cargo build` | `x build` | 构建项目 |
| `cargo run` | `x run` | 运行项目 |
| `cargo test` | `x test` | 运行测试 |
| `cargo check` | `x check` | 类型检查（不编译） |
| `cargo fmt` | `x fmt` | 代码格式化 |
| `cargo clippy` | `x lint` | 静态分析 |
| `cargo doc` | `x doc` | 生成文档 |
| `cargo publish` | `x publish` | 发布到仓库 |
| `cargo add` | `x add` | 添加依赖 |
| `cargo bench` | `x bench` | 运行基准测试 |
| `cargo install` | `x install` | 安装二进制工具 |

- **项目清单**：`x.toml`（对标 `Cargo.toml`）
- **锁文件**：`x.lock`（对标 `Cargo.lock`）
- **包仓库**：中心化仓库（对标 crates.io）
- **工作空间**：支持 monorepo 多包项目

---

## 10. Effect System（效果系统）

X 拥有**代数效果系统**，函数的副作用在类型签名中显式声明：

```x
// 函数类型签名: (参数) -> 返回值 with 效果
function readFile(path: String) -> String with IO, Throws<FileNotFound>

// 纯函数（无效果）
function add(a: Integer, b: Integer) -> Integer

// 多种效果
function processData(url: String) -> Data with Async, IO, Throws<NetworkError>
```

- **效果多态**：泛型函数可以对效果进行参数化
- **效果推断**：编译器自动推断函数的效果集
- **效果处理器（Handler）**：可以拦截和处理效果
- **核心效果**：`IO`、`Async`、`State<S>`、`Throws<E>`、`NonDet`

**设计准则**：副作用必须可见，才能被理解和控制。

---

## 11. 与 C 的 FFI

X 提供与 C 语言的**零开销外部函数接口（FFI）**：

```x
// 声明外部 C 函数
extern "C" {
    function printf(format: CString, ...) -> Integer
    function malloc(size: USize) -> Pointer<Void>
    function free(ptr: Pointer<Void>)
}

// 在 X 中调用
function main() {
    let msg = "Hello from X!\n"
    printf(msg.as_cstring())
}
```

- **直接调用 C 函数**：无包装开销
- **C 类型映射**：`CInt`、`CString`、`Pointer<T>` 等
- **头文件生成**：X 编译器可生成 C 头文件供 C 代码调用 X 函数
- **`unsafe` 边界**：FFI 调用位于 `unsafe` 块中，安全与不安全代码有清晰边界

---

## 12. 多后端架构

X 编译器采用**统一中间表示（X IR）+ 多后端**架构：

```
                    ┌─── C23 ──→ GCC / Clang / MSVC ──→ Native
                    │
X Source → X IR ────┼─── JVM Bytecode ──→ Java 生态
                    │
                    └─── CIL ──→ .NET CLR 生态
```

### 12.1 C 后端（主要后端）
- **目标**：X IR → C23 源码 → GCC / Clang / MSVC 编译
- **优势**：最大可移植性，利用成熟 C 编译器的优化
- **适用**：系统编程、嵌入式、性能关键场景

### 12.2 JVM 后端
- **目标**：X IR → JVM 字节码（`.class` / `.jar`）
- **优势**：接入 Java 生态、跨平台、成熟的 GC 和 JIT
- **适用**：企业应用、Android 开发、大数据（Spark/Flink）

### 12.3 .NET CLR 后端
- **目标**：X IR → CIL 字节码（`.dll` / `.exe`）
- **优势**：接入 .NET 生态、C# 互操作、Unity 游戏开发
- **适用**：Windows 应用、游戏开发、企业级 .NET 生态

### 后端选择
```bash
x build                        # 默认：C 后端 → native
x build --target jvm           # JVM 后端
x build --target dotnet        # .NET CLR 后端
x build --target c --cc clang  # 指定 C 编译器
```

---

## 13. 无异常：Option 与 Result

X 语言**没有异常机制**。所有错误处理通过类型系统完成：

### Option：表示"有或无"
```x
// Option<T> = Some(T) | None
function find(users: List<User>, id: Integer) -> Option<User> {
    users |> filter(.id == id) |> first
}

let user = find(users, 42)
match user {
    Some(u) => println("Found: {u.name}")
    None    => println("Not found")
}

// 便捷运算符
let name = user?.name ?? "anonymous"   // 链式访问 + 默认值
```

### Result：表示"成功或失败"
```x
// Result<T, E> = Ok(T) | Err(E)
function readFile(path: String) -> Result<String, IoError> {
    if not exists(path) {
        return Err(IoError.NotFound(path))
    }
    Ok(readBytes(path).decode())
}

// 模式匹配处理
match readFile("config.toml") {
    Ok(content) => parseConfig(content)
    Err(e)      => useDefault()
}

// ? 运算符：错误自动向上传播
function loadConfig() -> Result<Config, IoError> {
    let content = readFile("config.toml")?   // 失败时自动 return Err
    let config = parse(content)?
    Ok(config)
}
```

### 设计理由

| 异常机制 | Option/Result |
|---------|--------------|
| 隐式控制流，调用者不知道函数会抛什么 | 错误路径在类型签名中**显式可见** |
| 容易遗漏 catch，运行时崩溃 | 编译器**强制**处理每种情况 |
| 性能不可预测（栈展开开销） | **零成本**抽象，与正常返回相同开销 |
| 难以组合（try-catch 嵌套地狱） | 用 `?` 和 `\|>` **链式组合** |

**设计准则**：错误是值，不是控制流。所有可能失败的操作都必须在返回类型中体现。

---

---

## 14. 关键字：英文全称，含义准确

X 的关键字**必须使用英文全称**，且能**准确表达其语义**，不使用缩写或生造词：

### 设计规则

1. **全称，不缩写**：`function` 而非 `fn`/`func`，`integer` 而非 `int`，`boolean` 而非 `bool`
2. **含义自明**：读到关键字就知道它做什么，无需查文档
3. **来自自然英语**：优先使用普通英语词汇，而非编程术语黑话

### 关键字示例

| 类别 | X 关键字 | 含义 | 对比其他语言 |
|------|---------|------|-------------|
| 声明 | `let` | 不可变绑定 | Rust `let`, JS `const` |
| 声明 | `mutable` | 可变绑定标记 | Rust `mut`, Kotlin `var` |
| 函数 | `function` | 函数定义 | Rust `fn`, Go `func`, Python `def` |
| 返回 | `return` | 函数返回 | 通用 |
| 条件 | `if` / `else` | 条件分支 | 通用 |
| 循环 | `for` / `while` | 循环 | 通用 |
| 匹配 | `match` | 模式匹配 | Rust `match`, Scala `match` |
| 类型 | `type` | 类型别名/定义 | Go `type`, TS `type` |
| 类 | `class` | 类定义 | 通用 |
| 接口 | `trait` | 接口/行为约束 | Rust `trait`, Scala `trait` |
| 模块 | `module` | 模块声明 | — |
| 导入 | `import` | 导入符号 | Python/TS/Kotlin |
| 异步 | `async` / `await` | 异步操作 | 通用 |
| 并发 | `together` / `race` | 并发组合 | X 独创 |

### 反面示例（X 不会采用）

| 缩写/生造 | 问题 | X 的选择 |
|-----------|------|---------|
| `fn` | 不是英文单词 | `function` |
| `func` | 截断 | `function` |
| `def` | 含义模糊（define?） | `function` |
| `impl` | 缩写 | `implement` |
| `pub` | 缩写 | `public` |
| `mod` | 缩写，且与取模冲突 | `module` |
| `i32` / `u8` | 密码式命名 | `Integer` / `Byte` |

**设计准则**：代码应该像散文一样可读。关键字是语言的词汇表——每个词都必须是真正的英语单词，且含义精确。

---

## 15. 可读性放在第一位

> **Code is read far more often than it is written.**

可读性是 X 语言**最高优先级的设计约束**。当可读性与简洁性、性能、灵活性冲突时，**可读性胜出**。

### 核心准则

1. **一眼看懂**：任何代码片段，一个合格的开发者应当在几秒内理解其意图
2. **只有一种显而易见的写法**：避免提供多种等价但风格迥异的语法（Python 之禅）
3. **新手可读，专家可写**：语法对初学者友好，同时不限制高级用法

### 可读性如何指导设计

| 设计决策 | 可读性优先的选择 | 被拒绝的替代方案 |
|---------|----------------|----------------|
| 关键字 | `function`（完整英文） | `fn`、`func`、`def` |
| 可变性 | `let mutable x = 0` | `var x`（含义不够显式） |
| 模式匹配 | `match value { ... }` | 运算符重载式匹配 |
| 管道 | `data \|> filter() \|> map()` | 深层嵌套 `map(filter(data))` |
| 错误处理 | `Result<T, E>` + `?` | 隐式异常 |
| 效果 | 类型签名中显式声明 | 隐藏副作用 |
| 导入 | `import std.collections.HashMap` | `use std::collections::HashMap` |

### 可读性检验清单

在引入任何新语法或特性时，必须通过以下检验：

- [ ] **大声朗读测试**：把代码读出来，是否像在说一句通顺的英语？
- [ ] **六个月测试**：半年后重新看这段代码，能否立即理解？
- [ ] **新手测试**：一个没学过 X 的程序员，能否猜出这段代码的含义？
- [ ] **无需注释测试**：代码本身是否已经足够清晰，不需要注释来解释"做了什么"？

### 示例对比

```x
// X：可读性优先
function fibonacci(n: Integer) -> Integer {
    match n {
        0 => 0
        1 => 1
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

let results = numbers
    |> filter(is_even)
    |> map(square)
    |> take(10)
```

```rust
// 对比 Rust：简洁但符号密度高
fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

let res: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).take(10).collect();
```

**设计准则**：宁可多打几个字符，也不要牺牲可读性。写代码花一次时间，读代码花一百次。

---

## 16. 不使用奇怪的符号

X 的语法**只使用常见的、含义直观的符号**。如果一个符号需要查文档才能理解，它就不应该出现在 X 中。

### 设计规则

1. **键盘上能直接打出来**：不使用需要特殊输入法或 Unicode 查表才能输入的符号
2. **看到就知道什么意思**：每个符号的含义对大多数程序员来说应该是显而易见的
3. **宁可用关键字也不用符号**：当符号的含义不够清晰时，用英文单词代替

### X 使用的符号（全部是常见符号）

| 符号 | 含义 | 熟悉度 |
|------|------|--------|
| `+` `-` `*` `/` `%` | 算术运算 | 全人类都懂 |
| `=` | 赋值/绑定 | 所有语言通用 |
| `==` `!=` `<` `>` `<=` `>=` | 比较运算 | 所有语言通用 |
| `(` `)` `{` `}` `[` `]` | 分组/块/列表 | 所有语言通用 |
| `.` | 成员访问 | 所有语言通用 |
| `,` `:` `;` | 分隔符 | 所有语言通用 |
| `->` | 函数返回类型/Lambda | 广泛使用（Rust、Kotlin、Swift） |
| `=>` | 模式匹配分支 | 广泛使用（Scala、Kotlin、JS） |
| `\|>` | 管道运算符 | F#/Elixir 用户熟悉，含义直观 |
| `?` | 错误传播 | Rust/Swift 用户熟悉 |
| `?.` | 可选链 | Kotlin/TS/C# 用户熟悉 |
| `??` | 默认值 | C#/TS/Swift 用户熟悉 |
| `..` `..=` | 范围 | Rust/Ruby 用户熟悉 |
| `//` `/** */` | 注释 | 所有 C 系语言通用 |

### X 不使用的符号（反面示例）

| 符号 | 出现在 | 问题 | X 的替代方案 |
|------|--------|------|-------------|
| `\|>>`  | Haskell | 含义不直观，需要学习 | 不使用 |
| `·` (中间点) | — | 键盘上不易输入，不常见 | 不使用（效果系统改用关键字或常见符号） |
| `<\|>` | Haskell | 过于抽象 | 不使用 |
| `>>=` | Haskell | monad bind，需要函数式背景 | 不使用 |
| `::` | Haskell/Rust | 路径分隔用 `.` 代替 | `import std.io`（不是 `std::io`） |
| `#[...]` | Rust | 不直观的注解语法 | `@annotation`（通用的 `@` 语法） |
| `'a` | Rust | 生命周期标注，Perceus 不需要 | 不存在 |
| `<-` | Haskell | 不是赋值，容易混淆 | `=` |
| `~>` `~` | 某些语言 | 含义模糊 | 不使用 |
| `$` | Haskell/PHP | 含义因语言而异 | 不使用 |
| `@` 作为运算符 | 某些语言 | 保留给注解/装饰器 | 仅用于 `@annotation` |
| `^^` | — | 与 `^`（幂运算）混淆 | 不使用 |

### 效果系统的符号选择

此前规格中使用的 `·`（中间点 U+00B7）分隔效果声明，违反了"不使用奇怪符号"原则。修正为使用 `with` 关键字：

```x
// 旧语法（违反原则 16）
function readFile(path: String) -> String · IO, Throws<FileNotFound>

// 新语法（符合原则 16）
function readFile(path: String) -> String with IO, Throws<FileNotFound>
```

`with` 是一个自然英语单词，含义明确："这个函数返回 String，**带有** IO 和 Throws 效果"。

### 路径分隔符的符号选择

`::` 作为路径分隔符不如 `.` 直观，X 统一使用 `.` 作为路径分隔：

```x
// X：用 . 分隔，一致且直观
import std.collections.HashMap
module myapp.utils

// 对比 Rust：:: 需要额外学习
// use std::collections::HashMap
```

**设计准则**：符号应该是"看一眼就懂"的。如果需要解释，就用英文单词代替。键盘上的每个符号都很宝贵——只用最常见、最直观的那些。

---

## 附录：设计原则速查

| # | 原则 | 一句话描述 |
|---|------|-----------|
| 1 | 通用性 | 一门语言，从系统到应用 |
| 2 | 类型安全 | 编译通过 ≈ 无类型错误 |
| 3 | 内存安全 | Perceus：无 GC、无手动管理、无泄漏 |
| 4 | 多范式 | FP + OOP + 过程式 + 声明式，按需选择 |
| 5 | 博采众长 | 站在 Python/Rust/Go/Kotlin/Haskell/... 的肩膀上 |
| 6 | HM 推断 | 少写类型，多靠推断 |
| 7 | 默认不可变 | `let` 不可变，`let mutable` 可变，显式选择 |
| 8 | 多种并发 | goroutine + Actor + async/await，按需选择 |
| 9 | 完整工具链 | `x` CLI 1:1 对标 Cargo |
| 10 | 效果系统 | 副作用在类型中可见 |
| 11 | C FFI | 与 C 零开销互操作 |
| 12 | 多后端 | C / JVM / .NET，一次编写，多处运行 |
| 13 | 无异常 | `Option` + `Result` + `?` 运算符，错误即值 |
| 14 | 关键字全称 | 英文全称，含义自明，不缩写 |
| 15 | 可读性第一 | 写一次读百次，可读性永远胜出 |
| 16 | 不用奇怪符号 | 只用常见符号，看一眼就懂 |

代码是AI写的，要求清晰无误像Ada一样。但也不是必须，如果人来写代码或者修改，可以省略一些非必要的东西；编译器有个预编译阶段自动补齐。

---

*本文档由 X 语言核心团队维护，任何语言设计决策的最终裁决依据。*
