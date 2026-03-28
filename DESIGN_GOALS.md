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
- **无 null**：用 `Optional<T>` 代替 null，编译器强制处理
- **无异常**：用 `Result<T, E>` 代替异常，错误路径显式可见
- **穷尽匹配**：`match` 模式匹配必须覆盖所有情况

**设计准则**：如果编译通过，程序就不应出现类型错误。

---

## 3. 内存安全

X 采用与 **Koka** 相同的 **Perceus** 算法实现内存管理：

- **编译时引用计数**：编译器在编译期精确插入 `dup`（引用复制）和 `drop`（引用释放）
- **重用分析（Reuse Analysis）**：当值的引用计数为 1 时，函数式的「创建新值」可优化为原地更新
- **无垃圾回收器（No GC）**：无 stop-the-world 停顿，延迟确定
- **无手动管理**：开发者不需要手写 `malloc`/`free`，不需要标注生命周期
- **线程安全**：引用计数操作在必要时为原子操作

**设计准则**：安全的内存管理不应以牺牲性能或开发体验为代价。

---

## 4. 多范式支持

X 支持**函数式（FP）**、**面向对象（OOP）**、**过程式**与**声明式**的融合：

| 范式     | 核心特性 |
|----------|----------|
| 函数式   | 纯函数、不可变数据、高阶函数、管道 `\|>`、模式匹配 |
| 面向对象 | 类、继承、接口/trait、方法链、封装 |
| 过程式   | 可变变量 `let mutable`、`for`/`while` 循环、顺序执行 |
| 声明式   | `where`/`sort by` 查询语法、表达式导向 |

**设计准则**：为问题选择最合适的范式，而非强迫开发者适应单一范式。

---

## 5. 参考语言

X 的设计广泛参考以下语言的优秀实践：

| 参考语言   | 借鉴领域 |
|------------|----------|
| Python     | 可读性、简洁语法、开发体验 |
| Rust       | 所有权与安全性、模式匹配、枚举、`Result`/`Option`、cargo 工具链 |
| Go         | goroutine 并发模型、简洁性、快速编译 |
| Kotlin     | 表达式导向、空安全、扩展函数、协程 |
| TypeScript | 结构化类型、渐进类型化、联合类型 |
| Swift      | 值类型、可选类型、协议导向编程 |
| Haskell    | HM 类型推断、类型类、纯函数式、Effect 系统思想 |
| Scala 3    | FP+OOP 融合、ADT、上下文参数、元编程 |
| F#         | 管道运算符、类型推断、FP-first 设计、计算表达式 |
| Zig        | 编译期计算、无隐藏控制流、手动底层控制 |
| Koka       | Perceus 内存管理、代数效果系统 |

---

## 6. Hindley-Milner 类型推断

X 的类型推断基于 **Hindley-Milner（HM）算法**及其扩展：

- **局部类型推断**：函数体内几乎不需要类型注解
- **顶层签名推荐但可选**：公共 API 建议写类型签名以提升可读性
- **双向类型检查**：结合自上而下（期望类型）与自下而上（推断类型）
- **约束求解**：支持类型类约束、子类型约束、效果约束的统一求解

---

## 7. 默认不可变与值/引用区分

X 中所有绑定**默认不可变**；类型命名区分**值类型**与**引用类型**：

- **默认不可变**：`let` 不可变，`let mutable` 显式可变；有利于推理和 Perceus 重用分析
- **值类型（小写）**：`integer`、`float`、`boolean`、`string`、`character` 等为值类型，用于绝大多数计算
- **引用类型（大写）**：`Integer`、`Float`、`Boolean`、`String` 等为引用类型，用于需要对象语义或统一容器的场景
- **编译期常量**：`const` 声明的绑定在编译期即确定，不可修改，便于优化与语义清晰

**设计准则**：可变性与「值 vs 引用」均为显式选择，默认安全且可推断。

---

## 8. 多种并发模型

X 提供**多种并发模型**，开发者可根据场景选择：

- **Go 风格**：轻量级协程（goroutine-like）、channel
- **Actor 模型**：消息传递、封装状态
- **Async/Await**：结构化并发、`together`/`race` 等组合原语

**设计准则**：并发应简单、安全、高效；不同场景用不同模型，而非一刀切。

---

## 9. 工具链：对标 Cargo

X 的工具链 **`x`** 在功能上**对标 Rust 的 Cargo**：

- **核心命令**：`x new`、`x build`、`x run`、`x test`、`x check`、`x fmt`、`x lint`、`x doc`、`x publish`、`x add`、`x bench`、`x install`
- **项目清单**：`x.toml`（对标 `Cargo.toml`）；锁文件 `x.lock`；包仓库；工作空间支持 monorepo

**设计准则**：开箱即用，与主流生态心智模型一致。

---

## 10. Effect System（效果系统）

X 拥有**代数效果系统**，函数的副作用在类型签名中显式声明：

- **声明效果**：`(参数) -> 返回值 requires 效果`，函数签名使用 `requires` 声明所需效果
- **调用效果**：`needs Effect.operation()`，使用 `needs` 调用效果操作
- **处理效果**：`given Effect { ... }`，使用 `given` 提供效果处理器
- **纯函数**：无 `requires` 注解即表示无副作用
- **核心效果**：`IO`、`Async`、`State<S>`、`Throws<E>`、`NonDet`；支持用户自定义 effect
- **效果多态与推断**：泛型可对效果参数化；编译器可推断效果集

**设计准则**：副作用必须可见，才能被理解和控制。

---

## 11. C FFI（C 语言外部函数接口）

X 语言提供 **C 语言外部函数接口（FFI）**，允许直接调用 C 库函数和访问 C 数据结构：

### 外部函数声明

使用 `extern` 关键字声明外部 C 函数：

```x
// 声明外部 C 函数
extern function printf(format: string, ...args: any) -> integer
extern function malloc(size: integer) -> Pointer<unsigned 8-bit integer>
extern function free(ptr: Pointer<unsigned 8-bit integer>) -> Unit
extern function strlen(s: string) -> integer
```

### 外部类型

声明和使用 C 兼容的类型：

```x
// C 结构体映射
extern record CStat {
    st_dev: unsigned 64-bit integer
    st_ino: unsigned 64-bit integer
    st_mode: unsigned 32-bit integer
    st_nlink: unsigned 32-bit integer
    st_uid: unsigned 32-bit integer
    st_gid: unsigned 32-bit integer
    st_size: signed 64-bit integer
}

// 使用 C 类型
extern function stat(path: string, buf: Pointer<CStat>) -> integer
```

### 外部常量

```x
// C 常量
extern const O_RDONLY: integer = 0
extern const O_WRONLY: integer = 1
extern const O_RDWR: integer = 2
extern const O_CREAT: integer = 64
extern const O_TRUNC: integer = 512
```

### 调用约定

- **C 调用约定**：默认使用 C 调用约定（cdecl）
- **类型映射**：X 类型自动映射到 C 兼容表示
- **内存布局**：`extern record` 的布局与 C 结构体兼容

### 使用场景

1. **系统库调用**：调用 libc、POSIX API、Win32 API
2. **第三方库**：调用现有 C 库（数据库、图像处理、加密等）
3. **性能关键代码**：对性能极端敏感的场景调用手写汇编/优化的 C 代码
4. **平台特定功能**：访问 X 标准库未封装的平台特性

### 设计准则

- **显式标记**：`extern` 明确标识非 X 代码，边界清晰
- **类型安全**：X 的类型系统仍然应用于 FFI 边界
- **unsafe 标记**：直接操作指针和内存的 C 函数标记为 `unsafe`

---

## 12. 完整标准库

X 语言拥有**完整的标准库**，提供现代编程语言所需的全套功能：

### 核心类型

标准库提供语言特性必需的核心类型：

| 类型 | 说明 |
|------|------|
| `Any` | 顶层对象类型 |
| `Unit` | 单位类型 `()` |
| `Nothing` | 底类型（永不返回）|
| `Optional<T>` | 替代 null，表示可能存在或不存在的值 |
| `Result<T, E>` | 替代异常，表示可能成功或失败的操作 |
| `List<T>` | 可变长列表 |
| `Map<K, V>` | 键值对映射 |

### 系统能力实现

底层 IO、网络和文件系统**通过 C 库实现**，不直接使用系统调用：

| 模块 | 功能 | 实现方式 |
|------|------|----------|
| `std.io` | 输入输出（`print`、`println`、`read_line`）| C 库 `printf` / `fgets` |
| `std.fs` | 文件系统操作 | C 库 `fopen`、`fread`、`fwrite` |
| `std.net` | 网络操作 | C 库 `socket`、`bind`、`listen` |
| `std.process` | 进程管理 | C 库 `fork`、`exec`、`wait` |
| `std.types` | 核心类型（`Optional`、`Result`、`List`、`Map`）| 纯 X 语言实现 |
| `std.panic` | Panic 处理和栈回溯 | C 库 `abort` + 平台特定回溯 |

### C 库封装示例

```x
// std.io - 通过 C FFI 实现
extern function puts(s: string) -> integer
extern function printf(format: string, ...) -> integer

function println(message: string) -> Unit {
    unsafe {
        puts(message)
    }
}

// std.fs - 通过 C FFI 实现
extern record FILE

extern function fopen(path: string, mode: string) -> Pointer<FILE>
extern function fread(buf: Pointer<unsigned 8-bit integer>, size: integer, count: integer, file: Pointer<FILE>) -> integer
extern function fwrite(buf: Pointer<unsigned 8-bit integer>, size: integer, count: integer, file: Pointer<FILE>) -> integer
extern function fclose(file: Pointer<FILE>) -> integer
```

### Prelude（自动导入）

所有模块自动导入 `std.prelude` 中的定义：

- **IO**: `print`、`println`、`read_line`
- **控制**: `exit`、`panic`
- **断言**: `assert`、`assert_eq`、`todo`、`unreachable`
- **构造助手**: `Some()`、`None()`、`Ok()`、`Err()`

### 设计准则

1. **完整功能**：提供现代编程语言所需的全套标准库功能
2. **C 库基础**：系统能力通过 C 标准库实现，保证兼容性和可移植性
3. **安全封装**：底层 C 调用封装为安全的 X API，错误处理通过 `Result` 类型
4. **跨平台**：标准库封装平台差异，上层代码可移植

---

## 13. 多后端架构

X 编译器采用**统一中间表示（XIR）+ 多后端**架构：

- **Zig**：XIR → Zig 源码 → Zig 编译器 → 原生二进制或 Wasm；可移植性高，适用于系统编程、嵌入式与 Web 开发，自带跨平台编译能力
- **LLVM**：XIR → LLVM IR → 原生二进制；利于深度优化
- **JavaScript**：面向浏览器与 Node.js
- **JVM**：面向 Java 生态、Android、大数据
- **.NET**：面向 CLR、C# 互操作、Windows 与游戏开发

**设计准则**：一次编写，多处运行；后端可插拔，优化在 XIR 层共享。

---

## 14. 无异常：Optional 与 Result

X **没有异常机制**。所有错误与「有或无」通过类型系统表达：

- **Optional&lt;T&gt;**：表示「有或无」；模式匹配与 `?.`、`??` 等便捷写法
- **Result&lt;T, E&gt;**：表示「成功或失败」；`?` 运算符用于错误传播，编译器强制处理
- **设计理由**：错误是值而非控制流；零成本抽象；调用者从类型签名即可知可能失败

**设计准则**：可能失败的操作必须在返回类型中体现，由编译器保证处理。

---

## 15. 关键字：英文全称，含义准确

X 的关键字**使用英文全称**，**含义自明**，不使用缩写或生造词：

- **全称不缩写**：`function` 而非 `fn`/`func`，`integer` 而非 `int`，`boolean` 而非 `bool`
- **自然英语优先**：普通英语词汇优先于编程黑话
- **路径与模块**：使用 `.` 作为路径分隔（如 `import std.collections.HashMap`），不用 `::`

**设计准则**：代码应像散文一样可读；关键字是语言的词汇表，每个词都应是真正的英语单词且含义精确。

---

## 16. 可读性第一（最高优先级）

> **Code is read far more often than it is written.**

可读性是 X 语言**最高优先级的设计约束**。当与简洁性、性能、灵活性或实现难度冲突时，**可读性胜出**。

- **一眼看懂**：合格开发者应在几秒内理解代码意图
- **只有一种显而易见的写法**：避免多种等价但风格迥异的语法
- **新手可读，专家可写**：对初学者友好，同时不限制高级用法
- **规范与实现**：语言规范、标准库 API、风格指南在冲突时以「让未来读者一眼看懂」为裁决标准；实现与优化不得破坏可读性导向的语义

**可读性检验**：大声朗读像英语？半年后能立即理解？新手能猜出含义？代码本身是否已足够清晰而无需注释解释「做了什么」？

**设计准则**：宁可多打几个字符，也不要牺牲可读性。写代码花一次时间，读代码花一百次。

---

## 17. 不使用奇怪的符号

X 的语法**只使用常见的、含义直观的符号**；若需查文档才能理解，则不应出现：

- **键盘直接可输入**：不使用需特殊输入法或 Unicode 查表才能输入的符号
- **宁可用关键字也不用符号**：当符号含义不够清晰时，用英文单词代替（如效果用 `with` 而非 `·`，路径用 `.` 而非 `::`）
- **常用符号**：算术与比较、`()` `{}` `[]`、`.` `,` `:` `;`、`->` `=>`、`|>` `?` `?.` `??`、`..` `..=`、`//` `/** */`、`@` 仅用于注解

**设计准则**：符号应「看一眼就懂」；需要解释时就用英文单词代替。

---

## 18. AI 友好与预编译补全

X 假定**大量代码将由 AI 生成或辅助编写**，因此语言与工具链应对机器可读、语义明确：

- **严谨可推导**：鼓励显式类型、清晰控制流与错误通路，避免依赖隐式习惯
- **预编译补全阶段**：在类型检查与代码生成之前，编译器可运行**预编译阶段**，自动补齐可推导的类型、契约、效果、文档等，使内部表示处于「完整而冗余」的规范形态
- **人类与 AI 同一套语法**：不区分「给 AI 写」和「给人写」两套语法；同一套语法既要便于 AI 生成，也要便于人类阅读与局部编辑

**设计准则**：对机器友好优先；源代码可简洁，内部表示须完整；AI 与人类协作对等。

---

## 附录：设计原则速查

| #  | 原则         | 一句话描述 |
|----|--------------|------------|
| 1  | 通用性       | 一门语言，从系统到应用 |
| 2  | 类型安全     | 编译通过 ≈ 无类型错误 |
| 3  | 内存安全     | Perceus：无 GC、无手动管理、无泄漏 |
| 4  | 多范式       | FP + OOP + 过程式 + 声明式，按需选择 |
| 5  | 博采众长     | 站在 Python/Rust/Go/Kotlin/Haskell/... 肩上 |
| 6  | HM 推断      | 少写类型，多靠推断 |
| 7  | 默认不可变   | `let` 不可变，`let mutable` 可变；值/引用小写/大写；`const` 编译期常量 |
| 8  | 多种并发     | goroutine + Actor + async/await，按需选择 |
| 9  | 完整工具链   | `x` CLI 对标 Cargo |
| 10 | 效果系统     | 副作用在类型中可见（`requires`/`needs`） |
| 11 | C FFI        | 提供 C 语言外部函数接口，可直接调用 C 库 |
| 12 | 完整标准库   | 全套标准库功能，系统能力通过 C 库实现 |
| 13 | 多后端       | Zig / LLVM / JS / JVM / .NET，一次编写多处运行 |
| 14 | 无异常       | `Optional` + `Result` + `?`，错误即值 |
| 15 | 关键字全称   | 英文全称，含义自明，不缩写 |
| 16 | 可读性第一   | 写一次读百次，可读性永远胜出（最高优先级） |
| 17 | 不用奇怪符号 | 只用常见符号，看一眼就懂 |
| 18 | AI 友好      | 预编译补全，机器可读，人机同一套语法 |

---

*本文档由 X 语言核心团队维护，作为任何语言设计决策的最终裁决依据。*
