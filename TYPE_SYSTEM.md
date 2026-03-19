# X 语言类型系统

本文档详细介绍 X 语言的类型系统，包括值类型与引用类型的划分、基本类型定义，以及与主流编程语言的类型映射关系。

## 目录

1. [值类型与引用类型](#1-值类型与引用类型)
2. [基本类型](#2-基本类型)
3. [指针类型与 Unsafe](#3-指针类型与-unsafe)
4. [类型映射](#4-类型映射)
   - [4.1 与 C 的类型映射](#41-与-c-的类型映射)
   - [4.2 与 Zig 的类型映射](#42-与-zig-的类型映射)
   - [4.3 与 Rust 的类型映射](#43-与-rust-的类型映射)
   - [4.4 与 JVM (Java/Kotlin) 的类型映射](#44-与-jvm-javakotlin-的类型映射)
   - [4.5 与 CLR (C#/F#) 的类型映射](#45-与-clr-cf-的类型映射)
   - [4.6 与 TypeScript 的类型映射](#46-与-typescript-的类型映射)
   - [4.7 与 Python 的类型映射](#47-与-python-的类型映射)
   - [4.8 与 Swift 的类型映射](#48-与-swift-的类型映射)

---

## 1. 值类型与引用类型

X 语言将类型分为两大类：**值类型（Value Types）** 和 **引用类型（Reference Types）**。这种划分影响数据的存储方式、复制语义和内存管理行为。

### 1.1 值类型

值类型直接存储数据本身，赋值时会发生数据复制。值类型通常分配在栈上，具有以下特点：

- **复制语义**：赋值时复制整个值
- **栈分配**：通常在栈上分配，性能高效
- **确定性释放**：离开作用域时自动释放

| 类型类别 | 值类型名称 | 描述 |
|---------|-----------|------|
| 整数 | `integer` | 任意精度整数（默认）或固定位宽整数 |
| 无符号整数 | `unsigned integer` | 无符号整数 |
| 浮点数 | `float` | 浮点数（默认 64-bit 双精度） |
| 布尔 | `boolean` | 布尔值 (`true` / `false`) |
| 字符 | `character` | 单个 Unicode 字符 |
| 指针 | `*T` | 原始指针（需 unsafe 块） |
| 元组 | `(T1, T2, ...)` | 固定长度的异构集合 |
| 记录 | `{ field: T, ... }` | 具名字段的结构 |

### 1.2 引用类型

引用类型存储对数据的引用，赋值时复制引用而非数据本身。引用类型通常分配在堆上，由 Perceus 内存管理系统自动管理生命周期。

| 类型类别 | 引用类型名称 | 描述 |
|---------|-------------|------|
| 字符串 | `String` | 堆分配的 UTF-8 字符串 |
| 列表 | `List<T>` 或 `[T]` | 动态数组 |
| 字典 | `{K: V}` | 键值对映射 |
| 类实例 | 类类型的实例 | 面向对象的对象 |
| 接口 | 接口类型 | 抽象行为约束 |
| 盒装类型 | `Integer`、`Float` 等 | 基本类型的对象封装 |

### 1.3 类型语义对比

```x
// 值类型 - 复制语义
let a: integer = 42
let b: integer = a    // b 获得独立的副本
b = 100               // a 仍然是 42

// 引用类型 - 引用语义
let s1: String = "hello"
let s2: String = s1   // s2 引用同一份数据（写时复制）
s2 = "world"          // s1 仍然是 "hello"（Copy-on-Write）
```

---

## 2. 基本类型

### 2.1 整数类型

X 语言提供任意精度整数（默认）和固定位宽整数。

#### 任意精度整数

```x
let a: integer = 42
let big: integer = 1_000_000_000_000_000_000_000
```

#### 固定位宽整数

| X 类型 | 位宽 | 范围 |
|--------|------|------|
| `signed 8-bit integer` | 8-bit | -128 到 127 |
| `signed 16-bit integer` | 16-bit | -32,768 到 32,767 |
| `signed 32-bit integer` | 32-bit | -2,147,483,648 到 2,147,483,647 |
| `signed 64-bit integer` | 64-bit | -2⁶³ 到 2⁶³-1 |
| `signed 128-bit integer` | 128-bit | -2¹²⁷ 到 2¹²⁷-1 |
| `unsigned 8-bit integer` | 8-bit | 0 到 255 |
| `unsigned 16-bit integer` | 16-bit | 0 到 65,535 |
| `unsigned 32-bit integer` | 32-bit | 0 到 4,294,967,295 |
| `unsigned 64-bit integer` | 64-bit | 0 到 2⁶⁴-1 |
| `unsigned 128-bit integer` | 128-bit | 0 到 2¹²⁸-1 |

```x
let small: signed 8-bit integer = 127
let port: unsigned 16-bit integer = 8080
let size: unsigned 64-bit integer = 1_000_000_000
```

#### 任意位宽整数（C23 `_BitInt` 兼容）

X 语言支持任意位宽整数，与 C23 的 `_BitInt(N)` 完全对应：

| X 类型 | C23 类型 | 描述 |
|--------|----------|------|
| `signed N-bit integer` | `_BitInt(N)` | N 位有符号整数 |
| `unsigned N-bit integer` | `unsigned _BitInt(N)` | N 位无符号整数 |

其中 `N` 可以是任意正整数（受限于平台实现）。

```x
// 任意位宽整数示例
let tiny: signed 7-bit integer = 42       // 恰好 7 位有符号
let odd: signed 13-bit integer = -1000    // 13 位有符号
let huge: unsigned 256-bit integer = 0    // 256 位无符号
let precise: signed 96-bit integer = 0    // 96 位有符号
```

典型应用场景：
- 硬件寄存器精确建模
- 协议字段位宽匹配
- 内存紧凑存储

#### 指针大小与标准库类型

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `size` | `size_t` | 无符号大小类型（指针大小） |
| `ptrdiff` | `ptrdiff_t` | 指针差值有符号类型 |
| `intptr` | `intptr_t` | 指针大小有符号整数 |
| `uintptr` | `uintptr_t` | 指针大小无符号整数 |

```x
let len: size = array.length()
let offset: ptrdiff = ptr2 - ptr1
let addr: uintptr = ptr as uintptr
```

### 2.2 浮点类型

X 语言支持多种精度的浮点数，分为二进制浮点和十进制浮点两类。

#### 二进制浮点

二进制浮点遵循 IEEE 754 标准，适用于科学计算、图形处理等场景。

| X 类型 | 位宽 | 描述 |
|--------|------|------|
| `float` | 64-bit（默认） | IEEE 754 双精度浮点 |
| `16-bit float` | 16-bit | IEEE 754 半精度浮点（brain float） |
| `32-bit float` | 32-bit | IEEE 754 单精度浮点 |
| `64-bit float` | 64-bit | IEEE 754 双精度浮点 |
| `128-bit float` | 128-bit | IEEE 754 四精度浮点 |
| `256-bit float` | 256-bit | 八精度浮点（扩展精度） |

#### 十进制浮点

十进制浮点适用于金融、会计等需要精确十进制表示的场景。

| X 类型 | 位宽 | 用途 |
|--------|------|------|
| `32-bit decimal` | 32-bit | 十进制浮点（7位有效数字） |
| `64-bit decimal` | 64-bit | 十进制浮点（16位有效数字） |
| `128-bit decimal` | 128-bit | 十进制浮点（34位有效数字），金融/结算场景 |

#### 扩展精度浮点

扩展精度浮点类型 `long float` 对应 C 的 `long double`，其大小由平台决定：

| X 类型 | C 类型 | 典型大小 | 描述 |
|--------|--------|----------|------|
| `long float` | `long double` | 80/96/128-bit | 扩展精度浮点（x87 扩展精度或四精度） |

```x
// 二进制浮点
let pi: float = 3.1415926535
let half: 16-bit float = 0.5
let single: 32-bit float = 1.0
let quad: 128-bit float = 3.14159265358979323846264338327950288
let ext: long float = 3.14159265358979323846L  // 扩展精度

// 十进制浮点（金融场景推荐）
let price: 64-bit decimal = 123.45
let amount: 128-bit decimal = 1_000_000_000_000.0001
```

### 2.3 布尔类型

```x
let is_active: boolean = true
let has_error: boolean = false
```

### 2.4 字符类型

```x
let ch: character = '中'
let letter: character = 'A'
let emoji: character = '🎉'
```

### 2.5 字符串类型

X 语言提供多种字符串类型以适应不同场景：

| X 类型 | 编码 | 描述 |
|--------|------|------|
| `string` | UTF-8 | 值类型，UTF-8 编码字符串 |
| `utf-8 string` | UTF-8 | C23 `char8_t*` 兼容的 UTF-8 字符串 |
| `String` | UTF-8 | 引用类型，面向对象接口 |

#### 基本字符串

```x
let greeting: string = "Hello, X"
let multi = """
多行字符串
保留格式
"""
let name = "Alice"
let msg = "Hello, {name}!"  // 插值
```

#### UTF-8 字符串（C23 兼容）

`utf-8 string` 类型与 C23 的 `char8_t*` 直接对应，适用于 FFI 场景：

```x
// UTF-8 字符串，与 C23 char8_t* 兼容
let u8s: utf-8 string = "你好世界"

// 用于 FFI
foreign function some_c_function(s: *const utf-8 character) -> Unit
```

### 2.6 字符类型

X 语言支持多种字符类型以适应不同的编码需求：

| X 类型 | 编码 | 大小 | C 类型 | 描述 |
|--------|------|------|--------|------|
| `character` | Unicode 码点 | 4 字节 | `char32_t` | 单个 Unicode 字符 |
| `utf-8 character` | UTF-8 代码单元 | 1 字节 | `char8_t` (C23) | C23 UTF-8 字符单元 |
| `utf-16 character` | UTF-16 代码单元 | 2 字节 | `char16_t` (C11) | UTF-16 字符单元 |
| `utf-32 character` | UTF-32/Unicode 码点 | 4 字节 | `char32_t` (C11) | 等同于 `character` |

```x
// Unicode 码点（4 字节）
let ch: character = '中'
let emoji: character = '🎉'

// UTF-8 代码单元（1 字节，C23 char8_t 兼容）
let c8: utf-8 character = 'A'

// UTF-16 代码单元（2 字节）
let c16: utf-16 character = '字'
```

### 2.7 复数类型

X 语言支持复数类型，与 C 的 `_Complex` 类型对应：

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `complex float` | `double _Complex` | 复数（默认双精度） |
| `complex 32-bit float` | `float _Complex` | 单精度复数 |
| `complex 64-bit float` | `double _Complex` | 双精度复数 |
| `complex 128-bit float` | `_Float128 _Complex` | 四精度复数 |
| `complex long float` | `long double _Complex` | 扩展精度复数 |

```x
// 复数类型
let z: complex float = 1.0 + 2.0i
let f: complex 32-bit float = 3.0 + 4.0i
let precise: complex 128-bit float = 1.0 + 0.0i
```

### 2.8 虚数类型

X 语言支持虚数类型，与 C 的 `_Imaginary` 类型对应：

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `imaginary float` | `double _Imaginary` | 虚数（默认双精度） |
| `imaginary 32-bit float` | `float _Imaginary` | 单精度虚数 |
| `imaginary 64-bit float` | `double _Imaginary` | 双精度虚数 |

```x
// 虚数类型
let im: imaginary float = 2.0i
let im32: imaginary 32-bit float = 3.0i
```

### 2.9 原子类型

X 语言支持原子类型，与 C11 的 `_Atomic` 对应：

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `atomic T` | `_Atomic(T)` | 原子类型 |

```x
// 原子类型 - 用于并发编程
let counter: atomic integer = 0
let flag: atomic boolean = false
let ptr: atomic *integer = null

// 原子操作
atomic fetch_add(counter, 1)
atomic compare_exchange(flag, false, true)
```

### 2.10 Unit 与 Never 类型

```x
// Unit - 只有一个值 ()
function do_something() -> Unit {
    print("done")
}

// Never - 永不返回
function panic(message: String) -> Never {
    // 程序终止
}
```

---

## 3. 指针类型与 Unsafe

X 语言使用 `*T` 语法表示原始指针类型。指针操作绕过 X 语言的安全保证，因此必须放在 `unsafe` 代码块中。

### 3.1 指针类型

| X 类型 | 描述 |
|--------|------|
| `*T` | 可变指针，指向类型 T |
| `*const T` | 常量指针，指向只读的 T |
| `Null` | 类型安全的空指针类型（C23 `nullptr_t` 兼容） |

`Null` 类型是 C23 `nullptr_t` 的对应类型，表示类型安全的空指针：

```x
// Null 类型 - 类型安全的空指针
let np: Null = null

// Null 可以赋给任何指针类型
let p: *integer = null       // 隐式转换
let cp: *const float = null  // 隐式转换
```

### 3.2 Unsafe 块

所有指针操作必须在 `unsafe` 块中进行：

```x
// 声明外部 C 函数
foreign function printf(format: *const character, ...) -> signed 32-bit integer
foreign function malloc(size: unsigned 64-bit integer) -> *Void
foreign function free(ptr: *Void) -> Unit

function use_c_functions() -> Unit {
    unsafe {
        // 指针操作必须在 unsafe 块中
        let message: *const character = "Hello from C!\n"
        printf(message)

        let ptr: *Void = malloc(1024)
        // 使用指针...
        free(ptr)
    }
}
```

### 3.3 Void 类型

`Void` 类型仅用于 FFI 场景，表示 C 的 `void` 类型：

```x
let ptr: *Void  // void* 通用指针
```

### 3.4 安全边界

`unsafe` 块将不安全代码隔离在明确标记的区域中，提醒开发者：

- 指针可能为空
- 指针可能悬垂
- 内存可能未初始化
- 可能发生数据竞争

```x
function safe_wrapper() -> Result<String, String> {
    unsafe {
        let result = c_function_that_might_fail()
        if result == null {
            Err("C function failed")
        } else {
            Ok("Success")
        }
    }
}
```

### 3.5 类型限定符

X 语言支持 C 的类型限定符，用于 FFI 场景：

| X 限定符 | C 限定符 | 描述 |
|----------|----------|------|
| `volatile` | `volatile` | 易变变量，禁止编译器优化 |
| `restrict` | `restrict` | 指针别名提示（仅用于指针参数） |

```x
// volatile - 硬件寄存器、信号处理器共享变量
let hardware_reg: volatile *unsigned 32-bit integer = 0x1000

// restrict - 承诺指针无别名，允许编译器优化
function copy(dst: *character restrict, src: *const character restrict, n: size) -> Unit

// 组合使用
let p: volatile *const unsigned 32-bit integer = null
```

`volatile` 典型应用场景：
- 内存映射 I/O
- 信号处理器共享变量
- 多线程共享标志（不推荐，应使用 `atomic`）

`restrict` 注意事项：
- 承诺该指针是该内存区域的唯一访问途径
- 违反承诺是未定义行为
- 仅用于函数参数优化提示

---

## 4. 类型映射

### 4.1 与 C 的类型映射

X 语言的类型与 C23 类型直接对应，支持零开销的 FFI 互操作。

#### 整数类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `signed 8-bit integer` | `int8_t` | 8-bit 有符号整数 |
| `signed 16-bit integer` | `int16_t` | 16-bit 有符号整数 |
| `signed 32-bit integer` | `int32_t` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `int64_t` | 64-bit 有符号整数 |
| `signed 128-bit integer` | `__int128` | 128-bit 有符号整数 |
| `unsigned 8-bit integer` | `uint8_t` | 8-bit 无符号整数 |
| `unsigned 16-bit integer` | `uint16_t` | 16-bit 无符号整数 |
| `unsigned 32-bit integer` | `uint32_t` | 32-bit 无符号整数 |
| `unsigned 64-bit integer` | `uint64_t` | 64-bit 无符号整数 |
| `unsigned 128-bit integer` | `unsigned __int128` | 128-bit 无符号整数 |
| `signed N-bit integer` | `_BitInt(N)` | C23 任意位宽整数 |
| `unsigned N-bit integer` | `unsigned _BitInt(N)` | C23 任意位宽无符号整数 |
| `size` | `size_t` | 无符号大小类型 |
| `ptrdiff` | `ptrdiff_t` | 指针差值类型 |
| `intptr` | `intptr_t` | 指针大小有符号整数 |
| `uintptr` | `uintptr_t` | 指针大小无符号整数 |

#### 浮点类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `16-bit float` | `_Float16` | 半精度浮点 |
| `32-bit float` | `float` | 单精度浮点 |
| `64-bit float` | `double` | 双精度浮点 |
| `128-bit float` | `_Float128` | 四精度浮点 |
| `256-bit float` | 自定义结构体 | 八精度浮点（需库支持） |
| `long float` | `long double` | 扩展精度浮点（平台相关） |

#### 十进制浮点类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `32-bit decimal` | `_Decimal32` | 十进制浮点（7位有效数字） |
| `64-bit decimal` | `_Decimal64` | 十进制浮点（16位有效数字） |
| `128-bit decimal` | `_Decimal128` | 十进制浮点（34位有效数字） |

#### 复数与虚数类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `complex float` | `double _Complex` | 双精度复数（默认） |
| `complex 32-bit float` | `float _Complex` | 单精度复数 |
| `complex 64-bit float` | `double _Complex` | 双精度复数 |
| `complex 128-bit float` | `_Float128 _Complex` | 四精度复数 |
| `complex long float` | `long double _Complex` | 扩展精度复数 |
| `imaginary float` | `double _Imaginary` | 双精度虚数 |
| `imaginary 32-bit float` | `float _Imaginary` | 单精度虚数 |

#### 字符与字符串类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `character` | `char32_t` | Unicode 码点（4 字节） |
| `utf-8 character` | `char8_t` | C23 UTF-8 字符单元 |
| `utf-16 character` | `char16_t` | C11 UTF-16 字符单元 |
| `utf-32 character` | `char32_t` | C11 UTF-32 字符单元 |
| `string` | `const char*` | UTF-8 字符串 |
| `utf-8 string` | `const char8_t*` | C23 UTF-8 字符串 |

#### 原子类型映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `atomic T` | `_Atomic(T)` | C11 原子类型 |

#### 其他基本类型

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `boolean` | `bool` | 布尔类型（C23 关键字） |
| `Unit` | `void` | 空返回类型 |

#### 指针与限定符映射

| X 类型 | C 类型 | 描述 |
|--------|--------|------|
| `*T` | `T*` | 可变指针 |
| `*const T` | `const T*` | 常量指针 |
| `*volatile T` | `volatile T*` | 易变指针 |
| `*restrict T` | `restrict T*` | 限制别名指针 |
| `*Void` | `void*` | 通用指针 |
| `Null` | `nullptr_t` | C23 类型安全的空指针类型 |
| `null` | `nullptr` | C23 空指针常量 |

#### FFI 示例

```x
// 声明外部 C 函数
foreign function printf(format: *const character, ...) -> signed 32-bit integer
foreign function malloc(size: unsigned 64-bit integer) -> *Void
foreign function free(ptr: *Void) -> Unit

function use_c_functions() -> Unit {
    unsafe {
        let message = "Hello from C!\n"
        printf(message)

        let ptr = malloc(1024)
        // 使用指针...
        free(ptr)
    }
}
```

---

### 4.2 与 Zig 的类型映射

Zig 是 X 语言最成熟的后端，类型映射直接且高效。

#### 基本类型映射

| X 类型 | Zig 类型 | 描述 |
|--------|----------|------|
| `integer` | `i32` | 默认 32-bit 有符号整数 |
| `signed 8-bit integer` | `i8` | 8-bit 有符号整数 |
| `signed 16-bit integer` | `i16` | 16-bit 有符号整数 |
| `signed 32-bit integer` | `i32` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `i64` | 64-bit 有符号整数 |
| `signed 128-bit integer` | `i128` | 128-bit 有符号整数 |
| `unsigned 8-bit integer` | `u8` | 8-bit 无符号整数 |
| `unsigned 16-bit integer` | `u16` | 16-bit 无符号整数 |
| `unsigned 32-bit integer` | `u32` | 32-bit 无符号整数 |
| `unsigned 64-bit integer` | `u64` | 64-bit 无符号整数 |
| `unsigned 128-bit integer` | `u128` | 128-bit 无符号整数 |
| `float` | `f64` | 默认双精度浮点 |
| `16-bit float` | `f16` | 半精度浮点 |
| `32-bit float` | `f32` | 单精度浮点 |
| `64-bit float` | `f64` | 双精度浮点 |
| `128-bit float` | `f128` | 四精度浮点 |
| `256-bit float` | 自定义结构体 | 八精度浮点（需库支持） |
| `32-bit decimal` | 自定义结构体 | 十进制浮点 |
| `64-bit decimal` | 自定义结构体 | 十进制浮点 |
| `128-bit decimal` | 自定义结构体 | 十进制浮点 |
| `boolean` | `bool` | 布尔类型 |
| `string` | `[]const u8` | UTF-8 字符串切片 |
| `character` | `u8` | 字符（UTF-8 字节） |
| `Unit` | `void` | 空类型 |
| `Never` | `noreturn` | 永不返回 |

#### 复合类型映射

| X 类型 | Zig 类型 | 描述 |
|--------|----------|------|
| `List<T>` | `[]T` | 切片 |
| `{K: V}` | `std.AutoHashMap(K, V)` | 哈希映射 |
| `{String: V}` | `std.StringHashMap(V)` | 字符串键哈希映射 |
| `Option<T>` | `?T` | 可选类型 |
| `Result<T, E>` | `E!T` | 错误联合类型 |
| `(T1, T2, ...)` | `struct { T1, T2, ... }` | 元组映射为结构体 |
| `T -> R` | `fn(T) R` | 函数类型 |

#### 指针类型映射（Zig 后端）

| X 类型 | Zig 类型 | 描述 |
|--------|----------|------|
| `*T` | `*T` | 可变指针 |
| `*const T` | `*const T` | 常量指针 |

---

### 4.3 与 Rust 的类型映射

Rust 后端生成可读的 Rust 代码，类型映射自然直接。

#### 基本类型映射

| X 类型 | Rust 类型 | 描述 |
|--------|-----------|------|
| `integer` | `i32` | 默认 32-bit 有符号整数 |
| `signed 8-bit integer` | `i8` | 8-bit 有符号整数 |
| `signed 16-bit integer` | `i16` | 16-bit 有符号整数 |
| `signed 32-bit integer` | `i32` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `i64` | 64-bit 有符号整数 |
| `signed 128-bit integer` | `i128` | 128-bit 有符号整数 |
| `unsigned 8-bit integer` | `u8` | 8-bit 无符号整数 |
| `unsigned 16-bit integer` | `u16` | 16-bit 无符号整数 |
| `unsigned 32-bit integer` | `u32` | 32-bit 无符号整数 |
| `unsigned 64-bit integer` | `u64` | 64-bit 无符号整数 |
| `unsigned 128-bit integer` | `u128` | 128-bit 无符号整数 |
| `float` | `f64` | 默认双精度浮点 |
| `16-bit float` | `f16` | 半精度浮点（需 `half` crate） |
| `32-bit float` | `f32` | 单精度浮点 |
| `64-bit float` | `f64` | 双精度浮点 |
| `128-bit float` | `f128` | 四精度浮点（需库支持） |
| `256-bit float` | 自定义结构体 | 八精度浮点（需库支持） |
| `32-bit decimal` | 自定义结构体 | 十进制浮点（需 `rust_decimal` crate） |
| `64-bit decimal` | 自定义结构体 | 十进制浮点（需库支持） |
| `128-bit decimal` | 自定义结构体 | 十进制浮点（需 `rust_decimal` crate） |
| `boolean` | `bool` | 布尔类型 |
| `string` | `String` 或 `&str` | 字符串 |
| `character` | `char` | Unicode 字符 |
| `Unit` | `()` | 单元类型 |
| `Never` | `!` | 永不返回类型 |

#### 复合类型映射

| X 类型 | Rust 类型 | 描述 |
|--------|-----------|------|
| `List<T>` | `Vec<T>` | 动态数组 |
| `{K: V}` | `HashMap<K, V>` | 哈希映射 |
| `Option<T>` | `Option<T>` | 可选类型 |
| `Result<T, E>` | `Result<T, E>` | 结果类型 |
| `(T1, T2, ...)` | `(T1, T2, ...)` | 元组 |
| `{ field: T, ... }` | `struct { field: T, ... }` | 结构体 |
| `T -> R` | `fn(T) -> R` | 函数类型 |

#### 指针类型映射

| X 类型 | Rust 类型 | 描述 |
|--------|-----------|------|
| `*T` | `*mut T` | 可变指针 |
| `*const T` | `*const T` | 常量指针 |

---

### 4.4 与 JVM (Java/Kotlin) 的类型映射

JVM 后端将 X 类型映射为 Java 基本类型和对象类型。

#### 基本类型映射

| X 类型 | Java 类型 | 描述 |
|--------|-----------|------|
| `integer` | `int` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `long` | 64-bit 有符号整数 |
| `float` | `double` | 双精度浮点 |
| `16-bit float` | — | 半精度浮点（需库支持） |
| `32-bit float` | `float` | 单精度浮点 |
| `64-bit float` | `double` | 双精度浮点 |
| `128-bit float` | — | 四精度浮点（需库支持） |
| `256-bit float` | — | 八精度浮点（需库支持） |
| `32-bit decimal` | `java.math.BigDecimal` | 十进制浮点 |
| `64-bit decimal` | `java.math.BigDecimal` | 十进制浮点 |
| `128-bit decimal` | `java.math.BigDecimal` | 十进制浮点 |
| `boolean` | `boolean` | 布尔类型 |
| `string` | `String` | 字符串 |
| `character` | `char` | 字符 |
| `Unit` | `void` | 空返回类型 |

#### 盒装类型映射

| X 类型 | Java 类型 | 描述 |
|--------|-----------|------|
| `Integer` | `Integer` | 整数对象 |
| `Float` | `Double` | 浮点数对象 |
| `Boolean` | `Boolean` | 布尔对象 |

#### 复合类型映射

| X 类型 | Java 类型 | 描述 |
|--------|-----------|------|
| `List<T>` | `java.util.List<T>` | 列表接口 |
| `{K: V}` | `java.util.Map<K, V>` | 映射接口 |
| `Option<T>` | `java.util.Optional<T>` | 可选类型 |
| `(T1, T2)` | 自定义 Pair 类 | 元组 |

---

### 4.5 与 CLR (C#/F#) 的类型映射

.NET 后端将 X 类型映射为 C# 类型。

#### 基本类型映射

| X 类型 | C# 类型 | 描述 |
|--------|---------|------|
| `integer` | `int` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `long` | 64-bit 有符号整数 |
| `unsigned 32-bit integer` | `uint` | 32-bit 无符号整数 |
| `unsigned 64-bit integer` | `ulong` | 64-bit 无符号整数 |
| `float` | `double` | 双精度浮点 |
| `16-bit float` | `Half` | 半精度浮点（.NET 5+） |
| `32-bit float` | `float` | 单精度浮点 |
| `64-bit float` | `double` | 双精度浮点 |
| `128-bit float` | — | 四精度浮点（需库支持） |
| `256-bit float` | — | 八精度浮点（需库支持） |
| `32-bit decimal` | — | 十进制浮点（需库支持） |
| `64-bit decimal` | — | 十进制浮点（需库支持） |
| `128-bit decimal` | `decimal` | 十进制浮点（28位有效数字） |
| `boolean` | `bool` | 布尔类型 |
| `string` | `string` | 字符串 |
| `character` | `char` | 字符 |
| `Unit` | `void` | 空返回类型 |

#### 复合类型映射

| X 类型 | C# 类型 | 描述 |
|--------|---------|------|
| `List<T>` | `List<T>` | 动态列表 |
| `{K: V}` | `Dictionary<K, V>` | 字典 |
| `Option<T>` | `T?` | 可空类型 |
| `Result<T, E>` | 自定义 Result 类型 | 结果类型 |

#### 指针类型映射

| X 类型 | C# 类型 | 描述 |
|--------|---------|------|
| `*T` | `IntPtr` | 通用指针（需 unsafe 上下文） |
| `*const T` | `IntPtr` | 常量指针 |

---

### 4.6 与 TypeScript 的类型映射

TypeScript 后端将 X 类型映射为 TypeScript 类型。

#### 基本类型映射

| X 类型 | TypeScript 类型 | 描述 |
|--------|-----------------|------|
| `integer` | `number` | JavaScript 数值 |
| `float` | `number` | JavaScript 数值 |
| `16-bit float` | `number` | 半精度浮点（统一为 number） |
| `32-bit float` | `number` | 单精度浮点（统一为 number） |
| `64-bit float` | `number` | 双精度浮点（统一为 number） |
| `128-bit float` | `number` | 四精度浮点（统一为 number，精度损失） |
| `256-bit float` | `number` | 八精度浮点（统一为 number，精度损失） |
| `32-bit decimal` | `number` | 十进制浮点（统一为 number，精度损失） |
| `64-bit decimal` | `number` | 十进制浮点（统一为 number，精度损失） |
| `128-bit decimal` | `number` | 十进制浮点（统一为 number，精度损失） |
| `boolean` | `boolean` | 布尔类型 |
| `string` | `string` | 字符串 |
| `character` | `string` | 字符（作为单字符字符串） |
| `Unit` | `void` | 空类型 |
| `Never` | `never` | 永不返回 |

#### 复合类型映射

| X 类型 | TypeScript 类型 | 描述 |
|--------|-----------------|------|
| `List<T>` | `T[]` | 数组 |
| `{K: V}` | `Record<K, V>` | 记录类型 |
| `Option<T>` | `T \| null` | 可空联合类型 |
| `Result<T, E>` | `T`（简化） | 结果类型 |
| `(T1, T2, ...)` | `[T1, T2, ...]` | 元组（数组形式） |
| `T -> R` | `(t: T) => R` | 函数类型 |

#### 特殊映射

| X 类型 | TypeScript 类型 | 描述 |
|--------|-----------------|------|
| `None` | `undefined` | 无值 |
| `null` | `null` | 空值 |
| `Dynamic` | `any` | 动态类型 |

---

### 4.7 与 Python 的类型映射

Python 后端利用 Python 的动态类型特性。

#### 基本类型映射

| X 类型 | Python 类型 | 类型注解 | 描述 |
|--------|-------------|----------|------|
| `integer` | `int` | `int` | 任意精度整数 |
| `float` | `float` | `float` | 双精度浮点 |
| `16-bit float` | `float` | `float` | 半精度浮点（统一为 float） |
| `32-bit float` | `float` | `float` | 单精度浮点（统一为 float） |
| `64-bit float` | `float` | `float` | 双精度浮点（统一为 float） |
| `128-bit float` | `float` | `float` | 四精度浮点（统一为 float，精度损失） |
| `256-bit float` | `float` | `float` | 八精度浮点（统一为 float，精度损失） |
| `32-bit decimal` | `decimal.Decimal` | `decimal.Decimal` | 十进制浮点 |
| `64-bit decimal` | `decimal.Decimal` | `decimal.Decimal` | 十进制浮点 |
| `128-bit decimal` | `decimal.Decimal` | `decimal.Decimal` | 十进制浮点 |
| `boolean` | `bool` | `bool` | 布尔类型 |
| `string` | `str` | `str` | Unicode 字符串 |
| `character` | `str` | `str` | 单字符字符串 |
| `Unit` | `None` | `None` | 空值 |

#### 复合类型映射

| X 类型 | Python 类型 | 类型注解 | 描述 |
|--------|-------------|----------|------|
| `List<T>` | `list` | `list[T]` | 列表 |
| `{K: V}` | `dict` | `dict[K, V]` | 字典 |
| `Option<T>` | `T \| None` | `T \| None` | 可选类型 |
| `(T1, T2, ...)` | `tuple` | `tuple[T1, T2, ...]` | 元组 |

---

### 4.8 与 Swift 的类型映射

Swift 与 X 语言在类型系统设计上有许多相似之处。

#### 基本类型映射

| X 类型 | Swift 类型 | 描述 |
|--------|-----------|------|
| `integer` | `Int` | 平台相关整数（通常 64-bit） |
| `signed 32-bit integer` | `Int32` | 32-bit 有符号整数 |
| `signed 64-bit integer` | `Int64` | 64-bit 有符号整数 |
| `unsigned 32-bit integer` | `UInt32` | 32-bit 无符号整数 |
| `unsigned 64-bit integer` | `UInt64` | 64-bit 无符号整数 |
| `float` | `Double` | 双精度浮点 |
| `16-bit float` | `Float16` | 半精度浮点（Swift 5.9+） |
| `32-bit float` | `Float` | 单精度浮点 |
| `64-bit float` | `Double` | 双精度浮点 |
| `128-bit float` | — | 四精度浮点（需库支持） |
| `256-bit float` | — | 八精度浮点（需库支持） |
| `32-bit decimal` | — | 十进制浮点（需库支持） |
| `64-bit decimal` | — | 十进制浮点（需库支持） |
| `128-bit decimal` | `Decimal` | 十进制浮点 |
| `boolean` | `Bool` | 布尔类型 |
| `string` | `String` | Unicode 字符串 |
| `character` | `Character` | Unicode 字符 |
| `Unit` | `Void` | 空类型 |
| `Never` | `Never` | 永不返回 |

#### 复合类型映射

| X 类型 | Swift 类型 | 描述 |
|--------|-----------|------|
| `List<T>` | `[T]` | 数组 |
| `{K: V}` | `[K: V]` | 字典 |
| `Option<T>` | `T?` | 可选类型 |
| `Result<T, E>` | `Result<T, E>` | 结果类型 |
| `(T1, T2, ...)` | `(T1, T2, ...)` | 元组 |
| `T -> R` | `(T) -> R` | 函数类型 |

#### 指针类型映射

| X 类型 | Swift 类型 | 描述 |
|--------|-----------|------|
| `*T` | `UnsafeMutablePointer<T>` | 可变指针 |
| `*const T` | `UnsafePointer<T>` | 常量指针 |

---

## 附录：类型映射总表

### 整数类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `integer` | `int32_t` | `i32` | `i32` | `int` | `int` | `number` | `int` | `Int` |
| `signed 64-bit integer` | `int64_t` | `i64` | `i64` | `long` | `long` | `number` | `int` | `Int64` |
| `unsigned 32-bit integer` | `uint32_t` | `u32` | `u32` | — | `uint` | `number` | `int` | `UInt32` |
| `signed N-bit integer` | `_BitInt(N)` | — | — | — | — | — | — | — |
| `unsigned N-bit integer` | `unsigned _BitInt(N)` | — | — | — | — | — | — | — |
| `size` | `size_t` | `usize` | `usize` | — | `nuint` | `number` | `int` | `UInt` |
| `ptrdiff` | `ptrdiff_t` | `isize` | `isize` | — | `nint` | `number` | `int` | `Int` |
| `intptr` | `intptr_t` | `isize` | `isize` | — | `nint` | `number` | `int` | `Int` |
| `uintptr` | `uintptr_t` | `usize` | `usize` | — | `nuint` | `number` | `int` | `UInt` |

### 浮点类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `float` | `double` | `f64` | `f64` | `double` | `double` | `number` | `float` | `Double` |
| `16-bit float` | `_Float16` | `f16` | `f16` | — | `Half` | `number` | `float` | `Float16` |
| `32-bit float` | `float` | `f32` | `f32` | `float` | `float` | `number` | `float` | `Float` |
| `64-bit float` | `double` | `f64` | `f64` | `double` | `double` | `number` | `float` | `Double` |
| `128-bit float` | `_Float128` | `f128` | `f128` | — | — | `number` | `float` | — |
| `256-bit float` | 结构体 | 结构体 | 结构体 | — | — | `number` | `float` | — |
| `long float` | `long double` | `f80` | — | — | — | `number` | `float` | — |
| `32-bit decimal` | `_Decimal32` | 结构体 | 结构体 | `BigDecimal` | — | `number` | `Decimal` | — |
| `64-bit decimal` | `_Decimal64` | 结构体 | 结构体 | `BigDecimal` | — | `number` | `Decimal` | — |
| `128-bit decimal` | `_Decimal128` | 结构体 | 结构体 | `BigDecimal` | `decimal` | `number` | `Decimal` | `Decimal` |

### 复数与虚数类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `complex float` | `double _Complex` | `std.math.Complex(f64)` | `num_complex::Complex64` | — | `Complex` | — | `complex` | — |
| `complex 32-bit float` | `float _Complex` | `std.math.Complex(f32)` | `num_complex::Complex32` | — | — | — | `complex` | — |
| `imaginary float` | `double _Imaginary` | — | — | — | — | — | — | — |

### 字符与字符串类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `character` | `char32_t` | `u21` | `char` | `char` | `char` | `string` | `str` | `Character` |
| `utf-8 character` | `char8_t` | `u8` | — | — | — | `string` | `str` | `UTF8.UnicodeScalar` |
| `utf-16 character` | `char16_t` | `u16` | — | `char` | `ushort` | `string` | `str` | `UTF16.CodeUnit` |
| `utf-32 character` | `char32_t` | `u32` | `char` | `int` | `uint` | `string` | `str` | `UnicodeScalar` |
| `string` | `char*` | `[]const u8` | `String` | `String` | `string` | `string` | `str` | `String` |
| `utf-8 string` | `char8_t*` | `[]const u8` | — | — | — | `string` | `str` | — |

### 指针与特殊类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `*T` | `T*` | `*T` | `*mut T` | — | `IntPtr` | — | — | `UnsafeMutablePointer<T>` |
| `*const T` | `const T*` | `*const T` | `*const T` | — | `IntPtr` | — | — | `UnsafePointer<T>` |
| `*volatile T` | `volatile T*` | `*volatile T` | `*volatile T` | — | — | — | — | — |
| `Null` | `nullptr_t` | `?*anyopaque` | `std::ptr::NonNull` | `null` | `null` | `null` | `None` | `nil` |
| `atomic T` | `_Atomic(T)` | 自定义 | `std::sync::atomic` | `Atomic*` | `Interlocked` | — | — | `OSAtomic` |

### 复合类型

| X 类型 | C | Zig | Rust | Java | C# | TypeScript | Python | Swift |
|--------|---|-----|------|------|----|-----------:|--------|-------|
| `List<T>` | `T*` | `[]T` | `Vec<T>` | `List<T>` | `List<T>` | `T[]` | `list[T]` | `[T]` |
| `Option<T>` | `T*` | `?T` | `Option<T>` | `Optional<T>` | `T?` | `T \| null` | `T \| None` | `T?` |
| `Result<T, E>` | `int` | `E!T` | `Result<T, E>` | — | — | `T` | — | `Result<T, E>` |

> **注意**：
> - `—` 表示该语言无直接对应类型，需要使用其他方式表达
> - `结构体` 表示需要通过第三方库或自定义结构体实现
> - 任意位宽整数 `_BitInt(N)` 目前仅 C23 支持，其他语言需库实现
> - 十进制浮点类型在不同语言中的精度支持可能有所不同
> - `long float` / `long double` 的大小由平台决定（80/96/128-bit）

---

## 参考资料

- [X 语言规范 - 类型系统](spec/docs/02-types.md)
- [X 语言规范 - 高级特性（FFI）](spec/docs/11-advanced-features.md)
- [X 语言数据类型教程](docs/src/ch02-02-data-types.md)
