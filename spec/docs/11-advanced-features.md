# 第11章 高级特性

## 11.1 概述

X 语言提供了丰富的高级特性，使开发者能够编写更灵活、更高效的代码。这些特性包括元编程、编译期计算、反射和外部函数接口（FFI）。

```
Advanced Features: Powerful capabilities that enable flexible and efficient programming beyond basic language constructs.
```

X 的高级特性包括：

| 特性 | 机制 | 用途 |
|------|------|------|
| 元编程 | `const`、`const function`、泛型、宏 | 编译期计算与代码生成 |
| 反射 | 类型反射、值反射 | 运行时类型信息与操作 |
| FFI | `foreign` 关键字、C 类型映射 | 与其他语言互操作 |

---

## 11.2 元编程

元编程允许程序在编译期进行计算与决策，从而减少运行时开销并在编译期捕获错误。

### 11.2.1 编译期常量

#### 语法

```
ConstantDeclaration ::= 'const' Identifier ':' Type '=' ConstExpression ';'

ConstExpression ::= ConstLiteral
                  | ConstIdentifier
                  | ConstUnaryOp ConstExpression
                  | ConstExpression ConstBinaryOp ConstExpression
                  | ConstFunctionCall
```

常量在编译期求值，结果直接嵌入到生成的代码中。常量必须有显式类型注解，且只能用常量表达式初始化。

#### 示例

```x
const MAX_BUFFER_SIZE: Integer = 1024
const PI: Float = 3.14159265358979
const APP_NAME: String = "MyApp"
const DEBUG: Boolean = false
const MASK: Integer = 0xFF00 | 0x00FF
```

### 11.2.2 常量函数

#### 语法

```
ConstFunctionDeclaration ::= 'const' 'function' Identifier TypeParameters? Parameters '->' Type FunctionBody

FunctionBody ::= '=' ConstExpression ';'
               | Block   // block may only contain const-evaluable statements
```

`const function` 可在编译期被调用。函数体仅允许常量表达式、其他 `const function` 调用和受限控制流（常量条件分支、常量循环）。

#### 示例

```x
const function factorial(n: Integer) -> Integer {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

const function max(a: Integer, b: Integer) -> Integer {
    if a > b { a } else { b }
}

const function is_power_of_two(n: Integer) -> Boolean {
    n > 0 && (n & (n - 1)) == 0
}

const FACT_10: Integer = factorial(10)
const BUFFER: Integer = max(256, MAX_BUFFER_SIZE)
```

### 11.2.3 泛型与类型参数

#### 语法

```
GenericDeclaration ::= Identifier '<' TypeParameter (',' TypeParameter)* '>'

TypeParameter ::= Identifier
                | Identifier ':' TraitBound ('+' TraitBound)*
                | 'const' Identifier ':' Type
```

#### 泛型函数

类型参数使函数可以对任意类型工作，同时保持类型安全。

```x
function identity<T>(value: T) -> T {
    value
}

function swap<T>(pair: (T, T)) -> (T, T) {
    let (a, b) = pair
    (b, a)
}

function first<A, B>(pair: (A, B)) -> A {
    let (a, _) = pair
    a
}
```

#### 约束泛型

通过 trait bound 约束类型参数：

```x
function largest<T: Comparable>(list: List<T>) -> Option<T> {
    match list {
        []       => None
        [single] => Some(single)
        [head, ..tail] => {
            match largest(tail) {
                Some(t) if t > head => Some(t)
                _                   => Some(head)
            }
        }
    }
}

function printAll<T: Display>(items: List<T>) -> () with IO {
    for item in items {
        println("${item}")
    }
}
```

#### 常量泛型

用编译期常量参数化类型或函数：

```x
class FixedArray<T, const N: Integer> {
    private let data: Array<T>

    public function length() -> Integer {
        N
    }
}

function dot<const N: Integer>(a: FixedArray<Float, N>, b: FixedArray<Float, N>) -> Float {
    let mutable sum = 0.0
    for i in 0..N {
        sum = sum + a[i] * b[i]
    }
    sum
}
```

### 11.2.4 静态断言

#### 语法

```
StaticAssert ::= 'static_assert' '(' ConstExpression (',' StringLiteral)? ')' ';'
```

`static_assert` 在编译期求值条件表达式。若为 `false`，编译失败并输出可选的错误消息。

#### 示例

```x
const MAX: Integer = 1024
static_assert(MAX > 0, "MAX must be positive");
static_assert(is_power_of_two(MAX), "MAX must be a power of two");

const CACHE_LINE: Integer = 64
static_assert(CACHE_LINE >= 32, "cache line too small");
```

### 11.2.5 编译期条件

#### 语法

```
CompileTimeCondition ::= '#if' ConstExpression Block
                       | '#if' ConstExpression Block '#else' Block
                       | '#if' ConstExpression Block ('#elseif' ConstExpression Block)* ('#else' Block)?
```

`#if` 在编译期求值条件表达式，仅保留满足条件的分支参与编译。未选中的分支不进行类型检查，也不生成代码。

#### 示例

```x
const PLATFORM: String = "windows"

#if PLATFORM == "windows" {
    function lineEnding() -> String { "\r\n" }
} #elseif PLATFORM == "unix" {
    function lineEnding() -> String { "\n" }
} #else {
    function lineEnding() -> String { "\n" }
}

#if DEBUG {
    function log(msg: String) -> () with IO {
        println("[DEBUG] ${msg}")
    }
} #else {
    function log(msg: String) -> () {
        // no-op in release
    }
}
```

### 11.2.6 类型级编程

#### 类型作为值

```x
const INT_SIZE: Integer = sizeof(Integer)
const PTR_ALIGN: Integer = alignof(Pointer<Integer>)

static_assert(sizeof(MyStruct) <= 64, "MyStruct too large for cache line");
```

#### where 约束

```x
function serialize<T>(value: T) -> String
    where T: Serialize
{
    value.to_string()
}

function zip_map<A, B, C>(
    list_a: List<A>,
    list_b: List<B>,
    f: (A, B) -> C
) -> List<C>
    where A: Clone, B: Clone
{
    // ...
}
```

### 11.2.7 宏与代码生成

#### 宏调用

宏在编译管线的语法/词法层面进行代码变换。宏是泛型和 `const function` 的补充——仅在类型系统和常量计算无法表达的场景下使用。

```x
// 宏调用（假设 vec! 宏已定义）
let numbers = vec![1, 2, 3, 4, 5]

// derive 宏（通过属性触发）
@derive(Debug, Clone, Serialize)
class Point {
    let x: Float
    let y: Float
}
```

---

## 11.3 反射

反射是指程序在运行时获取和操作类型信息的能力。X 语言提供了一套完整的反射 API，使开发者能够在运行时检查类型、创建实例、调用方法等。

### 11.3.1 概述

反射在以下场景特别有用：
- 序列化和反序列化
- 依赖注入
- 动态类型检查
- 运行时代码生成

### 11.3.2 类型反射

类型反射允许程序在运行时获取类型的元数据，如名称、字段、方法等。

```x
import std.reflect

function inspect_type<T>(value: T) -> () with IO {
    let type_info = reflect::type_of(value)
    println("Type name: ${type_info.name}")
    println("Is primitive: ${type_info.is_primitive}")
    println("Size: ${type_info.size}")
}

inspect_type(42)  // 检查整数类型
inspect_type("hello")  // 检查字符串类型
```

### 11.3.3 值反射

值反射允许程序在运行时操作值，如获取和设置字段值、调用方法等。

```x
import std.reflect

class Person {
    let name: String
    let age: Integer
    
    public function new(name: String, age: Integer) -> Person {
        Person { name: name, age: age }
    }
    
    public function greet() -> String {
        "Hello, my name is ${name}"
    }
}

function reflect_person() -> () with IO {
    let person = Person.new("Alice", 30)
    let value_info = reflect::value_of(person)
    
    // 获取字段值
    let name = value_info.get_field("name")
    let age = value_info.get_field("age")
    println("Name: ${name}")
    println("Age: ${age}")
    
    // 调用方法
    let greet_result = value_info.call_method("greet", [])
    println("Greeting: ${greet_result}")
    
    // 设置字段值
    value_info.set_field("age", 31)
    let updated_age = value_info.get_field("age")
    println("Updated age: ${updated_age}")
}
```

### 11.3.4 反射 API

X 语言的反射 API 提供了以下核心功能：

| API | 描述 | 示例 |
|-----|------|------|
| `reflect::type_of<T>(value: T)` | 获取值的类型信息 | `let type_info = reflect::type_of(42)` |
| `reflect::value_of<T>(value: T)` | 获取值的反射表示 | `let value_info = reflect::value_of(person)` |
| `TypeInfo.name` | 获取类型名称 | `type_info.name` |
| `TypeInfo.fields` | 获取类型的字段信息 | `type_info.fields` |
| `TypeInfo.methods` | 获取类型的方法信息 | `type_info.methods` |
| `ValueInfo.get_field(name: String)` | 获取字段值 | `value_info.get_field("name")` |
| `ValueInfo.set_field(name: String, value: Any)` | 设置字段值 | `value_info.set_field("age", 31)` |
| `ValueInfo.call_method(name: String, args: List<Any>)` | 调用方法 | `value_info.call_method("greet", [])` |

---

## 11.4 FFI (外部函数接口)

X 语言提供了与 C 语言的零开销外部函数接口（FFI），使开发者能够调用 C 库函数和与 C 代码互操作。

### 11.4.1 概述

FFI 在以下场景特别有用：
- 调用操作系统 API
- 使用现有的 C 库
- 与其他语言编写的代码集成

### 11.4.2 调用 C 函数

要调用 C 函数，需要使用 `foreign` 关键字声明外部函数，并在 `unsafe` 块中调用它们。

```x
// 声明外部 C 函数
foreign function printf(format: CString, ...) -> CInt
foreign function malloc(size: CSize) -> Pointer<Void>
foreign function free(ptr: Pointer<Void>) -> ()

function use_c_functions() -> () {
    unsafe {
        // 调用 printf
        let message = c_string("Hello from C!\n")
        printf(message)
        
        // 调用 malloc 和 free
        let ptr = malloc(1024)
        // 使用指针...
        free(ptr)
    }
}
```

### 11.4.3 C 类型映射

X 语言提供了与 C 类型对应的类型，用于 FFI 调用：

| C 类型 | X 类型 | 描述 |
|--------|--------|------|
| `int` | `CInt` | 整型 |
| `long` | `CLong` | 长整型 |
| `float` | `CFloat` | 单精度浮点型 |
| `double` | `CDouble` | 双精度浮点型 |
| `char` | `CChar` | 字符型 |
| `void*` | `Pointer<Void>` | 通用指针 |
| `char*` | `CString` | C 字符串 |
| `size_t` | `CSize` | 大小类型 |

### 11.4.4 安全边界

FFI 调用位于 `unsafe` 块中，这是因为：
- C 函数可能不遵循 X 的内存安全规则
- C 函数可能返回无效指针
- C 函数可能有未定义行为

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

### 11.4.5 生成 C 头文件

X 编译器可以生成 C 头文件，使 C 代码能够调用 X 函数：

```x
// 在 X 代码中定义可从 C 调用的函数
@export
function x_add(a: CInt, b: CInt) -> CInt {
    a + b
}

@export
function x_greet(name: CString) -> CString {
    c_string("Hello, " + string_from_c_string(name) + "!")
}
```

编译时使用 `--generate-header` 选项生成头文件：

```bash
x build --generate-header
```

生成的头文件可以被 C 代码包含和使用：

```c
#include "x_exports.h"

int main() {
    int result = x_add(1, 2);
    const char* greeting = x_greet("World");
    printf("%d\n", result);
    printf("%s\n", greeting);
    return 0;
}
```

---

## 11.5 与其他章节的关系

| 章节 | 关系说明 |
|------|----------|
| 第2章 类型系统 | 泛型基于类型系统；反射扩展了类型系统的运行时能力 |
| 第5章 函数 | `const function` 是函数的编译期子集；FFI 函数是外部函数的声明 |
| 第8章 模块 | 常量、泛型和反射 API 的可见性遵循模块系统规则 |
| 第9章 模式匹配 | 反射可以与模式匹配结合使用，实现动态类型检查 |
| 第10章 内存管理 | FFI 需要手动管理从 C 分配的内存；反射操作可能涉及内存分配 |

---

## 11.6 小结

| 特性 | 关键字 / 语法 | 说明 |
|------|-------------|------|
| 编译期常量 | `const MAX: Integer = 1024` | 编译期求值并嵌入 |
| 常量函数 | `const function f(...) -> T` | 可在编译期调用的函数 |
| 泛型 | `function f<T>(x: T) -> T` | 参数多态，类型安全的代码复用 |
| 常量泛型 | `class Arr<T, const N: Integer>` | 编译期常量参数化类型 |
| 静态断言 | `static_assert(expr, msg)` | 编译期不变量检查 |
| 编译期条件 | `#if` / `#elseif` / `#else` | 条件编译，未选中分支不编译 |
| 类型查询 | `sizeof(T)` / `alignof(T)` | 编译期类型布局信息 |
| 宏 | `name!(...)` | 语法层面代码变换 |
| 反射 | `reflect::type_of` / `reflect::value_of` | 运行时类型信息与操作 |
| FFI | `foreign function` / `unsafe` | 与 C 语言互操作 |

---

**本章介绍了 X 语言的高级特性，包括元编程、编译期计算、反射和 FFI。这些特性使 X 语言更加灵活和强大，能够满足各种复杂场景的需求。元编程和编译期计算可以在编译期完成尽可能多的工作，减少运行时开销；反射提供了运行时类型信息和操作能力；FFI 则使 X 语言能够与其他语言特别是 C 语言无缝集成。**