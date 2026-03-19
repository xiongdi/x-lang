# 第4章 错误处理

## 4.1 概述

X 语言采用基于类型的错误处理机制，通过 `Option<T>` 和 `Result<T, E>` 类型来显式表示可能失败的操作，替代传统的异常机制。这种设计使得错误处理成为代码的一部分，而不是通过控制流的异常跳转来处理。

核心设计原则：
- **显式错误**：函数签名明确表明可能的错误类型
- **错误传播**：通过 `?` 运算符简洁地传播错误
- **类型安全**：编译器确保所有错误都被处理
- **组合性**：通过组合子和模式匹配处理错误

## 4.2 Option 类型

`Option<T>` 类型用于表示可能存在或不存在的值，是处理空值的安全方式。

### 语法定义

```
Option<T> ::= Some(T) | None
```

### 类型规则

```
Γ ⊢ e : T
────────────
Γ ⊢ Some(e) : Option<T>

────────────
Γ ⊢ None : Option<T> for any T
```

### 基本操作

| 方法 | 签名 | 描述 |
|------|------|------|
| `is_some` | `Option<T> -> Boolean` | 检查是否为 `Some` |
| `is_none` | `Option<T> -> Boolean` | 检查是否为 `None` |
| `unwrap` | `Option<T> -> T` | 提取值，`None` 时会 panic |
| `unwrap_or` | `Option<T> -> T -> T` | 提取值，`None` 时返回默认值 |
| `map` | `Option<T> -> (T -> U) -> Option<U>` | 转换内部值 |
| `and_then` | `Option<T> -> (T -> Option<U>) -> Option<U>` | 链式操作 |

### 使用示例

```x
let maybe_name: Option<String> = Some("Alice")
let empty: Option<String> = None

// 模式匹配
let length = match maybe_name {
    Some(name) => name.length()
    None => 0
}

// 方法调用
let safe_length = maybe_name.map(function(s) => s.length()).unwrap_or(0)

// 默认值
let display_name = maybe_name.unwrap_or("anonymous")
```

## 4.3 Result 类型

`Result<T, E>` 类型用于表示可能成功或失败的操作，包含成功值或错误信息。

### 语法定义

```
Result<T, E> ::= Ok(T) | Err(E)
```

### 类型规则

```
Γ ⊢ e : T
──────────────
Γ ⊢ Ok(e) : Result<T, E> for any E

Γ ⊢ e : E
──────────────
Γ ⊢ Err(e) : Result<T, E> for any T
```

### 基本操作

| 方法 | 签名 | 描述 |
|------|------|------|
| `is_ok` | `Result<T, E> -> Boolean` | 检查是否为 `Ok` |
| `is_err` | `Result<T, E> -> Boolean` | 检查是否为 `Err` |
| `unwrap` | `Result<T, E> -> T` | 提取成功值，`Err` 时会 panic |
| `unwrap_err` | `Result<T, E> -> E` | 提取错误值，`Ok` 时会 panic |
| `unwrap_or` | `Result<T, E> -> T -> T` | 提取成功值，`Err` 时返回默认值 |
| `map` | `Result<T, E> -> (T -> U) -> Result<U, E>` | 转换成功值 |
| `map_err` | `Result<T, E> -> (E -> F) -> Result<T, F>` | 转换错误值 |
| `and_then` | `Result<T, E> -> (T -> Result<U, E>) -> Result<U, E>` | 链式操作 |

### 使用示例

```x
function divide(a: Float, b: Float) -> Result<Float, String> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

// 模式匹配
let result = divide(10.0, 2.0)
match result {
    Ok(value) => print("Result: ${value}")
    Err(error) => print("Error: ${error}")
}

// 方法调用
let safe_result = divide(10.0, 0.0).unwrap_or(0.0)
let mapped = divide(10.0, 2.0).map(function(x) => x * 2)
```

## 4.4 错误传播

X 提供了简洁的错误传播语法，通过 `?` 运算符自动处理错误。

### `?` 运算符

`?` 运算符用于传播 `Result<T, E>` 或 `Option<T>` 类型的错误：
- 对于 `Result<T, E>`：如果是 `Ok(x)`，返回 `x`；如果是 `Err(e)`，立即从当前函数返回 `Err(e)`
- 对于 `Option<T>`：如果是 `Some(x)`，返回 `x`；如果是 `None`，立即从当前函数返回 `None`

### 类型规则

```
Γ ⊢ e : Result<T, E>
当前函数返回类型为 Result<_, E>
────────────────────────────────
Γ ⊢ e? : T

Γ ⊢ e : Option<T>
当前函数返回类型为 Option<_>
──────────────────────────────
Γ ⊢ e? : T
```

### 使用示例

```x
function read_and_parse_file(path: String) -> Result<Config, IoError> {
    let content = read_file(path)? // 传播 IoError
    let parsed = parse_config(content)? // 传播 ParseError（需与 IoError 兼容）
    Ok(parsed)
}

function find_user(id: Integer) -> Option<User> {
    let users = get_users()? // 传播 None
    users.find(function(user) => user.id == id)
}
```

### 错误类型转换

当需要在不同错误类型之间转换时，可以使用 `map_err` 方法：

```x
function process() -> Result<(), AppError> {
    let result = read_file("data.txt") // Result<String, IoError>
        .map_err(function(err) => AppError::Io(err))?
    // 处理 result...
    Ok(())
}
```

## 4.5 可选链与默认值

X 提供了 `?.`（可选链）和 `??`（默认值）运算符，用于处理 `Option` 类型的链式访问和默认值设置。

### 可选链（`?.`）

安全地访问 `Option` 值的成员或方法：

```x
let user: Option<User> = Some(User { name: "Alice", address: Some(Address { city: "New York" }) })

let city = user?.address?.city // Option<String>
let upper_name = user?.name?.to_uppercase() // Option<String>
```

### 默认值（`??`）

当左侧为 `None` 时返回右侧的默认值：

```x
let name = user?.name ?? "anonymous"
let port = config.get("port")?.parse_integer() ?? 8080
let display = title ?? description ?? "untitled"
```

## 4.6 错误处理最佳实践

### 1. 明确错误类型

定义具体的错误类型，而不是使用通用的字符串错误：

```x
enum NetworkError {
    ConnectionFailed
    Timeout
    InvalidResponse
}

function fetch_data(url: String) -> Result<Data, NetworkError> {
    // 实现...
}
```

### 2. 合理使用 `?` 运算符

对于不需要特殊处理的错误，使用 `?` 传播；对于需要特殊处理的错误，使用模式匹配：

```x
function process_file(path: String) -> Result<(), AppError> {
    // 简单传播
    let content = read_file(path)?
    
    // 需要特殊处理的错误
    match parse_content(content) {
        Ok(data) => {
            // 处理数据
            Ok(())
        }
        Err(ParseError::InvalidFormat) => {
            // 特殊处理格式错误
            Err(AppError::InvalidFormat)
        }
        Err(e) => {
            // 其他错误传播
            Err(AppError::Parse(e))
        }
    }
}
```

### 3. 提供有意义的错误信息

在错误类型中包含足够的上下文信息：

```x
enum FileError {
    NotFound(String), // 包含文件名
    PermissionDenied(String, String), // 包含文件名和权限
    ReadError(String, IoError), // 包含文件名和底层错误
}
```

### 4. 使用组合子处理错误

对于复杂的错误处理流程，使用 `map`、`and_then` 等组合子：

```x
function get_user_email(id: Integer) -> Result<String, AppError> {
    find_user(id)
        .ok_or(AppError::UserNotFound(id))
        .and_then(function(user) {
            user.email
                .ok_or(AppError::NoEmail(id))
        })
}
```

### 5. 避免过度使用 `unwrap`

`unwrap` 会在错误时 panic，只在确定值存在时使用：

```x
// 好的做法：使用模式匹配
match maybe_value {
    Some(value) => process(value),
    None => handle_error()
}

// 不好的做法：可能 panic
let value = maybe_value.unwrap()
```

### 6. 错误处理的层级

在应用的不同层级采用不同的错误处理策略：
- **底层**：使用具体的错误类型，保留详细信息
- **中层**：转换和组合错误，添加上下文
- **顶层**：处理和展示错误给用户

## 4.7 与其他语言的比较

| 特性 | X 语言 | 传统异常 | Go | Rust |
|------|--------|----------|----|------|
| 错误表示 | 类型化（`Result`/`Option`） | 异常对象 | 多返回值 `(T, error)` | 类型化（`Result`/`Option`） |
| 错误传播 | `?` 运算符 | `throw`/`catch` | 显式检查和返回 | `?` 运算符 |
| 编译时检查 | 强制处理 | 可选捕获 | 手动检查 | 强制处理 |
| 性能 | 零开销（无运行时异常） | 可能有性能开销 | 零开销 | 零开销 |

## 4.8 示例：完整的错误处理流程

```x
// 错误类型定义
enum AppError {
    Io(IoError),
    Parse(ParseError),
    Validation(String),
    UserNotFound(Integer),
}

// 读取配置文件
function load_config(path: String) -> Result<Config, AppError> {
    let content = read_file(path)
        .map_err(function(err) => AppError::Io(err))?
    
    let config = parse_config(content)
        .map_err(function(err) => AppError::Parse(err))?
    
    // 验证配置
    if config.port < 1 || config.port > 65535 {
        return Err(AppError::Validation("Invalid port number"))
    }
    
    Ok(config)
}

// 处理用户请求
function handle_request(req: Request) -> Result<Response, AppError> {
    let user_id = req.params.get("id")?
        .parse_integer()
        .map_err(function(_) => AppError::Validation("Invalid user ID"))?
    
    let user = find_user(user_id)
        .ok_or(AppError::UserNotFound(user_id))?
    
    Ok(Response::json(user))
}

// 顶层错误处理
function main() {
    match handle_request(request) {
        Ok(response) => send_response(response),
        Err(error) => {
            match error {
                AppError::Io(err) => {
                    log_error("IO error: ${err}")
                    send_error(500, "Internal server error")
                }
                AppError::UserNotFound(id) => {
                    send_error(404, "User ${id} not found")
                }
                _ => {
                    log_error("Error: ${error}")
                    send_error(400, "Bad request")
                }
            }
        }
    }
}
```

**本章介绍了 X 语言的错误处理机制，包括 `Option<T>` 和 `Result<T, E>` 类型、错误传播运算符以及最佳实践。这种基于类型的错误处理方式提供了类型安全、显式和可组合的错误处理能力，使代码更加健壮和可维护。**